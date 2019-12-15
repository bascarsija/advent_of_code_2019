use day02::intcode;

fn main() {
    let mut op_codes = intcode::input::input_op_codes();
    let op_codes = op_codes.as_mut_slice();

    intcode::replace_at_pos(op_codes, 1, 12);
    intcode::replace_at_pos(op_codes, 2, 2);
    intcode::process_op_codes(op_codes);

    let op_codes = op_codes.iter().map(ToString::to_string).collect::<Vec<String>>();

    println!("output op codes: {}", op_codes.join(","));
    println!("first value: {}", op_codes[0]);
}
