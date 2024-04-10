use std::fs;
use std::io;


fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: decompressor <file>");
        std::process::exit(0);
    }

    let file_name = std::path::Path::new(&args[1]);
    let file = fs::File::open(&file_name).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        
        let path_out = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} Comment: {comment}");
            }
        }
        if (file.name()).ends_with('/') {
            fs::create_dir_all(&path_out).unwrap();
            println!("File {i} extracted to \"{}\"", path_out.display());
        } else {
            if let Some(parent) = path_out.parent() {
                if !parent.exists() {
                    fs::create_dir_all(&parent).unwrap();
                }
            }
            let mut file_out = fs::File::create(&path_out).unwrap();
            io::copy(&mut file, &mut file_out).unwrap();
            println!("File {i} extracted to \"{}\" ({} bytes)", path_out.display(), file.size());
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&path_out, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}
