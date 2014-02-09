#[feature(globs)];

extern mod extra;

use datatypes::*;

use extra::json;
use extra::treemap::TreeMap;

use std::from_str::from_str;
use std::gc::Gc;
use std::hashmap::HashMap;
use std::io::File;
use std::path::Path;
use std::str::from_utf8;
use std::vec;

mod datatypes;

type JsonObj = TreeMap<~str, json::Json>;

/*// For simple slurping. Will abort program on failure.
fn slurp(filename: ~str) -> ~str {
  let read_result = io::read_whole_file(&Path(filename));
  match read_result {
    Ok(instr) => from_utf8(instr),
    Err(e) => fail!(e)
  }
}*/

pub fn get_key_or_fail<'r>(key: &~str, obj: &'r JsonObj) -> &'r json::Json {
  return match obj.find(key) {
    Some(ref val) => *val,
    _ => fail!("Could not find key " + *key)
  };
}

fn coerce_to_obj<'r>(j: &'r json::Json) -> &'r JsonObj {
  return match j {
    &json::Object(ref o) => &**o,
    _ => fail!("Expected object")
  };
}

fn coerce_to_str<'r>(s: &'r json::Json) -> &'r ~str {
  match s {
    &json::String(ref s) => s,
    _ => fail!("Expected string")
  }
}

fn get_int_field(key: &~str, obj: &JsonObj) -> int {
  let field = get_key_or_fail(key, obj);
  let retval = match field {
    &json::Number(ref i) => i,
    _ => fail!("Invalid int value")
  };
  return *retval as int;
}

fn get_obj_field<'r>(key: &~str, obj: &'r JsonObj) -> &'r JsonObj {
  let field = get_key_or_fail(key, obj);
  return coerce_to_obj(field);
}

fn get_str_field(key: &~str, obj: &JsonObj) -> ~str {
  let field = get_key_or_fail(key, obj);
  return coerce_to_str(field).clone();
}

fn extract_instruction(obj: &JsonObj) -> Instruction {
  let instr_field =  get_key_or_fail(&~"instr", obj);
  let instr_name = coerce_to_str(instr_field);

  match instr_name {
    &~"Take" => Take(get_int_field(&~"arg", obj)),
    &~"Enter" => Enter(extract_address(obj)),
    &~"Push" => Push(extract_address(obj)),
    _ => fail!("Unsupported instruction")
  }
}

fn extract_take (obj: &JsonObj) -> Instruction {
  let intval = match obj.find(&~"arg") {
    Some(&json::Number(ref i)) => i,
    _ => fail!("Invalid instruction arg")
  };

  return Take(*intval as int);
}

fn extract_address(obj: &JsonObj) -> Address {
  let addr_field = get_key_or_fail(&~"addr", obj);
  let addr_name = coerce_to_str(addr_field);

  match addr_name {
    &~"Arg" => Arg(get_int_field(&~"arg", obj)),
    &~"Comb" => Comb(get_str_field(&~"arg", obj)),
    &~"Const" => Const(get_int_field(&~"arg", obj)),
    &~"Label" => Label(get_str_field(&~"arg", obj)),
    _ => fail!("Invalid addr " + *addr_name)
  }
}

fn extract_instructions(node: &json::Json) -> InstructionList {
  let json_list = match node {
    &json::List(ref l) => l,
    _ => fail!("Expected list")
  };

  return vec::build(None, |append| {
    for j in json_list.iter() {
      let obj = match j {
        &json::Object(ref t) => t,
        _ => fail!("Invalid instruction")
      };

      append(extract_instruction(&**obj));
    }
  });
}

fn build_codestore(node: &json::Json) -> 
    CodeStore {
  let mut retval: CodeStore = ~HashMap::new();
  let json_obj = coerce_to_obj(node);

  for (key, value) in json_obj.iter() {
    retval.insert(key.clone(), extract_instructions(value));
  }

  return retval;
}

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