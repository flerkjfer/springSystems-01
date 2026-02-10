fn check_guess(guess: i32, secret: i32) -> i32 {

    if guess == secret {return 0;
        } else if guess > secret {return 1;
        } else {return -1; }

}


fn main() {
    println!("Hello, world!");
    let secret = 33; 
    let mut counter = 0; 
    let mut guess = 100; 
    let mut loopcount = 0; 


    while guess != secret { //this check if its == for us 
         if check_guess(guess, secret) == 1 {
            counter += 1; 
            loopcount += 1; 
            println!("Too high!");
            guess -= 1; 
        } else if check_guess(guess, secret) == -1 {
            counter += 1;
            loopcount += 1; 
            println!("Too low!");
            guess += 1; 
        }
    }

    if loopcount == 0 { //to acccount for an attempt exhausted by the loop
        counter += 1; 
        println!("It took you {} attempt(s)", counter); 
    }

    else{
        println!("It took you {} attempt(s)", counter); 
    }


}
