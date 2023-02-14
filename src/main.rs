use clap::{App, Arg}; // tell Rust you will use these two structs in clap
use lib::{run, Config}; // tell Rust you will use these two things from our "lib" module

fn main() {
    // Define command-line interface
    let matches = App::new("rust_find")
        .version("0.1.0")
        .author("Aamir Ali")
        .about("Find files that match a regex pattern")
        .arg(
            Arg::from("-p, --patterns=<patterns> 'List of file patterns to find'")
                .takes_value(true)
                .required(true)
                .multiple_values(true), // this argument can takes multiple values
        )
        .arg(
            Arg::from("-o, --output=<output> 'Write results to output file instead of stdout'")
                .takes_value(true) // argument if true or flag if false.
                .required(false), // this is an optional argument
        )
        .arg(
            Arg::from("-d, --dirs=<dirs> 'a set of directories'")
                .takes_value(true) // argument if true or flag if false.
                .required(true)
                .multiple_values(true),
        )
        .arg(
            Arg::from("-s, --size=<size> 'minimum size'")
                .takes_value(true) // argument if true or flag if false.
                .required(false), // this is an optional argument
        )

        .get_matches();
    // .get_matches_from(vec!["rust-find", "--patterns=.*/.rs", "--output=./tests.out"]);

    let args = Config::from_args(&matches); // will be defined later

    if let Err(err) = run(&args) {
        //Error handling here!
        panic!("{}", err)
    }
}
