use clock::Clock;

fn main() {
    println!("{}", Clock::new(23, 59).add_minutes(2).to_string());
}
