use datatypes::*;

use extra::json;
use extra::serialize;
use extra::treemap::TreeMap;

use std::hashmap::HashMap;

pub type JsonObj = TreeMap<~str, json::Json>;

pub fn coerce_to_obj<'r>(j: &'r json::Json) -> &'r JsonObj {
  return match j {
    &json::Object(ref o) => &**o,
    _ => fail!("Expected object")
  };
}

pub fn build_codestore(node: &json::Json) -> 
    CodeStore {
  let mut retval: CodeStore = ~HashMap::new();
  let json_obj = coerce_to_obj(node);

  for (key, value) in json_obj.iter() {
    let mut decoder = json::Decoder::new(value.clone());
    let instr_list: InstructionList = serialize::Decodable::decode(&mut decoder);
    retval.insert(key.clone(), instr_list);
  }

  return retval;
}
