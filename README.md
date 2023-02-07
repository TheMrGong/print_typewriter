# `Print Typewriter`

Simple learning project for a Rust Library that lets you print strings character by character in a configurable way.

## Usage

Typing out "hello" with each character taking 10 milliseconds to be printed

```rust
use print_typewriter::{char_duration, println_typed};

let duration = char_duration!(default 10.ms);
println_typed!(duration, "hello");
```

Typing "hello world" with each word being typed instantly and each space taking 250 milliesconds

```rust
use print_typewriter::{char_duration, println_typed};

let duration = char_duration!(' '->250.ms);
println_typed!(duration, "hello");
```

Typing a formatted string, "hello {} world" with spaces taking 250 milliseconds, periods taking 1 second, and everything else taking 90

```rust
use print_typewriter::{char_duration, println_typed};

let duration = char_duration!(default 90.ms, ' '->250.ms, '.'->1.s);
let beans = "beans";
println_typed!(duration, "hello {} world", beans);
```