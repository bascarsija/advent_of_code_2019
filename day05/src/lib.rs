pub mod intcode {
    mod optype {
        #[derive(Clone)]
        pub(crate) enum ParameterMode {
            POSITION,
            IMMEDIATE
        }

        impl ParameterMode {
            pub fn parse(mode: i32) -> Result<ParameterMode, String> {
                match mode {
                    0 => Ok(ParameterMode::POSITION),
                    1 => Ok(ParameterMode::IMMEDIATE),
                    _ => Err(format!("invalid parameter mode: {}", mode))
                }
            }
        }

        pub(crate) struct ParameterLayout {
            pub num_read: usize,
            pub has_write: bool
        }

        pub(crate) enum OpType {
            ADD,
            MULTIPLY,
            INPUT,
            OUTPUT,
            TERMINATE
        }

        impl OpType {
            pub fn parse(op_code: i32) -> Result<OpType, String> {
                match op_code {
                    1 => Ok(OpType::ADD),
                    2 => Ok(OpType::MULTIPLY),
                    3 => Ok(OpType::INPUT),
                    4 => Ok(OpType::OUTPUT),
                    99 => Ok(OpType::TERMINATE),
                    _ => Err(format!("invalid op code identifier: {}", op_code))
                }
            }

            pub(crate) fn parameter_layout(&self) -> &ParameterLayout {
                match self {
                    OpType::ADD | OpType::MULTIPLY => &ParameterLayout { num_read: 2, has_write: true },
                    OpType::INPUT => &ParameterLayout { num_read: 1, has_write: true },
                    OpType::OUTPUT | OpType::TERMINATE => &ParameterLayout { num_read: 0, has_write: false },
                }
            }

            pub(crate) fn num_parameters(&self) -> usize {
                let layout = self.parameter_layout();

                return layout.num_read + if layout.has_write { 1 } else { 0 };
            }
        }
    }

    mod opcode {
        use crate::intcode::optype::{OpType, ParameterMode};

        pub(crate) struct OpCode {
            pub op_type: OpType,
            pub read_param_modes: Box<[ParameterMode]>
        }

        impl OpCode {
            const DIGIT_BASE: i32 = 10;

            fn reversed_digits(value: i32) -> Box<[i32]> {
                let mut digits = Vec::new();

                let mut remainder = value;
                while value >= OpCode::DIGIT_BASE {
                    digits.push(remainder % OpCode::DIGIT_BASE);

                    remainder /= OpCode::DIGIT_BASE;
                }

                digits.push(remainder);

                return digits.into_boxed_slice();
            }

            fn assemble_int_from_reversed_digits(digits: Box<[i32]>) -> i32 {
                let mut op_code = 0;
                for digit in digits.iter().rev() {
                    op_code = OpCode::DIGIT_BASE * op_code + *digit;
                }

                return op_code;
            }

            fn parse_parameter_mode(digits: &Box<[i32]>, idx: usize) -> Result<ParameterMode, String> {
                return ParameterMode::parse(digits[idx]).map_err(|err| format!("unable to parse parameter mode at index {}: {}", idx, err));
            }

            pub fn parse_parameter_modes(op_type: &OpType, digits: Box<[i32]>) -> Result<Box<[ParameterMode]>, String> {
                let param_layout = op_type.parameter_layout();
                let num_params_expected = op_type.num_parameters();
                let num_params_found = digits.len();

                if num_params_found != num_params_expected {
                    return Err(format!("number of parameters specified in op code ({}) exceeds number expected ({})",
                                       num_params_found, num_params_expected));
                } else if param_layout.has_write {
                    if let ParameterMode::IMMEDIATE = OpCode::parse_parameter_mode(&digits, num_params_found - 1)? {
                        return Err("write parameter address mode specified in op code as immediate".to_string());
                    }
                }

                let mut read_modes = Vec::with_capacity(param_layout.num_read);

                for i in 0..num_params_found {
                    read_modes[i] = OpCode::parse_parameter_mode(&digits, i)?;
                }

                read_modes.resize(param_layout.num_read as usize, ParameterMode::POSITION);

                return Ok(read_modes.into_boxed_slice());
            }

            pub fn parse(value: i32) -> Result<OpCode, String> {
                let (op_code, param_modes) = {
                    let digits = OpCode::reversed_digits(value);
                    let num_digits = digits.len();

                    if num_digits < 1 {
                        return Err(format!("unable to parse digits from op code"));
                    }

                    let (op_code, param_modes) = digits.split_at(if num_digits > 1 { 2 } else { 1 });

                    let op_code = op_code.to_vec().into_boxed_slice();
                    let param_modes = param_modes.to_vec().into_boxed_slice();

                    (op_code, param_modes)
                };

                let op_type = OpType::parse(OpCode::assemble_int_from_reversed_digits(op_code))
                    .map_err(|err| format!("unable to parse op code from value: {}: {}", value, err))?;
                let read_param_modes = OpCode::parse_parameter_modes(&op_type, param_modes)
                    .map_err(|err| format!("unable to parse parameter modes from op code: {}: {}", value, err))?;

                return Ok(OpCode { op_type, read_param_modes });
            }
        }
    }

    mod processor {
        use crate::intcode::{opcode::OpCode, optype::ParameterMode};
        use crate::intcode::optype::OpType;
        use std::io::stdin;

        fn resolve_read_parameters(param_modes: &[ParameterMode], program: &[i32], instruction_ptr: usize) -> Result<Box<[i32]>, String> {
            let mut params = Vec::with_capacity(param_modes.len());

            for i in 0 .. param_modes.len() {
                let param_value = program[instruction_ptr + i];

                params[i] = match param_modes[i] {
                    ParameterMode::POSITION => {
                        if param_value < 0 || param_value as usize >= program.len() {
                            return Err(format!("parameter index {} in position mode refers to out of range program address: {}", i, param_value));
                        }

                        program[param_value as usize]
                    },
                    ParameterMode::IMMEDIATE => param_value
                };
            }

            return Ok(params.into_boxed_slice());
        }

        fn process_instruction(op_code: &OpCode, params: &Box<[i32]>) -> Result<Option<i32>, String> {
            match op_code.op_type {
                OpType::ADD => Ok(Some(params[0] + params[1])),
                OpType::MULTIPLY => Ok(Some(params[0] * params[1])),
                OpType::INPUT => {
                    let mut line = String::new();

                    stdin().read_line(&mut line).map_err(|err| format!("unable to read from stdin: {}", err))?;

                    line
                        .trim_end()
                        .parse::<i32>()
                        .map_err(|err| err.to_string())
                        .map(Option::Some)
                },
                OpType::OUTPUT => {
                    println!("{}", params[0]);

                    Ok(None)
                }
                OpType::TERMINATE => Ok(None)
            }
        }

        pub fn process(program: &mut [i32]) {
            let program_len = program.len();
            let mut instruction_ptr = 0;
            while instruction_ptr < program_len {
                let op_code_value = program[instruction_ptr];

                let op_code = match OpCode::parse(op_code_value) {
                    Ok(op_code) => op_code,
                    Err(err) => panic!("unable to parse op code: {}: {}", op_code_value, err)
                };

                let params = match resolve_read_parameters(&op_code.read_param_modes, program, instruction_ptr + 1) {
                    Ok(params) => params,
                    Err(err) => panic!("unable to resolve parameters for op code: {}: {}", op_code_value, err)
                };

                match process_instruction(&op_code, &params) {
                    Ok(None) => break,
                    Ok(Some(value)) => {
                        let param_layout = op_code.op_type.parameter_layout();

                        if param_layout.has_write {
                            let write_idx = params[param_layout.num_read] as usize;

                            if write_idx > program_len {
                                panic!("write address for op code {} refers to out of range program address at instruction pointer={}: {}",
                                    op_code_value, instruction_ptr, write_idx)
                            }

                            program[write_idx] = value;
                        }
                    },
                    Err(err) => panic!("error processing value as op code at instruction pointer={}: {}: {}", instruction_ptr, op_code_value, err)
                }

                instruction_ptr += op_code.op_type.num_parameters() + 1;
            }
        }
    }
}