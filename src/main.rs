extern crate clap;

use clap::{App, Arg};

extern crate lab;
use std::error;

#[cfg(test)]
mod tests {
    use crate::valley::convert;
    use std::env;
    use std::path::PathBuf;

    fn get_full_path(prefix: &str, id: &str) -> String {
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source = PathBuf::from(root_dir);
        source.push("tests/fixtures");
        let filename = format!("{}{}.png", prefix, id);
        source.push(filename);
        source.to_string_lossy().to_string()
    }

    #[test]
    fn test00() {
        test("00")
    }

    #[test]
    fn test01() {
        test("01")
    }

    #[test]
    fn test02() {
        test("02")
    }

    #[test]
    fn test03() {
        test("03")
    }

    fn test(id: &str) {
        convert(
            &get_full_path("input", id),
            &get_full_path("output_gen", id),
        )
        .unwrap();

        let image1 = lodepng::decode32_file(&get_full_path("output", id)).unwrap();
        let image2 = lodepng::decode32_file(&get_full_path("output_gen", id)).unwrap();

        assert_eq!(image1.width, image2.width);
        assert_eq!(image1.height, image2.height);
        assert_eq!(image1.buffer, image2.buffer);
    }
}

mod valley;

fn main() -> std::result::Result<(), Box<dyn error::Error>> {
    let matches = App::new("MyApp")
        .about("Accepts a black-and-white image file (colored images will be binarized), interprets as a height map with holes of rectangular cross-section, and converts it into triangular cross-section.")
        .version("0.1.0")
        .author("hsjoihs <hs.ioling.hs@gmail.com>")
        .arg(
            Arg::with_name("output")
                .help("the output file to use")
                .short("o")
                .long("output")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("input")
                .help("the input file to use")
                .index(1)
                .required(true),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    valley::convert(input, output)?;
    Ok(())
}
