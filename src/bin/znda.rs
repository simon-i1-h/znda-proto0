// znda # インタプリタ起動
// znda [-e 'code'] # コード実行
// znda [--] FILENAME # コード実行
// znda (--help|help)
// znda (--version|version)

use std::env;

enum OptParseErrorKind {
    Error
}

enum ProgMode {
    RunSnippet{ snippet: String },
    RunScript{ path: String },
    PrintHelp,
    PrintVersion,
}

// TODO 悪い意味で富豪的なので直す
fn parse_opts<T>(opts: T) -> Result<ProgMode, OptParseErrorKind>
    where T: IntoIterator,
          T::Item: AsRef<str>
{
    let mut into_opt_e = false;

    for s in opts.into_iter() {
        let s = s.as_ref();

        if into_opt_e {
            return Ok(ProgMode::RunSnippet{ snippet: s.to_string() });
        }

        match s {
            "-e" => into_opt_e = true,
            "--help" | "help" => return Ok(ProgMode::PrintHelp),
            "--version" | "version" => return Ok(ProgMode::PrintVersion),
            o => return Ok(ProgMode::RunScript{ path: o.to_string() }),
        }
    }

    Err(OptParseErrorKind::Error)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = &args[0];

    let mode = parse_opts(&args[1..]).unwrap_or_else(|_| panic!());
}
