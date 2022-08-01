pub mod cli {
    use std::result::Result;
    use std::path::Path;
    use std::fmt::Error;
    use clap::Parser;

    /// A simple program for pixel diffing PNG images
    #[derive(Parser, Debug)]
    #[clap(author = "Author: Tim Bush", version, about, long_about = None)]
    pub struct Args {

        /// File path of the first image
        #[clap(short = 'f', long = "first-image", validator = validate_file_path)]
        pub img1_name: String,

        /// File path of the second image
        #[clap(short = 's', long = "second-image", validator = validate_file_path)]
        pub img2_name: String,

        /// File path to where the output file should be placed.
        #[clap(short = 'o', long = "output-dir")]
        pub output_dir: String,

        /// A value between 0 (inclusive) and 1 (exclusive). A lower value means the the algorithm will be more sensitive to changes in pixel colors between the two images.
        #[clap(short, long, default_value = ".1", validator = validate_threshold)]
        pub threshold: f64,
    }


    pub fn parse_args() -> Args {
        Args::parse()
    }

    fn validate_threshold(v: &str) -> Result<(), String> {
        let f = v.parse::<f64>().expect("Failed to convert 'threshold to a valid f64"); 
        if (0.0..1.0).contains(&f) { return Ok(()); }
        
        Err(String::from("The 'threshold' value is not between 0 and 1"))
    }

    fn validate_file_path(v: &str) -> Result<(), String> {
        let path = Path::new(v);
        let extension = path.extension();
        if extension.unwrap() != "png" {
            panic!("Images must have an extension of '.png'");
        }
        Ok(())
    }


}