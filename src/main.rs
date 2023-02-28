extern crate ndarray as nd;
extern crate rustronomy_fits as rsf;
extern crate csv;
extern crate rustronomy_watershed as rws;

use ndarray::parallel::prelude::*;
use rws::prelude::*;

const ROOT_PATH: &str = "/net/vdesk/data2/wolters/FIRGGsims/";

fn main() {
    println!("Starting watershed transform of FIRGG simulations...");

    //Get le data
    let root = std::path::Path::new(ROOT_PATH)
        .canonicalize()
        .expect(&format!("could not canonicalize path \"{ROOT_PATH:?}\""));

    watershed(
        &root.join("LS_00041_allPPVres1.5.fits"),
        &root.join("PPV/")
    );

    watershed(
        &root.join("LS_00041_dens.fits"),
        &root.join("PPP/")
    );
}

fn watershed(input_file: &std::path::Path, output_folder: &std::path::Path) {
    //Get le data
    let data = open_cube(input_file).unwrap();

    //Run pre-processor
    let ws = rws::TransformBuilder::new_merging().build().unwrap();
    let cube = ws.pre_processor(data.view());
    drop(data); //dealloc old cube

    //Check that output folder exists
    if !output_folder.exists() {
        std::fs::create_dir(output_folder).expect(&format!(
            "could not create output folder {output_folder:?}"
        ))
    }

    //Do watershed
    cube.axis_iter(nd::Axis(2)).into_par_iter().enumerate().for_each(|(idx, slice)| {
        println!("Started transform on slice {idx}...");
        let mins = &ws.find_local_minima(slice.view());
        let lakes = &ws.transform_to_list(slice, mins);
        println!("Finished transform on slice {idx}...");

        //Save results to file
        let path = &output_folder.join(&format!("depth_{idx}.csv"));
        save_output(path, lakes);
    });
}

fn open_cube(path: &std::path::Path) -> Option<nd::Array3<f64>> {
    let mut fits = rsf::Fits::open(path).expect("could not open FITS file");
    let (header, data) = fits.remove_hdu(0).unwrap().to_parts();
    println!("{header}");

    if let Some(rsf::Extension::Image(img)) = data {
        Some(img.as_owned_f64_array().unwrap().into_dimensionality().unwrap())
    } else { None }
}

fn save_output(path: &std::path::Path, data: &[(u8, Vec<usize>)]) {
    let mut writer = csv::WriterBuilder::new()
        .from_path(path)
        .expect(&format!("could not create CSV writer at {path:?}"));
    data.into_iter().for_each(|(_water_level, lake_sizes)| {
      writer.write_record(lake_sizes.iter().map(|&x| format!("{x}"))).unwrap();
    });
    writer.flush().unwrap();
    println!("Saved output to {path:?}");
}