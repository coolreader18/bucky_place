pub fn convert_to_overlay<I: image::GenericImageView<Pixel = image::Rgba<u8>>>(
    canvas_dim: (u32, u32),
    images: impl IntoIterator<Item = (I, (u32, u32))>,
) -> image::RgbaImage {
    let (cw, ch) = canvas_dim;
    let mut overlay_img = image::RgbaImage::new(cw * 3, ch * 3);
    for (image, (lx, ly)) in images {
        let lx0 = lx * 3 + 1;
        let ly0 = ly * 3 + 1;
        let adjusted = |l0, len| (0..len).map(move |a| (a, l0 + (a * 3)));
        for (y, y2) in adjusted(ly0, image.height()) {
            for (x, x2) in adjusted(lx0, image.width()) {
                overlay_img.put_pixel(x2, y2, image.get_pixel(x, y));
            }
        }
    }
    overlay_img
}
