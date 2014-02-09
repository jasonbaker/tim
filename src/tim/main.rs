#[feature(globs)];

extern mod extra;

use datatypes::*;
use instructions::build_codestore;

use extra::json;

use std::from_str::from_str;
use std::gc::Gc;
use std::io::File;
use std::path::Path;
use std::str::from_utf8;

mod datatypes;
mod instructions;


// Given a json object reperesenting a source file, create an initial state
fn init_state(node: &json::Json) -> ~State {
  return ~State {
    instructions: ~[Enter(Label(~"main"))],
    stack: ~[],
    fidx: Gc::new(FrameNone),
    codestore: build_codestore(node),
  };
}

fn run_program(state: &mut State) {
  while !state.is_final() {
    step(state);
  }
}

fn step(state: &mut State) {
  let instr = state.pop_instruction();
  println(instr.to_str());
  match instr {
    Enter(addr) => handle_enter(addr, state),
    Push(addr) => handle_push(addr, state),
    Take(i) => state.alloc_frame(i),
  }
}

fn handle_enter(addr: Address, state: &mut State) {
  let closure = addr.to_closure(state);
  state.set_closure(closure);  
}

fn handle_push(addr: Address, state: &mut State) {
  let closure = addr.to_closure(state);
  state.push_closure(closure);
}

fn main() {
  let json_text = File::open(&Path::new(~"/Users/jason/src/tim/code.json")).read_to_end();

  let r: Result<json::Json, json::Error> = extra::json::from_str(from_utf8(json_text));
  let mut state = match r {
    Ok(j) => init_state(&j),
    Err(_) => fail!("Invalid JSON"),
  };

  run_program(state);

  println(state.to_str());
}