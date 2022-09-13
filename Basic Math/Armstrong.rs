

fn main (){


    let digit:i32 = 153;
    let NO_OF_DIGITS = countdigits(digit);

    let mut n = digit;
    let mut sum:i32 =0;
    while n!=0 {
        let digit =  n%10;
        sum += i32::pow(digit,NO_OF_DIGITS);
        n /=10;
    }

    if sum == digit {
        println!("It is Armstrong Number {}",digit);

    }
    else {
        println!("Not an Armstrong {}",digit );
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