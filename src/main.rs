use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io::Read;
use std::time::Instant;

#[derive(PartialEq, Debug)]
enum Symbol {
    PLUS,
    MINUS,
    LEFT(usize),
    RIGHT(usize),
    PUT,
    TAKE,
    START(usize),
    END(usize),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Error reading file!");
    println!("----Read file, found contents to be:----\n{}\n", contents);
    println!("----TETANUS----");

    let start = Instant::now();
    let ops = preprocess(contents);
    println!("Preprocessed ops: {:?}", ops);
    processed_interpret(ops);

    println!("\n----Interpreter complete----");
    println!("Interpreting took: {:.2?}", start.elapsed());
}

fn preprocess(code: String) -> Vec<Symbol> {
    let mut ops: Vec<Symbol> = Vec::new();
    let mut opens: VecDeque<usize> = VecDeque::new();

    let bytes: Vec<char> = code.chars().collect();
    let mut i = 0;

    let mut index: usize = 0;

    while i < bytes.len() {
        match bytes[i] {
            '>' => {
                if index > 0 {
                    match ops[index - 1] {
                        Symbol::RIGHT(count) => {
                            ops[index - 1] = Symbol::RIGHT(count + 1);
                        }
                        _ => {
                            ops.push(Symbol::RIGHT(1));
                            index += 1;
                        }
                    }
                } else {
                    ops.push(Symbol::RIGHT(1));
                    index += 1;
                }
            }
            '<' => {
                if index > 0 {
                    match ops[index - 1] {
                        Symbol::LEFT(count) => {
                            ops[index - 1] = Symbol::LEFT(count + 1);
                        }
                        _ => {
                            ops.push(Symbol::LEFT(1));
                            index += 1;
                        }
                    }
                } else {
                    ops.push(Symbol::LEFT(1));
                    index += 1;
                }
            }
            '+' => {
                ops.push(Symbol::PLUS);
                index += 1;
            }
            '-' => {
                ops.push(Symbol::MINUS);
                index += 1;
            }
            '.' => {
                ops.push(Symbol::PUT);
                index += 1;
            }
            ',' => {
                ops.push(Symbol::TAKE);
                index += 1;
            }
            '[' => {
                opens.push_back(index);
                ops.push(Symbol::START(0));
                index += 1;
            }
            ']' => {
                if opens.len() == 0 {
                    panic!("] found with no preceeding [! Aborting!");
                }

                let last_open = opens.pop_back().unwrap();
                ops[last_open] = Symbol::START(index);
                ops.push(Symbol::END(last_open));
                index += 1;
            }
            _ => {} // comment character
        };

        i += 1;
    }
    return ops;
}

fn processed_interpret(ops: Vec<Symbol>) {
    // Start by intiializing the memory space, memory pointer, program counter, etc.
    // No one seems to agree what happens when the data pointer moves to the left of 0... I'm going to say that's
    // fine and just add a cell wherever it ends up

    // data needs to start with at least one cell to make everything work
    let mut data: Vec<u8> = vec![0];
    let mut data_index: usize = 0;
    let mut pc: usize = 0;

    while pc < ops.len() {
        //println!("Processing op {:?}", ops[pc]);
        match ops[pc] {
            Symbol::PLUS => {
                if data[data_index] == 255 {
                    data[data_index] = 0;
                } else {
                    data[data_index] += 1;
                }
            }
            Symbol::MINUS => {
                if data[data_index] == 0 {
                    data[data_index] = 255;
                } else {
                    data[data_index] -= 1;
                }
            }
            Symbol::LEFT(count) => {
                // check if we're going to underflow initialized memory
                if count > data_index {
                    // need to add more cells to the left
                    for _ in 0..(count - data_index) {
                        data.insert(0, 0);
                    }
                    data_index = 0;
                } else {
                    data_index -= count;
                }
            }
            Symbol::RIGHT(count) => {
                // check if we're about to exceed the length of initialized memory
                if data_index + count >= data.len() {
                    for _ in 0..(data.len() - data_index + (count - 1)) {
                        data.push(0);
                    }
                } else {
                    data_index += count;
                }
                if data_index == data.len() - 1 {
                    data.push(0);
                }
            }
            Symbol::PUT => print!("{}", data[data_index] as char),
            Symbol::TAKE => {
                let input = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u8);
                data[data_index] = input.unwrap();
            }
            Symbol::START(index) => {
                if data[data_index] == 0 {
                    pc = index;
                }
            }
            Symbol::END(index) => {
                if data[data_index] != 0 {
                    pc = index;
                }
            }
        }

        pc += 1;
    }
}

