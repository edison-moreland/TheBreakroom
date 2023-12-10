// Originally from:
// https://github.com/Atakku/lpg

use std::fs::create_dir_all;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use image::{DynamicImage, ImageFormat, imageops::FilterType};

const POSTER_TEMPLATE: &[u8] = include_bytes!("../assets/poster_template.png");
// const PAINTING_TEMPLATE: &[u8] = include_bytes!("../assets/painting_template.png");

#[derive(Parser, Debug)]
struct Cli {
    kind: AssetKind,
    input_file: PathBuf,
    output_folder: PathBuf,
}

#[derive(ValueEnum, Debug, Clone)]
enum AssetKind {
    Poster,
    Painting,
    Tip,
}

fn main() {
    let cli = Cli::parse();

    let input_name = cli.input_file.file_name().expect("input file to have a file name");

    let input_image = image::open(&cli.input_file).expect("input file to load");

    let poster_output_dir = cli.output_folder.join("BepInEx/plugins/LethalPosters/posters");
    let tips_output_dir = cli.output_folder.join("BepInEx/plugins/LethalPosters/tips");
    create_dir_all(&poster_output_dir).expect("folder create please :(");
    create_dir_all(&tips_output_dir).expect("folder create please :(");

    match cli.kind {
        AssetKind::Poster => {

            let poster_template_image = image::load_from_memory_with_format(POSTER_TEMPLATE, ImageFormat::Png).expect("Poster template to load");

            let poster_atlas = generate_poster_atlas(
                &poster_template_image,
                &input_image,
            );

            let poster_output_file = poster_output_dir.join(input_name);

            poster_atlas.save(poster_output_file).expect("poster image to save correctly");
        }
        AssetKind::Painting => {
            // let painting_template_image = image::load_from_memory_with_format(PAINTING_TEMPLATE, ImageFormat::Png).expect("Poster template to load");
            unimplemented!()
        }
        AssetKind::Tip => { unimplemented!() }
    }
}

const POSTER_OFFSETS: &[&[u32; 4]; 5] = &[
    &[0, 0, 341, 559],
    &[346, 0, 284, 559],
    &[641, 58, 274, 243],
    &[184, 620, 411, 364],
    &[632, 320, 372, 672],
];

fn generate_poster_atlas(template: &DynamicImage, poster: &DynamicImage) -> DynamicImage {
    // This just generates an atlas filled with one image
    // We could have multiple images?
    // I think only one atlas of images is randomly selected
    // So all the posters will be the same image

    let mut base = template.clone();
    for o in POSTER_OFFSETS {
        let p = poster.resize(o[2], o[3], FilterType::Lanczos3);
        image::imageops::overlay(&mut base, &p, (o[0] + o[2] - p.width()) as i64, o[1] as i64);
    }
    base
}
