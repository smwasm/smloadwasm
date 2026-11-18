mod smwasm;

mod wasm;
mod wasm_import;
mod wasm_store;
mod wasm_util;

pub use smwasm::compile_wasm;
pub use smwasm::load_wasm;

pub fn init() -> bool {
    smwasm::_sm_init();
    return true;
}
