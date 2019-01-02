extern crate image;
extern crate serenity;
extern crate num_complex;
extern crate reqwest;

use serenity::model::channel::Message;
use std::env;

command!(start(_context, msg) {
    build_msg(msg)
 });

fn build_msg(msg: &Message) {
    if let Err(why) = msg.channel_id.send_message(|m| m
        .embed(|e| e
            .title("Generating fractal")
            .description("one sec...")
            .footer(|f| f
                .text(&format!("Requested by {}", &msg.author.name) as &str)
                .icon_url(&msg.author.avatar_url().unwrap() as &str)
            )
        )
    ) {
        println!("Error sending message: {:?}", why);
    }

    let fractal_file_path = make_fractal();
    let file = reqwest::multipart::Form::new().file("fractal", &fractal_file_path);
    let _body = reqwest::Client::new()
        .post(&format!("https://discordapp.com/api/channels/{}/messages", msg.channel_id) as &str)
        .multipart(file.unwrap())
        .header(
            "Authorization",
            &format!("Bot {}", &env::var("DISCORD_BOT_TOKEN").unwrap()) as &str
        )
        .send().unwrap();
}

/// Make a fractal and return its path
fn make_fractal() -> String {
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;


            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let data = (*pixel as image::Rgb<u8>).data;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    let file_path = String::from("src/res/fractal.png");
    let _img = imgbuf.save(&file_path).unwrap();
    return file_path;
}