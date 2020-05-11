fn main() {
  
    print_me(42);
    let mut retur:i32;

    let boolean_list: [bool;5] = [true,false,false,true,true];

    for y in &boolean_list
    {
        println!("{}",y);
        if *y
        {
            retur = return_integer(1,2,3);
        }
        else
        {
            retur = return_integer_2(1,2,3);
        }
        println!("{}",retur);
    }
    
}

fn print_me(x: i32) {

    println!("in print_me: {}",x);

}

fn return_integer(a: i32, b:i32, c:i32) -> i32 {
    a+b+c
}

fn return_integer_2(a: i32, b:i32, c:i32) -> i32 {
    a+b-c
}

