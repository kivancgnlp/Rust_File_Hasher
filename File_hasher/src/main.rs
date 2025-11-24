use std::io::Write;

mod hash_fonksiyonlari;


fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {

    let path = std::path::Path::new(".");

    let files  = path.read_dir()?;

    let mut report_file = std::fs::File::create("file_hashes.txt")?;


    let mut dir_entry;
    let mut read_buffer;
    let mut hash;
    //let mut file_name;
    for file in files {
        //println!("{:?}", file);
        dir_entry = file?;
        if dir_entry.path().is_dir() {
            println!("{} is a directory", dir_entry.path().display());
            continue;
        }
        read_buffer = std::fs::read(dir_entry.path())?;
        hash = hash_fonksiyonlari::hash_bytes(&read_buffer);
        //println!("file hash of {} is {}", dir_entry.path().file_name().unwrap().to_str().unwrap(), hash);

        if let Some(file_name) = dir_entry.path().file_name(){
            if let Some(file_name) = file_name.to_str() {

                let mut formatted_string = format!("{:<20} : {}", file_name, hash);
                println!("{}", formatted_string);
                
                formatted_string.push('\n');

                report_file.write_all(formatted_string.as_bytes())?;
            }
        }


    }

    Ok(())
}
