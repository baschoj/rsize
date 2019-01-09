fn main() {
    let matches = clap::App::new("rsize")
        .version("0.1")
        .author("Bruce Schojan <bruce@basjr.com>")
        .about("resizes input image to target width/height")
        .args_from_usage(
            "-i, --input  <FILE>  'input file'
             -w, --width  <WIDTH>  'resize width'
             -h, --height <HEIGHT> 'resize height'
             -o, --output [FILE] 'output filename'",
        )
        .get_matches();

    let input: Vec<&str> = matches.value_of("input").unwrap().split(".").collect();

    if input.len() != 2 {
        exit(1, Some("input file name must have an image extension"));
    }

    let filename = input[0];
    let ext = input[1];

    let image = match image::open(format!("{}.{}", filename, ext)) {
        Err(e) => match e {
            image::ImageError::IoError(e) => {
                exit(
                    1,
                    Some(&format!(
                        "unable to read input file, does file exist? {}",
                        e
                    )),
                );
                unreachable!()
            }
            _ => {
                exit(
                    1,
                    Some(&format!(
                        "could not parse image. make sure it is in a valid format. {}",
                        e
                    )),
                );
                unreachable!()
            }
        },
        Ok(i) => i,
    };

    let width = {
        if let Ok(width) = matches.value_of("width").unwrap().parse::<u32>() {
            width
        } else {
            exit(1, Some("width must be positive number"));
            unreachable!()
        }
    };
    let height = {
        if let Ok(height) = matches.value_of("height").unwrap().parse::<u32>() {
            height
        } else {
            exit(1, Some("height must be a positive number"));
            unreachable!()
        }
    };

    let resized_buff = image.thumbnail(width, height);

    let default_output = format!("{}_{}x{}.{}", filename, width, height, ext);
    match resized_buff.save(&std::path::Path::new(
        matches
            .value_of("output")
            .unwrap_or_else(|| &default_output),
    )) {
        Err(e) => exit(
            1,
            Some(&format!("could not save image: {}. {}", default_output, e)),
        ),
        Ok(()) => (),
    };
}

fn exit(code: i32, message: Option<&str>) {
    if let Some(message) = message {
        eprintln!("{}", message);
    }
    std::process::exit(code);
}
