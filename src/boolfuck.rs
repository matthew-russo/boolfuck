use std::collections::VecDeque;

struct Interpreter {
    code: Vec<Token>,
    input: Vec<u8>,
    code_pointer: usize,
    data_pointer: usize,
    data: VecDeque<u8>,
    output: Vec<u8>,
}

impl Interpreter {
    fn new(code: &str, input: Vec<u8>) -> Interpreter {
        let mut interpreter = Interpreter {
            code: code.to_token(),
            input,
            code_pointer: 0usize,
            data_pointer: 15000,
            data: VecDeque::new(),
            output: Vec::new(),
        };

        for _n in 0..30000 {
            interpreter.data.push_back(0u8);
        }

        interpreter
    }

    fn increment_pointer(&mut self) {
        if self.data_pointer == self.data.len() - 1 {
            self.data.push_back(0u8);
        }

        self.data_pointer += 1;
    }

    fn decrement_pointer(&mut self) {
        if self.data_pointer == 0 {
            // add element to list and keep pointer at 0;
            self.data.push_front(0u8);
            return;
        }

        self.data_pointer -= 1;
    }

    fn flip_bit(&mut self) {
        if self.data[self.data_pointer] == 0 {
            self.data[self.data_pointer] = 1;
        } else {
            self.data[self.data_pointer] = 0;
        }
    }

    fn output(&mut self) {
        self.output.push(self.data[self.data_pointer]);
    }

    fn input(&mut self) {
        self.data[self.data_pointer] = self.input.remove(0);
    }

    fn jump_forwards(&mut self) {
        if self.data[self.data_pointer] != 0u8 {
            return;
        }

        let mut needed_count = 1;
        while needed_count > 0 {
            self.code_pointer += 1;

            if self.code.get(self.code_pointer).unwrap() == &Token::BracketLeft {
                needed_count += 1;
            }

            if self.code.get(self.code_pointer).unwrap() == &Token::BracketRight {
                needed_count -= 1;
            }
        }
    }

    fn jump_backwards(&mut self) {
        if self.data[self.data_pointer] != 1u8 {
            return;
        }

        let mut needed_count = 1;
        while needed_count > 0 {
            self.code_pointer -= 1;

            if self.code.get(self.code_pointer).unwrap() == &Token::BracketRight {
                needed_count += 1;
            }

            if self.code.get(self.code_pointer).unwrap() == &Token::BracketLeft {
                needed_count -= 1;
            }
        }
    }

    fn run(&mut self) -> Vec<u8> {
        while self.code_pointer < self.code.len() {
            match self.code.get(self.code_pointer).unwrap() {
                &Token::MoveLeft     => self.increment_pointer(),
                &Token::MoveRight    => self.decrement_pointer(),
                &Token::Flip         => self.flip_bit(),
                &Token::Write        => self.output(),
                &Token::Read         => self.input(),
                &Token::BracketLeft  => self.jump_forwards(),
                &Token::BracketRight => self.jump_backwards(),
                _ => (),
            }
            
            self.code_pointer += 1;
        }

        let mut final_output: Vec<u8> = Vec::new();

        while self.output.len() > 8 {
            let buf = self.output.split_off(8);
            let binary_string: String = self.output.iter().rev().fold(String::new(), |acc, num| acc + &num.to_string());
            let intval = u8::from_str_radix(&binary_string, 2).unwrap();
            final_output.push(intval);
            self.output = buf;
        }

        while self.output.len() < 8 {
            self.output.push(0u8);
        }

        let binary_string: String = self.output.iter().rev().fold(String::new(), |acc, num| acc + &num.to_string());
        let intval = u8::from_str_radix(&binary_string, 2).unwrap();
        final_output.push(intval);

        final_output
    }
}

pub fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut binary_input: Vec<u8> = Vec::new();

    for n in input {
        let mut binary_string = format!("{:b}", n);

        if binary_string.len() < 8 {
            binary_string = binary_string.chars().rev().collect();
            while binary_string.len() < 8 {
                binary_string.push('0');
            }
        }

        for c in binary_string.chars() {
            binary_input.push(c.to_string().parse::<u8>().unwrap());
        }
    }

    let mut interpreter = Interpreter::new(code, binary_input);
    interpreter.run()
}





#[derive(PartialEq,Copy,Clone)]
pub enum Token {
    MoveLeft,
    MoveRight,
    BracketLeft,
    BracketRight,
    Read,
    Write,
    Flip,
    EOF,
}

/// Takes `&self` and creates a new independent string.
pub trait ToString {
    fn to_string(&self) -> String;
}

/// Takes `&self` and creates a new independent `Vec<Token>`.
pub trait ToToken {
    fn to_token(&self) -> Vec<Token>;
}


impl ToString for Vec<Token> {
    fn to_string(&self) -> String {
        let mut    res = String::new();
        for item in self.iter() {
            use self::Token::*;
            match *item {
                MoveLeft => res.push('<'),
                MoveRight => res.push('>'),
                BracketLeft => res.push('['),
                BracketRight => res.push(']'),
                Read => res.push(','),
                Write => res.push(';'),
                Flip => res.push('+'),
                EOF => res.push('\0'),
            }
        }
        res
    }
}

impl ToToken for str {
    fn to_token(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut brackets: i32 = 0;

        for character in self.chars() {
            match character {
                '<'  => tokens.push(Token::MoveLeft),
                '>'  => tokens.push(Token::MoveRight),
                '['  => { brackets += 1; tokens.push(Token::BracketLeft); },
                ']'  => {
                    if brackets > 0 {
                        brackets -= 1;
                        tokens.push(Token::BracketRight);
                    } else {
                        panic!("Found a right bracket without a preceding left one!");
                    }
                },
                ','  => tokens.push(Token::Read),
                ';'  => tokens.push(Token::Write),
                '+'  => tokens.push(Token::Flip),
                '\0' => tokens.push(Token::EOF),
                _ => (),
            }
        }
        tokens.push(Token::EOF);


        if brackets != 0 {
            panic!("There are currently {} bracket(s) without a matching partner!",brackets);
        }
            else {
                tokens
            }
    }
}
