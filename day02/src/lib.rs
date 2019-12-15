pub mod intcode {
    pub mod input {
        use advent_of_code_2019::input;

        pub fn input_op_codes() -> Vec<i64> {
            let input = input::single_line_from_arg_file();

            println!("input op codes: {}", input);

            let mut op_codes = Vec::new();
            for op_code in input.split(',') {
                op_codes.push(op_code.parse::<i64>().unwrap());
            }

            op_codes
        }
    }

    use std::fmt::{Formatter, Error};

    #[derive(Debug)]
    pub enum OpCode {
        ADD,
        MULTIPLY,
        TERMINATE
    }

    impl std::fmt::Display for OpCode {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f, "{:?}", self)
        }
    }

    const INT_VALUE_ADD: i64 = 1;
    const INT_VALUE_MULTIPLY: i64 = 2;
    const INT_VALUE_TERMINATE: i64 = 99;

    impl OpCode {
        fn of_int(op_code: i64) -> Option<OpCode> {
            match op_code {
                INT_VALUE_ADD => Some(OpCode::ADD),
                INT_VALUE_MULTIPLY => Some(OpCode::MULTIPLY),
                INT_VALUE_TERMINATE => Some(OpCode::TERMINATE),
                _ => None
            }
        }

        fn operation(&self) -> Option<&dyn Fn(i64, i64) -> i64> {
            match self {
                OpCode::ADD => Some(&|left, right| left + right),
                OpCode::MULTIPLY => Some(&|left, right| left * right),
                OpCode::TERMINATE => None
            }
        }

        fn process_at_pos(codes: &mut [i64], idx: usize) -> Option<usize> {
            Option::and_then(OpCode::of_int(codes[idx]), &mut |op_code: OpCode| op_code.process(codes, idx))
        }

        fn process(&self, codes: &mut [i64], idx: usize) -> Option<usize> {
            //print!("processing: @{}: {}", idx, self.to_string());

            let idx = Option::map(self.operation(), |op| {
                if codes.len() < idx + 3 {
                    panic!("opcode array does not contain expected opcode operands: {}", self.to_string());
                }

                let left_src = codes[idx + 1];
                let right_src = codes[idx + 2];
                let dest = codes[idx + 3];
                let left = codes[left_src as usize];
                let right = codes[right_src as usize];
                let result = op(left, right);

                //println!(": (@{}, @{} -> @{}) => {}, {} -> {}", left_src, right_src, dest, left, right, result);

                replace_at_pos(codes, dest as usize, result);

                idx + 4
            });

            //if idx.is_none() { println!() }

            idx
        }
    }

    pub fn replace_at_pos(codes: &mut [i64], pos: usize, code: i64) {
        //println!("replacing: @{}: {}", pos, code);

        codes[pos] = code;
    }

    pub fn process_op_codes(codes: &mut [i64]) {
        let mut idx = 0;
        while idx < codes.len() {
            match OpCode::process_at_pos(codes, idx) {
                Some(new_idx) => idx = new_idx,
                None => break
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::intcode::replace_at_pos;

        fn test_replace_at(op_codes: &mut [i64], idx: usize, value: i64, expected: &[i64]) {
            replace_at_pos(op_codes, idx, value);

            assert_eq!(op_codes, expected);
        }

        #[test]
        fn replace_at_first_pos() {
            test_replace_at(&mut [0, 1, 2, 3], 0, std::i64::MAX, &mut [std::i64::MAX, 1, 2, 3]);
        }

        #[test]
        fn replace_at_last_pos() {
            test_replace_at(&mut [0, 1, 2, 3], 3, std::i64::MAX, &mut [0, 1, 2, std::i64::MAX]);
        }

        #[test]
        fn replace_at_internal_pos() {
            test_replace_at(&mut [0, 1, 2, 3], 2, std::i64::MAX, &mut [0, 1, std::i64::MAX, 3]);
        }

        use crate::intcode::process_op_codes;

        fn test_process_op_codes(op_codes: &mut [i64], expected: &[i64]) {
            process_op_codes(op_codes);

            assert_eq!(op_codes, expected);
        }

        #[test]
        fn single_add_op_into_op_code() {
            test_process_op_codes(&mut [1, 0, 2, 0], &mut [3, 0, 2, 0]);
        }

        #[test]
        fn single_add_op_into_left_operand() {
            test_process_op_codes(&mut [1, 0, 3, 1], &mut [1, 2, 3, 1]);
        }

        #[test]
        fn single_add_op_into_right_operand() {
            test_process_op_codes(&mut [1, 0, 3, 2], &mut [1, 0, 3, 2]);
        }

        #[test]
        fn single_add_op_into_dest() {
            test_process_op_codes(&mut [1, 0, 3, 3], &mut [1, 0, 3, 4]);
        }

        #[test]
        fn multiple_add_op() {
            test_process_op_codes(&mut [1, 0, 3, 3, 1, 3, 7, 7], &mut [1, 0, 3, 4, 1, 3, 7, 11]);
        }

        #[test]
        fn single_multiply_op_into_op_code() {
            test_process_op_codes(&mut [2, 0, 3, 0], &mut [0, 0, 3, 0]);
        }

        #[test]
        fn single_multiply_op_into_left_operand() {
            test_process_op_codes(&mut [2, 0, 3, 1], &mut [2, 2, 3, 1]);
        }

        #[test]
        fn single_multiply_op_into_right_operand() {
            test_process_op_codes(&mut [2, 0, 3, 2], &mut [2, 0, 4, 2]);
        }

        #[test]
        fn single_multiply_op_into_dest() {
            test_process_op_codes(&mut [2, 0, 3, 3], &mut [2, 0, 3, 6]);
        }

        #[test]
        fn multiple_multiply_op() {
            test_process_op_codes(&mut [2, 0, 3, 3, 2, 3, 7, 7], &mut [2, 0, 3, 6, 2, 3, 7, 42]);
        }

        #[test]
        fn immediately_terminate() {
            test_process_op_codes(&mut [99], &mut [99]);
        }

        #[test]
        fn terminate_after_add() {
            test_process_op_codes(&mut [1, 2, 3, 3, 99], &mut [1, 2, 3, 6, 99]);
        }

        #[test]
        fn terminate_between_add_and_multiply() {
            test_process_op_codes(&mut [1, 2, 3, 3, 99, 2, 3, 2, 8],&mut [1, 2, 3, 6, 99, 2, 3, 2, 8]);
        }

        #[test]
        fn terminate_due_to_updated_op_code() {
            test_process_op_codes(&mut [1, 8, 0, 4, 1, 0, 0, 0, 98], &mut [1, 8, 0, 4, 99, 0, 0, 0, 98]);
        }

        #[test]
        fn supplied_test_case_1() {
            test_process_op_codes(&mut [1,0,0,0,99], &mut [2,0,0,0,99]);
        }

        #[test]
        fn supplied_test_case_2() {
            test_process_op_codes(&mut [2,3,0,3,99], &mut [2,3,0,6,99]);
        }

        #[test]
        fn supplied_test_case_3() {
            test_process_op_codes(&mut [2,4,4,5,99,0], &mut [2,4,4,5,99,9801]);
        }

        #[test]
        fn supplied_test_case_4() {
            test_process_op_codes(&mut [1,1,1,4,99,5,6,0,99], &mut [30,1,1,4,2,5,6,0,99]);
        }
    }
}
