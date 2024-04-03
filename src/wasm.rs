use smdton::{SmDtonBuffer, SmDtonBuilder};
use wasmtime::*;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::wasm_import::WasmState;
use crate::wasm_store::WS_STO;
use crate::wasm_util::{MAX_STORE, WS_MOD, WS_UTL};

const INA_REPEAT_VALUE: Option<WasmInstanceStub> = None;
pub const LOAD_WAY: i32 = 0x100;

lazy_static! {
    pub static ref WS_ENV: Wasm = Wasm {};
    pub static ref WS_INM: RwLock<HashMap<String, i32>> = RwLock::new(HashMap::new());
    pub static ref WS_JSN: RwLock<[i32; MAX_STORE]> = RwLock::new([0; MAX_STORE]);
    pub static ref WS_INA: [Option<WasmInstanceStub>; MAX_STORE] = {
        let mut arr: [Option<WasmInstanceStub>; MAX_STORE] = [INA_REPEAT_VALUE; MAX_STORE];
        for i in 0..MAX_STORE {
            arr[i] = Some(WasmInstanceStub::new(i));
        }
        arr
    };
}
pub struct SZ {}

impl SZ {
    pub const LEN: usize = 4;
    pub const TY: usize = 1;
    pub const NM: usize = 2;
    pub const TY_NM: usize = Self::TY + Self::NM;
    pub const LEN_TY: usize = Self::LEN + Self::TY;
    pub const LEN_TY_NM: usize = Self::LEN + Self::TY + Self::NM;
}

pub struct FL {}

impl FL {
    pub const INJSON: i32 = 0x100;
}

pub struct Wasm {}

impl Wasm {
    pub fn check_instance(&self, wasm_path: &str, pagenum: i32) -> bool {
        {
            let map = WS_INM.read().unwrap();
            let itm = map.get(wasm_path);
            if itm.is_some() {
                return *itm.unwrap() >= 0;
            }
        }

        let mut ins = WasmInstance::new(wasm_path.to_string(), pagenum);
        ins.init();
        let valid = ins.instance.is_some();
        if !valid {
            let mut map = WS_INM.write().unwrap();
            map.insert(wasm_path.to_string(), -1);
            return false;
        }

        let sn = ins.sn;

        if let Some(a) = WS_INA.get(sn) {
            if let Some(b) = a {
                if let Ok(mut ct) = b.ct.write() {
                    *ct = Some(ins);
                }
            }
        }
        {
            let mut map = WS_INM.write().unwrap();
            map.insert(wasm_path.to_string(), sn as i32);
        }

        self._wasm_init(sn);
        return true;
    }

