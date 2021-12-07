use clap::{App, Arg};
fn main() {
    let matches = App::new("recho")
        .version("0.1.0")
        .author("Zafar Hussain Luni <zafar.luni@gmail.com>")
        .about("A simple implementation of echo command in Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Sets the echo message")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline at the end")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of("text").unwrap().collect::<Vec<_>>();
    let omit_new_line = matches.is_present("omit_newline");

    print!(
        "{}{}",
        text.join(" "),
        if omit_new_line { "" } else { "\n" }
    );
}
