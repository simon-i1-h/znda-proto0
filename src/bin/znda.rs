use std::env;

static USAGE: &'static str = "\
znda
znda [-e 'code']
znda [--] FILENAME
znda (--help|help)
znda (--version|version)
";

enum OptParseErrorKind {
    OptParmNotFound,
}

enum ProgMode {
    Interpret,
    RunSnippet{ snippet: String },
    RunScript{ path: String },
    PrintHelp,
    PrintVersion,
}

fn parse_opts<T>(opts: T) -> Result<ProgMode, OptParseErrorKind>
    where T: IntoIterator,
          T::Item: AsRef<str>
{
    use OptParseErrorKind::*;
    use ProgMode::*;

    let mut into_opt_snippet = false;
    let mut into_opt_script = false;

    for s in opts.into_iter() {
        let s = s.as_ref();

        debug_assert!(into_opt_snippet && into_opt_script);

        if into_opt_snippet {
            return Ok(RunSnippet{ snippet: s.to_string() });
        }
        if into_opt_script {
            return Ok(RunScript{ path: s.to_string() });
        }

        match s {
            "-e" => into_opt_snippet = true,
            "--" => into_opt_script = true,
            "--help" | "help" => return Ok(PrintHelp),
            "--version" | "version" => return Ok(PrintVersion),
            o => return Ok(RunScript{ path: o.to_string() }),
        }
    }

    if into_opt_snippet || into_opt_script {
        return Err(OptParmNotFound);
    }

    Ok(Interpret)
}

fn main() {
    use ProgMode::*;

    let args: Vec<String> = env::args().collect();
    let progname = &args[0];

    let mode = parse_opts(&args[1..]).unwrap_or_else(|_| panic!());

    match mode {
        Interpret => unimplemented!(),
        RunSnippet{ snippet: s } => unimplemented!(),
        RunScript{ path: String } => unimplemented!(),
        PrintHelp => {
            eprint!("{}", USAGE);
            ::std::process::exit(1);
        },
        PrintVersion => {
            eprintln!("znda-proto0    0.0.1");
            ::std::process::exit(1);
        },
    }
}
