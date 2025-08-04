fn vector() {
    let v = vec![1, 2, 3];

    for x in v.into_iter() {
        println!("{}", x);
    }
}

fn hashmap() {
    let mut h = HashMap::new();
    h.insert(String::from("Hello"), String::from("World"));

    for (key, value) in h.iter() {
        println!("{}: {}", key, value);
    }
}

fn array() {
    let a = vec![1, 2, 3];

    for x in a.into_iter() {
        println!("{}", x);
    }
}

fn for_each() {
    let v = Vec!["Hello", "World", "!"].into_iter();

    v.for_each(|world| {
        println!("{}", word);
    });
}

fn collect() {
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter();

    let _: Vec<64> = x.collect();
}

fn from_iter() {
    let x = vec![(1, 2), (3, 4), (5, 6)].into_iter();

    let _: HashMap<u64, u64> = HashMap::from_iter(x);
}

fn reduce() {
    let values = vec![1, 2, 3, 4, 5].into_iter();

    let _sum = values.reduce(|acc, x| acc + x);
}

fn fold() {
    let values = vec!["Hello", "World", "!"].into_iter();

    let _sentence = values.fold(String::new(), |acc, x| acc + x);
}

fn filter() {
    let v = vec![-1, 2, -3, 4, 5].into_iter();

    let _positive_numbers: Vec<i32> = v.filter(|x|.is_positive()).collect());
}

fn inspect() {
    let v = vec![-1, 2, -3, 4, 5].into_iter();
    let _positive_numbers: Vec<i32> = v
    .inspect(|x| println!("Before filter: {}", x))
    .filter(|x: &i32| x.is_positive())
    .inspect(|x| println!("After filter: {}", x))
    .collect();
}

fn map() {
    let v = vec!["Hello", "World", "!"].into_iter();

    let w: Vec<String> = v.map(String::from).collect();
}

fn filter_map() {
    let v = vec!["Hello", "World", "!"].into_iter();
    let w: Vec<String> = v
        .filter_map(|x| {
            if x.len() > 2 {
                Some(String::from(x))
            } else {
                None
            }
        })
        .collect();

    assert_eq!(w, vec!["Hello".to_string(), "World".to_string()]);
}

assert_eq!(w, vec!["Hello".to_string(), "World".to_string()]);

fn chain() {
    let x = vec![1, 2, 3, 4, 5].into_iter();
    let y = vec![6, 7, 8, 9, 10].into_iter();

    let z: Vec<u64> = x.chain(y).collect();
    assert_eq!(z.len(), 10);
}

fn flatten() {
    let x = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]].into_iter();
    
    let z: Vec<u64> = x.flatten().collect();
    assert_eq!(z.len(), 10);
}


fn main() {
    println!("Hello, world!");
}
