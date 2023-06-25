const worker = new Worker("worker.js", { type: "module" });

const run = () => {
	// === SAMPLE STATE OBJECT === 
	const grid = [
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		0,0,0,0,0,0,0,0,0,0,	
		1,1,0,0,0,0,0,0,0,1,	
		1,0,0,0,1,1,1,1,1,1,	
		1,1,0,1,1,1,1,1,1,1,	
	];
	// Structure copied from Cestris/pub/js/state.mjs
	const state = {
		grid: grid,
		queue: [2, 5, 1, 3, 4], 
		piece: { type: 6 },
		hold: 1,
		b2b: 2,
		combo: 1,
	};
	worker.postMessage(["run", state]);	
}

//const interval = setInterval( run, 100 );

worker.onmessage = (e) => {
	console.log("worker msg: ", e.data);
}
