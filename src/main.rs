use clap::Parser;
use image::open;
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
    #[arg(long, default_value_t = 10, help = "max height of smear")]
    s_height: u32,
    #[arg(long, default_value_t = 10, help = "max width of smear")]
    s_width: u32,
    #[arg(
        long,
        default_value_t = false,
        help = "dont allow blending calculations to overflow"
    )]
    no_overflow: bool,
}

fn main() {
    let args = Args::parse();

    if args.chance < 0. || args.chance > 1. {
        eprintln!("please provide a chance value between 0.0 and 1.0");
        return;
    }

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
            let smear_len = args.s_height / ((rng.random::<u32>() % 40) + 2);
            for newy in y - smear_len..(y + smear_len).min(height - 2) {
                let s_width = args.s_width / ((rng.random::<u32>() % 40) + 2);
                for newx in x - s_width..(x + s_width).min(width - 2) {
                    let pix = in_file.get_pixel_mut(newx, newy);
                    if args.no_overflow {
                        for i in 0..3 {
                            pix.0[i] = ((p.0[i] as u16 + pix.0[i] as u16) / 2) as u8;
                        }
                    } else {
                        for i in 0..3 {
                            pix.0[i] = (p.0[i] + pix.0[i]) / 2;
                        }
                    }
                }
            }
        }
    }

    in_file.save(args.output_file).expect("failed to save file");
}
