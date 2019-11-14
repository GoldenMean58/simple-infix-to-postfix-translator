use std::io;
use std::process::exit;

struct Parser<'a> {
    input: &'a str,
    lookahead: char,
}

impl<'a> Parser<'a> {
    fn parse(&mut self, input: &'a str) {
        self.input = input;
        self.lookahead = input.chars().next().unwrap();
        self.expr();
    }
    fn match_char(&mut self, c: char) {
        if self.lookahead == c {
            self.input = &self.input[1..];
            self.lookahead = self.input.chars().next().unwrap();
        } else {
            println!("Bad input({} expected): {}", c, self.lookahead); 
            exit(1);
        }
    }
    fn expr(&mut self) {
        self.term();
        self.rest();
    }
    fn rest(&mut self) {
        match self.lookahead {
            '+' => {
                self.match_char('+');
                self.term();
                print!("+ ");
                self.rest();
            },
            '-' => {
                self.match_char('-');
                self.term();
                print!("- ");
                self.rest();
            },
            _ => {},
        };
    }
    fn term(&mut self) {
        match self.lookahead {
            num @ '0' ..= '9' => {
                self.match_char(num);
                print!("{} ", num);
            },
            bad_input @ _ => {println!("Bad input(term expected): {}", bad_input); exit(1)}
        }
    }
}

fn main() {
    let mut p = Parser{input:"", lookahead:'0'};
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    p.parse(&input);
    print!("\n");
}
