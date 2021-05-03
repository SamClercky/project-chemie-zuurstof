//@ts-ignore
import App from "./App.svelte";
import {connect} from "./ws";

let app = new App({
	target: document.body
});

// Start WS connection
connect();

export default app;
