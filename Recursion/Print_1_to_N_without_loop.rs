
fn main(){
    // INput for which to print 1 to n .
    let input:i32 =21;
   
    recursion(input,1);
}


fn recursion( input:i32,mut n:i32) {

    if n<=input{
        println!("input {}",n);
        n+=1;
        recursion(input,n);
    }
    
}