use clap::Parser;
use image::{open};
use rand::Rng;

#[derive(Parser)]
struct Args {
    #[arg(help = "input file for smearing")]
    input_file: String,
    #[arg(
        short,
        long,
        default_value_t = String::from("output.png"), 
        help = "output file name"
        )]
    output_file: String,
    #[arg(short, long, default_value_t = 0.5, help = "chance of smear per pixel")]
    chance: f32,
    #[arg(long, default_value_t = 200.0, help = "max height of smear")]
    s_height: f32,
    #[arg(long, default_value_t = 1.0, help = "max width of smear")]
    s_width: f32,
    #[arg(long, default_value_t = false, help = "allow blending calculations to overflow")]
    allow_overflow: bool,
}

fn main() {
    let args = Args::parse();

    let mut in_file = open(args.input_file)
        .expect("could not open file")
        .to_rgb8();
    let mut rng = rand::rng();

    let (width, height) = in_file.dimensions();
    let clone_in_file = in_file.clone();
    let mut pixels: Vec<_> = clone_in_file.enumerate_pixels().collect();
    pixels.reverse();

    for (x, y, p) in pixels {
        if rng.random::<f32>() <= args.chance {
            let smear_len = (args.s_height * rng.random::<f32>()).ceil() as u32 / 2;
            for newy in y - smear_len..=y + smear_len {
                if newy >= height {
                    break;
                }
                let s_width = ((args.s_width - 1.) * rng.random::<f32>()).ceil() as u32;
                for newx in x - s_width..=x + s_width {
                    if newx >= width {
                        break;
                    }
                    let pix = in_file.get_pixel_mut(newx, newy);
                    if args.allow_overflow {
                        for (i, channel) in pix.0.into_iter().enumerate() {
                            pix.0[i] = (p.0[i] + channel) / 2;
                        }
                    } else {
                        for (i, channel) in pix.0.into_iter().enumerate() {
                            pix.0[i] = ((p.0[i] as u16 + channel as u16) / 2) as u8;
                        }
                    }
                }
            }
        }
    }

    in_file.save(args.output_file).expect("failed to save file");
}
