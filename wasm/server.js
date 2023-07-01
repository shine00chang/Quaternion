const express = require('express');
const app = express();

function headerSetter (res, path, stat) {
    res.set("Cross-Origin-Opener-Policy", "same-origin");
    res.set("Cross-Origin-Embedder-Policy", "require-corp");
}

app.use(express.static('./', { setHeaders: headerSetter }));

app.listen(8001, () => {
    console.log("listening on port: ", 8001);
})
