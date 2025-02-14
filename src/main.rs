use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

// TODO: find and change all .test to .git when done
fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "init" => {
            let _ = init(if args.len() > 2 {
                Some(args[2].as_str())
            } else {
                None
            });
        }
        _ => eprintln!("command not supported"),
    }
}

fn init(repo_name: Option<&str>) -> Result<bool, ()> {
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
    let is_initialized = Path::new(&git_dir).is_dir();

    if is_initialized {
        println!("Reinitialized existing Git repository in {git_dir}/");
        return Ok(true);
    }

    if repo_name.is_some() {
        fs::create_dir(repo_name.unwrap()).unwrap();
    }

    // dirs under the .git dir
    fs::create_dir(git_dir.clone()).unwrap();
    fs::create_dir(format!("{}/hooks", git_dir.clone())).unwrap();
    fs::create_dir(format!("{}/info", git_dir.clone())).unwrap();
    fs::create_dir(format!("{}/objects", git_dir.clone())).unwrap();
    fs::create_dir(format!("{}/refs", git_dir.clone())).unwrap();

    // files under the .git dir
    File::create(format!("{}/config", git_dir.clone())).unwrap();
    File::create(format!("{}/description", git_dir.clone())).unwrap();
    File::create(format!("{}/HEAD", git_dir.clone())).unwrap();

    return Ok(true);
}
