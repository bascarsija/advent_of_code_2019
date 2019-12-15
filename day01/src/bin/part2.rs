use day01::fuel;

fn main() {
    let fuel = fuel::fuel_for_modules(Option::Some(&|fuel| fuel + extra_fuel_for_fuel_mass(fuel)));

    println!("total fuel needed (counting fuel itself): {}", fuel);
}

fn extra_fuel_for_fuel_mass(mass: u64) -> u64 {
    let mut sum = 0;
    let mut extra = mass;
    loop {
        let sum_snap = sum;
        let extra_snap = extra;

        extra = fuel::fuel_for_mass(extra);
        sum += extra;

        println!("extra fuel: {} + ({} -> {}) = {}", sum_snap, extra_snap, extra, sum);

        if extra <= 0 {
            break sum;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::extra_fuel_for_fuel_mass;

    #[test]
    fn mass_of_0() {
        assert_eq!(extra_fuel_for_fuel_mass(0), 0);
    }

    #[test]
    fn mass_of_2() {
        assert_eq!(extra_fuel_for_fuel_mass(2), 0);
    }

    #[test]
    fn mass_of_9() {
        assert_eq!(extra_fuel_for_fuel_mass(9), 1);
    }

    #[test]
    fn mass_of_1_969() {
        assert_eq!(extra_fuel_for_fuel_mass(1_969), 966);
    }

    #[test]
    fn mass_of_100_756() {
        assert_eq!(extra_fuel_for_fuel_mass(100_756), 50_346);
    }
}
