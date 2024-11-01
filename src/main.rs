use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
/*
Done:
Parts 1-10

To do:
Extra credit

*/


#[derive(Clone)]
struct Rgb([u8; 3]);

struct RgbImage {
    width: u16,
    height: u16,
    data: Vec<Rgb>,
}


struct FileHeader {
    id_length: u8,
    color_map_type: u8,
    image_type: u8,
    color_map_origin: u16,
    color_map_length: u16,
    color_map_depth: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bits_per_pixel: u8,
    image_descriptor: u8,
}

impl FileHeader {
    fn init(width: u16, height: u16) -> Self {
        Self {
            id_length: 0,
            color_map_type: 0,
            image_type: 2,
            color_map_origin: 0,
            color_map_length: 0,
            color_map_depth: 0,
            x_origin: 0,
            y_origin: 0,
            width,
            height,
            bits_per_pixel: 24,
            image_descriptor: 0,
        }
    }

    fn write_to_file(&self, file: &mut File) -> io::Result<()> {
        file.write_all(&[
            self.id_length,
            self.color_map_type,
            self.image_type,
            (self.color_map_origin & 0xFF) as u8,
            (self.color_map_origin >> 8) as u8,
            (self.color_map_length & 0xFF) as u8,
            (self.color_map_length >> 8) as u8,
            self.color_map_depth,
            (self.x_origin & 0xFF) as u8,
            (self.x_origin >> 8) as u8,
            (self.y_origin & 0xFF) as u8,
            (self.y_origin >> 8) as u8,
            (self.width & 0xFF) as u8,
            (self.width >> 8) as u8,
            (self.height & 0xFF) as u8,
            (self.height >> 8) as u8,
            self.bits_per_pixel,
            self.image_descriptor,
        ])
    }
}

impl RgbImage {
    fn new(width: u16, height: u16) -> Self {
        if width == 0 || height == 0 {
            panic!("Width and height must be greater than zero.");
        }
        let data = vec![Rgb([0, 0, 0]); (width as usize) * (height as usize)];
        Self { width, height, data }
    }

    fn get_pixel(&self, x: u16, y: u16) -> Option<&Rgb> {
        if x < self.width && y < self.height {
            Some(&self.data[(y as usize * self.width as usize) + x as usize])
        } else {
            None
        }
    }

    fn put_pixel(&mut self, x: u16, y: u16, pixel: Rgb) {
        if x < self.width && y < self.height {
            self.data[(y as usize * self.width as usize) + x as usize] = pixel;
        }
    }

    fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

fn write_file(name: &str, image: &RgbImage) -> io::Result<()> {
    let header = FileHeader::init(image.width, image.height);
    let mut file = File::create(format!("output/{}.tga", name))?;
    header.write_to_file(&mut file)?;

    for pixel in &image.data {
        file.write_all(&pixel.0)?;
    }
    Ok(())
}

fn multiply(image1: &RgbImage, image2: &RgbImage) -> RgbImage {
    let (width, height) = image1.dimensions();
    let mut multiplied = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let p1 = image1.get_pixel(x, y);
            let p2 = image2.get_pixel(x, y);

            let r = ((p1.0[0] as u32 * p2.0[0] as u32) / 255).min(255) as u8;
            let g = ((p1.0[1] as u32 * p2.0[1] as u32) / 255).min(255) as u8;
            let b = ((p1.0[2] as u32 * p2.0[2] as u32) / 255).min(255) as u8;

            multiplied.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    multiplied
}

fn add(image1: &RgbImage, image2: &RgbImage) -> RgbImage {
    let (width, height) = image1.dimensions();
    let mut added = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let p1 = image1.get_pixel(x, y);
            let p2 = image2.get_pixel(x, y);

            let r = (p1.0[0] as u16 + p2.0[0] as u16).min(255) as u8;
            let g = (p1.0[1] as u16 + p2.0[1] as u16).min(255) as u8;
            let b = (p1.0[2] as u16 + p2.0[2] as u16).min(255) as u8;

            added.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    added
}

fn subtract(image1: &RgbImage, image2: &RgbImage) -> RgbImage {
    let (width, height) = image1.dimensions();
    let mut subtracted = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let p1 = image1.get_pixel(x, y);
            let p2 = image2.get_pixel(x, y);

            let r = (((p1.0[0] as f32 + p2.0[0] as f32)/255.0 - 1.0)*255.0).max(0.0) as u8;
            let g = (((p1.0[1] as f32 + p2.0[1] as f32)/255.0 - 1.0)*255.0).max(0.0) as u8;
            let b = (((p1.0[2] as f32 + p2.0[] as f32)/255.0 - 1.0)*255.0).max(0.0) as u8;
            subtracted.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    subtracted
}

fn overlay(image1: &RgbImage, image2: &RgbImage) -> RgbImage {
    let (width, height) = image1.dimensions();
    let mut result = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let p1 = image1.get_pixel(x, y);
            let p2 = image2.get_pixel(x, y);

            let r = if p1.0[0] < 128 {
                (2 * p1.0[0] as u16 * p2.0[0] as u16 / 255).min(255) as u8
            } else {
                (255 - 2 * (255 - p1.0[0] as u16) * (255 - p2.0[0] as u16) / 255).min(255) as u8
            };

            let g = if p1.0[1] < 128 {
                (2 * p1.0[1] as u16 * p2.0[1] as u16 / 255).min(255) as u8
            } else {
                (255 - 2 * (255 - p1.0[1] as u16) * (255 - p2.0[1] as u16) / 255).min(255) as u8
            };

            let b = if p1.0[2] < 128 {
                (2 * p1.0[2] as u16 * p2.0[2] as u16 / 255).min(255) as u8
            } else {
                (255 - 2 * (255 - p1.0[2] as u16) * (255 - p2.0[2] as u16) / 255).min(255) as u8
            };

            result.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    result
}

fn change_channel(image: &RgbImage, channel: usize, factor: f32, r_offset: u8, g_offset: u8, b_offset: u8) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut result = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let mut pixel = image.get_pixel(x, y).clone();
            pixel.0[channel] = ((pixel.0[channel] as f32 * factor).min(255.0)) as u8;

