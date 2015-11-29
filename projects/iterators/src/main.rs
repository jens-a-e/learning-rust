//! Playing with Iterators

fn main() {

    for x in 0..10 {
        println!("{}", x);
    }

    let mut range = 0..10;

    loop {
        match range.next() {
            Some(x) => {
                println!("from loop Some: {}", x);
            },
            None => { break }
        }
    }

    let nums = vec![1, 2, 3];

    for num in &nums {
        println!("Iterating a vector: {}", num);
    }

    // We need to tell collect, what we want to collect
    let one_to_one_hundred = (1..101).collect::<Vec<_>>();

    println!("Collected one hundred: {:?}", one_to_one_hundred);

    let greater_than_forty_two = (0..100)
        .find(|x| *x > 42);

    match greater_than_forty_two {
        Some(result) => println!("We found {} is greater than 42", result),
        None => println!("No numbers found :("),
    }
}
