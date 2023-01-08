struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaa!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let num = 0.5;
    let animal = random_animal(num);

    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    )
}
