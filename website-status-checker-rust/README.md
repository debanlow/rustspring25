# website status checker rust

A concurrent website monitoring tool written in Rust. It checks the availability and response time of multiple websites using a fixed-size worker thread pool. The program prints real-time results to the console and saves all data to a JSON file.


## Build Instructions
To compile the project in release mode for optimal performance:

```bash
cargo build --release
```

The compiled binary will be located in the `target/release` directory:

```bash
./target/release/website_checker [OPTIONS] [URL ...]
```


## Usage
You can provide URLs directly as arguments or from a file using the `--file` flag.


### Checking URLs 
```bash
cargo run -- "https://example.com" "https://google.com"
```

'''output
https://example.com is UP (status 200) in 65.365232ms
https://google.com is UP (status 200) in 129.822072ms
Done. Results saved to status.json

