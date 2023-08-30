#[allow(dead_code)]

pub fn run_iterators() {
    println!("iterators in rust");

    let mut v1 = vec![1,2,3];
    let mut v1_iter = v1.iter_mut();
    // println!("{:?}", v1_iter);

    assert_eq!(v1_iter.next(), Some(&mut 1));
    assert_eq!(v1_iter.next(), Some(&mut 2));

    iter_map();

    let counter: Counter = Counter::new(10);

    for i in counter {
        println!("{} ", i);
    }
    println!("");

    for i in Counter::new(5) {
        print!("{} ", i * i);
    }
    println!("");
}

fn iter_map() {
    let v1: Vec<i32> = vec![1,2,3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2,3,4]);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // we cant use 'v1_iter' as sum() took ownership of 'v1_iter' while calculating the sum
    // println!("{:?}", v1_iter);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe{ size: 10, style: String::from("sneaker") } ,
        Shoe{ size: 13, style: String::from("sandal") } ,
        Shoe{ size: 10, style: String::from("boot") },
    ];
    
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe{ size: 10, style: String::from("sneaker")} ,
            Shoe{ size: 10, style: String::from("boot") },
        ]
    );
}

struct Counter {
    count: u32,
    range: u32,
}

impl Counter {
    fn new(range: u32 ) -> Counter {
        Counter{ count: 0, range }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < self.range {
            Some(self.count)
        }else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new(6);

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new(6).zip(Counter::new(6).skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
