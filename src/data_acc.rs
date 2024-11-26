use std::io;
pub fn main()
{
    study_efficiency();
    break_efficiency();
}
fn study_efficiency() -> i32
{
    loop 
    {
        let mut scale = String::new();
        println!("On a scale of 1 to 5 how effective were your study times? 1 being completly wasted 5 being Very efficient and high focus.");
        io::stdin()
            .read_line(&mut scale)
            .expect("Could not read input");
        let scale = scale.trim();
        match (
            scale.parse::<i32>(),
        ) {
            (Ok(no),) =>
            {
                return no;
            }
            _ => println!("Valid input not received."),  
        }    
    }
}

fn break_efficiency() -> i32
{
    loop
    {
        let mut scale = String::new();
        println!("How effective were your break times on a scale of 1 to 5?");
        io::stdin()
            .read_line(&mut scale)
            .expect("Could not read input");
        let scale = scale.trim();
        match(scale.parse::<i32>(),
        ){
            (Ok(no),) =>
            {
                return no;
            }
            _ => println!("Valid input not received."),
        }
    }
}