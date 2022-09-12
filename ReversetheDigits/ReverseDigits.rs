fn main(){

    let mut  digit = 12345;
    let mut  no_of_digits  = countdigits(digit);
    println!("no of digits is {}",no_of_digits);
    let mut answer =0;
    while digit !=0{
        answer = i32::pow(10,no_of_digits -1) * (digit%10) +answer;
        println!("answer is {}",answer);
        digit /=10;
        no_of_digits -=1;
    } 

    println!("Reverse of number is {}",answer);



}

fn countdigits( mut n:i32 ) -> u32{
    let mut counter = 0;
    while n !=0{
        n /= 10;
        counter +=1
    }
    counter
}