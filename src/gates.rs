use crate::{Incrementer, NandOp, Ops};
use std::fmt::Debug;

pub trait Gate: Debug {
  fn create(&self, incrementer: &Incrementer) -> Ops;
}

#[derive(Debug)]
pub struct Nand {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for Nand {
  fn create(&self, _: &Incrementer) -> Ops {
    vec![NandOp(self.a, self.a, self.out)]
  }
}

#[derive(Debug)]
pub struct Not {
  pub a: usize,
  pub out: usize,
}

impl Gate for Not {
  fn create(&self, _: &Incrementer) -> Ops {
    vec![NandOp(self.a, self.a, self.out)]
  }
}

#[derive(Debug)]
pub struct And {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for And {
  fn create(&self, incrementer: &Incrementer) -> Ops {
    let nand = Nand {
      a: self.a,
      b: self.b,
      out: incrementer.next(),
    };
    let not = Not {
      a: nand.out,
      out: self.out,
    };

    let mut ops: Ops = vec![];
    ops.extend(nand.create(incrementer));
    ops.extend(not.create(incrementer));

    ops
  }
}

#[derive(Debug)]
pub struct Or {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for Or {
  fn create(&self, incrementer: &Incrementer) -> Ops {
    let nand_a = Nand {
      a: self.a,
      b: self.a,
      out: incrementer.next(),
    };
    let nand_b = Nand {
      a: self.b,
      b: self.b,
      out: incrementer.next(),
    };
    let nand = Nand {
      a: nand_a.out,
      b: nand_b.out,
      out: self.out,
    };

    let mut ops: Ops = vec![];
    ops.extend(nand_a.create(incrementer));
    ops.extend(nand_b.create(incrementer));
    ops.extend(nand.create(incrementer));

    ops
  }
}

#[derive(Debug)]
pub struct Nor {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for Nor {
  fn create(&self, incrementer: &Incrementer) -> Ops {
    let or = Or {
      a: self.a,
      b: self.b,
      out: incrementer.next(),
    };
    let not = Not {
      a: or.out,
      out: self.out,
    };

    let mut ops: Ops = vec![];
    ops.extend(or.create(incrementer));
    ops.extend(not.create(incrementer));

    ops
  }
}

#[derive(Debug)]
pub struct Xor {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for Xor {
  fn create(&self, incrementer: &Incrementer) -> Ops {
    let or = Or {
      a: self.a,
      b: self.b,
      out: incrementer.next(),
    };
    let nand = Nand {
      a: self.a,
      b: self.b,
      out: incrementer.next(),
    };
    let and = And {
      a: or.out,
      b: nand.out,
      out: self.out,
    };

    let mut ops: Ops = vec![];
    ops.extend(or.create(incrementer));
    ops.extend(nand.create(incrementer));
    ops.extend(and.create(incrementer));

    ops
  }
}
