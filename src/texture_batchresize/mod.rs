use image::ImageFormat;
use imageproc::drawing::Canvas;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use yaml_rust::YamlLoader;

async fn _resize_img(im: &image::DynamicImage, fo_size: &(u32, u32), f_p: &PathBuf) {
    let fo = &mut File::create(f_p).unwrap();
    let im_r = im.thumbnail(fo_size.0, fo_size.1);
    let _ = im_r.write_to(fo, ImageFormat::Png);
}

pub async fn resize(
    to_sizes: &Vec<(u32, u32)>,
    to_files: &Vec<String>,
    img_path: &str,
    out_path: Option<&str>,
) {
    let im = image::open(&img_path).unwrap();
    println!(
        "load texture from {:}, dimensions={:?} color={:?}",
        img_path,
        im.dimensions(),
        im.color()
    );
    let cur_dir = std::env::current_dir().unwrap();
    let cur_path = cur_dir.to_str().unwrap();
    let mut idx = 0;
    let output_path = if out_path.is_some() {
        out_path.unwrap()
    } else {
        cur_path
    };
    for o in to_sizes {
        let f = to_files.get(idx).unwrap().as_str();
        let f_p = &Path::new(&output_path).join(f);
        let fo_size = (o.0, o.1);
        let _ = fs::create_dir_all(f_p.parent().unwrap());
        println!(
            "[texture-batchresize]output file:{} <size->w={}, h={}>",
            f_p.as_path().to_str().unwrap(),
            fo_size.0,
            fo_size.1
        );

        let f = _resize_img(&im, &fo_size, &f_p);
        futures::join!(f);
        idx += 1;
    }
}

pub async fn resize_by_yml(path: &str) {
    let cf = fs::read_to_string(path).unwrap_or_else(|e| {
        println!("Load config failed!\n{}", e.to_string());
        std::process::exit(2)
    });
    let out = YamlLoader::load_from_str(cf.as_str()).unwrap_or_else(|e| {
        println!("Load config failed!\n{}", e.to_string());
        std::process::exit(2)
    });
    let c = &out[0].to_owned();

    // The color method returns the image's ColorType
    let o_s = c["vec_size"]
        .as_vec()
        .map(|f| {
            // let mut
            let mut r: Vec<(u32, u32)> = Vec::new();
            for o in f {
                let fo_size = (o[0].as_i64().unwrap() as u32, o[1].as_i64().unwrap() as u32);
                r.push(fo_size);
            }
            r
        })
        .unwrap();

    let o_f = c["vec_f"]
        .as_vec()
        .map(|f| {
            // let mut
            let mut r: Vec<String> = Vec::new();
            for o in f {
                let to_f = o.as_str().unwrap().to_string();
                r.push(to_f);
            }
            r
        })
        .unwrap();

    let base_f = c["image_path"]
        .as_str()
        .expect("get config of <image_path> failed!");
    futures::join!(resize(&o_s, &o_f, base_f, c["out_path"].as_str()));
}
