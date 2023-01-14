use std::process::ExitCode;
use std::env::args;
use std::path::Path;
use std::fs::copy;
use same_file::is_same_file;

// Use at .git/config
// [merge "binary-merge"]
//     name = Create base, ours, theirs file for conflict
//     driver = binary-conflict %O %A %B %P

fn copy_file(from: &str, name: &str, ext: &str, cmp_type: &str) {
    let to_name = name.to_string() + cmp_type;
    let to_file = (&to_name[..]).to_string() + ext;
    let to_exist = Path::new(&to_file).exists();

    if to_exist {
        let name = to_name + cmp_type;
        copy_file(from, &name, ext, cmp_type);
    }

    match copy(from, &to_file) {
        Ok(_v) => println!("Created file, {}", &to_file),
        Err(_e) => println!("Error at create file, {}", &to_file)
    }
}

fn main() -> ExitCode {
    let base_file   = args().nth(1).expect("Needs base file args, use %O");
    let our_file    = args().nth(2).expect("Needs our file args, use %A");
    let their_file  = args().nth(3).expect("Needs their file args, use %B");
    let target_file = args().nth(4).expect("Needs target file args, use %P");

    let target_info = Path::new(&target_file);
    let target_name = target_info.file_stem().unwrap().to_str().unwrap();
    let target_dir  = target_info.parent().unwrap().to_str().unwrap_or("");
    let target_ext  =  if let Some(ext) = target_info.extension() {
        let ext = ".".to_string() + ext.to_str().unwrap();
        ext
    } else {
        "".to_string()
    };

    let conflict_check = | cmp_file: &str, cmp_type: &str | {
        let is_same = is_same_file(&our_file, cmp_file).unwrap();
        if is_same {
            return false;
        }
        else {
            let target_name = String::from(
                Path::new(target_dir).join(target_name).to_string_lossy()
            );
            copy_file(cmp_file, &target_name, &target_ext, cmp_type);
            return true;
        }
    };

    let base_check  = conflict_check(&base_file, ".base");
    let their_check = conflict_check(&their_file, ".their");
    if base_check | their_check {
        return ExitCode::from(1)
    }
    ExitCode::SUCCESS
}
