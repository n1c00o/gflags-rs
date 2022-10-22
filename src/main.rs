pub mod flag;

use std::fmt::Debug;
use lazy_static::lazy_static;


inventory::submit! {
    Flag { name: "foo", value: FlagValue::U8(8) }
}

inventory::submit! {
    Flag { name: "bar", value: FlagValue::U8(255) }
}

lazy_static! {
    static ref FLAGS_FOO: u8 = resolve_flag::<u8>("foo");
}

fn resolve_flag<T: TryFrom<&'static FlagValue, Error = impl Debug>>(flag_name: &'static str) -> T {
    let flag: &'static Flag = inventory::iter::<Flag>
        .into_iter()
        .find(|f| f.name == flag_name)
        .expect("no flag found for given name");

    T::try_from(&flag.value).expect("cannot convert to the given type")
}

fn main() {
    println!("{}", *FLAGS_FOO);
}
