pub mod fuel {
    pub fn fuel_for_mass(mass: u64) -> u64 {
        match mass / 3 {
            0 | 1 => 0,
            quotient => quotient - 2
        }
    }

    pub fn fuel_for_modules(fuel_transform: Option<&dyn Fn(u64) -> u64>) -> u64 {
        use advent_of_code_2019::input;

        let fuel_transform = match fuel_transform {
            Some(fuel_transform) => fuel_transform,
            None => &|val| val
        };

        let mut sum = 0;
        for line in input::lines_from_arg_file() {
            let line = line.unwrap();
            let mass = line.parse::<u64>().unwrap();
            let fuel = fuel_transform(fuel_for_mass(mass));
            let snap = sum;

            sum += fuel;

            println!("running fuel mass total: {} + ({} -> {}) = {}", snap, mass, fuel, sum);
        }

        sum
    }

    #[cfg(test)]
    mod tests {
        use crate::fuel::fuel_for_mass;

        #[test]
        fn mass_of_0_underflow_check() {
            assert_eq!(fuel_for_mass(0), 0);
        }

        #[test]
        fn mass_of_max_u64_overflow_check() {
            assert_eq!(fuel_for_mass(std::u64::MAX), 6_148_914_691_236_517_203);
        }

        #[test]
        fn mass_of_12() {
            assert_eq!(fuel_for_mass(12), 2);
        }

        #[test]
        fn mass_of_14() {
            assert_eq!(fuel_for_mass(14), 2);
        }

        #[test]
        fn mass_of_1_969() {
            assert_eq!(fuel_for_mass(1_969), 654);
        }

        #[test]
        fn mass_of_100_756() {
            assert_eq!(fuel_for_mass(100_756), 33_583);
        }
    }
}
