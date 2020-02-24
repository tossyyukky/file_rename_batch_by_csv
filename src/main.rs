extern crate rust;

// use self::rust::*;
// use std::error::Error;
use std::fs;
use std::io;
use std::process;

fn read() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        let origin_file = format!("ranking_image/{}.jpg", &record[0]);
        let category_id = &record[1];
        println!("file_name: {}", &origin_file);
        println!("category_id: {}", &category_id);
        // let record = result?;
        // println!("{:?}", record);
        let image_path: String = format!("ranking_image/{}.jpg", category_id);

        match fs::rename(origin_file, image_path) {
            Ok(()) => {
                println!("ok");
            }
            Err(err) => {
                println!("{}", err);
            }
        }

        // let foo: SocketAddr = String::from_utf8_lossy(&fs::read(image_path)?).parse()?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    // println!("Hello, world!");
    if let Err(err) = read() {
        println!("error running example: {}", err);
        process::exit(1);
    }
    Ok(())
}
