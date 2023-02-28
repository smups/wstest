extern crate ndarray as nd;
use ndarray::parallel::prelude::*;

use rustronomy_fits as rsf;

use rustronomy_watershed as rws;
use rws::prelude::*;

const PATH: &str = "/net/vdesk/data2/wolters/FIRGGsims/";

fn main() {
    println!("Starting watershed transform of FIRGG simulations...");

    //Get le data
    let root = std::path::Path::new(PATH)
        .canonicalize()
        .expect(&format!("could not canonicalize path \"{PATH:?}\""));
    let mut fits = rsf::Fits::open(&root.join("LS_00041_allPPVres1.5.fits"))
        .expect("could not open FITS file");
    let (header, data) = fits.remove_hdu(0).unwrap().to_parts();
    println!("{header}");

    let data = if let Some(rsf::Extension::Image(img)) = data {
        img
            .as_owned_f64_array()
            .unwrap()
            .into_dimensionality::<nd::Ix3>()
            .unwrap()
    } else {
        panic!("ahhhh")
    };

    //Do watershed
    data.axis_iter(nd::Axis(3));
}