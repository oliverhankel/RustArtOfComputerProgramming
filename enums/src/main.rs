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
}