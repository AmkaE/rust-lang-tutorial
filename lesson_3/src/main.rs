fn main() {
    // implicit type assignment
    // let x = 7;

    // variable x is immutable meaning it cannot be changed
    // x = 8; // error: cannot assign twice to immutable variable `x`

    // explicit type assignment
    // let mut y: u32 = 8;

    // this fixes warning "unused variable: `y`"
    // println!("y {}", y); // without this line, the compiler will warn that y is not used

    // y is mutable, because of the "mut" keyword
    // y = 9;

    // println!("The value of x is: {} and y is {}",x, y);

    // let x = 7;

    // println!("x is: {}", x); // x is: 7

    // creating a new scope
    {
        // creating a new variable with the same name as the outer x
        // let x = 8;

        // println!("x is: {}", x); // x is: 8
    }

    // creating a new scope where x is string
    {
        // let x = "Hello";

        // println!("x is: {}", x); // x is: Hello
    }

    // this x is still the same as the first one it cannot access the x inside the scope
    // println!("x is: {}", x); // x is: 7

    // changing the value of x without using "mut" keyword,
    // because we are creating a new variable with the same name
    // let x = 10;

    // println!("x is: {}", x); // x is: 10

    // constants
    // constants are immutable and names are all uppercase with underscores between words and they must be annotated
    // example: const MAX_POINTS: u32 = 100_000;
    const MAX_POINTS: u32 = 100_000;

    println!("MAX_POINTS is: {}", MAX_POINTS); // MAX_POINTS is: 100000

    // also constant variables cannot be re-assigned or re-declared
    // const MAX_POINTS: u32 = 100_000;
    // ERROR: the name `MAX_POINTS` is defined multiple times `MAX_POINTS` must be defined only once in the value namespace of this block

    {
        // this is allowed because it is in a different scope
        const MAX_POINTS: u32 = 700_000;
        println!("MAX_POINTS is: {}", MAX_POINTS); // MAX_POINTS is: 100000
    }
}
