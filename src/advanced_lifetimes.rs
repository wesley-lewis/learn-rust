pub fn run_lifetimes() {
    println!("Advanced Lifetimes");
    let context = Context("Wesley Lewis");
    let parser = Parser {context: &context };

    println!("{:?}", parse_context(context).unwrap());
}

// lifetime subtyping
struct Context<'s>(&'s str);

struct Parser <'c, 's>{
    context: &'c Context<'s>,
}

impl<'c, 's> Parser <'c, 's>{
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
// remaining
