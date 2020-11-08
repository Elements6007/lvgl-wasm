:: Copy outputs to docs/lvgl.js, lvgl.wasm, script.js, script.wasm

if exist wasm\lvgl.wasm (
    copy wasm\lvgl.wasm docs\lvgl.wasm
)

if exist wasm\lvgl.js (
    copy wasm\lvgl.js   docs\lvgl.js
)

if exist wasm\script.wasm (
    copy wasm\script.wasm docs\script.wasm
)

if exist wasm\script.js (
    copy wasm\script.js   docs\script.js
)
