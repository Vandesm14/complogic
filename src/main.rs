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
  Not {
    inputs: [usize; 1],
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
      Self::Not { inputs, outputs } => {
        let mut missing_pins: Vec<usize> = vec![];
        let pin_a = &inputs[0];

        let a = match pins.get(pin_a) {
          Some(a) => a,
          None => {
            missing_pins.push(*pin_a);
            &false
          }
        };

        if !missing_pins.is_empty() {
          return Err(EvalError::MissingPins { pins: missing_pins });
        }

        let result = !*a;

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
