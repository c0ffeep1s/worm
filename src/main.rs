extern crate clap;

use clap::{App, Arg};

pub mod utils;

fn print_usage_and_exit() {
    println!("Make sure to specify args --infile and --outfile. Maybe try the .png extension.");
    std::process::exit(-1);
}

fn main() {
    let matches = App::new("worm")
        .version("1.0")
        .author("atd832")
        .about("command line tool that does a few basic image processing functions")
        .arg(
            Arg::with_name("infile")
                .long("infile")
                .short("i")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("outfile")
                .long("outfile")
                .short("o")
                .required(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("blur").long("blur").takes_value(true))
        .arg(
            Arg::with_name("brighten")
                .long("brighten")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("crop")
                .long("crop")
                // do i need all this?
                .takes_value(true)
                .number_of_values(4)
                .multiple(true),
        )
        .arg(Arg::with_name("rotate").long("rotate").takes_value(true))
        .arg(Arg::with_name("invert").long("invert"))
        .arg(Arg::with_name("grayscale").long("grayscale"))
        .get_matches();

    if matches.is_present("infile") && matches.is_present("outfile") {
        // prog spinner
        let sp = utils::spinner::Spinner::new("creating new image...".to_string());

        let infile_ = matches.value_of("infile").unwrap().to_string();
        let outfile_ = matches.value_of("outfile").unwrap().to_string();
        let mut imgproc = utils::image_processor::ImageProcessor {
            outfile: outfile_,
            imgobj: image::open(infile_).expect("Failed to open INFILE."),
        };

        if matches.is_present("blur") {
            let sigma_ = matches.value_of("blur").unwrap().parse::<f32>().unwrap();
            imgproc.blur(sigma_);
        }

        if matches.is_present("brighten") {
            let lux_ = matches
                .value_of("brighten")
                .unwrap()
                .parse::<i32>()
                .unwrap();

            imgproc.brighten(lux_);
        }

        if matches.is_present("crop") {
            // handle 0s. can't use 0s for image crop.
            let vals: Vec<&str> = matches.values_of("crop").unwrap().collect();
            if vals.contains(&"0") {
                println!("Could not crop.");
                println!("Crop cannot take 0 as any of its values.");
            } else {
                // let's do this more elegantly.
                let x_ = vals[0].parse::<u32>().unwrap();
                let y_ = vals[1].parse::<u32>().unwrap();
                let width_ = vals[2].parse::<u32>().unwrap();
                let height_ = vals[3].parse::<u32>().unwrap();

                imgproc.crop(x_, y_, width_, height_);
            }
        }

        if matches.is_present("rotate") {
            let degree_ = matches.value_of("rotate").unwrap().parse::<i32>().unwrap();
            imgproc.rotate(degree_);
        }

        if matches.is_present("invert") {
            imgproc.invert();
        }

        if matches.is_present("grayscale") {
            imgproc.grayscale();
        }

        imgproc.save_file();
        sp.stop();
        println!("\nNew image saved.");
    } else {
        print_usage_and_exit();
    }
}
