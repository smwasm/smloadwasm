use std::collections::HashMap;
use std::sync::RwLock;

use json::JsonValue;
use lazy_static::lazy_static;

use smcore::{smh, smu};

use crate::wasm::{WS_ENV, WS_INA, WS_INM};
use smdton::{SmDtonBuffer, SmDtonMap, SmDtonReader};

lazy_static! {
    // $usage to sn
    pub static ref WS_NAM: RwLock<HashMap<String, i32>> = RwLock::new(HashMap::new());
    pub static ref JS_EMP: JsonValue = json::parse("{}").unwrap();
}

const SM_PREFIX: &str = "smwasm";
const SMKER_GET_ALL: &str = "smker.get.all";
const USAGE: &str = "$usage";

pub fn load_wasm(_wp: &str, pagenum: i32) -> bool {
    if !WS_ENV.check_instance(&_wp, pagenum) {
        return false;
    }
    let sn: usize;
    {
        let map = WS_INM.read().unwrap();
        sn = *map.get(_wp).unwrap() as usize;
    }

    let mut smp = SmDtonMap::new();
    smp.add_string(USAGE, SMKER_GET_ALL);
    let smb = smp.build();
    if let Some(a) = WS_INA.get(sn as usize) {
        if let Some(inst) = a {
            let ptr = inst.set_input(SMKER_GET_ALL, &smb);
            let out_smb = inst.call(ptr);

            let rd = SmDtonReader::new(out_smb.get_buffer());
            let opall = rd.to_json(1);
            match opall {
                Some(jsn) => {
                    for x in jsn.entries() {
                        if x.0 == SMKER_GET_ALL {
                            continue;
                        }
                        let mut smp = SmDtonMap::new();
                        smp.add_string(USAGE, x.0);
                        smp.add_from_json(x.1);
                        smh.register(smp.build(), _sm_call_outside);
                        {
                            let mut map = WS_NAM.write().unwrap();
                            map.insert(x.0.to_string(), sn as i32);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    return true;
}

pub fn call_wasm(sn: i32, name: &str, _input: &SmDtonBuffer) -> SmDtonBuffer {
    if sn < 0 {
        return SmDtonBuffer::new();
    }

    if let Some(a) = WS_INA.get(sn as usize) {
        if let Some(inst) = a {
            let ptr = inst.set_input(name, _input);
            return inst.call(ptr);
        }
    }

    return SmDtonBuffer::new();
}

fn _sm_call_outside(_input: &SmDtonBuffer) -> SmDtonBuffer {
    let smp = SmDtonReader::new(_input.get_buffer());
    let name = smp.get_string(1, USAGE).unwrap();

    let map = WS_NAM.read().unwrap();
    let op = map.get(name);
    if op.is_some() {
        let sn = *op.unwrap();
        return call_wasm(sn, name, &_input);
    }

    return SmDtonBuffer::new();
}

pub fn _sm_init() {
    smu.log(&format!(
        "--- sm init --- from smloadwasm --- {} ---",
        SM_PREFIX
    ));
}
