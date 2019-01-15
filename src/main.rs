use std::fs;
use std::io;
use std::env;
use std::path::Path;
use std::time::UNIX_EPOCH;
use std::time;

use ls::Config;


fn main() {
    let Config { path, long, all }  = Config::new(env::args()); 

    let paths = get_paths(path).unwrap();

    if long {
        ls_fmt_long(paths);
    } else {
        ls_format(paths);
    }

}


fn get_paths(path: String) -> io::Result<Vec<fs::DirEntry>> {
    let mut dirs = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        dirs.push(entry);
    }

    Ok(dirs)

}

fn ls_format(files: Vec<fs::DirEntry>) -> io::Result<()> {
    let mut output = String::new();

    for f in files {
        let partial = format!(" {} ", fmt_fname(&f));
        output.push_str(&partial);
    }

    println!("{}", output);

    Ok(())
}

fn ls_fmt_long(files: Vec<fs::DirEntry>) -> io::Result<()> {
    let mut output = String::new();

    for f in files {

        let partial = format!(" {} {} \n", metadata_string(&f), fmt_fname(&f));
        output.push_str(&partial);
    }

    println!("{}", output);

    Ok(())
}

fn metadata_string(f: &fs::DirEntry) -> String {
    let data = match f.metadata() {
        Ok(d) => d,
        Err(_) => return String::from("Could not get")
    };

    let type_char = if data.is_dir() {
        'd'
    } else {
        '-'
    };

    let modified = match data.modified() {
        Ok(t) => t,
        Err(_) => UNIX_EPOCH
    };
    
    
    let since = match modified.duration_since(UNIX_EPOCH) {
        Ok(t) => t.as_secs(),
        Err(t) => 0u64
    };

    format!("{} {:?}", type_char, since)
}

fn fmt_fname(f: &fs::DirEntry) -> String  {

    let is_dir = match f.file_type() {
        Ok(t) => t.is_dir(),
        Err(_) => false
    };

    let name = match f.file_name().into_string() {
        Ok(n) => n,
        Err(_) => String::from("Unknown.")
    };

    let bold_purple = "\x1b[1m\x1b[34m";
    let clear = "\x1b[0m";

    if is_dir {
        format!("{}{}{}", bold_purple, name, clear)
    } else {
        name
    }
}

