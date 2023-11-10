use std::collections::HashMap;
use std::fmt::Debug;

type FlatBoolOps = Vec<(usize, BoolOp)>;
type CompiledPins = HashMap<usize, (BoolOp, bool)>;

#[derive(Debug, Clone)]
enum BoolOp {
  And(Box<BoolOp>, Box<BoolOp>),
  Or(Box<BoolOp>, Box<BoolOp>),
  Not(Box<BoolOp>),

  Pin(usize),
  Static(bool),
}

impl BoolOp {
  fn eval(&self, pins: &CompiledPins) -> bool {
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
      Self::Pin(pin) => match pins.get(pin) {
        Some((ops, _)) => ops.eval(pins),
        None => false,
      },
      Self::Static(bool) => *bool,
    }
  }
}

#[derive(Debug)]
struct Gate {
  inputs: Vec<usize>,
  outputs: FlatBoolOps,
}

#[derive(Debug)]
struct Simulation {
  /// All gates
  gates: Vec<Gate>,

  // TODO: Use a lifetime and reference the op
  /// A list of pins and their compiled boolean operations
  compiled_pins: CompiledPins,
}

impl Simulation {
  /// Step through the simulation once
  fn step(&mut self) {
    let new_compiled_pins = CompiledPins::from_iter(
      self.compiled_pins.iter().map(|(id, (ops, bool))| {
        let op = ops.clone();
        let result = op.eval(&self.compiled_pins);

        (*id, (op, result))
      }),
    );

    self.compiled_pins = new_compiled_pins;
  }

  /// Compiles the simulation
  fn compile(&mut self) {
    self.gates.iter().for_each(|gate| {
      gate.outputs.iter().for_each(|(pin, op)| {
        self.compiled_pins.insert(*pin, (op.clone(), false));
      });
    });
  }
}

fn main() {
  let and_gate = Gate {
    inputs: vec![0, 1],
    outputs: vec![(
      2,
      BoolOp::And(Box::new(BoolOp::Pin(0)), Box::new(BoolOp::Pin(1))),
    )],
  };

  let not_gate = Gate {
    inputs: vec![2],
    outputs: vec![(3, BoolOp::Not(Box::new(BoolOp::Pin(2))))],
  };

  let mut simulation = Simulation {
    gates: vec![and_gate, not_gate],
    compiled_pins: CompiledPins::from_iter(vec![
      (0, (BoolOp::Static(true), false)),
      (1, (BoolOp::Static(true), false)),
    ]),
  };

  simulation.compile();
  println!("Compiled: {:#?}", simulation.compiled_pins);

  simulation.step();
  println!("Step: {:#?}", simulation.compiled_pins);
}

#[cfg(test)]
mod tests {
  #[test]
  fn and_and_not_gate() {
    use super::*;
    let and_gate = Gate {
      inputs: vec![0, 1],
      outputs: vec![(
        2,
        BoolOp::And(Box::new(BoolOp::Pin(0)), Box::new(BoolOp::Pin(1))),
      )],
    };

    let not_gate = Gate {
      inputs: vec![2],
      outputs: vec![(3, BoolOp::Not(Box::new(BoolOp::Pin(2))))],
    };

    let mut simulation = Simulation {
      gates: vec![and_gate, not_gate],
      compiled_pins: CompiledPins::from_iter(vec![
        (0, (BoolOp::Static(true), false)),
        (1, (BoolOp::Static(true), false)),
      ]),
    };

    simulation.compile();
    simulation.step();

    // The output pin of the AND gate
    let and_output = simulation.compiled_pins.get(&2);

    // The output pin of the NOT gate
    let not_output = simulation.compiled_pins.get(&3);

    assert!(and_output.is_some());
    assert!(not_output.is_some());

    assert_eq!(and_output.unwrap().1, true);
    assert_eq!(not_output.unwrap().1, false);
  }

  #[test]
  fn and_and_not_gate_and_false() {
    use super::*;
    let and_gate = Gate {
      inputs: vec![0, 1],
      outputs: vec![(
        2,
        BoolOp::And(Box::new(BoolOp::Pin(0)), Box::new(BoolOp::Pin(1))),
      )],
    };

    let not_gate = Gate {
      inputs: vec![2],
      outputs: vec![(3, BoolOp::Not(Box::new(BoolOp::Pin(2))))],
    };

    let mut simulation = Simulation {
      gates: vec![and_gate, not_gate],
      compiled_pins: CompiledPins::from_iter(vec![
        (0, (BoolOp::Static(false), false)),
        (1, (BoolOp::Static(true), false)),
      ]),
    };

    simulation.compile();
    simulation.step();

    // The output pin of the AND gate
    let and_output = simulation.compiled_pins.get(&2);

    // The output pin of the NOT gate
    let not_output = simulation.compiled_pins.get(&3);

    assert!(and_output.is_some());
    assert!(not_output.is_some());

    assert_eq!(and_output.unwrap().1, false);
    assert_eq!(not_output.unwrap().1, true);
  }
}
