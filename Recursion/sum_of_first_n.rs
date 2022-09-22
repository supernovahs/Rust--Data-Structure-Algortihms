
fn main(){
    let input:i32 = 20;
    println!("Printing 1 to n {}",input);
    printthenum(20,1);
}

fn printthenum(input:i32,mut n:i32){

    if input <n{
        return;
    }
    println!("{}",n);

    n+=1;
    printthenum(input,n);
    

}