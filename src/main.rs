use std::rc::Rc;

use complogic::{FullAdder, Simulation};

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

  let full_adder_1 = Rc::new(FullAdder {
    a: a1,
    b: b1,
    cin: simulation.alloc(),
    s: simulation.alloc(),
    cout: simulation.alloc(),
  });
  let full_adder_2 = Rc::new(FullAdder {
    a: a2,
    b: b2,
    cin: full_adder_1.cout,
    s: simulation.alloc(),
    cout: simulation.alloc(),
  });
  let full_adder_3 = Rc::new(FullAdder {
    a: a3,
    b: b3,
    cin: full_adder_2.cout,
    s: simulation.alloc(),
    cout: simulation.alloc(),
  });
  let full_adder_4 = Rc::new(FullAdder {
    a: a4,
    b: b4,
    cin: full_adder_3.cout,
    s: simulation.alloc(),
    cout: simulation.alloc(),
  });

  simulation.compile(vec![
    full_adder_1.clone(),
    full_adder_2.clone(),
    full_adder_3.clone(),
    full_adder_4.clone(),
  ]);

  let a = 0b0101;
  let b = 0b0011;

  simulation.run(
    &number_to_bin_vec(a, 4)
      .into_iter()
      .chain(number_to_bin_vec(b, 4))
      .collect::<Vec<_>>(),
  );

  println!("Expected: {:04b}", a + b);

  println!("1: {}", simulation.register(full_adder_1.s));
  println!("2: {}", simulation.register(full_adder_2.s));
  println!("3: {}", simulation.register(full_adder_3.s));
  println!("4: {}", simulation.register(full_adder_4.s));
  println!("cout: {}", simulation.register(full_adder_4.cout));
}
