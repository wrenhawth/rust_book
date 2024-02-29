fn _move_box(y: Box<[u32; 1000]>) {
    let b = y;
    println!("{}", b[0]);
}

fn main() {
    // let a = Box::new([0; 1_000]);
    // move_box(a);
    // println!("{}", a[0]);


    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}