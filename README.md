# `Print Typewriter`

Simple learning project for a Rust Library that lets you print strings character by character in a configurable way.

## Usage

Typing out "hello" with each character taking 10 milliseconds to be printed

```rust
use print_typewriter::{CharDurations, Writer};
use std::time::Duration;
use std::collections::HashMap;

let ten_millis = Duration::from_millis(10);

let chat_durations = CharDurations::new(ten_millis, HashMap::new());

Writer::print_typed(&chat_durations, &"hello".to_owned());
```

Typing "hello world" with each word being typed instantly and each space taking 250 milliesconds

```rust
use print_typewriter::{CharDurations, Writer};
use std::time::Duration;
use std::collections::HashMap;

let two_fifty_millis = Duration::from_millis(250);

let chat_durations = CharDurations::new(Duration::ZERO, HashMap::from([(' ', two_fifty_millis)]));

Writer::print_typed(&chat_durations, &"hello".to_owned());
```