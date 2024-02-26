use clap::Parser;
use google_image_dl::Client;
use std::path::Path;

#[derive(clap::Parser, Debug)]
struct Opts {
    #[clap(short, long)]
    query: String,

    #[clap(short, long)]
    api_key: String,

    #[clap(short, long)]
    engine_id: String,

    #[clap(short, long, default_value = "images")]
    ouput: String,

    #[clap(short, long, default_value = "500")]
    target: u64,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();

    let target = opts.target;

    let client = Client::new(opts.api_key, opts.engine_id);

    let mut offset = 0;

    let mut n_images = 0;

    loop {
        let response = client
            .search(&opts.query, offset)
            .await
            .expect("Failed to search");

        for image in response.items.iter() {
            println!("{:?}", image);
            if let Err(err) = download_image(&image.link, Path::new(&opts.ouput)).await {
                eprintln!("Failed to download image: {}", err);
            } else {
                n_images += 1;
            }

            if n_images >= target {
                break;
            }
        }

        if n_images >= target {
            break;
        }

        offset += response.items.len() as u64;
    }

    println!("Downloaded {} images", n_images);
}

async fn download_image(url: &str, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(format!("Failed to download image: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;

    // file extension
    let mut ext = url.split('.').last().unwrap_or("jpg");

    // srip query string
    if let Some(index) = ext.find('?') {
        ext = &ext[..index];
    }

    let uuid = uuid::Uuid::new_v4();

    // make output directory
    std::fs::create_dir_all(output)?;

    let output = output.join(format!("{}.{}", uuid, ext));

    let mut out = std::fs::File::create(output)?;
    let mut reader = std::io::Cursor::new(bytes);
    std::io::copy(&mut reader, &mut out)?;
    Ok(())
}
