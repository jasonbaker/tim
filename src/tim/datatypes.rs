use std::gc::Gc;
use std::hashmap;

type Frame = ~[Gc<Closure>];
pub type InstructionList = ~[Instruction];
pub type CodeStore = ~hashmap::HashMap<~str, InstructionList>;

#[deriving(ToStr,Clone,Eq,Decodable)]
pub enum Address {
  Arg(int),
  Comb(~str),
  Const(int),
  Label(~str),
}

impl Address {
  pub fn to_closure(self, state: &mut State) -> Gc<Closure> {
    match self {
      Const(i) => return Gc::new(Closure {instrs: ~[PushV(CurrentFrame), Return], fidx: Gc::new(FrameInt(i))}),
      Label(l) =>  {
        let instrs = state.codestore.get(&l).clone();
        return Gc::new(Closure {instrs: instrs, fidx: state.fidx})
      },
      Arg(n) => {
        match state.fidx.borrow() {
          &FramePtr(ref f) => return f[n],
          _ => fail!("Expected frame pointer") 
        }
      }
      _ => fail!("Unsupported Address")
    }
  }
}

#[deriving(ToStr,Clone,Eq,Decodable)]
pub enum ValueAddress {
  CurrentFrame,
  IntVal(int) 
}

#[deriving(ToStr,Clone,Eq,Decodable)]
pub enum ValueOp {Sub, Add, Div, Mul}

#[deriving(ToStr,Clone,Decodable)]
pub enum Instruction {
  Take(int),
  Push(Address),
  PushV(ValueAddress),
  Enter(Address),
  Return,
  Op(ValueOp)
}

#[deriving(ToStr)]
pub struct State {
  instructions: ~[Instruction],
  stack: ~[Gc<Closure>],
  vstack: ~[int],
  fidx: Gc<FrameIndex>,
  codestore: CodeStore,
}

impl State {
  pub fn is_final(&self) -> bool {
    return self.instructions.len() == 0;
  }

  pub fn alloc_frame(&mut self, n: int) {
    let mut frame: Frame = ~[];
    for _ in range(0, n) {
      frame.push(self.stack.pop());
    }
    self.fidx = Gc::new(FramePtr(frame));
  }

  pub fn push_closure(&mut self, c: Gc<Closure>) {
    self.stack.push(c);
  }

  pub fn set_closure(&mut self, c: Gc<Closure>) {
    let closure = c.borrow();
    self.instructions = closure.instrs.clone();
    self.fidx = closure.fidx;
  }

  pub fn pop_instruction(&mut self) -> Instruction {
    return self.instructions.shift();
  }

  pub fn pop_stack(&mut self) -> Gc<Closure> {
    return self.stack.shift();
  }

  pub fn push_frame_value(&mut self) {
    match self.fidx.borrow() {
      &FrameInt(i) => self.push_value(i),
      _ => fail!("Unexpected frame value " + self.fidx.to_str())
    }
  }

  pub fn push_value(&mut self, val: int) {
    self.vstack.push(val);
  }

  pub fn pop_value(&mut self) -> int {
    return self.vstack.shift();
  }
}

#[deriving(ToStr)]
pub struct Closure {
  instrs: InstructionList,
  fidx: Gc<FrameIndex>
}

impl ToStr for Gc<Closure> {
  fn to_str(&self) -> ~str {
    let closure = self.borrow();
    return closure.to_str();
  }
}

#[deriving(ToStr)]
pub enum FrameIndex {
  FramePtr(Frame),
  FrameInt(int),
  FrameNone 
}

impl ToStr for Gc<FrameIndex> {
  fn to_str(&self) -> ~str {
    let fidx = self.borrow();
    return fidx.to_str();
  } 
}
