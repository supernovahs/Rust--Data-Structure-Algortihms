
// Count No of Digits in the input 
fn main(){
    let mut n = 208889; // Random number whose digits we calculate
    let mut counter = 0;
    while n !=0{
        n /= 10;
        counter +=1
    }
    println!(" No of digits are {}",counter); // Prints the number of digits in 'n'
}