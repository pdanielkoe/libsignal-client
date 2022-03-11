import init, { hello_world, ecprivate_key_generate } from "./pkg/libsignal_client_wasm.js";

init().then(() => {
    console.log(hello_world())
    console.log(ecprivate_key_generate())
});