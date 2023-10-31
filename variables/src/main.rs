<<<<<<< HEAD
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The 3h in seconds is {}", THREE_HOURS_IN_SECONDS);

    let x = 5;
    println!("THe value of x is: {}", x); 

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
=======
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The 3h in seconds is {}", THREE_HOURS_IN_SECONDS);

    let x = 5;
    println!("THe value of x is: {}", x); 

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
>>>>>>> def5ed8c33e35007c31adda8c594035b2256c8a4
