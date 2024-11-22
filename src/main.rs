
fn main()
{
    println!("hello world, This is a me learning rust!");
    //notice the {} gets filled with the argument after the comma.
    println!("format {} arguments", "some");
    println!();
    types();
    println!();
    integer();
    float();
    character();
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
    println!("age is {}", age);
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