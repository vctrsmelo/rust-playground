use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name) // lending with &mut permits read_line to mutate your_name.
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),   
    ];

    let mut allow_in = false;
    for visitor in &visitor_list {
        if visitor.name == name {
            allow_in = true;
            visitor.greet_visitor();
        }
    }

    if !allow_in {
        println!("Sorry, you are not on the list");
    }
}
