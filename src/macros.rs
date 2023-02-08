/// Creates a [`CharDurations`] containing the arguments.
///
/// [`char_duration`] allows [`CharDurations`]s to be defined using a method syntax while specifying if the
/// durations should be milliseconds or seconds using `.ms` or `.s` respectively.
/// There are three forms of this macro:
///
/// - Create a [`CharDurations`] with only a default duration of 20 milliseconds:
///
/// ```
/// use print_typewriter::char_duration;
/// use std::time::Duration;
/// use std::collections::HashMap;
///
/// let d = char_duration!(default 20.ms);
/// assert_eq!(d.default_duration, Duration::from_millis(20));
/// assert_eq!(d.specific_duration, HashMap::new());
/// assert_eq!(*d.duration(' '), Duration::from_millis(20));
/// assert_eq!(*d.duration('a'), Duration::from_millis(20));
/// ```
///
/// - Create a [`CharDurations`] with a default duration of 50 milliseconds, a specific duration for
/// spaces of 1 second, and another for commas with 100 milliseconds
///
/// ```
/// use print_typewriter::char_duration;
/// use std::time::Duration;
/// use std::collections::HashMap;
///
/// let d = char_duration!(default 50.ms, ' '->1.s, ','->100.ms);
/// assert_eq!(d.default_duration, Duration::from_millis(50));
/// assert_eq!(d.specific_duration, HashMap::from([(' ', Duration::from_secs(1)), (',', Duration::from_millis(100))]));
/// assert_eq!(*d.duration(' '), Duration::from_secs(1));
/// assert_eq!(*d.duration(','), Duration::from_millis(100));
/// assert_eq!(*d.duration('a'), Duration::from_millis(50));
/// ```
/// - Create a [`CharDurations`] with a specific duration for spaces of 1 second
///
/// ```
/// use print_typewriter::char_duration;
/// use std::time::Duration;
/// use std::collections::HashMap;
///
/// let d = char_duration!(' '->1.s);
/// assert_eq!(d.default_duration, Duration::ZERO);
/// assert_eq!(d.specific_duration, HashMap::from([(' ', Duration::from_secs(1))]));
/// assert_eq!(*d.duration(' '), Duration::from_secs(1));
/// assert_eq!(*d.duration('a'), Duration::ZERO);
/// ```
///
/// [`CharDurations`]: crate::CharDurations
/// [`char_duration`]: crate::char_duration
///
#[macro_export]
macro_rules! char_duration {
    (default $duration:literal.$map_type:ident) => {
        $crate::CharDurations::new($crate::map_type!($map_type)($duration), std::collections::HashMap::new())
    };
    (default $duration:literal.$default_map_type:ident, $( $char:literal->$char_duration:literal.$char_map_type:ident ),+) => {
        $crate::CharDurations::new($crate::map_type!($default_map_type)($duration), std::collections::HashMap::from([$( ($char, $crate::map_type!($char_map_type)($char_duration)) ),+]))
    };
    ($( $char:literal->$char_duration:literal.$char_map_type:ident ),+) => {
        $crate::CharDurations::new(std::time::Duration::ZERO, std::collections::HashMap::from([$( ($char, $crate::map_type!($char_map_type)($char_duration)) ),+]))
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! map_type {
    (ms) => {
        std::time::Duration::from_millis
    };
    (s) => {
        std::time::Duration::from_secs
    };
}

/// Prints a formatted string using the provided [`CharDurations`]
/// Uses [`Writer::print_typed`] to print to the standard output one character at a time, with a newline.
///
/// # Examples
///
/// - Printing "hello world" one word at a time
///
/// ```
/// use print_typewriter::{char_duration, println_typed};
///
/// let duration = char_duration!(' '->150.ms);
/// println_typed!(duration, "hello world");
/// ```
///
/// - Printing a formatted string, one character at a time
///
/// ```
/// use print_typewriter::{char_duration, println_typed};
///
/// let example_variable = "beans";
/// let duration = char_duration!(default 50.ms);
/// println_typed!(duration, "hello {} world", example_variable);
/// ```
///
/// [`Writer::print_typed`]: struct.Writer.html#method.print_typed
/// [`CharDurations`]: crate::CharDurations
///
#[macro_export]
macro_rules! println_typed {
    ($duration:tt, $($arg:tt)*) => {
        {
            let mut output = format!($($arg)*);
            output += "\n";
            $crate::Writer::print_typed(&$duration, &output);
        }
    };
}

/// Prints a formatted string using the provided [`CharDurations`]
/// Uses [`Writer::print_typed`] to print to the standard output one character at a time, without newline.
///
/// # Examples
///
/// - Printing "hello world" one word at a time
///
/// ```
/// use print_typewriter::{char_duration, print_typed};
///
/// let duration = char_duration!(' '->150.ms);
/// print_typed!(duration, "hello world");
/// ```
///
/// - Printing a formatted string, one character at a time
///
/// ```
/// use print_typewriter::{char_duration, print_typed};
///
/// let example_variable = "beans";
/// let duration = char_duration!(default 50.ms);
/// print_typed!(duration, "hello {} world", example_variable);
/// ```
///
/// [`Writer::print_typed`]: struct.Writer.html#method.print_typed
/// [`CharDurations`]: crate::CharDurations
///
#[macro_export]
macro_rules! print_typed {
    ($duration:tt, $($arg:tt)*) => {
        {
            $crate::Writer::print_typed(&$duration, &format!($($arg)*));
        }
    };
}
