pub mod password {
    const NUM_DIGITS: usize = 6;
    const DIGIT_MAX: u32 = 10;
    const PAIR_LEN: u8 = 2;

    #[inline]
    fn extract_digits(num: u32) -> [u8; NUM_DIGITS] {
        let mut digits = [0; NUM_DIGITS];

        let mut remainder = num;
        for i in (1 .. NUM_DIGITS).rev() {
            digits[i] = (remainder % DIGIT_MAX) as u8;

            remainder /= DIGIT_MAX;
        }
        digits[0] = remainder as u8;

        return digits;
    }

    #[allow(non_camel_case_types)]
    pub enum MatchingPairStrategy {
        ANY_RUN,
        PAIR_ONLY
    }

    #[inline]
    fn set_seen_if_unseen_and_pair(seen_valid_pair: &mut bool, repetition_len: u8) {
        if !*seen_valid_pair && repetition_len == PAIR_LEN {
            *seen_valid_pair = true;
        }
    }

    #[inline]
    fn is_valid_password(password: u32, matching_pair_strategy: &MatchingPairStrategy) -> bool {
        let mut seen_valid_pair = false;
        let mut curr_repetition_count = 0;
        let mut prev_digit = None;
        for curr_digit in extract_digits(password).iter() {
            match prev_digit {
                Some(prev_digit_value) => {
                    if curr_digit == prev_digit_value {
                        curr_repetition_count += match curr_repetition_count { 0 => PAIR_LEN, _ => 1 };

                        if let MatchingPairStrategy::ANY_RUN = matching_pair_strategy {
                            if !seen_valid_pair {
                                seen_valid_pair = true;
                            }
                        }
                    }
                    else {
                        if curr_digit < prev_digit_value {
                            return false;
                        }

                        if let MatchingPairStrategy::PAIR_ONLY = matching_pair_strategy {
                            set_seen_if_unseen_and_pair(&mut seen_valid_pair, curr_repetition_count);
                        }

                        curr_repetition_count = 0;
                        prev_digit = Some(curr_digit);
                    }
                },
                None => prev_digit = Some(curr_digit)
            }
        }

        if let MatchingPairStrategy::PAIR_ONLY = matching_pair_strategy {
            set_seen_if_unseen_and_pair(&mut seen_valid_pair, curr_repetition_count);
        }

        return seen_valid_pair;
    }

    pub fn find_valid_passwords_in_range(min: u32, max: u32, matching_pair_strategy: &MatchingPairStrategy) -> Vec<u32> {
        let mut found = Vec::new();

        for value in min..(max + 1) {
            if is_valid_password(value, matching_pair_strategy) {
                found.push(value);
            }
        }

        return found;
    }

    #[cfg(test)]
    pub mod tests {
        use crate::password::{is_valid_password, extract_digits, find_valid_passwords_in_range, MatchingPairStrategy};

        #[test]
        fn extract_111111() {
            assert_eq!(extract_digits(111111), [1,1,1,1,1,1]);
        }

        #[test]
        fn extract_223450() {
            assert_eq!(extract_digits(223450), [2,2,3,4,5,0]);
        }

        #[test]
        fn extract_123789() {
            assert_eq!(extract_digits(123789), [1,2,3,7,8,9]);
        }

        #[test]
        fn test_111111_is_valid_any_run() {
            assert_eq!(is_valid_password(111111, &MatchingPairStrategy::ANY_RUN), true);
        }

        #[test]
        fn test_223450_is_invalid_any_run() {
            assert_eq!(is_valid_password(223450, &MatchingPairStrategy::ANY_RUN), false);
        }

        #[test]
        fn test_123789_is_invalid_any_run() {
            assert_eq!(is_valid_password(123789, &MatchingPairStrategy::ANY_RUN), false);
        }

        #[test]
        fn test_part1() {
            assert_eq!(find_valid_passwords_in_range(165432, 707912, &MatchingPairStrategy::ANY_RUN).len(), 1716);
        }

        #[test]
        fn test_112233_is_valid_pair_only() {
            assert_eq!(is_valid_password(112233, &MatchingPairStrategy::PAIR_ONLY), true);
        }

        #[test]
        fn test_123444_is_invalid_pair_only() {
            assert_eq!(is_valid_password(123444, &MatchingPairStrategy::PAIR_ONLY), false);
        }

        #[test]
        fn test_111122_is_valid_pair_only() {
            assert_eq!(is_valid_password(111122, &MatchingPairStrategy::PAIR_ONLY), true);
        }
    }
}
