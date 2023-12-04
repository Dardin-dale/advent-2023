use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut result: u32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(val) = line {
                let mut first = '\0';
                let mut found_first = false;
                let mut last = '\0';
                let mut i = 0;
                let mut j = 1;
                while i < val.len() {
                    let c;
                    if j >= val.len() {
                        c = val[i..].chars().next().unwrap();
                    } else {
                        c = val[i..j].chars().next().unwrap();
                    }

                    if is_digit(&c) && !found_first {
                        first = c;
                        last = c;
                        found_first = true;
                        i += 1;
                        j += 1;
                        continue;
                    }
                    if is_digit(&c) {
                        last = c;
                        i += 1;
                        j += 1;
                        continue;
                    }
                    // PART 2
                    if is_worth_parse(&c) {
                        j = j + 2;
                        while j-i < 6 {
                            let num_str;
                            if j >= val.len() {
                                num_str = &val[i..];
                            } else {
                                num_str = &val[i..j];
                            }
                            let num_char = parse_digit(&num_str);
                            if num_char != '\0' {
                                if !found_first {
                                    first = num_char;
                                    last = num_char;
                                    found_first = true;
                                } else {
                                    last = num_char;
                                }
                                i = j - 2;
                                break;
                            }
                            j += 1;
                        }
                        i += 1;
                        j = i + 1;
                        continue;
                    } 
                    i += 1;
                    j += 1;
                }
                let mut s1 = String::from("");
                s1.push(first);
                s1.push(last);
                let n1: u32 = s1.parse().expect(&("Not a Number: ".to_owned()+&s1));
                result += n1;
                // println!("{}, {}, {}", val, n1, result);
            }
        }
    }
    println!("{}", result)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_worth_parse(c: &char) -> bool {
    match c {
        'e'|'f'|'n'|'o'|'s'|'t'|'z' => return true,
        _ => return false
    }
}

fn is_digit(c: &char) -> bool {
    match c {
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => return true,
        _ => return false
    }
}

fn parse_digit(str: &str) -> char {
    match str {
        "zero" => return '0',
        "one" => return '1',
        "two" => return '2',
        "three" => return '3',
        "four" => return '4',
        "five" => return '5',
        "six" => return '6',
        "seven" => return '7',
        "eight" => return '8',
        "nine" => return '9',
        _ => return '\0'
    };
}
