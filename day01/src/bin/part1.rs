use day01::fuel;

fn main() {
    let sum = fuel::fuel_for_modules(Option::None);

    println!("total fuel needed: {}", sum);
}
