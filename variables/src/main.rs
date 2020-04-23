fn main() {
    let y = 1;
    let mut x =5; // let mut ==> mut means that it is mutable
    println!("x is: {}", x);
    x = 7;
    
    //Visibility of variables

    { // Scope of x within the brackets
        let x = 9;
        println!("x within bracket: {}", x);
    
    }

    println!("x = {0}; y = {1}",x,y);

    // data types:

    let a: bool = true;

    let chara: char = 'ü';

    if a 
    {
        println!("{}",chara);
    }



}
