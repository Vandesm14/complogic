use std::rc::Rc;

use complogic::{FourBitAdder, FullAdder, Simulation};

fn number_to_bin_vec(number: usize, size: usize) -> Vec<bool> {
  let mut vec = vec![];
  let mut number = number;
  while number > 0 {
    vec.push(number % 2 == 1);
    number /= 2;
  }

  while vec.len() < size {
    vec.push(false);
  }

  vec.reverse();
  vec
}

fn main() {
  let mut simulation = Simulation::new(8);
  let [a4, a3, a2, a1, b4, b3, b2, b1] = [0, 1, 2, 3, 4, 5, 6, 7];

  simulation.incrementer.next_n(5);
  let [s4, s3, s2, s1, cout] = [8, 9, 10, 11, 12];

  let four_bit_adder = Rc::new(FourBitAdder {
    a1,
    a2,
    a3,
    a4,
    b1,
    b2,
    b3,
    b4,
    s1,
    s2,
    s3,
    s4,
    cout,
  });

  simulation.compile(vec![four_bit_adder.clone()]);

  let a = 0b0101;
  let b = 0b0011;

  simulation.run(
    &number_to_bin_vec(a, 4)
      .into_iter()
      .chain(number_to_bin_vec(b, 4))
      .collect::<Vec<_>>(),
  );

  println!("Expected: {:04b}", a + b);

  println!("4: {}", simulation.register(four_bit_adder.s4));
  println!("3: {}", simulation.register(four_bit_adder.s3));
  println!("2: {}", simulation.register(four_bit_adder.s2));
  println!("1: {}", simulation.register(four_bit_adder.s1));
  println!("cout: {}", simulation.register(four_bit_adder.cout));
}
