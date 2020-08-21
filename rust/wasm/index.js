/*
Fails with error:
lvgl_wasm_rust_bg.wasm:1 Failed to load module script: The server responded with a non-JavaScript MIME type of "application/wasm". Strict MIME type checking is enforced for module scripts per HTML spec.
TypeError: Failed to fetch dynamically imported module: https://appkaki.github.io/lvgl-wasm/rust/pkg/lvgl_wasm_rust.js
Promise.catch (async)
(anonymous) @ index.js:18
*/
import('./pkg/lvgl_wasm_rust.js')
    .then(wasm => {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        const realInput = document.getElementById('real');
        const imaginaryInput = document.getElementById('imaginary');
        const renderBtn = document.getElementById('render');

        renderBtn.addEventListener('click', () => {
            const real = parseFloat(realInput.value) || 0;
            const imaginary = parseFloat(imaginaryInput.value) || 0;
            wasm.draw(ctx, 600, 600, real, imaginary);
        });

        wasm.draw(ctx, 600, 600, -0.15, 0.65);
    })
    .catch(console.error);
