/// Problem : Print fibonacci numbers upto nth term

fn main(){

    let n:u32 = 5;
    let index_zero:u32 = 0 ;
    let index_one:u32 = 1;
    println!("{}",index_zero);
    println!("{}",index_one);

    fibonacciprint(0,1,2,n);

}

fn fibonacciprint(first:u32,second:u32,mut index:u32,n:u32){

    if index <=n {
        println!("{}",first + second);
        index +=1;
        fibonacciprint(second,first+second,index,n);
    }


}