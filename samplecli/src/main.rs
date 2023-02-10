use clap::{App, Arg};

fn main() {
    let matches = App::new("My RPN program")
        .version("1.0.0")
        .author("crashRT")
        .about("Super awesome sample RPN calculator")
        .arg(
            Arg::new("formula_file")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .required(false),
            )
            .get_matches();
    
        match matches.value_of("formula_file") {
            Some(file) => println!("File specipied: {}", file),
            None => println!("No file specified"),
        }

        let verbose = matches.is_present("verbose");
        println!("is verbosity specified?: {}", verbose);
    }
