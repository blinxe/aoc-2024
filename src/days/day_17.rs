use crate::utils::input::read_input;

type Register = usize;
type Opcode = u8;

#[derive(Debug)]
struct VirtualMachine {
    a: Register,
    b: Register,
    c: Register,
    pc: usize,
    program: Vec<Opcode>,
    output: Vec<u8>,
}

impl VirtualMachine {
    fn new(input: &str) -> Self {
        fn _next_value<'a>(iter: &'a mut core::str::Lines) -> &'a str {
            iter.next().unwrap().split(": ").last().unwrap()
        }
        let mut lines = input.lines();
        let a = _next_value(&mut lines).parse().unwrap();
        let b = _next_value(&mut lines).parse().unwrap();
        let c = _next_value(&mut lines).parse().unwrap();
        lines.next(); // skip empty line
        let program = _next_value(&mut lines)
            .split(',')
            .map(|c| c.parse::<Opcode>().unwrap())
            .collect();

        Self {
            a: a,
            b: b,
            c: c,
            pc: 0,
            program: program,
            output: Vec::new(),
        }
    }

    fn reset(&mut self, a: Register) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.pc = 0;
        self.output.clear();
    }

    fn get_combo_operand(&mut self) -> usize {
        let op = self.program[self.pc];
        self.pc += 1;

        match op {
            0..=3 => op as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        }
    }

    fn run_adv(&mut self) {
        let num = self.a;
        let denom = 1 << self.get_combo_operand();
        self.a = num / denom;
    }
    fn run_bxl(&mut self) {
        self.b = self.b ^ (self.program[self.pc] as usize);
        self.pc += 1;
    }
    fn run_bst(&mut self) {
        self.b = self.get_combo_operand() % 8;
    }
    fn run_jnz(&mut self) {
        if self.a != 0 {
            self.pc = self.program[self.pc] as usize;
        } else {
            self.pc += 1;
        }
    }
    fn run_bxc(&mut self) {
        self.b = self.b ^ self.c;
        self.pc += 1; // legacy
    }
    fn run_out(&mut self) {
        let n = (self.get_combo_operand() % 8) as u8;
        self.output.push(n);
    }
    fn run_bdv(&mut self) {
        let num = self.a;
        let denom = 1 << self.get_combo_operand();
        self.b = num / denom;
    }
    fn run_cdv(&mut self) {
        let num = self.a;
        let denom = 1 << self.get_combo_operand();
        self.c = num / denom;
    }

    fn run_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        let instruction = self.program[self.pc];
        self.pc += 1;
        match instruction {
            0 => self.run_adv(),
            1 => self.run_bxl(),
            2 => self.run_bst(),
            3 => self.run_jnz(),
            4 => self.run_bxc(),
            5 => self.run_out(),
            6 => self.run_bdv(),
            7 => self.run_cdv(),
            _ => panic!(),
        }

        true
    }

    fn run_program(&mut self) {
        while self.run_instruction() {}
    }
}

fn solve_part_1(input: &str) {
    let mut vm = VirtualMachine::new(input);
    println!("{:?}", vm);
    vm.run_program();
    let output_str = vm
        .output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("Program output: {}", output_str);
}

fn get_number_from_octal_digits(digits: &[u8]) -> Register {
    digits
        .iter()
        .fold(0, |acc, &digit| acc * 8 + digit as usize)
}

fn recurse(vm: &mut VirtualMachine, a_od: &mut [u8], index: usize) -> bool {
    let start = if index == 0 { 1 } else { 0 };
    for digit in start..=7 {
        a_od[index] = digit;
        vm.reset(get_number_from_octal_digits(&a_od));
        vm.run_program();
        if vm.output[vm.program.len() - index - 1] != vm.program[vm.program.len() - index - 1] {
            continue;
        }
        if index == vm.program.len() - 1 {
            return true;
        }
        if recurse(vm, a_od, index + 1) {
            return true;
        }
    }
    a_od[index] = 0;
    false
}

fn solve_part_2(input: &str) {
    let mut vm = VirtualMachine::new(input);

    // tentative A as a list of octal digits
    let mut a_od = vec![0u8; vm.program.len()];
    recurse(&mut vm, &mut a_od, 0);

    println!(
        "Replicator A: {} ({:?})",
        get_number_from_octal_digits(&a_od),
        &a_od
    );
}

pub fn part_1() {
    let input = read_input(module_path!());
    solve_part_1(input.as_str());
}

pub fn part_2() {
    let input = read_input(module_path!());
    solve_part_2(input.as_str());
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE_1: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = indoc! {"
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
    "};

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
