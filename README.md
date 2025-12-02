Run projects inside the Rust workspace:

```
cargo run -p day01
```

Add new days:

```
cargo new --vcs=none dayNN
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

In `Cargo.toml`:

```toml
[dependencies]
util = { path = "../util" }
```

In rust code:

```rs
use util::read_input_lines;

fn main() {
    let lines = read_input_lines("input/day01.txt").unwrap();
    // ...
}
```
