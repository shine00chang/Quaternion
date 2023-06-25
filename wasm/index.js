const express = require('express');

const app = express();

app.use(express.static('./pub'))
app.use(express.static('./pkg'))

app.get("/", (req, res) => {
    return res.sendFile('pub/index.html', {root: __dirname});
});

app.listen(3030, () => {
    console.log("listening on port: ", 3030);
});
