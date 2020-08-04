use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
	// File hosts must exist in current path before this produces output
	if let Ok(lines) = read_lines("./test.adoc") {
		// Consumes the iterator, returns an (Optional) String
		for (row, line) in lines.enumerate() {
			if let Ok(ip) = line {
				for (col, letter) in ip.chars().enumerate() {
					if let Ok(type_name) = get_type_name(letter) {
						let token = Token {
							type_name,
							raw: letter,
							pos: Position {
								col: col as u32,
								row: row as u32 + 1,
							},
						};
						println!(
							"COL {}, ROW {}, RAW: {}, TYPE: {} ",
							token.pos.col,
							token.pos.row,
							token.raw,
							token.type_name.name()
						);
					} else {
						println!("ERROR")
					}
				}
			}
		}
	}
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
	P: AsRef<Path>,
{
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

macro_rules! enum_str {
    (enum $name:ident {
        $($variant:ident),*,
    }) => {
        enum $name {
            $($variant),*
        }

        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

enum_str! {
	enum Types {
		Section,
		Pargraph,
	}
}

struct Token {
	type_name: Types,
	raw: char,
	pos: Position,
}

struct Position {
	col: u32,
	row: u32,
}

fn get_type_name(parsed: char) -> Result<Types, String> {
	match parsed as u8 {
		b'#' => Ok(Types::Section),
		_ => Ok(Types::Pargraph),
	}
}
