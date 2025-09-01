use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    PSH = 0,
    POP = 1,
    ADD = 2,
    SUB = 3,
    MUL = 4,
    DIV = 5,
    SET = 6,
    HLT = 7,
    UNK = 9,
}

const STACK_SIZE: usize = 256;

//registers
const A: usize = 0;
const B: usize = 1;
const D: usize = 2;
const E: usize = 3;
const F: usize = 4;
const G: usize = 5;
const IP: usize = 6; //instruction pointer -- for op codes
const SP: usize = 7; //stack pointer --for values
const NUM_OF_REGISTERS: usize = 8;

fn string_to_instruction(token: &str) -> Instruction {
    match token {
        "PSH" => Instruction::PSH,
        "POP" => Instruction::POP,
        "ADD" => Instruction::ADD,
        "SUB" => Instruction::SUB,
        "MUL" => Instruction::MUL,
        "DIV" => Instruction::DIV,
        "SET" => Instruction::SET,
        "HLT" => Instruction::HLT,
        _ => Instruction::UNK,
    }
}

fn load_program(filename: &str) -> io::Result<Vec<i32>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut program: Vec<i32> = Vec::new();
    for line_res in reader.lines() {
        let mut line = line_res?;
        if let Some(idx) = line.find('#') {
            line.truncate(idx);
        }
        let line = line.trim();
        if line.is_empty(){
            continue;
        }
        
        let tokens : Vec<&str> = line.split_whitespace().collect();
        let mut i = 0;
        while i < tokens.len(){ // if it is push then push opcode + next value eg: PSH 10 as [0 10] in stack;
            let tok = tokens[i];
            let op_code = string_to_instruction(tok);
            if op_code == Instruction::PSH{
                if i+1 < tokens.len(){
                    if let Ok(num) = tokens[i + 1].parse::<i32>(){
                        program.push(Instruction::PSH as i32);
                        program.push(num);
                    }
                    i += 2;
                    continue;
                }else {
                    i += 1;
                    continue;
                }
            }else if op_code != Instruction::UNK{ // if it is not unkown push in the Instruction Array;
                program.push(op_code as i32);
            }else{ // if unkown skip this ;
                i +=1;
                continue;
            }
            i += 1;
        }
    }

    Ok(program)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Improper use : use {} <filename>.vm", args[0]);
        return;
    }
    let filename = &args[1];

    println!("{:?}", args);
}
