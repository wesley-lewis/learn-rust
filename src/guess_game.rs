pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess {
            value
        }
    }
}

#[test]
#[should_panic]
fn test_guess() {
    let guess = Guess::new(500);
    assert_eq!(guess.value, 500);
}
