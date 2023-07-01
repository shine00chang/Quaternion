// Generate Grid Input field
const input_grid = document.getElementById('input-grid');
for (let y=0; y<20; y++) {
    const row = document.createElement('div');
    for (let x=0; x<10; x++) {
        const elem = document.createElement('input');
        elem.type  = "checkbox";
        elem.id    = `grid-${x}-${y}`;
        elem.style = "margin: 0px;";
        row.appendChild(elem);
    }
    input_grid.appendChild(row);
}

// Generate Queue Input field.
const input_queue = document.getElementById('input-queue');
const input_queue_label = document.getElementById('input-queue-lbl');
const pieces = ["T","O","I","L","J","S","Z"];

for (const piece of pieces)
    input_queue_label.innerText += `...${piece}`;

for (let i=0; i<6; i++) {
    const name = `queue-${i}`;
    const row  = document.createElement("div");
    for (let j=0; j<7; j++) {
        const radio = document.createElement("input");
        radio.type = "radio";
        radio.name = name;
        radio.value = pieces[j];
        if (i == j) radio.checked = true;
        row.appendChild(radio);
    }
    input_queue.appendChild(row);
}

const run_button = document.getElementById('run');
const duration = document.getElementById('duration');

const worker = new Worker("worker.js");

worker.onmessage = e => {
    console.log("worker message: ", e.data);
    const cmd = Array.isArray(e.data) ? e.data[0] : e.data;
    const args = Array.isArray(e.data) ? e.data.slice(1) : undefined;

    // On init done
    if (cmd == "init done") {
        run_button.disabled = false;
    }
    // On run done, reporting elapsed time.
    if (cmd == "elapsed") {
        duration.innerText = `time elapsed: ${args[0]}`;
    }

    if (cmd == "solution") {
        const output = args[0];
        
        console.log(output);
    }
}

function run() {
    const grid = [];
    for (let y=0; y<20; y++) 
        for (let x=0; x<10; x++) 
            grid.push(document.getElementById(`grid-${x}-${y}`).checked);
    const radio_value = (name) => {
        const elems = document.getElementsByName(name);
        for (const elem of elems) 
            if (elem.checked)
                return elem.value;
        return undefined;
    };
    const piece = radio_value('queue-0');
    const queue = [];
    for (let i=0; i<5; i++)
        queue.push(radio_value(`queue-${i}`));

    const state = {
        grid,
        piece,
        queue
    };

    worker.postMessage(["run", state]);
}


run_button.onclick = run;
