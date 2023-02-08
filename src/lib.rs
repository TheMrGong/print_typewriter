//! This crate provides a simple way to print a string and have each letter take a
//! duration of time to be printed out.
//!
//! # Examples
//!
//! Typing out "hello" with each character taking 10 milliseconds to be printed
//!
//! ```
//! use print_typewriter::{char_duration, println_typed};
//!
//! let duration = char_duration!(default 10.ms);
//! println_typed!(duration, "hello");
//! ```
//!
//! Typing "hello world" with each word being typed instantly and each space taking 250 milliesconds
//!
//! ```
//! use print_typewriter::{char_duration, println_typed};
//!
//! let duration = char_duration!(' '->250.ms);
//! println_typed!(duration, "hello world");
//! ```
//!
//! Typing a formatted string, "hello {} world" with spaces taking 250 milliseconds, periods taking 1 second, and everything else taking 90
//!
//! ```
//! use print_typewriter::{char_duration, println_typed};
//!
//! let duration = char_duration!(default 90.ms, ' '->250.ms, '.'->1.s);
//! let beans = "beans";
//! println_typed!(duration, "hello {} world", beans);
//! ```

use std::{
    collections::HashMap,
    io::{self, Write},
    thread,
    time::Duration,
};
mod macros;

/// A `CharDurations` type to represent how long [`Writer::print_typed`] should take
/// to print out the inputted [`String`]
///
/// It defines how long [`Writer`] should wait after printing each character.
///
/// [`default duration`]: struct.CharDurations.html#structfield.default_duration
/// [`specific duration`]: struct.CharDurations.html#structfield.specific_duration
/// [`Writer::print_typed`]: struct.Writer.html#method.print_typed
/// # Examples
///
/// ```
/// use print_typewriter::CharDurations;
/// use std::time::Duration;
/// use std::collections::HashMap;
///
/// const two_hundred_millis: Duration = Duration::from_millis(200);
/// const one_hundred_millis: Duration = Duration::from_millis(100);
/// const fifty_millis: Duration = Duration::from_millis(50);
///
/// let per_word = CharDurations::new(
///     Duration::ZERO,
///     HashMap::from([
///         (' ', two_hundred_millis)
///     ])
/// );
/// let per_letter = CharDurations::new(
///     fifty_millis,
///     HashMap::from([
///         (' ', one_hundred_millis),
///         (',', one_hundred_millis),
///         ('.', two_hundred_millis)
///     ])
/// );
///
/// assert_eq!(*per_word.duration(' '), two_hundred_millis);
/// assert_eq!(*per_word.duration('a'), Duration::ZERO);
///
/// assert_eq!(*per_letter.duration(' '), one_hundred_millis);
/// assert_eq!(*per_letter.duration('.'), two_hundred_millis);
/// assert_eq!(*per_letter.duration('b'), fifty_millis);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct CharDurations {
    pub default_duration: Duration,
    pub specific_duration: HashMap<char, Duration>,
}

impl CharDurations {
    /// Constructs a new `CharDurations` that is initialized with random keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use print_typewriter::CharDurations;
    /// use std::time::Duration;
    /// use std::collections::HashMap;
    ///
    /// let s = CharDurations::new(Duration::from_millis(40), HashMap::from([]));
    /// ```
    #[inline]
    pub fn new(dur: Duration, specific_durations: HashMap<char, Duration>) -> Self {
        CharDurations {
            default_duration: dur,
            specific_duration: specific_durations,
        }
    }
}

impl CharDurations {
    /// Returns the duration to wait for a character
    ///
    /// Will return [`default duration`] if supplied character isn't in the [`specific duration`] [`HashMap`] as a key
    ///
    /// [`default duration`]: struct.CharDurations.html#structfield.default_duration
    /// [`specific duration`]: struct.CharDurations.html#structfield.specific_duration
    ///
    /// # Examples
    ///
    /// ```
    /// use print_typewriter::CharDurations;
    /// use std::time::Duration;
    /// use std::collections::HashMap;
    ///
    /// let ten_millis = Duration::from_millis(10);
    /// let one_hundred_millis = Duration::from_millis(100);
    ///
    /// let chat_durations = CharDurations::new(ten_millis, HashMap::from([(' ', one_hundred_millis)]));
    ///
    /// assert_eq!(*chat_durations.duration(' '), one_hundred_millis);
    /// assert_eq!(*chat_durations.duration('a'), ten_millis);
    ///
    /// ```
    #[inline]
    pub fn duration(&self, ch: char) -> &Duration {
        match self.specific_duration.get(&ch) {
            Some(dur) => dur,
            _ => &self.default_duration,
        }
    }
}

/// A `Writer` to print out given strings one letter at a time using the provided [`CharDurations`]
///
/// The writer will block the current thread after printing each character and flushing [`Stdout`]
///
/// [`Stdout`]: https://doc.rust-lang.org/1.67.0/std/io/struct.Stdout.html#method.flush
///
/// # Examples
///
/// # Examples
///
/// ```
/// use print_typewriter::{CharDurations, Writer};
/// use std::time::Duration;
/// use std::collections::HashMap;
///
/// let ten_millis = Duration::from_millis(10);
///
/// let chat_durations = CharDurations::new(ten_millis, HashMap::new());
///
/// Writer::print_typed(&chat_durations, &"hello".to_owned());
///
/// ```
pub struct Writer;

impl Writer {
    /// Prints a character one at a time, flushing [`Stdout`] after every print.
    ///
    /// Uses the provided [`CharDurations`] to determine how long to wait between characters
    /// and blocks the current thread for that duration. If flushing a character does not succeed,
    /// printing will exit early with the message "Failed to flush stdout" printed.
    ///
    /// [`Stdout`]: https://doc.rust-lang.org/1.67.0/std/io/struct.Stdout.html#method.flush
    ///
    /// # Examples
    ///
    /// ```
    /// use print_typewriter::{CharDurations, Writer};
    /// use std::time::Duration;
    /// use std::collections::HashMap;
    ///
    /// let ten_millis = Duration::from_millis(10);
    ///
    /// let chat_durations = CharDurations::new(ten_millis, HashMap::new());
    ///
    /// Writer::print_typed(&chat_durations, &"hello".to_owned());
    ///
    /// ```
    pub fn print_typed(durations: &CharDurations, str: &str) {
        for l in str.chars() {
            let wait_duration = durations.duration(l);
            print!("{l}");
            if let Ok(()) = io::stdout().flush() {
                if wait_duration.as_millis() > 0 {
                    thread::sleep(*wait_duration);
                }
            } else {
                println!("Failed to flush stdout");
                break;
            }
        }
    }
}
