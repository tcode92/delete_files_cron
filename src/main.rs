use std::{
    env,
    fs::{self, DirEntry},
    io::{self, Write},
    path::PathBuf,
    process::exit,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args.get(0).unwrap();
    let first_arg = args.get(1).expect("No enough arguments.");
    if first_arg == "help" {
        print_help(program);
        exit(0);
    }
    let path = args.get(1).expect("Path not found");
    let max = args
        .get(2)
        .expect("No max files to keep specified.")
        .parse()
        .expect("Invalid max files.");
    let files = files_to_delete(path, max);
    let skip_confirm = args.get(3);

    if skip_confirm.is_some() && skip_confirm.unwrap() == "--skip-confirm" {
        for file in &files {
            delete_file(file)
        }
        println!("Deleted {} files.", files.len());
    } else {
        println!("--------------------");
        for file in &files {
            println!("{}", file);
        }
        println!("--------------------");

        print!("\nAre you sure you want to delete this files? [Y/n]: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() == "y" {
            for file in &files {
                delete_file(file);
            }
            println!("Deleted {} files.", files.len());
        }
        exit(0);
    }
}
fn print_help(program: &String) {
    println!("This program delete all files in the given path based on last modified time.");
    println!("By default the program will ask for confirmation.");
    println!("Use --skip-confirm to delete the files without confirmation prompt.");
    println!(
        "Usage: {} <path> <max number of files to keep> [--skip-confirm]",
        program
    );
}

fn files_to_delete(path: &str, max: usize) -> Vec<String> {
    // construct absolute path or exit
    let p = match PathBuf::from(path).canonicalize() {
        Ok(path) => path,
        Err(_) => panic!("Path not found."),
    };
    // read all files in the path
    let mut files: Vec<DirEntry> = p.read_dir().unwrap().map(|x| x.unwrap()).collect();

    files.sort_by(|a, b| {
        let a_mtime = a.metadata().unwrap().modified().unwrap();
        let b_mtime = b.metadata().unwrap().modified().unwrap();
        b_mtime.cmp(&a_mtime)
    });
    if files.len() <= max {
        println!("No files to delete.");
        exit(0);
    }
    let (_, files_to_delete) = files.split_at(max);
    let mut can_delete_file: Vec<String> = Vec::new();
    for f in files_to_delete {
        can_delete_file.push(f.path().to_str().unwrap().to_string())
    }
    return can_delete_file;
}

fn delete_file(path: &String) {
    let r = fs::remove_file(path);
    if r.is_err() {
        eprintln!("Cannot delete file {} because: {}", path, r.unwrap_err())
    }
}
