use std::env;
use std::path::Path;
use std::process::Command;

// TODO: find and change all .test to .git when done
fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "init" => init(if args.len() > 2 {
            Some(args[2].as_str())
        } else {
            None
        }),
        _ => eprintln!("command not supported"),
    }
}

fn init(repo_name: Option<&str>) {
    // check if a git folder has already been created or not
    let current_dir = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let full_name = match repo_name {
        Some(name) => format!("/{}/.git", name),
        None => "/.git".to_string(),
    };

    let git_dir = current_dir + full_name.as_str();
    println!("{git_dir}");
    let is_initialized = Path::new(&git_dir).is_dir();

    if is_initialized {
        println!("Reinitialized existing Git repository in {git_dir}/");
        return;
    }

    if repo_name.is_some() {
        let _ = execute_programs(
            String::from("mkdir"),
            Some(vec![String::from(repo_name.unwrap())]),
        );
    }

    let cmds: Vec<Vec<String>> = vec![
        vec![String::from("mkdir"), git_dir.clone()], // create an empty .test dir
        // files and dirs under the .test dir
        vec![String::from("touch"), format!("{}/config", git_dir.clone())],
        vec![
            String::from("touch"),
            format!("{}/description", git_dir.clone()),
        ],
        vec![String::from("touch"), format!("{}/HEAD", git_dir.clone())],
        vec![String::from("mkdir"), format!("{}/hooks", git_dir.clone())],
        vec![String::from("mkdir"), format!("{}/info", git_dir.clone())],
        vec![
            String::from("mkdir"),
            format!("{}/objects", git_dir.clone()),
        ],
        vec![String::from("mkdir"), format!("{}/refs", git_dir.clone())],
    ];

    for cmd in &cmds {
        match execute_programs(String::from(cmd[0].as_str()), Some(cmd[1..].to_vec())) {
            Ok(_v) => (),
            Err(e) => panic!("Error initialising GIT repository: {e}"),
        }
    }
}

fn execute_programs(cmd: String, args: Option<Vec<String>>) -> Result<String, String> {
    let output;
    match args {
        Some(args) => output = Command::new(cmd).arg(args.join(" ")).output(),
        None => output = Command::new(cmd).output(),
    }

    return match &output {
        Ok(v) => Ok(String::from_utf8_lossy(&v.stdout).to_string()),
        Err(e) => Err(format!("{e}")),
    };
}
