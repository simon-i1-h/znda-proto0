// znda # インタプリタ起動
// znda [-e 'code'] # コード実行
// znda [--] FILENAME... # コード実行
// znda (--help|help)
// znda (--version|version)

extern crate getopts;
use getopts::Options;
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
fn parse_opts<T>(opts: T) -> Result<(), OptParseErrorKind>
    where T: IntoIterator,
          T::Item: AsRef<str>
{
//    let mode = Node; TODO help_flagなどを使わずにProgModeを使う?
    let mut into_opt_e_parse = false;
    let mut pieces = Vec::new();
    let mut help_flag = false;
    let mut version_flag = false;
    let mut filenames = Vec::new();

    for s in opts.into_iter() {
        let s = s.as_ref();

        if help_flag || version_flag {
            return Err(OptParseErrorKind::Error);
        }

        match s {
            "-e" => into_opt_e_parse = true,
            "--help" | "help" => help_flag = true,
            "--version" | "version" => version_flag = true,
            o => {
                if into_opt_e_parse {
                    pieces.push(s.to_string());
                    into_opt_e_parse = false;
                } else if !pieces.is_empty() {
                    filenames.push(s.to_string());
                }
            }
        }
    }

    debug_assert!(help_flag && version_flag);
    if help_flag {
        // TODO print usage and exit? -> ではなく、Okを返す
        unimplemented!()
    } else if version_flag {
        // TODO print version and exit? -> ではなく、Okを返す
        unimplemented!()
    }

    if into_opt_e_parse {
        // TODO return error
        unimplemented!()
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = &args[0];

    let res = parse_opts(&args[1..]).unwrap_or_else(|_| panic!());
}
