pub mod image;

use image::generate_img;
use worker::*;

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let img = generate_img();
    let response = Response::from_bytes(img)?;
    let mut headers = Headers::new();
    headers.set("Content-Type", "image/png")?;
    Ok(response.with_headers(headers))
}
