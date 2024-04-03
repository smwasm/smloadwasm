use smdton::SmDtonBuilder;
use wasmtime::*;

use lazy_static::lazy_static;
use smcore::{smh, smu};

use crate::wasm::WS_INA;
use crate::wasm_util::{MAX_STORE, WS_UTL};

const IMP_REPEAT_VALUE: Option<WasmImportSupport> = None;

lazy_static! {
    pub static ref WS_IMP: [Option<WasmImportSupport>; MAX_STORE] = {
        let mut arr: [Option<WasmImportSupport>; MAX_STORE] = [IMP_REPEAT_VALUE; MAX_STORE];
        for i in 0..MAX_STORE {
            arr[i] = Some(WasmImportSupport::new(i));
        }
        arr
    };
}

pub struct WasmState {
    pub sn: usize,
}

pub struct WasmImportSupport {
    pub sn: usize,
}

impl WasmImportSupport {
    pub fn new(sn: usize) -> WasmImportSupport {
        let obj = WasmImportSupport { sn: sn };
        obj
    }

    pub fn hostdebug(&self, _d1: i32, _d2: i32) {
        println!("+++ {} --- < < --- {} --- {} ---", self.sn, _d1, _d2);
    }

    pub fn hostgetms(&self) -> i64 {
        return smu.get_current_ms() as i64;
    }

    pub fn hostputmemory(&self, mut _caller: Caller<'_, WasmState>, ptr: usize, ty: i32) {
        if ty != 10 {
            return;
        }

        let mr = _caller.get_export("memory").unwrap();
        let mem = mr.into_memory().unwrap();
        let txt = WS_UTL.get_buffer_text(_caller.as_context_mut(), mem, ptr);
        println!("+++ {} {}", self.sn, txt);
    }

    pub fn hostcallsm(&self, mut _caller: Caller<'_, WasmState>, ptr: usize) -> i32 {
        let mr = _caller.get_export("memory").unwrap();
        let mem = mr.into_memory().unwrap();
        if WS_UTL.is_json(self.sn) {
            let calltxt = WS_UTL.get_buffer_text(_caller.as_context_mut(), mem, ptr);

            if calltxt.len() > 0 {
                let callobj = json::parse(&calltxt).unwrap();
                let usage = smu.get_string(&callobj, "$usage").unwrap();
                let mut sb = SmDtonBuilder::new_from_json(&callobj);
                let _ret = smh.call(&usage, sb.build());

                if let Some(a) = WS_INA.get(self.sn) {
                    if let Some(inst) = a {
                        let ptr = inst.output_memory(_caller.as_context_mut(), &usage, &_ret);
                        return ptr;
                    }
                }
            }
        } else {
            let (name, smb) = WS_UTL.get_buffer_smb(_caller.as_context_mut(), mem, ptr);

            if smb.buf.len() > 0 {
                let ret = smh.call(&name, smb);

                if let Some(a) = WS_INA.get(self.sn) {
                    if let Some(inst) = a {
                        let ptr = inst.output_memory(_caller.as_context_mut(), &name, &ret);
                        return ptr;
                    }
                }
            }
        }
        return 0;
    }

    pub fn clock_time_get(
        &self,
        mut _caller: Caller<'_, WasmState>,
        _p1: i32,
        _p2: i64,
        _p3: i32,
    ) -> i32 {
        let nsec = smu.get_current_ms() * 1000 * 1000;
        let bytes: [u8; 8] = (nsec as i64).to_le_bytes();

        let mr = _caller.get_export("memory").unwrap();
        let mem = mr.into_memory().unwrap();
        mem.write(_caller, _p3 as usize, &bytes).unwrap();

        return 0;
    }
}
