use rustronomy_fits as rsf;
use rustronomy_watershed as rws;
use rws::prelude::*;

const PATH: &str = "/net/vdesk/data2/wolters/FIRGGsims/";

fn main() {
    println!("Starting watershed transform of FIRGG simulations...");
    let root = std::path::Path::new(PATH)
        .canonicalize()
        .expect(&format!("could not canonicalize path \"{PATH:?}\""));
    let mut fits = rsf::Fits::open(&root.join("LS_00041_allPPVres1.5.fits"))
        .expect("could not open FITS file");
    println!("{fits:?}");
}