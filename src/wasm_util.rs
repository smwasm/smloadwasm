use smdton::SmDtonBuffer;
use std::collections::HashMap;
use std::sync::RwLock;
use wasmtime::*;

use lazy_static::lazy_static;

use crate::wasm::{FL, LOAD_WAY, SZ, WS_JSN};
use crate::wasm_import::WasmState;

macro_rules! get_buf_len {
    ($stc: expr, $mem: expr, $poff: expr, $len: ident) => {
        let mut u8a4: [u8; 4] = [0; 4];
        let stc_buf_len = $stc.as_context_mut();
        $mem.read(stc_buf_len, $poff, &mut u8a4).unwrap();
        let $len = i32::from_le_bytes(u8a4) as usize;
    };
}

macro_rules! get_name_len {
    ($stc: expr, $mem: expr, $poff: expr, $nmlen: ident) => {
        let mut u8b2: [u8; 2] = [0; 2];
        let stc_nmlen = $stc.as_context_mut();
        $mem.read(stc_nmlen, $poff + SZ::LEN_TY, &mut u8b2).unwrap();
        let $nmlen = u16::from_le_bytes(u8b2) as usize;
    };
}

macro_rules! get_smb_name {
    ($stc: expr, $mem: expr, $poff: expr, $nmlen: expr, $name: ident) => {
        let mut vec: Vec<u8> = vec![0; $nmlen - 1];
        let piece = &mut vec;
        let stc_name = $stc.as_context_mut();
        $mem.read(stc_name, $poff + SZ::LEN_TY_NM, piece).unwrap();
        let $name = String::from_utf8(vec).unwrap();
    };
}

macro_rules! get_smb {
    ($stc: expr, $mem: expr, $poff: expr, $total: expr, $nmlen: expr, $smb: ident) => {
        let mut vec: Vec<u8> = vec![0; $total - SZ::NM - $nmlen];
        let piece: &mut [u8] = &mut vec;
        let stc_smb = $stc.as_context_mut();
        $mem.read(stc_smb, $poff + SZ::LEN_TY_NM + $nmlen, piece)
            .unwrap();
        let $smb = SmDtonBuffer { off: 0, buf: vec };
    };
}

macro_rules! get_buf_txt {
    ($stc: expr, $mem: expr, $poff: expr, $len: expr, $vec: ident) => {
        let mut $vec: Vec<u8> = vec![0; $len];
        let piece: &mut [u8] = &mut $vec;
        let stc_buf_txt = $stc.as_context_mut();
        $mem.read(stc_buf_txt, $poff + SZ::LEN, piece).unwrap();
    };
}

pub const MAX_STORE: usize = 256;

lazy_static! {
    pub static ref WS_UTL: WasmUtil = WasmUtil::new();
    pub static ref WS_SSN: RwLock<usize> = RwLock::new(0);
    pub static ref WS_MOD: RwLock<HashMap<String, Option<Module>>> = RwLock::new(HashMap::new());
}

pub struct WasmUtil {
    pub engine: Engine,
}

impl WasmUtil {
    fn new() -> WasmUtil {
        WasmUtil {
            engine: Engine::default(),
        }
    }

    pub fn load(&self, wasm_path: &str) -> Option<Module> {
        let _r = Module::from_file(&self.engine, wasm_path);
        match _r {
            Ok(_mod) => {
                return Some(_mod);
            }
            _ => {}
        }
        return None;
    }

    pub fn get_ssn(&self) -> usize {
        {
            let mut ssn = WS_SSN.write().unwrap();
            let dsn = *ssn;
            *ssn = (*ssn + 1) % MAX_STORE;
            return dsn;
        }
    }

    pub fn is_json(&self, sn: usize) -> bool {
        {
            let jsn = WS_JSN.read().unwrap();
            if jsn[sn] & FL::INJSON == FL::INJSON && LOAD_WAY & FL::INJSON == FL::INJSON {
                return false;
            }
            return true;
        }
    }

    pub fn check_module(&self, wasm_path: &str) -> bool {
        {
            let map = WS_MOD.read().unwrap();
            if map.contains_key(wasm_path) {
                let itm = map.get(wasm_path).unwrap();
                if itm.is_some() {
                    return true;
                }
                return false;
            }
        }
        let m = self.load(wasm_path);
        {
            let mut map = WS_MOD.write().unwrap();
            if m.is_some() {
                map.insert(wasm_path.to_string(), m);
                return true;
            } else {
                println!("--- load wasm error --- {}", wasm_path);
                map.insert(wasm_path.to_string(), None);
            }
        }
        return false;
    }

    pub fn get_buffer_text(
        &self,
        mut stc: StoreContextMut<'_, WasmState>,
        mem: Memory,
        poff: usize,
    ) -> String {
        get_buf_len!(stc, mem, poff, len);
        get_buf_txt!(stc, mem, poff, len, vec);

        let txt = String::from_utf8(vec).unwrap();
        return txt;
    }

    pub fn get_buffer_smb(
        &self,
        mut stc: StoreContextMut<'_, WasmState>,
        mem: Memory,
        poff: usize,
    ) -> (String, SmDtonBuffer) {
        get_buf_len!(stc, mem, poff, total);
        get_name_len!(stc, mem, poff, nmlen);
        get_smb_name!(stc, mem, poff, nmlen, name);
        get_smb!(stc, mem, poff, total, nmlen, smb);
        return (name, smb);
    }
}