            pixel.0[0] = pixel.0[0].saturating_add(r_offset);
            pixel.0[1] = pixel.0[1].saturating_add(g_offset);
            pixel.0[2] = pixel.0[2].saturating_add(b_offset);

            result.put_pixel(x, y, pixel);
        }
    }
    result
}


fn load_image(path: &Path) -> io::Result<RgbImage> {
    let mut file = File::open(path)?;
    let mut header = [0u8; 18];
    file.read_exact(&mut header)?;

    let width = u16::from_le_bytes([header[12], header[13]]);
    let height = u16::from_le_bytes([header[14], header[15]]);
    let mut data = vec![0u8; (width as usize) * (height as usize) * 3];
    file.read_exact(&mut data)?;

    let pixels = data.chunks(3).map(|p| Rgb([p[0], p[1], p[2]])).collect();

    Ok(RgbImage { width, height, data: pixels })
}

fn get(name: &str) -> RgbImage {
    let x = format!("input\\{}.tga",name);
    let path = Path::new(&x);
    load_image(&path).expect(&format!("Failed to load image: {}", path.display()))
}

fn main() {
    let part1 = multiply(&get("layer1"), &get("pattern1"));
    write_file("part1", &part1).expect("Failed to write part1");

    let part2 = subtract(&get("layer2"), &get("car"));
    write_file("part2", &part2).expect("Failed to write part2");

    let mut part3 = multiply(&get("layer1"), &get("pattern2"));
    part3 = subtract(&get("text"), &part3);
    write_file("part3", &part3).expect("Failed to write part3");

    let mut part4 = multiply(&get("layer2"), &get("circles"));
    part4 = subtract(&get("pattern2"), &part4);
    write_file("part4", &part4).expect("Failed to write part4");

    let part5 = overlay(&get("layer1"), &get("pattern1"));
    write_file("part5", &part5).expect("Failed to write part5");

    let part6 = change_channel(&get("car"), 1, 1.0, 0, 200, 0);
    write_file("part6", &part6).expect("Failed to write part6");

    let mut part7 = change_channel(&get("car"), 0, 4.0, 0, 0, 0);
    part7 = change_channel(&part7, 1, 0.0, 0, 0, 0);
    write_file("part7", &part7).expect("Failed to write part7");

    let part8_r = change_channel(&get("car"), 0, 1.0, 0, 0, 0);
    write_file("part8_r", &part8_r).expect("Failed to write part8_r");
    
    let part8_g = change_channel(&get("car"), 1, 1.0, 0, 0, 0);
    write_file("part8_g", &part8_g).expect("Failed to write part8_g");
    
    let part8_b = change_channel(&get("car"), 2, 1.0, 0, 0, 0);
    write_file("part8_b", &part8_b).expect("Failed to write part8_b");

    let part9 = add(&add(&get("layer_red"), &get("layer_blue")), &get("layer_green"));
    write_file("part9", &part9).expect("Failed to write part9");

    let part10 = &get("text2");
    write_file("part10", &part10).expect("Failed to write part10");
}
