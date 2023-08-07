fn main() {
    const N: u32 = 100_000;
    println!("The value of N is {}", N);
    
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2; // new var
        println!("The value of x is {}", x);
    }

    println!("The value of x is {}", x);
}
