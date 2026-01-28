
fn double(x:i32) -> i32 { //-> implies return type 
    //return x*2; 
    {
        let y = 10; 
        x*2*y
    }
}

fn main() {


    println!("Double {} = {}", 5, double(5))

}

/*

    //let mut result : f32 = 0.0; //int
    //let x :i32 = 5; //float
    //result = result + x as f32; //no implicit conversion exists in rust!!

    //println!("{}", result )

    let mut x :i32 = 5; 
    //x = 1.012; cannot do like this

    let x :f32 = x as f32 + 1.012; 

    //println!("{}", x)

    //shadowing happens when you dont use mut 


*/
