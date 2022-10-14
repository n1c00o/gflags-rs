use crate::Flag;
use getopts::{Fail, Matches, Options};
use std::process::exit;

/// Parses the arguments passed when executing the program according to the defined flags.
///
/// # Example
///
/// ```rs
/// parse(std::env::args());
/// ```
pub fn parse(version: &str, args: Vec<String>) {
    let program: &String = &args[0];
    let mut opts: Options = Options::new();

    opts.long_only(true);

    // Insert registered commandline flags
    let _: Vec<()> = inventory::iter::<Flag>
        .into_iter()
        .map(|f: &'static Flag| {
            if f.boolean {
                // Boolean commandline flags have an additional --noNAME flag
                opts.optflag("", f.name, f.description);
                opts.optflag(
                    "",
                    format!("no{}", &f.name).as_str(),
                    format!("Sets {} to false.", f.name).as_str(),
                );
            } else {
                opts.optopt("", f.name, f.description, "");
            }
        })
        .collect();

    // Add additional flags: https://gflags.github.io/gflags/#special.
    // I am not sure to implement file specific flags command (--helpshort, --helpon, ...) for now,
    // since it would require storing the file path / package of the registration.

    // Automatic help flag which creates a help description based on the usage of each flags.
    opts.optflag("h", "help", "Shows the help.");

    // Automatic version flags which returns the program version.
    opts.optflag(
        "V",
        "version",
        format!("Shows the version of {}.", program).as_str(),
    );

    // todo: --undefok (<-- remove unknown flag error)
    // todo: --fromenv
    // todo: --tryfromenv
    // todo: --flagfile

    let matches: Matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!(
                "{}",
                match f {
                    Fail::UnrecognizedOption(o) => format!("unregistered flag {o} passed"),
                    Fail::OptionDuplicated(o) => format!("flag {o} was passed multiple times"),
                    Fail::UnexpectedArgument(a) => format!("flag {a} does not expect a value"),
                    Fail::ArgumentMissing(a) => format!("flag {a} requires a value"),
                    f => format!("internal error: {f}"),
                }
            );
            exit(1);
        }
    };

    // Help command
    if matches.opt_present("help") {
        // todo: create custom help?
        eprintln!("{}", opts.usage(""));
        eprintln!("{} version {}", program, version);
        exit(0);
    }

    // Version command
    if matches.opt_present("version") {
        eprintln!("{} version {}", program, version);
        exit(0);
    }

    // todo: create variables for flags &c.
}
