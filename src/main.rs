#![allow(dead_code)]

mod character_set;
mod fsm;
mod nfa_matcher;
mod parser;

use std::io;
use std::io::{BufRead, BufReader};

use fsm::*;
use nfa_matcher::*;
use parser::*;

fn main() {
    let reader = BufReader::new(io::stdin());
    let mut lines = reader.lines();
    let line = lines.next().unwrap().unwrap();

    let y = parse_regex(&line).unwrap();

    let state_machine = create_fsm(y).unwrap();
    println!("{line} parses into \n{:#?}", state_machine);

    for line in lines {
        let input: Vec<u8> = line.unwrap().chars().map(|x| x as u8).collect();
        let ret = match_input(&input, &state_machine, state_machine.entry_state);
        println!("Ret = {:?}", ret);
    }
}
