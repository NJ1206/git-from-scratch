use std::env;
use std::path::Path;
use std::process::Command;

// TODO: find and change all .test to .git when done
fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "init" => init(),
        _ => eprintln!("command not supported"),
    }
}

fn init() {
    // check if a git folder has already been created or not
    let current_dir = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let git_dir = current_dir + "/.git";
    let is_initialized = Path::new(&git_dir).is_dir();

    if is_initialized {
        println!("Reinitialized existing Git repository in {git_dir}/");
        return;
    }

    let _cmds: Vec<Vec<&str>> = vec![
        vec!["mkdir", ".test"], // create an empty .test dir
        // files and dirs under the .test dir
        vec!["touch", ".test/config"],
        vec!["touch", ".test/description"],
        vec!["touch", ".test/HEAD"],
        vec!["mkdir", ".test/hooks"],
        vec!["mkdir", ".test/info"],
        vec!["mkdir", ".test/objects"],
        vec!["mkdir", ".test/refs"],
    ];
}

fn execute_programs(cmd: String, args: Option<Vec<&str>>) -> Result<(bool, String), ()> {
    let output;
    match args {
        Some(args) => output = Command::new(cmd).arg(args.join(" ")).output(),
        None => output = Command::new(cmd).output(),
    }

    return match &output {
        Ok(v) => Ok((true, String::from_utf8_lossy(&v.stdout).to_string())),
        Err(e) => Ok((false, format!("{e}"))),
    };
}
