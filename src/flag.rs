#[derive(Debug, PartialOrd, PartialEq)]
/// The possible values for a flag.
pub enum FlagValue {
    /// A bool value.
    Bool(bool),

    /// A i32 value.
    Int32(i32),

    /// A i64 value.
    Int64(i64),

    /// A u64 value.
    Uint64(u64),

    /// A f64 value.
    Double(f64),

    /// A &'static str value.
    String(&'static str),
}

#[derive(Debug, PartialOrd, PartialEq)]
/// A commandline flag.
pub struct Flag {
    /// The flag name, which should be a lowercase string in snake_case.
    pub name: &'static str,

    /// The default value.
    pub default: FlagValue,

    /// The help message for the flag.
    pub description: &'static str,

    // todo: RegisterFlagValidator (a closure to verify (during parse-time) the value passed to a flag)
}

impl Flag {
    /// Creates a new flag.
    pub const fn new(name: &'static str, description: &'static str, default: FlagValue) -> Self {
        Flag { name, default, description }
    }
}

inventory::collect!(Flag);