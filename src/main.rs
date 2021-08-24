#[macro_use] extern crate rocket;

use rocket::{Data, response::Debug, tokio, data::ToByteUnit};
use std::{io::Error, io::Write, env, fs::File, fs};

#[get("/")]
fn index() -> &'static str {
    "Web interface in development, not yet implemented, please use the command line client"
}

#[post("/post/<hostname>/<user>/passwd", format = "plain", data = "<data>")]
async fn passwd(data: Data<'_>, hostname: String, user: String) -> std::io::Result<()> {
    let passfile = data.open(512.kibibytes())
                        .stream_to(tokio::io::stdout())
                        .await?;
    println!("{}", passfile);
    let path = format!("{}/{}/passwd", hostname, user);
    let mkpath = format!("{}/{}", hostname, user);
    fs::create_dir_all(mkpath)?;
    let mut output = File::create(path)?;
    write!(output, "{}", passfile);
    
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, passwd])
}
