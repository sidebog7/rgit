use std::fs::create_dir;
use std::path::Path;
use std::process;

pub fn init(paths: Vec<&str>) {
    for path in &paths {
        init_path(path);
    }
}

const COREDIRS: &'static [&'static str] = &["objects", "refs"];

fn init_path(path: &str) {
    println!("Init Path {}", path);
    let passed_path = Path::new(path);

    if !passed_path.exists() {
        eprintln!("Path {} doesn't exist", path);
        process::exit(1);
    }

    if !passed_path.is_dir() {
        eprintln!("{} is not a directory", path);
        process::exit(1);
    }

    let root_buf = passed_path.join(".git");
    let root = root_buf.as_path();

    if root.exists() {
        eprintln!("{} already has been initialised", path);
        process::exit(1);
    }

    match create_dir(root) {
        Err(err) => {
            eprintln!(
                "Error creating structure in {} due to the following error: {}",
                path, err
            );
            process::exit(1);
        }
        Ok(_) => {}
    }

    create_init_paths(&root);

    // TODO: Need to convert root into absolute path
    println!("Initialised empty rgit repository in {:?}", root);
}

fn create_init_paths(root: &Path) {
    let dir_iter = COREDIRS.into_iter();
    dir_iter.for_each(|dir| {
        let dir_path = root.join(dir);
        match create_dir(dir_path) {
            Err(err) => {
                eprintln!("Error creating {:?}, due to {}", dir, err);
                process::exit(1);
            }
            Ok(_) => {}
        }
    });
}
