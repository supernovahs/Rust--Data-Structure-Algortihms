use std::convert::TryInto;

fn main(){

    let arr: [u32;5] = [1,2,3,4,5];
    calcforFiveDigits(arr);
}

fn calcforFiveDigits( mut arr:[u32;5]) {
    let mut len:u32 = arr.len().try_into().unwrap();
    // If the array length is odd . 
    if len % 2 !=0 {
        let mut s = (len -1) /2 ;
        let mut i =0;
        while s>0 {
            let a:u32 = len -1 ;
            println!("print {}",a);
            arr[a as usize] = arr[i];
            arr[i] = len;
            println!("arri {}",arr[i]);
            i+=1;
            len-=1;
            s-=1;
        }
        println!("Reversed arr elements are ,{} {} {} {} {}",arr[0],arr[1],arr[2],arr[3],arr[4]);
    }
    
}