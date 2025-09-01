use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

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

struct VM {
    runniing: bool,
    stack: [i32; STACK_SIZE],
    registers: [i32; NUM_OF_REGISTERS],
}

impl VM {
    fn new() -> Self {
        let mut vm = VM {
            runniing: false,
            stack: [0; STACK_SIZE],
            registers: [0; NUM_OF_REGISTERS],
        };
        vm.registers[SP] = -1; //stack pointer at -1;
        vm.registers[IP] = 0; // instruction pointer at 0;
        vm
    }

    fn sp(&self) -> i32 {
        self.registers[SP]
    }

    fn sp_mut(&mut self) -> &mut i32 {
        &mut self.registers[SP]
    }

    fn ip(&self) -> i32 {
        self.registers[IP]
    }

    fn ip_mut(&mut self) -> &mut i32 {
        &mut self.registers[IP]
    }
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
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        let mut i = 0;
        while i < tokens.len() {
            // if it is push then push opcode + next value eg: PSH 10 as [0 10] in stack;
            let tok = tokens[i];
            let op_code = string_to_instruction(tok);
            if op_code == Instruction::PSH {
                if i + 1 < tokens.len() {
                    if let Ok(num) = tokens[i + 1].parse::<i32>() {
                        program.push(Instruction::PSH as i32);
                        program.push(num);
                    }
                    i += 2;
                    continue;
                } else {
                    i += 1;
                    continue;
                }
            } else if op_code != Instruction::UNK {
                // if it is not unkown push in the Instruction Array;
                program.push(op_code as i32);
            } else {
                // if unkown skip this ;
                i += 1;
                continue;
            }
            i += 1;
        }
    }

    Ok(program)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Improper use : use {} <filename>.vm", args[0]);
        return Ok(());
    }
    let filename = &args[1];
    if Path::new(filename).extension() != Some(OsStr::new("vm")) {
        eprintln!("Error: Only .vm files are accepted.");
        return Ok(());
    }

    let program = load_program(filename).unwrap();
    println!("{:?} ", program);

    let mut vm = VM::new();
    println!("Initial IP: {}, SP: {}", vm.ip(), vm.sp());

    Ok(())
}
