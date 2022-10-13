gflags_rs::string!(string_flag, "", "A string commandline flag.");
gflags_rs::bool!(bool_flag, false, "A boolean flag.");
gflags_rs::double!(double_flag, 0.0, "A double flag.");
gflags_rs::int64!(int64_flag, 0, "An int64 flag");
gflags_rs::int32!(int32_flag, 0, "An int32 flag.");
gflags_rs::uint64!(uint64_flag, 0, "An uint64 flag.");

fn main() {
    for flag in gflags_rs::parse() {
        println!("{flag:?}");
    }
}
