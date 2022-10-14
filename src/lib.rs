// type!(NAME, DEFAULT, "DESCRIPTION")
// ex: bool!(with_debug, false, "Enables the debug mode.")
// and then used as FLAGS_NAME -> value or default
// ex: if `--with_debug` then FLAGS_with_debug -> true otherwise FLAGS_with_debug -> false

//https://abseil.io/docs/python/guides/flags
// for boolean flags, a --[no]NAME variant exists and is preferred to disable
// so --foo sets FLAGS_foo to true and --nofoo sets FLAGS_foo to false (equivalent to not setting --foo)

// required for macros
pub extern crate inventory;

mod flag;
mod parse;

pub use {
    flag::*,
    parse::*,
};

#[macro_export]
/// A macro that generates a commandline flag of type double, given a name, a default value and
/// a help message. The flag returns a `&'static str` value.
///
/// A commandline flag name should be meaningful, in lowercase, and in snake_case.
/// For instance, `verbose`, `db_url` &c.
///
/// The default value is optional (when unset, it is `None`) but when set, it should be a valid
/// `&'static str` or `None`.
///
/// The help message should be a precise and short description of the flag. It is displayed in the
/// help menu automatically.
///
/// # Example
///
/// ```rs
/// // Creates a commandline flag named user_name which takes a username.
/// string!(user_name, "The name of the user.")
///
/// // Creates a commandline flag named db_url with the default value `https://example.com`.
/// double!(db_url, "https://example.com", "URL to connect the database.")
/// ```
macro_rules! string {
    ($name:ident, $default:expr, $description:expr) => {
        $crate::inventory::submit! {
            $crate::Flag::new(
                stringify!($name),
                $description,
                $crate::FlagValue::String($default)
            )
        }
    };
}

#[macro_export]
/// A macro that generates a boolean commandline flag, given a name, a default value and a help message.
/// The flag returns a `bool` value.
///
/// A commandline flag name should be meaningful, in lowercase, and in snake_case.
/// For instance, `verbose`, `db_url` &c.
///
/// The default value is optional (when unset, it is `false`) but when set, it should be a valid
/// boolean or `None`.
///
/// The help message should be a precise and short description of the flag. It is displayed in the
/// help menu automatically.
///
/// # Example
///
/// ```rs
/// // Creates a commandline flag named some_flag of type bool.
/// bool!(some_flag, "A boolean commandline flag.")
///
/// // Creates a commandline flag named verbose of type bool with the default value `true`.
/// bool!(verbose, true, "Prints more logging information.")
/// ```
macro_rules! bool {
    ($name:ident, $default:expr, $description:expr) => {
        $crate::inventory::submit! {
            $crate::Flag::new(
                stringify!($name),
                $description,
                $crate::FlagValue::Bool($default)
            )
        }
    };
}

#[macro_export]
/// A macro that generates an int32 commandline flag, given a name, a default value and a help message.
/// The flag returns a `i32` value.
///
/// A commandline flag name should be meaningful, in lowercase, and in snake_case.
/// For instance, `verbose`, `db_url` &c.
///
/// The default value is optional (when unset, it is `None`) but when set, it should be a valid
/// i32 or `None`.
///
/// The help message should be a precise and short description of the flag. It is displayed in the
/// help menu automatically.
///
/// # Example
///
/// ```rs
/// // Creates a commandline flag named number_to_double which takes a number to double.
/// int32!(number_to_double, "A number which will be doubled.")
///
/// // Creates a commandline flag named wait of with the default value `4`.
/// int32!(wait, 4, "The number of seconds to wait before retrying.")
/// ```
macro_rules! int32 {
    ($name:ident, $default:expr, $description:expr) => {
        $crate::inventory::submit! {
            $crate::Flag::new(
                stringify!($name),
                $description,
                $crate::FlagValue::Int32($default)
            )
        }
    };
}

#[macro_export]
/// A macro that generates an int64 commandline flag, given a name, a default value and a help message.
/// The flag returns a `i64` value.
///
/// A commandline flag name should be meaningful, in lowercase, and in snake_case.
/// For instance, `verbose`, `db_url` &c.
///
/// The default value is optional (when unset, it is `None`) but when set, it should be a valid
/// i64 or `None`.
///
/// The help message should be a precise and short description of the flag. It is displayed in the
/// help menu automatically.
///
/// # Example
///
/// ```rs
/// // Creates a commandline flag named number_to_double which takes a number to double.
/// int64!(number_to_double, "A number which will be doubled.")
///
/// // Creates a commandline flag named wait of with the default value `4`.
/// int64!(wait, 4, "The number of seconds to wait before retrying.")
/// ```
macro_rules! int64 {
    ($name:ident, $default:expr, $description:expr) => {
        $crate::inventory::submit! {
            $crate::Flag::new(
                stringify!($name),
                $description,
                $crate::FlagValue::Int64($default)
            )
        }
    };
}

#[macro_export]
/// A macro that generates an uint64 commandline flag, given a name, a default value and a help message.
/// The flag returns a `u64` value.
///
/// A commandline flag name should be meaningful, in lowercase, and in snake_case.
/// For instance, `verbose`, `db_url` &c.
///
/// The default value is optional (when unset, it is `None`) but when set, it should be a valid
/// u64 or `None`.
///
/// The help message should be a precise and short description of the flag. It is displayed in the
/// help menu automatically.
///
/// # Example
///
/// ```rs
/// // Creates a commandline flag named number_to_double which takes a number to double.
/// uint64!(number_to_double, "A number which will be doubled.")
///
/// // Creates a commandline flag named wait of with the default value `4`.
/// uint64!(wait, 4, "The number of seconds to wait before retrying.")
/// ```
macro_rules! uint64 {
    ($name:ident, $default:expr, $description:expr) => {
        $crate::inventory::submit! {
            $crate::Flag::new(
                stringify!($name),
                $description,
                $crate::FlagValue::Uint64($default)
            )
        }
    };
}

#[macro_export]
/// A macro that generates a commandline flag of type double, given a name, a default value and
/// a help message. The flag returns a `f64` value.
///
/// A commandline flag name should be meaningful, in lowercase, and in snake_case.
/// For instance, `verbose`, `db_url` &c.
///
/// The default value is optional (when unset, it is `None`) but when set, it should be a valid
/// f64 or `None`.
///
/// The help message should be a precise and short description of the flag. It is displayed in the
/// help menu automatically.
///
/// # Example
///
/// ```rs
/// // Creates a commandline flag named number_to_double which takes a number to double.
/// double!(number_to_double, "A number which will be doubled.")
///
/// // Creates a commandline flag named wait of with the default value `4.0`.
/// double!(wait, 4.0, "The number of seconds to wait before retrying.")
/// ```
macro_rules! double {
    ($name:ident, $default:expr, $description:expr) => {
        $crate::inventory::submit! {
            $crate::Flag::new(
                stringify!($name),
                $description,
                $crate::FlagValue::Double($default)
            )
        }
    };
}

