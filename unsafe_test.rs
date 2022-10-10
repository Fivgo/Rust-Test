fn main(){

    let begin = 5;
    
    println!("Printing begin: {}", begin);
    let r1 = begin;
    
    println!("Printing begin after borrowing: {}", begin);
    unsafe{
        println!("Printing r1: {}", r1);
    }

}