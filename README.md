# Rickle Pick
`ricklepick` is a library for unpickling Python pickle files in pure Rust. It implements the PM as described in the [pickletools](https://docs.python.org/3/library/pickletools.html) implementation. Should allow for safe reading and conversion to Rust types.

![crabified pickle rick?](https://raw.githubusercontent.com/deathbyknowledge/ricklepick/main/ricklepick.png)

## Usage
`ricklepick` defines its own Value type that encapsulates all possible Python values with a corresponding Rust value. The `load` function reads a bytestream and starts decoding it, returning the resulting Value.

The [cli example](/examples/cli.rs) allows you to try it out easily:
```
$ cargo run --example=cli -- mypicklefile
Opening file mypicklefile

(1, 2, 3, 4, (5, 6, 7), 'Test', ('This is just a test.', [2, 4, 6, 8]), 'One', 'Two', 'Three')
```