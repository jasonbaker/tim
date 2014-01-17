extern mod extra;

use std::hashmap;

type Frame = ~[@Closure];
pub type InstructionList = @[Instruction];
pub type CodeStore = ~hashmap::HashMap<~str, InstructionList>;

#[deriving(ToStr)]
pub enum Value {
  String(~str),
  Int(int),
  Float(float)
}

#[deriving(ToStr,Clone)]
pub enum Address {
  Arg(int),
  Comb(~str),
  Const(int),
  Label(~str),
}

impl Address {
  pub fn to_closure(&self, state: &State) -> @Closure {
    match self {
      &Const(i) => return @Closure {instrs: @[], fidx: @FrameInt(i)},
      &Label(ref l) =>  {
        return @Closure {instrs: *state.codestore.get(l), fidx: state.fidx}
      },
      &Arg(n) => {
        match state.fidx {
          @FramePtr(ref f) => return f[n],
          _ => fail!("Expected frame pointer") 
        }
      }
      _ => fail!("Unsupported Address")
    }
  }
}

#[deriving(ToStr,Clone)]
pub enum Instruction {
  Take(int),
  Push(Address),
  Enter(Address)
}

#[deriving(ToStr)]
pub struct State {
  instructions: ~[Instruction],
  stack: ~[@Closure],
  fidx: @FrameIndex,
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
    self.fidx = @FramePtr(frame);
  }

  pub fn push_closure(&mut self, c: @Closure) {
    self.stack.push(c);
  }

  pub fn set_closure(&mut self, c: @Closure) {
    self.instructions = c.instrs.into_owned();
  }

  pub fn pop_instruction(&mut self) -> Instruction {
    return self.instructions.shift();
  }
}

#[deriving(ToStr)]
pub struct Closure {
  instrs: InstructionList,
  fidx: @FrameIndex
}

impl ToStr for @Closure {
  fn to_str(&self) -> ~str {
    let closure: Closure = **self;
    return closure.to_str();
  }
}

#[deriving(ToStr)]
pub enum FrameIndex {
  FramePtr(Frame),
  FrameInt(int),
  FrameNone 
}