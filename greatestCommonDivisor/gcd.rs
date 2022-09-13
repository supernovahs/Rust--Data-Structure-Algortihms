// Calculating Greatest Common Divisor for 2 inputs !!!
fn main(){


    let  first:i32 = 40;
    let  second:i32 = 30;

    let smalldigit:i32 = if  first > second  {second}  else {first} ;
    let largedigit:i32 = if smalldigit == first  {second} else{ first};
    
    let mut  n = smalldigit;
    // Iterate starting from smalldigit till 1 . 
    // Break if a number divides both inputs without leaving any remainder
    while n!=0{
        if largedigit % n ==0 && smalldigit % n ==0 {
            println!("Greatest Common Divisor {}", n );
            break;
        }
        n -=1;
    }


}