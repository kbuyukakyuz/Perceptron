use image::*;
//This file merges weights files into a GIF
pub fn merge_ppm_into_gif(ppm_files: Vec<String>, gif_file: String){
    let mut images: Vec<image::DynamicImage> = Vec::new();
    for file in ppm_files{
        images.push(image::open(&file).unwrap());
    }
    let mut gif_encoder = image::gif::Encoder::new(
        &mut std::fs::File::create(&gif_file).unwrap(),
        images[0].width(),
        images[0].height(),
    );
    for image in images{
        gif_encoder.write_frame(&image).unwrap();
    }
}
pub fn merge_png_into_gif(png_files: Vec<String>, gif_file: String){
    let mut images: Vec<image::DynamicImage> = Vec::new();
    for file in png_files{
        images.push(image::open(&file).unwrap());
    }
    let mut gif_encoder = image::gif::Encoder::new(
        &mut std::fs::File::create(&gif_file).unwrap(),
        images[0].width(),
        images[0].height(),
    );
    for image in images{
        gif_encoder.write_frame(&image).unwrap();
    }
}
