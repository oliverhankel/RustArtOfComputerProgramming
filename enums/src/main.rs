enum Move{
    Stay,
    Forward{steps: i32},
    Backward{steps: i32}
}

fn main() {
    let my_move = Move::Backward{steps:34};
    print_move(my_move);

}

fn print_move(m: Move)
{

    match m
    {
        Move::Stay => println!("Stay"),
        Move::Forward{steps} => println!("Move Forward: {}",steps),
        Move::Backward{steps} => println!("Move Backward: {}",steps),
        _ => println!("something"),
    };


    //more match

    let x: i32 = 13;
    let y: i32 = 43;
    match x
    { 
        a @ 1 ... 10 if y > 44 => println!("{}",a),
        12 ... 20 => println!("{} is between 12 and 20",x),
        _ => println!("Case not covered"),


    };

}