fn main(){

    let mut  digit = 121;
    let original_digit = digit;
    let mut  no_of_digits  = countdigits(digit);
    println!("no of digits is {}",no_of_digits);
    let mut answer =0;
    while digit !=0{
        answer = i32::pow(10,no_of_digits -1) * (digit%10) +answer;
        digit /=10;
        no_of_digits -=1;
    } 
    println!("Reverse of number is {}",answer);
    println!("original_digit{}", original_digit);
    if original_digit == answer{

        println!("It is a pallendrom {}",original_digit);
    }

}

fn countdigits( mut n:i32 ) -> u32{
    let mut counter = 0;
    while n !=0{
        n /= 10;
        counter +=1
    }
    counter
}