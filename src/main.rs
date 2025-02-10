mod pairer;

fn main() {
    env_logger::init();

    // read list of people as CSV, check for uniqueness or panic

    let pairs = pairer::pair();
}
