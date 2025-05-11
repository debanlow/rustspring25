use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use reqwest::blocking::Client;

struct WebsiteStatus {
    url: String,
    action_status: Result<u16, String>,
    response_time: Duration,
    timestamp: SystemTime,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut urls = vec![];
    let mut workers = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let mut timeout_secs = 5;
    let mut retries = 0;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                if i + 1 >= args.len() {
                    eprintln!("Missing file path after --file");
                    std::process::exit(2);
                }
                let path = &args[i + 1];
                urls.extend(parse_urls_from_file(path));
                i += 2;
            }
            "--workers" => {
                workers = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(workers);
                i += 2;
            }
            "--timeout" => {
                timeout_secs = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(5);
                i += 2;
            }
            "--retries" => {
                retries = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(0);
                i += 2;
            }
            _ if args[i].starts_with("--") => {
                eprintln!("Unknown flag: {}", args[i]);
                std::process::exit(2);
            }
            _ => {
                urls.push(args[i].clone());
                i += 1;
            }
        }
    }

    if urls.is_empty() {
        eprintln!("Usage: website_checker [--file path] [--workers N] [--timeout S] [--retries N] [URL ...]");
        std::process::exit(2);
    }

    let (tx, rx) = mpsc::channel::<String>();
    let rx = Arc::new(Mutex::new(rx));
    let client = Arc::new(Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .expect("Failed to build HTTP client"));

    let mut handles = vec![];

    for _ in 0..workers {
        let rx = Arc::clone(&rx);
        let client = Arc::clone(&client);

        let handle = thread::spawn(move || {
            while let Ok(url) = rx.lock().unwrap().recv() {
                let status = check_website(&url, &client, retries);
                match &status.action_status {
                    Ok(code) => println!("{} is UP (status {}) in {:?}", status.url, code, status.response_time),
                    Err(e) => println!("{} is DOWN ({}) in {:?}", status.url, e, status.response_time),
                }
                save_result(&status);
            }
        });

        handles.push(handle);
    }

    for url in urls {
        let _ = tx.send(url);
    }

    drop(tx);

    for h in handles {
        h.join().unwrap();
    }

    println!("Done. Results saved to status.json");
}

fn parse_urls_from_file<P: AsRef<Path>>(path: P) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| {
            let l = line.ok()?.trim().to_string();
            if l.is_empty() || l.starts_with('#') {
                None
            } else {
                Some(l)
            }
        })
        .collect()
}

fn check_website(url: &str, client: &Client, retries: u32) -> WebsiteStatus {
    let mut attempts = 0;
    let start = Instant::now();

    let result = loop {
        match client.get(url).send() {
            Ok(resp) => break Ok(resp.status().as_u16()),
            Err(e) => {
                if attempts >= retries {
                    break Err(e.to_string());
                }
                attempts += 1;
                thread::sleep(Duration::from_millis(100));
            }
        }
    };

    WebsiteStatus {
        url: url.to_string(),
        action_status: result,
        response_time: start.elapsed(),
        timestamp: SystemTime::now(),
    }
}

fn save_result(status: &WebsiteStatus) {
    let timestamp = status.timestamp.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();

    let json_object = format!(
        r#"{{
  "url": "{}",
  "status": {},
  "response_time_ms": {},
  "timestamp_unix": {}
}}"#,
        status.url,
        match &status.action_status {
            Ok(code) => code.to_string(),
            Err(e) => format!(r#""{}""#, e),
        },
        status.response_time.as_millis(),
        timestamp
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("status.json")
        .unwrap();

    writeln!(file, "{}", json_object).unwrap();
}
