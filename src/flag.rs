use std::fmt::{Display, Formatter};
use syn::__private::bool;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
/// A flag represents a commandline argument to parse from the run command.
///
/// # Example
///
/// ```rs
/// // --foo=<STRING> (A flag; default:"")
/// Flag::new("foo", "A flag", FlagValue::String(""));
/// ```
pub struct Flag {
    /// The commandline flag name.
    pub name: &'static str,

    /// The commandline flag description or help message, used in the help message.
    pub description: &'static str,

    /// The commandline flag default value.
    pub default_value: FlagValue,

    /// The value passed to the flag when parsed.
    pub parsed_value: Option<FlagValue>,

    // todo(n1c00o): add flag validator (https://gflags.github.io/gflags/#validate)
    // Could be done with declarative macros by referencing a function,
    // or passing a closure...

    // todo(n1c00o): register the package where a flag is declared
    // This can then be used to implement custom flags such as --helpshort (only displays the
    // flags from the main package), --helpon...
}

impl Flag {
    /// Create a Flag which can be collected by the parser.
    pub fn new(
        name: &'static str,
        description: &'static str,
        default_value: FlagValue
    ) -> Self {
        Self {
            name,
            description,
            default_value,
            parsed_value: None
        }
    }

    /// Generate a string message to be displayed in the help menu.
    ///
    /// The help message provides the flag's name, description and default value.
    /// The current help format is inspired by kubectl's one:
    /// --NAME=DEFAULT_VALUE:
    ///     DESCRIPTION
    ///
    /// # Example
    ///
    /// ```rs
    /// let foo: Flag = Flag::new("foo", "A very descriptive and long message.", FlagValue::String("__default"));
    /// println!("{}", foo.help_message());
    /// // Output:
    /// //  --foo="__default":
    /// //      A very descriptive and long message.
    /// ```
    pub fn help_message(&self) -> String {
        format!("--{}={}\n\t{}", self.name, self.default_value,self.description)
    }
}

inventory::collect!(Flag);

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
/// The possible types a commandline flag accepts.
///
/// While it is possible to add an unlimited of types (using traits), we limit the available types
/// to the one described in the C++ implementation of GFlags (https://gflags.github.io/gflags/#define).
/// It allows us to automatically create utility flags (such as --noNAME for booleans) in addition
/// to simplifying the API exposed by tools.
///
/// To provide more complex types, you can use validators (TODO)
/// or document a specific format to pass as a string.
/// For instance, a list could be represented as a comma "," separated string of elements:
/// --support_languages="fr_FR,en_US,de"
pub enum FlagValue {
    /// A boolean value, parsed as `bool` in Rust.
    Bool(bool),

    /// A signed 32 bit integer value, parsed as `i32` in Rust.
    Int32(i32),

    /// A signed 64 bit integer value, parsed as `i64` in Rust.
    Int64(i64),

    /// An unsigned 64 bit integer value, parsed as `u64` in Rust.
    Uint64(u64),

    /// A 64 bit floating value, parsed as `f64` in Rust.
    Float64(f64),

    /// A string value, parsed as a `&'static str` in Rust.
    String(&'static str),
}

impl Display for FlagValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FlagValue::Bool(b) => write!(f, "{}", b),
            FlagValue::Int32(i) => write!(f, "{}", i),
            FlagValue::Int64(i) => write!(f, "{}", i),
            FlagValue::Uint64(u) => write!(f, "{}", u),
            FlagValue::Float64(fl) => write!(f, "{}", fl),
            FlagValue::String(s) => write!(f, "\"{}\"", s)
        }
    }
}

/// A simple macro_rule to build the `impl` blocks required for each types.
macro_rules! __impls_for_type {
    // t -> a type (bool, &'static str &c).
    // v -> a variant of the FlagValue enum.
    ($t:ty, $v:ident) => {
        // for type $t
        impl From<$t> for FlagValue {
            fn from(value: $t) -> Self {
                FlagValue::$v(value)
            }
        }

        impl TryFrom<&'static FlagValue> for $t {
            type Error = ();

            fn try_from(value: &'static FlagValue) -> Result<Self, Self::Error> {
                match value {
                    FlagValue::$v(b) => Ok(*b),
                    _ => Err(())
                }
            }
        }
    };
}

__impls_for_type!(bool, Bool);
__impls_for_type!(i32, Int32);
__impls_for_type!(i64, Int64);
__impls_for_type!(u64, Uint64);
__impls_for_type!(f64, Float64);
__impls_for_type!(&'static str, String);
