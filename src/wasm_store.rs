use std::sync::Mutex;
use wasmtime::*;

use lazy_static::lazy_static;

use crate::wasm_import::{WasmState, WS_IMP};
use crate::wasm_util::{MAX_STORE, WS_UTL};

const STO_REPEAT_VALUE: Option<WasmStoreStub> = None;

lazy_static! {
    pub static ref WS_STO: [Option<WasmStoreStub>; MAX_STORE] = {
        let mut arr: [Option<WasmStoreStub>; MAX_STORE] = [STO_REPEAT_VALUE; MAX_STORE];
        for i in 0..MAX_STORE {
            arr[i] = Some(WasmStoreStub::new(i));
        }
        arr
    };
}

pub struct WasmStoreStub {
    sn: usize,
    lnk: Linker<WasmState>,
    pub st: Mutex<Store<WasmState>>,
}

impl WasmStoreStub {
    fn new(id: usize) -> Self {
        let wsta = WasmState {};

        let wimp = WS_IMP[id].as_ref().unwrap();

        let mut _store = Store::new(&WS_UTL.engine, wsta);

        let mut lnk: Linker<WasmState> = Linker::new(&WS_UTL.engine);

        //-------- host impl --------
        // for debug
        lnk.func_wrap("env", "hostdebug", |_d0: i32, _d1: i32| {
            wimp.hostdebug(_d0, _d1)
        })
        .unwrap();

        // for ms
        lnk.func_wrap("env", "hostgetms", || wimp.hostgetms())
            .unwrap();

        // for memory
        lnk.func_wrap(
            "env",
            "hostputmemory",
            |mut _caller: Caller<'_, WasmState>, _d1: i32, _d2: i32| {
                wimp.hostputmemory(_caller, _d1 as usize, _d2);
            },
        )
        .unwrap();

        // for sm
        lnk.func_wrap(
            "env",
            "hostcallsm",
            |_caller: Caller<'_, WasmState>, _d1: i32| wimp.hostcallsm(_caller, _d1 as usize),
        )
        .unwrap();

        //-------- env temp --------
        lnk.func_wrap(
            "env",
            "emscripten_notify_memory_growth",
            |_caller: Caller<'_, WasmState>, _p1: i32| wimp.f_i_i4_o(_caller, 1),
        )
        .unwrap();

        //-------- wasi_snapshot_preview1 impl --------
        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "clock_time_get",
            |mut _caller: Caller<'_, WasmState>, _p1: i32, _p2: i64, _p3: i32| {
                wimp.clock_time_get(_caller, _p1, _p2, _p3)
            },
        )
        .unwrap();

        //-------- wasi_snapshot_preview1 temp --------
        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "proc_exit",
            |_caller: Caller<'_, WasmState>, _p1: i32| wimp.f_i_i4_o(_caller, 2),
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "environ_sizes_get",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| {
                wimp.f_i_i4_2_o_i4(_caller, 1, _p2)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "environ_get",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| {
                wimp.f_i_i4_2_o_i4(_caller, 2, _p2)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "fd_write",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32, _p4: i32| {
                wimp.f_i_i4_4_o_i4(_caller, 1, _p2, _p3, _p4)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "fd_close",
            |_caller: Caller<'_, WasmState>, _p1: i32| wimp.f_i_i4_o_i4(_caller, 1),
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "fd_seek",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i64, _p3: i32, _p4: i32| {
                wimp.f_i_i4_i8_i4_i4_o_i4(_caller, 1, _p2, _p3, _p4)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "sched_yield",
            |_caller: Caller<'_, WasmState>| wimp.f_i_o_i4(_caller),
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "args_get",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| {
                wimp.f_i_i4_2_o_i4(_caller, 3, _p2)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "args_sizes_get",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| {
                wimp.f_i_i4_2_o_i4(_caller, 4, _p2)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "random_get",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| {
                wimp.f_i_i4_2_o_i4(_caller, 5, _p2)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "poll_oneoff",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32, _p4: i32| {
                wimp.f_i_i4_4_o_i4(_caller, 2, _p2, _p3, _p4)
            },
        )
        .unwrap();

        // for wasi

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "emscripten_resize_heap",
            |_caller: Caller<'_, WasmState>, _p1: i32| wimp.f_i_i4_o_i4(_caller, 2),
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "emscripten_memcpy_js",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32| {
                wimp.f_i_i4_3_o(_caller, 1, _p2, _p3)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "abort",
            |_caller: Caller<'_, WasmState>| wimp.f_i_o(_caller),
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "strftime_l",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32, _p4: i32, _p5: i32| {
                wimp.f_i_i4_5_o_i4(_caller, 1, _p2, _p3, _p4, _p5)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "emscripten_notify_memory_growth",
            |_caller: Caller<'_, WasmState>, _p1: i32| wimp.f_i_i4_o_i4(_caller, 3),
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "__cxa_throw",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32| {
                wimp.f_i_i4_3_o(_caller, 2, _p2, _p3)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "emscripten_date_now",
            |_caller: Caller<'_, WasmState>| wimp.f_i_o_f8(_caller),
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "__syscall_openat",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32, _p4: i32| {
                wimp.f_i_i4_4_o_i4(_caller, 3, _p2, _p3, _p4)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "__syscall_fstat64",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| {
                wimp.f_i_i4_2_o_i4(_caller, 6, _p2)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "__syscall_stat64",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| {
                wimp.f_i_i4_2_o_i4(_caller, 7, _p2)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "__syscall_newfstatat",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32, _p4: i32| {
                wimp.f_i_i4_4_o_i4(_caller, 4, _p2, _p3, _p4)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "__syscall_lstat64",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| {
                wimp.f_i_i4_2_o_i4(_caller, 8, _p2)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "__syscall_fcntl64",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32| {
                wimp.f_i_i4_3_o_i4(_caller, 1, _p2, _p3)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "__syscall_ioctl",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32| {
                wimp.f_i_i4_3_o_i4(_caller, 2, _p2, _p3)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "_tzset_js",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32| {
                wimp.f_i_i4_3_o(_caller, 3, _p2, _p3)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "_localtime_js",
            |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32| {
                wimp.f_i_i4_3_o(_caller, 4, _p2, _p3)
            },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "_munmap_js",
            |_caller: Caller<'_, WasmState>,
             _p1: i32,
             _p2: i32,
             _p3: i32,
             _p4: i32,
             _p5: i32,
             _p6: i32,
             _p7: i32| { wimp.f_i_i4_7_o_i4(_caller, 1, _p2, _p3, _p4, _p5, _p6, _p7) },
        )
        .unwrap();

        lnk.func_wrap(
            "wasi_snapshot_preview1",
            "_mmap_js",
            |_caller: Caller<'_, WasmState>,
             _p1: i32,
             _p2: i32,
             _p3: i32,
             _p4: i32,
             _p5: i32,
             _p6: i32,
             _p7: i32,
             _p8: i32| {
                wimp.f_i_i4_8_o_i4(_caller, 1, _p2, _p3, _p4, _p5, _p6, _p7, _p8)
            },
        )
        .unwrap();

        let ct = Mutex::new(_store);

        WasmStoreStub {
            sn: id,
            lnk: lnk,
            st: ct,
        }
    }

    pub fn get_instance(&self, module: &Module) -> Instance {
        let _ws = WS_STO[self.sn].as_ref().map(|x| x).unwrap();
        let mut _store = _ws.st.lock().unwrap();
        let mut stc = _store.as_context_mut();

        let _instance = self.lnk.instantiate(&mut stc, &module).unwrap();
        return _instance;
    }
}
