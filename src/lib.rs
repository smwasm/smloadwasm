mod smwasm;
mod wasm;
mod wasm_import;
mod wasm_store;
mod wasm_util;

use smcore::smu;

pub fn init() -> bool {
    smu.set_wasm(0, None);
    smwasm::_sm_init();
    return true;
}

pub fn load_wasm(_wp: &str, pagenum: i32) {
    smwasm::load_wasm(_wp, pagenum);
}
