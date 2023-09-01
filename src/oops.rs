pub fn run_oops() {
   let mut average_collection = AverageCollection {
        list: vec![1,2,3,4],
        average: 10 as f64 / 4 as f64,
   };

   print_the_struct(&average_collection);
   average_collection.add(6);
   print_the_struct(&average_collection); 
   average_collection.remove();
    
   print_the_struct(&average_collection);
}

fn print_the_struct(ac: &AverageCollection) {
    println!("{:?}", ac.list);
    println!("{}", ac.average);
}

struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

