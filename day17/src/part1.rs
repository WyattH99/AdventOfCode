use regex::Regex;
use std::fs;

struct Instruction {
    opcode: usize,
    operand: usize,
}

impl Instruction {
    fn new(opcode: usize, operand: usize) -> Self {
        Self { opcode, operand }
    }
}

struct Program {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    instructions: Vec<Instruction>,
}

impl Program {
    fn new() -> Self {
        //let data = fs::read_to_string("example.txt").unwrap();
        let data = fs::read_to_string("input.txt").unwrap();
        let data = data.trim();
        println!("data: {}\n", data);

        let re = Regex::new(r"\d+").unwrap();
        let data: Vec<_> = re
            .captures_iter(data)
            .map(|cap| cap[0].parse().unwrap())
            .collect();

        let [reg_a, reg_b, reg_c, instructions @ ..] = &data[..] else {
            panic!("program new");
        };
        let instructions = instructions
            .chunks(2)
            .map(|chunk| Instruction::new(chunk[0], chunk[1]))
            .collect();

        Self {
            reg_a: *reg_a,
            reg_b: *reg_b,
            reg_c: *reg_c,
            instructions,
        }
    }

    fn combo_operand(&self, operand: usize) -> usize {
        match operand {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => operand,
        }
    }

    fn solve(&mut self) {
        let mut i = 0;
        let mut out: Vec<_> = Vec::new();

        while let Some(instruction) = self.instructions.get(i) {
            i += 1;
            let combo = self.combo_operand(instruction.operand);
            let literal = instruction.operand;
            match instruction.opcode {
                0 => self.reg_a = self.reg_a >> combo,
                6 => self.reg_b = self.reg_a >> combo,
                7 => self.reg_c = self.reg_a >> combo,
                1 => self.reg_b = self.reg_b ^ literal,
                2 => self.reg_b = combo % 8,
                3 => {
                    if self.reg_a != 0 {
                        i = literal / 2;
                    }
                }
                4 => self.reg_b = self.reg_b ^ self.reg_c,
                5 => out.push(combo % 8),
                _ => panic!("solve"),
            }
        }
        let str: String = out
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",");
        println!("str: {}", str);
    }
}

fn main() {
    /*
     *  Operands:
     *  0 - 3 are literal
     *  4 => reg_a
     *  5 => reg_b
     *  6 => reg_c
     *  7 => reserved
     *
     * Opcodes:
     *   adv -> 0 => division: reg_a = reg_a / 2^operand
     *   bxl -> 1 => bitwise XOR: reg_b = reg_b xor literal_operand
     *   bst -> 2 => modulo: reg_b = operand mod 8
     *   jnz -> 3 => jump: if reg_a == 0 then nothing else jump instruction pointer to the
     *   value of its literal operand and dont increase instruction pointer by 2
     *   bxc -> 4 => bitwise XOR: reg_b = reg_b xor reg_c
     *   out -> 5 => output modulo: print = operand mod 8
     *   bdv -> 6 => division: reg_b = reg_a / 2^operand
     *   cdv -> 7 => division: reg_c = reg_a / 2^operand
     */

    let mut program = Program::new();
    program.solve();
    // 7,4,2,5,1,4,6,0,4
}
