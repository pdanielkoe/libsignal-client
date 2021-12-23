import init, { helloWorld } from "./pkg/libsignal_client_wasm.js";

init().then(() => {
    console.log(helloWorld())
});