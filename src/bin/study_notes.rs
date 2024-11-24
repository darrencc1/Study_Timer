/*
This is a standalone program, which is why I am using another binary, this is simply notes and practice for the actual project and learning rust. 

cargo run
mkdir src/bin ... multiple binaries are allowed, this creates a new binary in the src folder
mv src/study_notes.rs src/bin/study_notes.rs ... This moves the file to where you want by the command prompt, you can manually move it as well.
cargo run --bin study_notes ... this runs the specific binary folder study_notes 
 
 ***NOTE*** you can directly run a file without making another binary:
 rustc src/study_notes.rs (File is not in bin for this), This compiles the file directly.
 then: ./study_notes to run the compiled binary. This does NOT use Cargos features like dependency management. 
 */



/*For more information on specifics, refer to functions in main to learn more about them.
 */
fn main()
{
    println!("hello world, This is a me learning rust!");
    //notice the {} gets filled with the argument after the comma.
    println!("format {} arguments", "some");

    types();
    integer();
    float();
    character();
    constants();
    strings();
    bitwise_op();
}

fn types()
{
    /* rust is a statically typed language. Meaning, data types, data types can be 
    inferred by by the compiler based on the value the variable is assigned
    Default: Variables are immutable(read only in Rust.) So if I try to set time_float = 25.2 after I already declared it 
    it will produce and error. Mutable variables are possible see below. 
    */

    /*Note: you can shadow variables: declare variables with same name and the last one declared will override the previous ones. You can also change the type.  */
    let timer_purpose_string = "To help maximize time of studying and its efficiency.";
    let time_float = 20.05; //float
    let timer_is_set = false; 
    println!("The purpose of this Timer is: {}", timer_purpose_string);
    println!("Study time for: {}", time_float);
    println!("Is the timer set?: {}", timer_is_set);
    let mut study_time = 25.05;
    println!("Your studytime is {}", study_time);
    study_time = 20.05;//You do not have to redeclare that it is mutable or its type. 
    println!("Your study time is {}\n", study_time);

    let name = "Chambersan";//note this is a string
    let name = name.len();//this is now usize 
    println!("name Chambersan is now and integer : {}", name);

}   
/*Scalar Types: represent a single value. 4 main, Int, Floating-point, Bool, Characters
INTEGER: Signed, pos and negative valuse ex. 8 bit - i8
INTEGER: Unsigned, Only positive values ex. 8 bit - u8
The size of an integer can be **arch**, will depend on the architecture of the machine. 
example. x64 machhine will have 64 bits, arch integers is primarily used when indexing a collection
*/
fn integer()
{
    let result = 25;//i32 by default
    let age:u32 = 33;//This will cause a compile error if you use floating-point instead.
    let sum = 15-75;
    let mark:isize = 10;
    let count:usize= 30;
    println!("result value is {}", result);
    println!("sum is {} and age is {}", sum, age);
    println!("mark is {} and count is {}\n", mark, count);
    let age:u8 = 255;

    //0 to 255 is all u8 can handle 
    //let weight:u8 = 256; //overflow value is 0
    //let height:u8 = 257; //overflow value is 1
    println!("age is {}\n", age);
}

/*Floats: f32 type is a single precision float
f64 type is a double precision. This is default
cannot put an integer in a float and vice versa!
 */
fn float()
{
    let result = 10.00; //f64 by default
    let interest:f32 = 8.35;
    let cost:f64 = 15000.600; //double precision
    println!("result value is {}", result);
    println!("interest is {}", interest);
    println!("The cost is {}", cost);

    //Number seperator just lets you see numbers better, output is the same. 
    let float_seperator = 11_000.555_001;
    println!("float value {}", float_seperator);

    let int_seperator = 50_000_000;
    println!("int value {}\n", int_seperator);
}
/*Boolean is the same for most languages, true, or false, use :bool keyword to declare a bool */

