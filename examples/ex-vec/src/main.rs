#[derive(Debug)]

pub struct VM {
  registers: [i32; 32],
  pc: usize,
  program: Vec<u8>,
}

impl VM {
  pub fn new() -> VM {
    VM {
      registers: [0; 32],
      program: vec![],
      pc: 0,
    }
  }

  pub fn next_8_bits(&mut self) -> u8 {
    let result = self.program[self.pc];
    self.pc += 1;
    return result;
  }

  pub fn next_16_bits(&mut self) -> u16 {
    let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
    self.pc += 2;
    return result;
  }

  pub fn get_test_vm() -> VM {
    let mut test_vm = VM::new();
    //test_vm.registers[0] = 1;
    //test_vm.registers[1] = 10;
    test_vm
  }
}

fn main() {
  let t = vec![1, 0, 1, 244];
  let mut test_vm = VM::get_test_vm();
  test_vm.program = vec![1, 0, 1, 244];
  test_vm.pc += 1;
  // Remember, this is how we represent 500 using
  // two u8s in little endian format
  let register = test_vm.next_8_bits() as usize;
  let number = test_vm.next_16_bits() as u16;
  println!("{register}");
  println!("{number}");
  test_vm.registers[register] = number as i32;
}

