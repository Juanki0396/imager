use std::path;
use std::process::ExitCode;
use std::env;

pub struct CLI {
    help: bool,
    verbose: bool,
    command_path: path::PathBuf,
    image_path: Option<path::PathBuf>
}

impl CLI {

    pub const HELP: &str = "Draw a picture using instructions in a text file.

USAGE:
    imager [options] INSTRUSTIONS_FILE

META OPTIONS:
    -h, --help              Print the instructions
    -v, --verbose           Run in verbose mode

PATHS
    -i, --image_path PATH   Select the output path
    ";

    pub fn help(&self) -> bool {
        self.help
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn command_path(&self) -> &path::PathBuf {
        &self.command_path
    }

    pub fn image_path(&self) -> &Option<path::PathBuf> {
        &self.image_path
    }

    pub fn parse() -> Result<Self, ExitCode> {
        let args_iter = env::args();
        if args_iter.len() > 5 || args_iter.len() == 1 {
            eprintln!("ERROR: Invalid number of arguments");
            println!("{}", CLI::HELP);
            return Err(ExitCode::FAILURE);
        }

        let mut help = false;
        let mut verbose = false;
        let mut image_path: Option<path::PathBuf> = None;
        let mut last: String = String::new();

        for arg in args_iter {
            if arg == "-h" {
                help = true;
                break;
            } else if arg == "-v" || arg == "--verbose" {
                verbose = true;
            } else if last == "-i" || last == "--image-path" {
                image_path = Some(path::PathBuf::from(arg.clone()));
            }
            last = arg
        }

        let command_path = path::PathBuf::from(last);

        Ok(Self {
            help,
            verbose,
            command_path,
            image_path,
        })
    }

    fn check_path(img_path: &path::PathBuf) -> Result<(), ExitCode> {
        if !img_path.is_dir() && img_path.extension().is_some() {
            Ok(())
        } else {
            Err(ExitCode::FAILURE)
        }
    }

    pub fn check_paths(&self) -> Result<(), ExitCode> {
        if let Some(p) = &self.image_path {
            CLI::check_path(p)?
        }
        CLI::check_path(&self.command_path)
    }

    pub fn vprint(&self, text: &str) {
        if self.verbose() {
            println!("VERBOSE: {}", text);
        }
    }
}


fn main() -> ExitCode {
    let cli = match CLI::parse() {
        Ok(c) => c,
        Err(e) => return e,
    };
    if cli.help() {
        println!("{}", CLI::HELP);
        return ExitCode::SUCCESS;
    }
    cli.vprint("Checking if paths are valid");
    if let Err(e) = cli.check_paths() {
        eprintln!("ERROR: provided paths are not valid");
        return e;
    }
    cli.vprint(format!("Paths are valid. Parsing {:?}", cli.command_path()).as_str());
    return ExitCode::SUCCESS;
}