    fn _wasm_init(&self, sn: usize) {
        if let Some(sto) = WS_STO.get(sn) {
            if let Some(_ws) = sto {
                let mut _store = _ws.ct.write().unwrap();
                let stc = _store.as_context_mut();

                if let Some(ina) = WS_INA.get(sn) {
                    if let Some(b) = ina {
                        let mut c = b.ct.write().unwrap();
                        if let Some(t) = c.as_mut() {
                            let way = t.sminit.unwrap().call(stc, LOAD_WAY).unwrap();
                            {
                                let mut w = WS_JSN.write().unwrap();
                                w[sn] = way;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct WasmInstanceStub {
    sn: usize,
    pub ct: RwLock<Option<WasmInstance>>,
}

impl WasmInstanceStub {
    fn new(id: usize) -> Self {
        WasmInstanceStub {
            sn: id,
            ct: RwLock::new(None),
        }
    }

    fn _do_call(&self, mut _caller: StoreContextMut<'_, WasmState>, ptr: i32) -> SmDtonBuffer {
        let rd = self.ct.read().unwrap();
        if let Some(ref ins) = *rd {
            let stc1 = _caller.as_context_mut();
            let ptr_ret = ins.smcall.unwrap().call(stc1, (ptr, 1)).unwrap() as usize;

            if WS_UTL.is_json(ins.sn) {
                let stc2 = _caller.as_context_mut();
                let mem = ins.instance.unwrap().get_memory(stc2, "memory").unwrap();
                let ret = WS_UTL.get_buffer_text(_caller.as_context_mut(), mem, ptr_ret);

                let stc3 = _caller.as_context_mut();
                ins.smdealloc.unwrap().call(stc3, ptr_ret as i32).unwrap();

                let jsn = json::parse(&ret).unwrap();
                let mut sb = SmDtonBuilder::new_from_json(&jsn);
                return sb.build();
            } else {
                let _name;
                let smb;
                if WS_UTL.is_json(ins.sn) {
                    smb = SmDtonBuffer::new();
                } else {
                    let stc2 = _caller.as_context_mut();
                    let mem = ins.instance.unwrap().get_memory(stc2, "memory").unwrap();
                    (_name, smb) = WS_UTL.get_buffer_smb(_caller.as_context_mut(), mem, ptr_ret);
                }

                let stc3 = _caller.as_context_mut();
                ins.smdealloc.unwrap().call(stc3, ptr_ret as i32).unwrap();
                return smb;
            }
        }
        return SmDtonBuffer::new();
    }

    pub fn call(&self, ptr: i32) -> SmDtonBuffer {
        if let Some(a) = WS_STO.get(self.sn) {
            if let Some(_ws) = a {
                let mut _store = _ws.ct.write().unwrap();

                return self._do_call(_store.as_context_mut(), ptr);
            }
        }
        return SmDtonBuffer::new();
    }

    pub fn output_memory(
        &self,
        mut _caller: StoreContextMut<'_, WasmState>,
        name: &str,
        smb: &SmDtonBuffer,
    ) -> i32 {
        let rd = self.ct.read().unwrap();
        if let Some(ref ins) = *rd {
            if WS_UTL.is_json(ins.sn) {
                let txt = smb.stringify().unwrap();
                let bvo = txt.as_bytes();

                let stc1 = _caller.as_context_mut();
                let poff = ins.smalloc.unwrap().call(stc1, bvo.len() as i32).unwrap() as usize;

                let stc2 = _caller.as_context_mut();
                let m1 = ins.instance.unwrap().get_memory(stc2, "memory");
                let mem = m1.unwrap();

                let stc3 = _caller.as_context_mut();
                mem.write(stc3, poff + SZ::LEN, bvo).unwrap();

                return poff as i32;
            } else {
                let mut nmbytes = name.as_bytes().to_vec();
                nmbytes.push(0);
                let nmlen = nmbytes.len();

                let buf = smb.get_buffer();
                let total = nmlen + SZ::TY_NM + buf.len();

                let stc1 = _caller.as_context_mut();
                let poff = ins.smalloc.unwrap().call(stc1, total as i32).unwrap() as usize;

                let stc2 = _caller.as_context_mut();
                let m1 = ins.instance.unwrap().get_memory(stc2, "memory");
                let mem = m1.unwrap();

                let fb = (2 as u8).to_le_bytes();
                let stc6 = _caller.as_context_mut();
                mem.write(stc6, poff + SZ::LEN, &fb).unwrap();

                let u1 = (nmlen as u16).to_le_bytes();
                let stc3 = _caller.as_context_mut();
                mem.write(stc3, poff + SZ::LEN_TY, &u1).unwrap();

                let stc4 = _caller.as_context_mut();
                mem.write(stc4, poff + SZ::LEN_TY_NM, &nmbytes).unwrap();

                let stc5 = _caller.as_context_mut();
                mem.write(stc5, poff + SZ::LEN_TY_NM + nmlen, buf).unwrap();

                return poff as i32;
            }
        } else {
        }
        return 0;
    }

    pub fn set_input(&self, name: &str, smb: &SmDtonBuffer) -> i32 {
        if let Some(a) = WS_STO.get(self.sn) {
            if let Some(_ws) = a {
                let mut _store = _ws.ct.write().unwrap();
                return self.output_memory(_store.as_context_mut(), name, smb);
            }
        }
        return 0;
    }
}

pub struct WasmInstance {
    path: String,
    page: i32,
    ready: bool,
    pub sn: usize,
    pub instance: Option<Instance>,

    pub sminit: Option<TypedFunc<i32, i32>>,
    smcall: Option<TypedFunc<(i32, i32), i32>>,
    smalloc: Option<TypedFunc<i32, i32>>,
    smdealloc: Option<TypedFunc<i32, ()>>,
}

impl WasmInstance {
    pub fn new(wasm_path: String, pagenum: i32) -> WasmInstance {
        WasmInstance {
            path: wasm_path.to_string(),
            page: pagenum,
            ready: false,
            sn: 0,
            instance: None,
            sminit: None,
            smcall: None,
            smalloc: None,
            smdealloc: None,
        }
    }

    pub fn init(&mut self) {
        if !WS_UTL.check_module(&self.path) {
            return;
        }
        let map = WS_MOD.read().unwrap();
        let _module = map.get(&self.path).unwrap().as_ref().map(|x| x).unwrap();

        self.sn = WS_UTL.get_ssn();

        let _ws = WS_STO[self.sn].as_ref().map(|x| x).unwrap();
        let _instance = _ws.get_instance(&_module);

        let mut _store: std::sync::RwLockWriteGuard<'_, Store<WasmState>> = _ws.ct.write().unwrap();

        let stc0 = _store.as_context_mut();
        let opmem = _instance.get_memory(stc0, "memory");
        match opmem {
            Some(mem) => {
                // for memory
                let stc1 = _store.as_context_mut();
                let msize = mem.size(stc1) as i32;

                if self.page > msize {
                    let stc2 = _store.as_context_mut();
                    let r = mem.grow(stc2, (self.page - msize) as u64);
                    match r {
                        Ok(_size) => {
                            let stc3 = _store.as_context_mut();
                            let newsize = mem.size(stc3) as i32;
                            println!(
                                "--- {} --- original --- {} --- new page number --- {} ---",
                                self.path, _size, newsize
                            );
                        }
                        _ => {}
                    }
                } else {
                    println!(
                        "--- {} --- original page number --- {} ---",
                        self.path, msize
                    );
                }
            }
            _ => {}
        }

        let stc1 = _store.as_context_mut();
        let _sminit: TypedFunc<i32, i32> = _instance
            .get_typed_func::<i32, i32>(stc1, "sminit")
            .unwrap();

        let stc2 = _store.as_context_mut();
        let _smcall: TypedFunc<(i32, i32), i32> = _instance
            .get_typed_func::<(i32, i32), i32>(stc2, "smcall")
            .unwrap();

        let stc3 = _store.as_context_mut();
        let _smalloc: TypedFunc<i32, i32> = _instance
            .get_typed_func::<i32, i32>(stc3, "smalloc")
            .unwrap();

        let stc4 = _store.as_context_mut();
        let _smdealloc: TypedFunc<i32, ()> = _instance
            .get_typed_func::<i32, ()>(stc4, "smdealloc")
            .unwrap();

        self.instance = Some(_instance);
        self.sminit = Some(_sminit);
        self.smcall = Some(_smcall);
        self.smalloc = Some(_smalloc);
        self.smdealloc = Some(_smdealloc);

        self.ready = true;
    }
}
