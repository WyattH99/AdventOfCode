use regex::Regex;
use std::fs;

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
    org_b: usize,
    org_c: usize,
    expected: Vec<usize>,
}

impl Program {
    fn new() -> Self {
        //let data = fs::read_to_string("example.txt").unwrap();
        //let data = fs::read_to_string("example2.txt").unwrap();
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

        let expected = instructions.to_vec();

        let instructions = instructions
            .chunks(2)
            .map(|chunk| Instruction::new(chunk[0], chunk[1]))
            .collect();

        Self {
            reg_a: *reg_a,
            reg_b: *reg_b,
            reg_c: *reg_c,
            instructions,
            org_b: *reg_b,
            org_c: *reg_c,
            expected,
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

    fn solve(&mut self) -> usize {
        let mut i = 0;
        let mut output_index = 0;

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
                5 => {
                    let o = combo % 8;
                    if self.expected.get(output_index).is_some_and(|&e| e == o) {
                        output_index += 1;
                    } else {
                        if output_index > self.expected.len() {
                            output_index += 1;
                        }
                        return output_index;
                    }
                }
                _ => panic!("solve"),
            }
        }
        output_index
    }

    fn reset(&mut self) {
        self.reg_b = self.org_b;
        self.reg_c = self.org_c;
    }
}

fn main() {
    let mut program = Program::new();
    let goal = program.expected.len();
    let mut guaranteed_bits = vec![];
    println!("goal: {}", goal);

    for min in 1..goal {
        let mut potential_as = vec![];
        for a in 0..10_000_000 {
            let mut a = a;
            for (index, bit) in guaranteed_bits.iter() {
                a = match bit {
                    Bit::One => insert_one_at_index(a, *index),
                    Bit::Zero => insert_zero_at_index(a, *index),
                }
            }

            program.reg_a = a;
            program.reset();
            let result = program.solve();
            if result > min {
                potential_as.push(a);
                if result == goal {
                    println!("a: {}", a);
                    return;
                }
            }
        }

        guaranteed_bits = common_bit_indices(&potential_as);
        println!("found: {} guaranteed_bits", guaranteed_bits.len());
    }

    // 7,4,2,5,1,4,6,0,4
    // a: 1775905381641917 too high
    // a: 164278764924605
}

#[derive(Debug, PartialEq, Eq)]
enum Bit {
    One,
    Zero,
}

fn common_bit_indices(nums: &Vec<usize>) -> Vec<(usize, Bit)> {
    let mut result = Vec::new();
    let mut zeroes = Vec::new();

    for bit_index in 0..usize::BITS as usize {
        if nums.iter().all(|&num| (num & (1 << bit_index) != 0)) {
            result.push((bit_index, Bit::One));
        }
        if nums.iter().all(|&num| (num & (1 << bit_index) == 0)) {
            zeroes.push(bit_index);
        }
    }

    zeroes.reverse();

    for bit_index in (0..usize::BITS as usize).rev() {
        if zeroes.get(0).is_some_and(|z| *z == bit_index) {
            zeroes.remove(0);
        } else {
            break;
        }
    }

    for zero in zeroes.iter() {
        result.push((*zero, Bit::Zero));
    }
    result.sort_by_key(|e| e.0);
    result
}

fn insert_one_at_index(n: usize, index: usize) -> usize {
    let mask = !0 << index;
    let higher_bits = n & mask;
    let lower_bits = n & !mask;
    (higher_bits << 1) | (1 << index) | lower_bits
}
fn insert_zero_at_index(n: usize, index: usize) -> usize {
    let mask = !0 << index;
    let higher_bits = n & mask;
    let lower_bits = n & !mask;
    (higher_bits << 1) | lower_bits
}

// Unit tests that are ran with "cargo test"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_bit_indicies() {
        let numbers = vec![0b101, 0b100101, 0b111101];
        let result = common_bit_indices(&numbers);
        let expected = vec![(0, Bit::One), (1, Bit::Zero), (2, Bit::One)];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_insert_one_at_index() {
        let n = 0b1001011101;
        let i = 2;
        assert_eq!(0b10010111101, insert_one_at_index(n, i));
    }

    #[test]
    fn test_insert_zero_at_index() {
        let n = 0b1001011101;
        let i = 2;
        assert_eq!(0b10010111001, insert_zero_at_index(n, i));
    }
}
