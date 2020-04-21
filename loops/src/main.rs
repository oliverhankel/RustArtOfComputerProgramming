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
        if i==3 {
            println!("BREAK");
            break;
        }

    }

    'name_outer_loop: for a in 0..5 {
        println!("in outer loop: {}",a);

        'name_inner_loop: for b in 0..10 {

            if b == 7 {
                break 'name_inner_loop;
            }

            println!("in inner loop: {}", b)


        }
    }
    

}
