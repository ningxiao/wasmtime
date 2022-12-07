'use strict';
const { WASI } = require('wasi');
const { join } = require('node:path');
const { argv, env } = require('node:process');
const { readFile } = require('node:fs/promises');
// 执行main.rs文件
(async () => {
    const wasi = new WASI(
        {
            env,
            args: argv,
            preopens: {
                '.': '/opt/www/wasmtime/hello-world',
            },
        }
    );
    const wasm = await WebAssembly.compile(
        await readFile(join(__dirname, '../target/wasm32-wasi/release/hello-world.wasm')),
    );
    const instance = await WebAssembly.instantiate(wasm, {
        wasi_snapshot_preview1: wasi.wasiImport,
        wasm_import_module: {
            jsAbs(x) {
                console.log('rust_call_node_abs', Array(Math.abs(x)).fill("*").join(""));
                return Math.abs(x);
            }
        }
    });
    wasi.start(instance);
})();
// 执行lib.rs文件
(async () => {
    const wasi = new WASI(
        {
            env,
            args: argv,
            preopens: {
                '.': '/opt/www/wasmtime/hello-world',
            },
        }
    );
    const wasm = await WebAssembly.compile(
        await readFile(join(__dirname, '../target/wasm32-wasi/release/hello_world.wasm')),
    );
    const instance = await WebAssembly.instantiate(wasm, { wasi_snapshot_preview1: wasi.wasiImport });
    wasi.initialize(instance);
    const { add_sum, print_hello, run_async_main } = instance.exports;
    print_hello();
    console.log(add_sum(12, 34));
    console.log('-----',run_async_main(8));
})();