
import init, { Input, Output, Piece } from '../tetron_wasm.js'

console.log("===== TETRON-WASM Driver Test =====");

let wasm, memory;
let loaded = false;
let running = false;
// Booter
async function run () {
	wasm = await init();
	memory = wasm.memory;
	loaded = true;

	console.log("loaded WASM");
	Input.new().test(Piece.T);



}
run ();


function to_wasm_piece (js_v) {
	if (js_v === undefined || js_v === null) return Piece.None;
	switch (js_v) {
		case "T": return Piece.T;
		case "I": return Piece.I;
		case "O": return Piece.O;
		case "J": return Piece.J;
		case "L": return Piece.L;
		case "S": return Piece.S;
		case "Z": return Piece.Z;

		case 0: return Piece.L;
		case 1: return Piece.J;
		case 2: return Piece.Z;
		case 3: return Piece.S;
		case 4: return Piece.I;
		case 5: return Piece.O;
		case 6: return Piece.T;

		default: return Piece.Some;
	}
}
// Bot driving function
const runBot = async (state) => {
	running = true;
	const input = Input.new();		
	
	{ // Parse CESTRIS.State into TETRON-WASM.Input
		// Write board
		for (let y=0; y<20; y++) 
			for (let x=0; x<10; x++)
				input.set_board(x, y, state.grid[y*10 + x] > 0 ? Piece.Some : Piece.None );

		// Write Pieces
		console.log(state);
		input.set_pieces(0, to_wasm_piece(state.piece.type));
		for (let i=0; i<state.queue.length; i++) 
			input.set_pieces(i+1, to_wasm_piece(state.queue[i]));

		// Write Hold
		input.set_hold(to_wasm_piece(state.hold));	
	}

	const start = new Date();
	const output = input.run();
	postMessage(`input.run() bench (ms): ${(new Date()) - start}`);

	const keys = [];

	{ // Parse output into array of keys
		const add = (k) => {
			keys.push(k+"-down");
			keys.push(k+"-up");
		}

		let r = output.s() == -1 ? output.r() : output.s();
		if (r == 1) { add("up"); }
		if (r == 2) { add("z"); add("z"); }
		if (r == 3) { add("z"); }

		let d = Math.abs(output.x() - 4);
		if (0 > d) for (let i=0; i<d; i++) add("left");
		if (0 < d) for (let i=0; i<d; i++) add("right");

		// If spun
		if (output.s() != -1) {
			add("down"); // Softdrop 
			let d = output.r() - output.s();
			if (d ==  1) { add("up"); }
			if (d == -1) { add("z"); }
		}
		add(" ");
		console.log("Transcribed output: ", keys);
	}

	running = false;
	return keys;
}

onmessage = e => {
	const cmd = e.data[0];
	switch (cmd) {
	case "run":
		if (!loaded) {
			postMessage("still loading...");
			return;	
		}
		if (running) {
			postMessage("still running...");
			return;
		}
		const state = e.data[1];
		runBot(state);
		postMessage("done running.");
		break;
	default: 
		postMessage("unknown command: ", cmd);
	}
}
