fn main() {
    let mut my_vec: Vec<i64> = Vec::new();

    thread::spawn(move || {
        add_to_vec(&mut my_vec);
    });

    my_vec.push(34)
}

fn add_to_vec(vec: &mut Vec<i64>) {
    vec.push(42);
}
