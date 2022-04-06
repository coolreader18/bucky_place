use clap::*;
use image::codecs::png;
use itertools::Itertools;
use std::{fs, io};

fn main() -> image::ImageResult<()> {
    let mut app = Command::new("placegen")
        .arg(
            Arg::new("out")
                .required(true)
                .takes_value(true)
                .value_name("OUT"),
        )
        .arg(
            Arg::new("img")
                .long("template")
                .required(true)
                .multiple_occurrences(true)
                .number_of_values(3)
                .value_names(&["IMG", "X", "Y"]),
        );
    let m = app.get_matches_mut();
    let out = m.value_of_os("out").unwrap();
    let images = m.values_of_os("img").unwrap().tuples().map(|(path, x, y)| {
        let image = image::open(path).unwrap_or_else(|e| {
            eprintln!("{:#?}", e);
            std::process::exit(1);
        });
        let image = image.into_rgba8();
        let mut p = |s: &std::ffi::OsStr| {
            s.to_str()
                .unwrap()
                .parse()
                .unwrap_or_else(|e| app.error(ErrorKind::ValueValidation, e).exit())
        };
        (image, (p(x), p(y)))
    });
    let img = placegen::convert_to_overlay((2000, 2000), images);
    write_png_with_compression(
        &img,
        png::CompressionType::Fast,
        png::FilterType::NoFilter,
        out.as_ref(),
    )?;
    Ok(())
}

fn write_png_with_compression(
    img: &image::RgbaImage,
    compression: png::CompressionType,
    filter: png::FilterType,
    path: &std::path::Path,
) -> image::ImageResult<()> {
    match image::ImageFormat::from_path(path)?.into() {
        image::ImageOutputFormat::Png => {
            use image::ImageEncoder;
            png::PngEncoder::new_with_quality(
                io::BufWriter::new(fs::File::create(path)?),
                compression,
                filter,
            )
            .write_image(img, img.width(), img.height(), image::ColorType::Rgba8)
        }
        image::ImageOutputFormat::Unsupported(msg) => Err(image::ImageError::Unsupported(
            image::error::UnsupportedError::from_format_and_kind(
                image::error::ImageFormatHint::Unknown,
                image::error::UnsupportedErrorKind::Format(image::error::ImageFormatHint::Name(
                    msg,
                )),
            ),
        )),
        _ => unreachable!(),
    }
}
