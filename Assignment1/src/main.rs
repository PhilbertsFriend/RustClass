const FREEZE:f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
    let t = f;
    return (t - FREEZE) * (5.0/9.0);
}

/*
fn celsius_to_fahrenheit(c: f64) -> f64{
    let t = c;
    return (t * (9.0/5.0)) + FREEZE;
}
*/

fn main() {
    let mut x:f64 = 32.0;
    let mut i = 0.0;

    loop{
        i = i + 1.0;
        let z = x + i;
        println!("{}°F is {}°C", z, fahrenheit_to_celsius(z));
        if i == 5.0 {
            break;
        }
}      
}