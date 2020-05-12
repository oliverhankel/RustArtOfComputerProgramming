struct LikeAClass{
    x: i32,
    y: i32,
}

fn main() {
    let mut my_new_like_a_class = LikeAClass{x: 2, y:3};
    
    println!("{}", my_new_like_a_class.x);
    let my_new_like_a_class_2 = LikeAClass{x:3, ..my_new_like_a_class};
    println!("{}", my_new_like_a_class_2.x);

}
