fn main()
{
    println!("hello world, This is a me learning rust!");
    //notice the {} gets filled with the argument after the comma.
    println!("format {} arguments", "some");
    println!();
    types()
}

fn types()
{
    /* rust is a statically typed language. Meaning, data types, data types can be 
    inferred by by the compiler based on the value the variable is assigned
    */
    let timer_purpose_string = "To help maximize time of studying and its efficiency.";
    let time_float = 20.05; //float
    let timer_is_set = false; 
    println!("The purpose of this Timer is: {}", timer_purpose_string);
    println!("Study time for: {}", time_float);
    println!("Is the timer set?: {}", timer_is_set);
}
