use std::collections::HashMap;
use std::fmt::Debug;

type Pins = HashMap<usize, bool>;
type FlatPins = Vec<(usize, bool)>;

#[derive(Debug)]
enum EvalError {
  MissingPins { pins: Vec<usize> },
}

#[derive(Debug)]
enum Gate {
  And {
    inputs: [usize; 2],
    outputs: [usize; 1],
  },
}

impl Gate {
  fn eval(&self, pins: &Pins) -> Result<FlatPins, EvalError> {
    match self {
      Self::And { inputs, outputs } => {
        let mut missing_pins: Vec<usize> = vec![];

        let [pin_a, pin_b] = inputs;

        let a = match pins.get(pin_a) {
          Some(a) => a,
          None => {
            missing_pins.push(*pin_a);
            &false
          }
        };
        let b = match pins.get(pin_b) {
          Some(b) => b,
          None => {
            missing_pins.push(*pin_b);
            &false
          }
        };

        if !missing_pins.is_empty() {
          return Err(EvalError::MissingPins { pins: missing_pins });
        }

        let result = *a && *b;

        Ok(vec![(outputs[0], result)])
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

      if let Ok(result) = result {
        // Update each pin in the map
        result.iter().for_each(|(pin, val)| {
          self.pins.insert(*pin, *val);
        });
      }
    })
  }
}

fn main() {
  let and_gate = Gate::And {
    inputs: [0, 1],
    outputs: [2],
  };

  let mut simulation = Simulation {
    pins: Pins::from_iter(vec![(0, true), (1, true)]),
    gates: vec![and_gate],
  };

  simulation.step();

  println!("Pins: {:?}", simulation.pins);
}
