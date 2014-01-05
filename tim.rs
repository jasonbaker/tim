extern mod extra;

use extra::json;
use extra::treemap::TreeMap;

use std::at_vec;
use std::from_str::from_str;
use std::io;
use std::path::Path;
use std::str::from_utf8;
use std::vec;

#[deriving(ToStr)]
enum Value {
  String(~str),
  Int(int),
  Float(float)
}

#[deriving(ToStr,Clone)]
enum Address {
  Arg(int),
  Comb(~str),
  Label(~str),
}

#[deriving(ToStr,Clone)]
enum Instruction {
  Take(int),
  Push(Address),
  Enter(Address)
}

struct Closure {
  instr: Instruction,
  environ: ~[Closure]
}

// For simple slurping. Will abort program on failure.
fn slurp(filename: ~str) -> ~str {
  let read_result = io::read_whole_file(&Path(filename));
  match read_result {
    Ok(instr) => from_utf8(instr),
    Err(e) => fail!(e)
  }
}

fn get_key_or_fail<'r>(key: &~str, obj: &'r TreeMap<~str, json::Json>) -> &'r json::Json {
  return match obj.find(key) {
    Some(ref val) => *val,
    _ => fail!("Could not find key" + *key)
  };
}

fn coerce_to_str<'r>(s: &'r json::Json) -> &'r ~str {
  match s {
    &json::String(ref s) => s,
    _ => fail!("Expected string")
  }
}

fn get_int_field(key: &~str, obj: &TreeMap<~str, json::Json>) -> int {
  let field = get_key_or_fail(key, obj);
  let retval = match field {
    &json::Number(ref i) => i,
    _ => fail!("Invalid int value")
  };
  return *retval as int;
}

fn get_str_field(key: &~str, obj: &TreeMap<~str, json::Json>) -> ~str {
  let field = get_key_or_fail(key, obj);
  return coerce_to_str(field).clone();
}

fn extract_instruction(obj: &TreeMap<~str, json::Json>) -> Instruction {
  let instr_field =  get_key_or_fail(&~"instr", obj);
  let instr_name = coerce_to_str(instr_field);

  match instr_name {
    &~"Take" => extract_take(obj),
    &~"Enter" => Enter(extract_address(obj)),
    &~"Push" => Push(extract_address(obj)),
    _ => fail!("Unsupported instruction")
  }
}

fn extract_take (obj: &TreeMap<~str, json::Json>) -> Instruction {
  let intval = match obj.find(&~"arg") {
    Some(&json::Number(ref i)) => i,
    _ => fail!("Invalid instruction arg")
  };

  return Take(*intval as int);
}

fn extract_address(obj: &TreeMap<~str, json::Json>) -> Address {
  let addr_field = get_key_or_fail(&~"addr", obj);
  let addr_name = coerce_to_str(addr_field);

  match addr_name {
    &~"Arg" => Arg(get_int_field(&~"arg", obj)),
    &~"Comb" => Comb(get_str_field(&~"arg", obj)),
    &~"Label" => Label(get_str_field(&~"arg", obj)),
    _ => fail!("Invalid addr " + *addr_name)
  }
}

fn walk_json(node: json::Json) -> @[Instruction] {
  let json_list = match node {
    json::List(l) => l,
    _ => fail!("Expected list")
  };

  let retval = do vec::build(None) |append| {
    for j in json_list.iter() {
      let obj = match j {
        &json::Object(ref t) => t,
        _ => fail!("Invalid instruction")
      };

      append(extract_instruction(&**obj));
    }
  };
  return at_vec::to_managed(retval);
}

fn main() {
  let json_text = slurp(~"/Users/jason/src/tim/code.json");

  let r: Result<json::Json, json::Error> = extra::json::from_str(json_text);
  let val = match r {
    Ok(j) => walk_json(j),
    Err(_) => fail!("Invalid JSON"),
  };

  println(val.to_str());

  let mut v_stack: ~[@Value] = ~[];
}