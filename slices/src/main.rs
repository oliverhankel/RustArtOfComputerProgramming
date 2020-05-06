fn main() {
    let x = [3,4,5,4,3,4,7,47,334];
    for y in &x[1..3]
    {
        println!("{}",y);
    }
    
    let y = &x[2..3];
    //{
    println!("{}",y.first() == Some(&5));  //Some checks whether it is an object of a certain type
    //}


}
