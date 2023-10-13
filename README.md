# Quaternion

***A Tetris bot***<br>
Implemented with parallel workers expanding a shared game tree.
Rooted in a heuristic evaluation function.

build it locally:
- `cd driver`
- `cargo run --release sandbox`

May come to [Cestris](https://shine00chang.github.io/Cestris/) at some point


## Stats
On 2020 Apple M1 @ 3pps
- ~550k nodes per iteration

Sandbox:
- ~0.55 attacks per piece
- ~100 apm

75% Backfire:
- ~0.65 attacks per piece
- ~115 apm
