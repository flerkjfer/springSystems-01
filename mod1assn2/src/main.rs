fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    }
    return false; 
}


fn main() {
    println!("Hello, assignment 2!");

    let myarr: [i32; 10] = [20, 53, 74, 56, 88, 100, 9, 10, 99, 117]; //making a basic array, notice asserted type and size 
    for currentval in myarr {
        if is_even(currentval) {
            println!("even"); 
        }
        else {
            println!("odd"); 
        }

        if currentval % 3 == 0 && currentval % 5 == 0 {
            println!("FizzBuzz"); 
        }
        else if currentval % 3 == 0 {
            println!("Fizz"); 
        }
        else if currentval % 5 == 0 {
            println!("Buzz"); 
        }
    }

    let mut x = 0; //mutable since THEY WILL CHANGE/MUTATE
    let mut sum = 0;

    while x < 10 {
        sum += myarr[x]; 
        x += 1; 
        println!("The current sum is: {}", sum);
    }

    let mut maxy = myarr[0]; 

    for index in myarr {
        if index > maxy {
            maxy = index; 
        }
    }

    println!("The greatest number is: {}", maxy); 


}

