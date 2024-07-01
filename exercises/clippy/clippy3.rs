// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if let Some(n) = my_option {};

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<u8> = vec![];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let value_a = 45;
    let value_b = 66;
    // Let's swap these two!
    let (value_a, value_b) = (value_b, value_a);
    println!("value a: {}; value b: {}", value_a, value_b);
}
