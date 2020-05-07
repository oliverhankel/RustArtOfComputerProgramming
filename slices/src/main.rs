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
    

    //Working with tupels

    let a: (i32, &str)  = (1, "hello");

    let(i,s) = a;
    println!("{}",i);


    let t = (1,);  // tupel needs a comma
    println!("{}",t.0)


}
