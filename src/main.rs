use std::env;
use std::{fs::{read_dir, DirEntry},
          path::{Path, PathBuf}};

enum TruckEntry {
    File(Box<Path>),
    Dir(Box<Path>, Vec<TruckEntry>),
}

fn get_filtered_tree<F>(dir: &PathBuf, is_match: &F) -> Option<TruckEntry>
where
    F: Fn(DirEntry) -> bool,
{
    let entries = read_dir(dir).unwrap();

    let entries: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let file_type = entry.file_type().unwrap();
            let path = entry.path();

            if file_type.is_dir() {
                match get_filtered_tree(&path, is_match) {
                    Some(x) => Some(x),
                    None => {
                        if is_match(entry) {
                            Some(TruckEntry::Dir(path.into_boxed_path(), vec![]))
                        } else {
                            None
                        }
                    }
                }
            } else if file_type.is_file() || file_type.is_symlink() {
                if is_match(entry) {
                    Some(TruckEntry::File(path.into_boxed_path()))
                } else {
                    None
                }
            } else {
                panic!()
            }
        })
        .collect();

    if entries.len() != 0 {
        Some(TruckEntry::Dir(dir.clone().into_boxed_path(), entries))
    } else {
        None
    }
}

fn print_tree(root: &TruckEntry, level: i32) {
    let print_entry = |path: &Box<Path>| {
        println!(
            "{}├ {}",
            "│ ".repeat(level as usize),
            path.file_name().unwrap().to_str().unwrap()
        )
    };

    match root {
        TruckEntry::File(path) => print_entry(path),
        TruckEntry::Dir(path, entries) => {
            print_entry(path);
            for entry in entries {
                print_tree(entry, level + 1);
            }
        }
    }
}

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let search_string = args.next().unwrap();

    let dir = args.next()
        .map(|dir| PathBuf::from(&dir))
        .unwrap_or(env::current_dir().unwrap());

    let tree = get_filtered_tree(&dir, &|entry: DirEntry| {
        entry
            .path()
            .file_name()
            .map(|file_name| file_name.to_str().unwrap().contains(&search_string))
            .unwrap_or(false)
    });

    print_tree(&tree.unwrap(), 0);
}
