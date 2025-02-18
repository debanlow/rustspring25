const Freezing_Point : f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - Freezing_Point) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + Freezing_Point 
}

fn main() {
    let mut temp_f = 32.0;

    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{:.2}°F is equal to {:.2}°C", temp_f, temp_c);


    for i in 1..=5 {
        let new_temp_f = temp_f + i as f64;
        let new_temp_c = fahrenheit_to_celsius(new_temp_f);

        println!("{:.2}°F is equal to {:.2}°C", new_temp_f, new_temp_c);
    }
}