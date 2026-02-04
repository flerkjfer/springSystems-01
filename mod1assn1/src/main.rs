fn fahrenheit_to_celsius(f: f64) -> f64 { //function name f:64 meaning float 64 --> return type f:64

    (f - 32.0) * 5.0 / 9.0

}

fn celsius_to_fahrenheit(c: f64) -> f64 {

    (c * 9.0 / 5.0) + 32.0

}

fn main() {

    const freezingpoint: i64 = 32; //constant

    //Declare a mutable variable example
    let mut tempFaran: f64 = 32.0; //mutable variable 
    println!("Faranheight, {},  to celsius: {}", tempFaran, fahrenheit_to_celsius(tempFaran));

    for i in 1..=5 { //inclusive range for loop using ..=
        let tempVar = tempFaran + (i as f64); //i is int so convert to f64
        println!("Faranheight, {}, to celsius: {}", tempVar, fahrenheit_to_celsius(tempVar)); 
    }


}
