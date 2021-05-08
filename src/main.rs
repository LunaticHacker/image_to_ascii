use std::env;
use image::GenericImageView;
use image::imageops;

fn main() {
    let mut scale=1u32;
    let args: Vec<String> = env::args().collect();
    if args.len() ==1 {println!("Please provide atleast one argument"); return}
    else if args.len() ==3 
    { scale = args[2].parse::<u32>().unwrap()}
    
    let path = &args[1];
    let mut img = image::open(path).unwrap();
    let w = img.dimensions().0/scale;
    let h = img.dimensions().1/scale;
    let pixel_ascii_map = "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    img =img.resize( w,h,imageops::FilterType::Lanczos3);
    for y in 0..h-1 {
        for x in 0..w-1 {
            let pixel = img.get_pixel(x,y);
            let bright:i32 = (pixel[0] as i32+pixel[1] as i32+pixel[2] as i32)/3;
            let brightindex = (bright * pixel_ascii_map.len() as i32 -1)/255;
            print!("{}",pixel_ascii_map.as_bytes()[brightindex  as usize] as char);
        }
        print!("\n")
    }
}