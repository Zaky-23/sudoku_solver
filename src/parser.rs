use std::{
    env,
    fs::File,
    io::Read,
    process::exit
};

pub fn parse_args() -> (String, u32){
    let args: Vec<String> = env::args().collect();
    let file_name;
    let num_of_solutions;

	if args.len() == 1 {
		println!(
			"invalid command: missing arguments; `file_path` `number of solutions`"
		);
		exit(0);
	}

    if args.len() >= 3 {
        file_name = args[1].to_string();
        num_of_solutions = match args[2].parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
                print!("invalid value `number of desired solutions`: ");
                match e.kind() {
                    std::num::IntErrorKind::Empty => 1,
                    std::num::IntErrorKind::InvalidDigit => {
                        println!("invalid digit");
                        exit(1);
                    },
                    std::num::IntErrorKind::NegOverflow => {
                        println!("integer too small");
                        exit(1);
                    },
                    std::num::IntErrorKind::PosOverflow => {
                        println!("integer too large");
                        exit(1);
                    },
                    _ => 1
                }
            }, 
        };
    } else {
        num_of_solutions = 1;
        file_name = String::from("");
    }
    (file_name, num_of_solutions)
}

pub fn parse_file(file: String) -> [[u8; 9]; 9] {
    let mut file = match File::open(file) {
        Ok(f) => f,
        Err(e) => {
            println!("invalid file: {}", e.to_string());
            exit(1)
        }
    };

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    
    let arr = buf.lines()
        .map(|line| {
            let s = line.replace(",", "");
            let s = s.split_whitespace();
            s.collect::<String>()
        })
        .collect::<String>();

    let mut grid = [[0; 9]; 9];

    if arr.len() != 81 {
        println!("invalid input: insufficient input, `{}` different than 81", arr.len());
        exit(1);
    }

    let mut x = 0;
    let mut y = 0;

    for c in arr.chars() {
        let c = match String::from(c).parse::<u8>() {
            Ok(n) => n,
            Err(e) => match e.kind() {
                std::num::IntErrorKind::InvalidDigit => {
                    println!("invalid input: invalid digit `{}`", c);
                    exit(1);
                },
                _ => exit(1)
            }
        };

        if c > 9 {
            println!("invalid input: value greater than `9`");
            exit(1);
        }

        grid[y][x] = c;

        x += 1;

        if x == 9 {
            x = 0;
            y += 1;
        }
    }

    grid
}
