use std::collections::HashMap;
use std::fmt::Debug;

type Pins = HashMap<usize, bool>;
type FlatPins = Vec<(usize, bool)>;

type FlatBoolOps = Vec<(usize, BoolOp)>;

enum BoolOp {
  And(Box<BoolOp>, Box<BoolOp>),
  Or(Box<BoolOp>, Box<BoolOp>),
  Not(Box<BoolOp>),

  Pin(usize),
}

impl BoolOp {
  fn eval(&self, pins: &Pins) -> bool {
    match self {
      Self::And(a, b) => {
        let a = a.eval(pins);
        let b = b.eval(pins);

        a && b
      }
      Self::Or(a, b) => {
        let a = a.eval(pins);
        let b = b.eval(pins);

        a || b
      }

      Self::Not(a) => {
        let a = a.eval(pins);

        !a
      }
      Self::Pin(pin) => *pins.get(pin).unwrap_or(&false),
    }
  }
}

#[derive(Debug)]
enum Gate {
  And {
    inputs: [usize; 2],
    outputs: [usize; 1],
  },
  Not {
    inputs: [usize; 1],
    outputs: [usize; 1],
  },
}

impl Gate {
  fn eval(&self, pins: &Pins) -> FlatBoolOps {
    match self {
      Self::And { inputs, outputs } => {
        let [pin_a, pin_b] = inputs;

        let result = BoolOp::And(
          Box::new(BoolOp::Pin(*pin_a)),
          Box::new(BoolOp::Pin(*pin_b)),
        );

        vec![(outputs[0], result)]
      }
      Self::Not { inputs, outputs } => {
        let pin_a = &inputs[0];

        let result = BoolOp::Not(Box::new(BoolOp::Pin(*pin_a)));

        vec![(outputs[0], result)]
      }
    }
  }
}

#[derive(Debug)]
struct Simulation {
  /// All inputs and outputs (pins)
  pins: Pins,

  /// All gates
  gates: Vec<Gate>,
}

impl Simulation {
  /// Step through the simulation once
  fn step(&mut self) {
    self.gates.iter().for_each(|gate| {
      // Eval the current gate
      let result = gate.eval(&self.pins);

      // Update each pin in the map
      result.iter().for_each(|(pin, val)| {
        self.pins.insert(*pin, val.eval(&self.pins));
      });
    })
  }
}

fn main() {
  let and_gate = Gate::And {
    inputs: [0, 1],
    outputs: [2],
  };

  let not_gate = Gate::Not {
    inputs: [2],
    outputs: [3],
  };

  let mut simulation = Simulation {
    pins: Pins::from_iter(vec![(0, true), (1, true)]),
    gates: vec![and_gate, not_gate],
  };

  simulation.step();

  println!("Pins: {:?}", simulation.pins);
}

#[cfg(test)]
mod tests {
  #[test]
  fn and_and_not_gate() {
    use super::*;
    let and_gate = Gate::And {
      inputs: [0, 1],
      outputs: [2],
    };

    let not_gate = Gate::Not {
      inputs: [2],
      outputs: [3],
    };

    let mut simulation = Simulation {
      pins: Pins::from_iter(vec![(0, true), (1, true)]),
      gates: vec![and_gate, not_gate],
    };

    simulation.step();

    // The output pin of the AND gate
    assert_eq!(simulation.pins.get(&2), Some(&true));

    // The output pin of the NOT gate
    assert_eq!(simulation.pins.get(&3), Some(&false));
  }

  #[test]
  fn and_and_not_gate_and_false() {
    use super::*;
    let and_gate = Gate::And {
      inputs: [0, 1],
      outputs: [2],
    };

    let not_gate = Gate::Not {
      inputs: [2],
      outputs: [3],
    };

    let mut simulation = Simulation {
      pins: Pins::from_iter(vec![(0, false), (1, true)]),
      gates: vec![and_gate, not_gate],
    };

    simulation.step();

    // The output pin of the AND gate
    assert_eq!(simulation.pins.get(&2), Some(&false));

    // The output pin of the NOT gate
    assert_eq!(simulation.pins.get(&3), Some(&true));
  }
}
