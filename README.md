Run projects inside the Rust workspace:

```
cargo run -p day01
```

Add input files into the workspace `input` directory:

```
root
|--- day01
|--- util
|--- input
     |--- day01.txt
```

Reference input files in the `input` directory from code using the `util` module:

```rs
use util::read_input_lines;

fn main() {
    let lines = read_input_lines("input/day01.txt").unwrap();
    // ...
}
```
