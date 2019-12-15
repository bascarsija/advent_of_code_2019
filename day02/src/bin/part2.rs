use day02::intcode;

fn main() {
    let input = intcode::input::input_op_codes();

    'TOP: for noun in 0..100 {
        for verb in 0..100 {
            let mut op_codes = input.clone();
            let op_codes = op_codes.as_mut_slice();

            intcode::replace_at_pos(op_codes, 1, noun);
            intcode::replace_at_pos(op_codes, 2, verb);
            intcode::process_op_codes(op_codes);

            let output = op_codes[0];
            let op_codes = op_codes.iter().map(ToString::to_string).collect::<Vec<String>>().join(",");

            println!("output op codes: {}", op_codes);
            println!("program output (@0): {}", output);

            if output == 19_690_720 {
                let predicate = 100 * noun + verb;

                println!("found predicate: 100 * {} + {} = {}", noun, verb, predicate);

                break 'TOP;
            }
        }
    }
}
