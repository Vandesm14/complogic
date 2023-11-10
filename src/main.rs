use std::collections::HashMap;
use std::fmt::Debug;

type Pins = HashMap<usize, bool>;

#[derive(Debug)]
enum Gate {
  And {
    inputs: [usize; 2],
    outputs: [usize; 1],
  },
}

impl Gate {
  fn eval(&self, pins: &Pins) -> Pins {
    match self {
      Self::And { inputs, outputs } => {
        let [pin_a, pin_b] = inputs;

        let a = pins.get(pin_a).unwrap();
        let b = pins.get(pin_b).unwrap();
        let result = *a && *b;

        let mut new_pins = pins.clone();
        new_pins.insert(outputs[0], result);

        new_pins
      }
    }
  }
}

fn main() {
  let pins: Pins = Pins::from_iter(vec![(0, true), (1, true)]);
  let and_gate = Gate::And {
    inputs: [0, 1],
    outputs: [2],
  };

  let result = and_gate.eval(&pins);

  println!("{:?}", result);
}
