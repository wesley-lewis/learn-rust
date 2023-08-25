use std::fs::File;

pub fn run_errors() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file, 
        Err(error) => {
            panic!("there was a problem opening the file: {:?}", error)
        },
    };
}