/*Character:Rust allows you to us numbers, alphabets, Uncode and special characters.
use char to declare a character data type. **use '' not "" for char**
 */
fn character()
{
    let special_char = '@';//note if you use doube "" it will be a string
    let alphabet = 'A';
    let emoji = '😁';
    println!("The special Character is {}, We also have a fun emoji!: {}, Remember, Letters can be chars to: {}\n",special_char,emoji,alphabet);
}

/*CONSTANTS: values that cannot be changed, no way to make them mutable
naming convention: upper case
***YOU MUST MANUALLY DECLARE TYPE***
constants cannot be changed with calculations, cannot be set by results from other functions, or any other computation.
Can be declared in the global scope.
Cannot be shadowed like other variables. 
 */
fn constants()
{
    const USER_LIMIT:i32 = 100;
    const PI:f32 = 3.14;
    println!("User limit is {} and the value of pi is {}\n", USER_LIMIT,PI)

}

/*String Literal: &str, is used when value of a string is known at compile time. 
Hard coded set of characters in a variable. 

String Object: String, 
    - not part of core language
    - growable collection 
    - mutable and UTF-8 encoded type
    -
    - *USE*: represent string values that are provided AT runtime. 
    - Allcated in the heap. 
*/

fn strings()
{
    //object strings
    let empty_string = String::new();
    println!("Length is {}", empty_string.len());
    let content_string = String::from("This is my String");
    println!("The length of this string is {}", content_string.len());
    //.pust_str
    let mut my_string = String::new();
    my_string.push_str("This is how to push a string into a new string object...");
    println!("{}\n", my_string);
    
    //convert literal string to Object type with .to_string()
    let name = "My Name is Chambers".to_string();
    println!("{}", name);
    
    //replace()
    let name2 = name.replace("Chambers", "srebmahc");//Find string, then replace it
    println!("{}", name2);
    
    //as_str() extracts string slice containing entire string. 
    let example_string = String::from("example_string");
    print_literal(example_string.as_str());
    fn print_literal(data:&str)
    {
        println!("displaying literal string {}", data);
    }

    //push(), appends given character to the end of a string. 
    let mut company = "Tutorial".to_string();
    company.push('s');//notice the '' not ""
    println!("{}", company);//displays Tutorials

    //push_str()
    let mut company = "Tutorials".to_string();
    company.push_str(" are FUN!");
    println!("{}", company);//output Tutorials are FUN!

    //len() used in other functions company.len()

    //split_whitespace(), splits strings into seperate strings, returns an iterator 
    let message = "This is the message I will have splite and tokenized".to_string();
    let mut i = 1;
    for token in message.split_whitespace()
    {
        println!("token {} {}",i,token);
        i+=1;
    }    

    //split() **result CANNOT be stored for later use.** HOWEVER, result can be stored as a VECTOR.

    let fullname = "Michael James Taylor";
    for token in fullname.split(" ")
    {
        println!("toekn is {}", token);
    }
    //store in vector
    let tokens:Vec<&str>= fullname.split(" ").collect();
    println!("firstName is {}", tokens[0]);
    println!("middleName is {}", tokens[1]);
    println!("lastName is {}", tokens[2]);

    //chars() is used for individual characters in a string. 
    let n1 = "Grabbing Characters".to_string();
    for n in n1.chars()
    {
        println!("{}", n);
    }

    //append / concatenate / interpolation, The result of this is a new **STRING OBJECT**
    let n2 = " and concatonating two srings".to_string();
    let n3 = n1 + &n2;
    println!("{}\n", n3);




}
//logical operators
// && == and || == or ! == not
/*bit wise operators
*/
fn bitwise_op()
{
    let A = 4;
    let B = 5;
    println!("A = {} B = {}", A, B);
    // will reverse all the bits in the operand
    println!("reverse B (!B)= {}", !B);
    //This moves bits to the side specified by the 2nd operand.
    println!("Move A bits left by 2 = {} Move A bits right 2 = {}", A << 2, A >> 2);

}