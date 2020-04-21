fn main() {
    let mut x = 10;

    while x > 0 {
        println!("X is now: {}",x);
        x = x-1; 
    }
    
   

    for (i,j) in (0..5).enumerate() 
    {
        println!("i is now in the for loop: {}",i);
        println!("j is now in the for loop: {}",j);
    }
    

    for i in 0..5
    {
        println!("i is now in the for loop: {}",i);

    }
    

}
