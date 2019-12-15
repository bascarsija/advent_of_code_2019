pub mod input {
    use std::env;
    use std::io::prelude::*;
    use std::fs::File;
    use std::io::{BufReader, Lines};

    pub fn single_line_from_arg_file() -> String {
        match lines_from_arg_file().next() {
            Some(Ok(line)) => line,
            Some(Err(err)) => panic!("error reading line: {}", err),
            None => panic!("expected single input line")
        }
    }

    pub fn lines_from_arg_file() -> Lines<BufReader<File>> {
        let args: Vec<String> = env::args().collect();

        if args.len() != 2 {
            panic!("expected exactly one filename argument");
        }

        let filename =
            match args.get(1) {
                Some(filename) => filename,
                None => panic!("unable to get filename argument")
            };

        let file = match File::open(filename) {
            Ok(file) => file,
            Err(err) => panic!("{}: {}", filename, err)
        };

        BufReader::new(file).lines()
    }
}
