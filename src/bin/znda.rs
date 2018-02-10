// znda # インタプリタ起動
// znda [-e 'code'] # コード実行
// znda [--] FILENAME # コード実行
// znda (--help|help)
// znda (--version|version)

use std::env;
use std::ffi::OsStr;

enum OptParseErrorKind {
    Error
}

enum ProgMode {
    RunCodePieces{ code_pieces: Vec<String> },
    RunScript{ filenames: Vec<String> },
    PrintHelp,
    PrintVersion
}

// TODO 悪い意味で富豪的なので直す
fn parse_opts<T>(opts: T) -> Result<ProgMode, OptParseErrorKind>
    where T: IntoIterator,
          T::Item: AsRef<str>
{
//    let mode = Node; TODO help_flagなどを使わずにProgModeを使う?
    let mut into_opt_e = false;
    let mut pieces = Vec::new();
    let mut filenames = Vec::new();

    for s in opts.into_iter() {
        let s = s.as_ref();

        match s {
            "-e" => into_opt_e = true,
            "--help" | "help" => return Ok(ProgMode::PrintHelp),
            "--version" | "version" => return Ok(ProgMode::PrintVersion),
            o => {
                if into_opt_e {
                    pieces.push(s.to_string());
                    into_opt_e = false;
                } else if !pieces.is_empty() {
                    filenames.push(s.to_string());
                }
            }
        }
    }

    if into_opt_e {
        // TODO return error
        unimplemented!()
    }

    unimplemented!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = &args[0];

    let res = parse_opts(&args[1..]).unwrap_or_else(|_| panic!());
}
