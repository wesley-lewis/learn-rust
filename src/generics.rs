pub fn run_generics() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest_num = largest(&number_list);
    println!("The largest number is {}", largest_num);

    largest_num = largest(&vec![10, 30, 20, 5, 6]);
    println!("The largest number is {}", largest_num);

    let char_list = vec!['a', 'b', 'z', 'e', 'c'];
    let mut largest_char = largest(&char_list);
    println!("The largest char is {}", largest_char);

    let integer = Point{ x: 5, y: 10 };
    let float = Point{ x: 5.0, y: 10.1};
    println!("{:?}", integer);
    println!("{:?}", float);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T 
{
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

