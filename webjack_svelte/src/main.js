import App from "./App.svelte";
import wasm from "../../rust/Cargo.toml";

// wasm import must be done asynchronously, so this doesn't work
// const app = new App({
// 	target: document.body,
// 	props: {
// 		name: 'world'
// 	}
// });

// export default app;

const init = async () => {
  const bindings = await wasm();
  const app = new App({
    target: document.body,
    props: {
      bindings,
    },
  });
};

init();