#[allow(dead_code)]
fn old_interpret(code: String) {
    // Start by intiializing the memory space, memory pointer, program counter, etc.
    // No one seems to agree what happens when the data pointer moves to the left of 0... I'm going to say that's
    // fine and just add a cell wherever it ends up

    // data needs to start with at least one cell to make everything work
    let mut data: Vec<u8> = vec![0];
    let mut data_index: usize = 0;
    let mut pc: usize = 0;

    // It would be *really cool* to preprocess the code String into a vector of some kind of
    // operation tokens that could be then run more quickly, without requiring character comparisons
    // every op, but that's gonna be a later problem for me to implement
    let ops: Vec<char> = code.chars().collect();

    while pc < ops.len() {
        // perform the correct action for the current operation
        match ops[pc] {
            '>' => {
                // check if we're about to exceed the length of initialized memory
                if data_index == data.len() - 1 {
                    data.push(0);
                }

                data_index += 1;
            }
            '<' => {
                // check if we're going to underflow initialized memory
                if data_index == 0 {
                    data.insert(0, 0);
                } else {
                    data_index -= 1;
                }
            }
            '+' => {
                if data[data_index] == 255 {
                    data[data_index] = 0;
                } else {
                    data[data_index] += 1;
                }
            }
            '-' => {
                if data[data_index] == 0 {
                    data[data_index] = 255;
                } else {
                    data[data_index] -= 1;
                }
            }
            '.' => print!("{}", data[data_index] as char),
            ',' => {
                let input = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u8);
                data[data_index] = input.unwrap();
            }
            '[' => {
                if data[data_index] == 0 {
                    // jump forwards, making PC the byte of the closing ] character, so I can increment PC later
                    // would be more efficient if we preprocessed the code and matched up brackets,
                    // for now just look ahead for the first unmatched closing bracket
                    let mut count = 0;
                    let mut i = 1;

                    while pc + i < ops.len() {
                        match ops[pc + i] {
                            '[' => count += 1,
                            ']' => {
                                if count == 0 {
                                    break;
                                } else {
                                    count -= 1;
                                }
                            }
                            _ => (),
                        }

                        i += 1;
                    }
                    if pc + i >= ops.len() {
                        // This is an error case, the brackets in the program aren't matched up
                        // Ideally a preprocessor should catch this but I haven't written one yet
                        panic!("Unmatched [ found in program, aborting!");
                    }

                    // set the program counter to the closing brace
                    pc += i;
                }
            }
            ']' => {
                if data[data_index] != 0 {
                    // copied the code for ] and changed as needed
                    let mut count = 0;
                    let mut i = 1;

                    loop {
                        // Check for underflow error first
                        if (pc as isize - i as isize) < 0 {
                            panic!("Unmatched ] found in program, aborting!")
                        }
                        match ops[pc - i] {
                            ']' => count += 1,
                            '[' => {
                                if count == 0 {
                                    break;
                                } else {
                                    count -= 1;
                                }
                            }
                            _ => (),
                        }

                        i += 1;
                    }

                    // set the program counter to the opening brace
                    pc -= i;
                }
            }
            _ => (), // comment character, skip it
        }

        // increment the program counter
        pc += 1;
    }
}
