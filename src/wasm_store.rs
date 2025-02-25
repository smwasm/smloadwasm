use std::collections::HashMap;
use std::sync::RwLock;
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
    fmap: HashMap<String, Func>,
    pub ct: RwLock<Store<WasmState>>,
}

impl WasmStoreStub {
    fn new(id: usize) -> Self {
        let wsta = WasmState {};

        let wimp = WS_IMP[id].as_ref().unwrap();

        let mut _store = Store::new(&WS_UTL.engine, wsta);
        let mut fmap: HashMap<String, Func> = HashMap::new();

        // for ms
        fmap.insert(
            "hostgetms[0]".to_string(),
            Func::wrap(&mut _store, || wimp.hostgetms()),
        );
        // for memory
        fmap.insert(
            "hostputmemory[2]".to_string(),
            Func::wrap(
                &mut _store,
                |mut _caller: Caller<'_, WasmState>, _d1: i32, _d2: i32| {
                    wimp.hostputmemory(_caller, _d1 as usize, _d2)
                },
            ),
        );
        // for debug
        fmap.insert(
            "hostdebug[2]".to_string(),
            Func::wrap(&mut _store, |_d0: i32, _d1: i32| wimp.hostdebug(_d0, _d1)),
        );
        // for sm
        fmap.insert(
            "hostcallsm[1]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _d1: i32| -> i32 {
                    return wimp.hostcallsm(_caller, _d1 as usize);
                },
            ),
        );

        // for wbindgen
        fmap.insert(
            "__wbindgen_init_externref_table[0]".to_string(),
            Func::wrap(&mut _store, |_caller: Caller<'_, WasmState>| {
                println!("--- host func --- __wbindgen_init_externref_table ---");
            }),
        );

        //
        fmap.insert(
            "sched_yield[0]".to_string(),
            Func::wrap(&mut _store, |_caller: Caller<'_, WasmState>| -> i32 {
                println!("--- host func --- sched_yield ---");
                return 0;
            }),
        );
        fmap.insert(
            "args_get[2]".to_string(),
            Func::wrap(&mut _store, |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| -> i32 {
                println!("--- host func --- args_get ---");
                return 0;
            }),
        );
        fmap.insert(
            "args_sizes_get[2]".to_string(),
            Func::wrap(&mut _store, |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| -> i32 {
                println!("--- host func --- args_sizes_get ---");
                return 0;
            }),
        );
        fmap.insert(
            "random_get[2]".to_string(),
            Func::wrap(&mut _store, |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| -> i32 {
                println!("--- host func --- random_get ---");
                return 0;
            }),
        );
        fmap.insert(
            "poll_oneoff[4]".to_string(),
            Func::wrap(&mut _store, |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32, _p4: i32| -> i32 {
                println!("--- host func --- poll_oneoff ---");
                return 0;
            }),
        );

        // for wasi
        fmap.insert(
            "emscripten_resize_heap[1]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32| -> i32 {
                    println!("--- host func --- emscripten_resize_heap ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "emscripten_memcpy_js[3]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32| {
                    println!("--- host func --- emscripten_memcpy_js ---");
                },
            ),
        );
        fmap.insert(
            "fd_write[4]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32, _p4: i32| -> i32 {
                    println!("--- host func --- fd_write ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "fd_read[4]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32, _p4: i32| -> i32 {
                    println!("--- host func --- fd_read ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "fd_close[1]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32| -> i32 {
                    println!("--- host func --- fd_close ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "fd_seek[4]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i64, _p3: i32, _p4: i32| -> i32 {
                    println!("--- host func --- fd_seek ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "fd_seek[5]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>,
                 _p1: i32,
                 _p2: i32,
                 _p3: i32,
                 _p4: i32,
                 _p5: i32|
                 -> i32 {
                    println!("--- host func --- fd_seek ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "abort[0]".to_string(),
            Func::wrap(&mut _store, |_caller: Caller<'_, WasmState>| {
                println!("--- host func --- abort ---");
            }),
        );
        fmap.insert(
            "environ_sizes_get[2]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| -> i32 {
                    println!("--- host func --- environ_sizes_get ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "environ_get[2]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| -> i32 {
                    println!("--- host func --- environ_get ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "strftime_l[5]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>,
                 _p1: i32,
                 _p2: i32,
                 _p3: i32,
                 _p4: i32,
                 _p5: i32|
                 -> i32 {
                    println!("--- host func --- strftime_l ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "clock_time_get[3]".to_string(),
            Func::wrap(
                &mut _store,
                |mut _caller: Caller<'_, WasmState>, _p1: i32, _p2: i64, _p3: i32| {
                    wimp.clock_time_get(_caller, _p1, _p2, _p3)
                },
            ),
        );
        fmap.insert(
            "emscripten_notify_memory_growth[1]".to_string(),
            Func::wrap(&mut _store, |_caller: Caller<'_, WasmState>, _p1: i32| {
                println!("--- host func --- emscripten_notify_memory_growth ---");
            }),
        );
        fmap.insert(
            "proc_exit[1]".to_string(),
            Func::wrap(&mut _store, |_caller: Caller<'_, WasmState>, _p1: i32| {
                println!("--- host func --- proc_exit ---");
            }),
        );
        fmap.insert(
            "__cxa_throw[3]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32| {
                    println!("--- host func ---  __cxa_throw ---");
                },
            ),
        );
        fmap.insert(
            "emscripten_date_now[0]".to_string(),
            Func::wrap(&mut _store, |_caller: Caller<'_, WasmState>| -> f64 {
                println!("--- host func ---  emscripten_date_now ---");
                return 0.0;
            }),
        );
        fmap.insert(
            "__syscall_openat[4]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32, _p4: i32| -> i32 {
                    println!("--- host func ---  __syscall_openat ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "__syscall_fstat64[2]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| -> i32 {
                    println!("--- host func ---  __syscall_fstat64 ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "__syscall_stat64[2]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| -> i32 {
                    println!("--- host func ---  __syscall_stat64 ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "__syscall_newfstatat[4]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32, _p4: i32| -> i32 {
                    println!("--- host func ---  __syscall_newfstatat ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "__syscall_lstat64[2]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32| -> i32 {
                    println!("--- host func ---  __syscall_lstat64 ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "__syscall_fcntl64[3]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32| -> i32 {
                    println!("--- host func ---  __syscall_fcntl64 ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "__syscall_ioctl[3]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32| -> i32 {
                    println!("--- host func ---  __syscall_ioctl ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "_tzset_js[3]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32| {
                    println!("--- host func ---  __syscall_ioctl ---");
                },
            ),
        );
        fmap.insert(
            "_localtime_js[3]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>, _p1: i32, _p2: i32, _p3: i32| {
                    println!("--- host func ---  hostoutputpush ---");
                },
            ),
        );
        fmap.insert(
            "_munmap_js[7]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>,
                 _p1: i32,
                 _p2: i32,
                 _p3: i32,
                 _p4: i32,
                 _p5: i32,
                 _p6: i32,
                 _p7: i32|
                 -> i32 {
                    println!("--- host func ---  _munmap_js ---");
                    return 0;
                },
            ),
        );
        fmap.insert(
            "_mmap_js[8]".to_string(),
            Func::wrap(
                &mut _store,
                |_caller: Caller<'_, WasmState>,
                 _p1: i32,
                 _p2: i32,
                 _p3: i32,
                 _p4: i32,
                 _p5: i32,
                 _p6: i32,
                 _p7: i32,
                 _p8: i32|
                 -> i32 {
                    println!("--- host func ---  _mmap_js ---");
                    return 0;
                },
            ),
        );

        let ct = RwLock::new(_store);

        WasmStoreStub {
            sn: id,
            fmap: fmap,
            ct: ct,
        }
    }

    fn inc_sn(&self, sn: &mut usize, ids: &mut Vec<String>, key: &str) {
        *sn += 1;
        ids.push(key.to_string());
    }

    pub fn get_instance(&self, module: &Module) -> Instance {
        let _ws = WS_STO[self.sn].as_ref().map(|x| x).unwrap();
        let mut _store = _ws.ct.write().unwrap();
        let stc = _store.as_context_mut();

        let mut sn = 0;
        let mut ids: Vec<String> = Vec::new();
        let imports = module.imports();
        for imp in imports {
            match imp.ty() {
                ExternType::Func(func_type) => {
                    let mut name = imp.name();
                    let np = func_type.params().len();
                    if name.starts_with("__wbg") {
                        name = &imp.name()[6..name.len() - 17];
                    }
                    let key: &str = &format!("{}[{}]", name, np);

                    let opitm = self.fmap.get(key);
                    let mut found = false;
                    match opitm {
                        Some(_itm) => {
                            self.inc_sn(&mut sn, &mut ids, key);
                            found = true;
                        }
                        _ => {}
                    }
                    println!("--- import --- {} --- {} --- {:?}", found, key, func_type);
                }
                ExternType::Global(_global_type) => {}
                ExternType::Table(_table_type) => {}
                ExternType::Memory(_memory_type) => {}
            }
        }

        for _i in sn..41 {
            ids.push(ids.get(0).unwrap().to_string());
        }

        let _imports: [Extern; 40] = [
            (*self.fmap.get(ids.get(0).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(1).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(2).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(3).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(4).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(5).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(6).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(7).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(8).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(9).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(10).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(11).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(12).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(13).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(14).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(15).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(16).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(17).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(18).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(19).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(20).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(21).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(22).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(23).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(24).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(25).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(26).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(27).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(28).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(29).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(30).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(31).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(32).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(33).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(34).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(35).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(36).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(37).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(38).unwrap()).unwrap()).into(),
            (*self.fmap.get(ids.get(39).unwrap()).unwrap()).into(),
        ];

        let _instance = Instance::new(stc, module, &_imports[0..sn]).unwrap();
        return _instance;
    }
}
