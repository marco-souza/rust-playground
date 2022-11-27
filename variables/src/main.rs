fn main() {
    mut_var();
    shadowing();
}

fn shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("This value x in the inner scope is: {x}");
    }

    println!("This value x is: {x}");
}

fn mut_var() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
