/*
    Make a Rust program that converts:

        Fahrenheit → Celsius
        Celsius → Fahrenheit

        C = (F - 32) * 5 / 9
        F = (C * 9 / 5) + 32
*/

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_farhenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    let f = 98.6;
    let c = fahrenheit_to_celsius(f);
    println!("{f}°F = {c}°C");

    let c2 = 37.0;
    let f2 = celsius_to_farhenheit(c2);
    println!("{c2}°C = {f2}°F");
}
