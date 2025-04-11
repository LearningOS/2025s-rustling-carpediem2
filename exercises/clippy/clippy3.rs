// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    //if my_option.is_none() {
    //    my_option.unwrap();
    //}
     // 移除了无意义的unwrap()调用
     // 不需要这个if块，因为当option为None时，unwrap()会panic
    // 且is_none()检查在此处是多余的

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    //let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    //println!("This Vec is empty, see? {:?}", my_empty_vec);
    vec![1, 2, 3, 4, 5].resize(0, 5);// resize()返回()
    println!("This Vec is empty, see? {:?}", ());

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    //value_a = value_b;
    //value_b = value_a;
    std::mem::swap(&mut value_a, &mut value_b); // 正确交换
    println!("value a: {}; value b: {}", value_a, value_b);
}
