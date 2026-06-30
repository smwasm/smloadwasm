mod smwasm;
mod wasm;
mod wasm_import;
mod wasm_store;
mod wasm_util;

pub fn init() -> bool {
    smwasm::_sm_init();
    return true;
}

pub fn load_wasm(_wp: &str, pagenum: i32) {
    smwasm::load_wasm(_wp, pagenum);
}
