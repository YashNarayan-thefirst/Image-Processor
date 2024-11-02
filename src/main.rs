use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
/*
Done:
Parts 1-10

To do:
Extra credit

*/

#[derive(Clone)]

struct FileHeader {
    id_length: u8,
    color_map_type: u8,
    image_type_code: u8,
    color_map_origin: Vec<u8>,
    color_map_length: Vec<u8>,
    color_map_depth: u8,
    x_origin: Vec<u8>,
    y_origin: Vec<u8>,
    width: Vec<u8>,
    height: Vec<u8>,
    bits_per_pixel: u8,
    image_descriptor:u8
}

impl FileHeader {
    fn init(width: u32, height: u32) -> FileHeader {
        let id_length: u8 = 0;
        let color_map_type: u8 = 0;
        let image_type_code: u8 = 2;
        let color_map_origin: Vec<u8> = vec![0, 0, 0, 0];
        let color_map_length: Vec<u8> = vec![0, 0, 0, 0];
        let color_map_depth: u8 = 0;
        let x_origin: Vec<u8> = vec![0, 0, 0, 0];
        let y_origin: Vec<u8> = vec![0, 0, 0, 0];
        let width: Vec<u8> = vec![(width >> 24) as u8, (width >> 16) as u8, (width >> 8) as u8, width as u8];
        let height: Vec<u8> = vec![(height >> 24) as u8, (height >> 16) as u8, (height >> 8) as u8, height as u8];
        let bits_per_pixel: u8 = 24;
        let image_descriptor:u8 = 0;
        FileHeader { id_length, color_map_type, image_type_code, color_map_origin, color_map_length, color_map_depth, x_origin, y_origin, width, height, bits_per_pixel, image_descriptor }
    }
}

#[derive(Clone, Copy)]
struct Pixel{
    r: u8,
    g: u8,
    b: u8
}

#[derive(Clone)]
struct RgbImage{
    header: FileHeader,
    pixel_data: Vec<Pixel>
}

impl RgbImage {
    fn new(width: u32, height: u32) -> RgbImage {
        let header = FileHeader::init(width, height);
        let pixel_count = (width * height) as usize;
        let pixel_data = vec![Pixel { r: 0, g: 0, b: 0 }; pixel_count];
        RgbImage { header, pixel_data }
    }
    fn save(&self, path: &str) -> io::Result<()> {
        let raw_data = self.generate_raw_data();
        let mut file = File::create(path)?;
        file.write_all(&raw_data)?;
        Ok(())
    }

    fn put_pixel(&mut self, x: usize, y: usize, p: Pixel) {
        let width = u16::from_le_bytes([self.header.width[2], self.header.width[3]]) as usize;
        let index = y * width + x;
        if index < self.pixel_data.len() {
            self.pixel_data[index] = p;
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        let width = u16::from_le_bytes([self.header.width[2], self.header.width[3]]) as usize;
        let index = y * width + x;
        if index < self.pixel_data.len() {
            self.pixel_data[index]
        } else {
            Pixel { r: 0, g: 0, b: 0 } 
        }
    }
    fn generate_raw_data(&self) -> Vec<u8> {
        let mut raw_data = vec![];

        raw_data.push(self.header.id_length);
        raw_data.push(self.header.color_map_type);
        raw_data.push(self.header.image_type_code);
        raw_data.extend_from_slice(&self.header.color_map_origin);
        raw_data.extend_from_slice(&self.header.color_map_length);
        raw_data.push(self.header.color_map_depth);
        raw_data.extend_from_slice(&self.header.x_origin);
        raw_data.extend_from_slice(&self.header.y_origin);
        raw_data.extend_from_slice(&self.header.width);
        raw_data.extend_from_slice(&self.header.height);
        raw_data.push(self.header.bits_per_pixel);
        raw_data.push(self.header.image_descriptor);

        for pixel in &self.pixel_data {
            raw_data.push(pixel.r);
            raw_data.push(pixel.g);
            raw_data.push(pixel.b);
        }

        raw_data
    }
    fn from_file(path: &Path) -> io::Result<RgbImage> {
        let mut file = File::open(path)?;
        let mut header = [0u8; 18];
        file.read_exact(&mut header)?;

        let width = u16::from_le_bytes([header[12], header[13]]) as u32;
        let height = u16::from_le_bytes([header[14], header[15]]) as u32;
        let bits_per_pixel = header[16];

        let pixel_count = (width * height) as usize;
        let mut pixel_data = Vec::with_capacity(pixel_count);

        for _ in 0..pixel_count {
            let mut pixel_bytes = [0u8; 3];
            file.read_exact(&mut pixel_bytes)?;
            pixel_data.push(Pixel {
                r: pixel_bytes[0],
                g: pixel_bytes[1],
                b: pixel_bytes[2],
            });
        }

        Ok(RgbImage {
            header: FileHeader::init(width, height),
            pixel_data,
        })
    }
}
fn write_file(filename: &str, image: &RgbImage) -> io::Result<()> {
    let nm = format!("output//{}.tga",filename);
    let path = Path::new(&nm);
    let mut file = File::create(path)?;
    file.write_all(&image.generate_raw_data())?;
    Ok(())
}


fn read_file(path: &str) -> io::Result<RgbImage> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let header = FileHeader {
        id_length: buffer[0],
        color_map_type: buffer[1],
        image_type_code: buffer[2],
        color_map_origin: buffer[3..7].to_vec(),
        color_map_length: buffer[7..11].to_vec(),
        color_map_depth: buffer[11],
        x_origin: buffer[12..16].to_vec(),
        y_origin: buffer[16..20].to_vec(),
        width: buffer[20..24].to_vec(),
        height: buffer[24..28].to_vec(),
        bits_per_pixel: buffer[28],
        image_descriptor: buffer[29],
    };

    let pixel_data_start = 30;
    let width = u16::from_le_bytes([header.width[2], header.width[3]]) as usize;
    let height = u16::from_le_bytes([header.height[2], header.height[3]]) as usize;
    let mut pixel_data = Vec::with_capacity(width * height);

    for i in (pixel_data_start..buffer.len()).step_by(3) {
        let pixel = Pixel {
            r: buffer[i],
            g: buffer[i + 1],
            b: buffer[i + 2],
        };
        pixel_data.push(pixel);
    }

    Ok(RgbImage { header, pixel_data })
}


fn multiply(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    for i in 0..top.pixel_data.len() {
        let t_pixel = top.pixel_data[i];
        let b_pixel = bottom.pixel_data[i];
        let r = (t_pixel.r as f32 * b_pixel.r as f32 / 255.0 +0.5) as u8;
        let g = (t_pixel.g as f32 * b_pixel.g as f32 / 255.0 +0.5) as u8;
        let b = (t_pixel.b as f32 * b_pixel.b as f32 / 255.0 +0.5) as u8;
        result.pixel_data[i] = Pixel { r, g, b };
    }
    result
}

fn subtract(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    for i in 0..top.pixel_data.len() {
        let t_pixel = top.pixel_data[i];
        let b_pixel = bottom.pixel_data[i];
        let r = (b_pixel.r - t_pixel.r).max(0) as u8;
        let g = (b_pixel.g - t_pixel.g).max(0) as u8;
        let b = (b_pixel.b - t_pixel.b).max(0) as u8;
        result.pixel_data[i] = Pixel { r, g, b };
    }
    result
}

fn add(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    for i in 0..top.pixel_data.len() {
        let t_pixel = top.pixel_data[i];
        let b_pixel = bottom.pixel_data[i];
        let r = (b_pixel.r + t_pixel.r).min(255) as u8;
        let g = (b_pixel.g + t_pixel.g).min(255) as u8;
        let b = (b_pixel.b + t_pixel.b).min(255) as u8;
        result.pixel_data[i] = Pixel { r, g, b };
    }
    result
}

fn overlay(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    for t_pixel in &top.pixel_data {
        let b_pixel = bottom.get_pixel(t_pixel.r as usize, t_pixel.g as usize);
        let r = overlay_channel(t_pixel.r, b_pixel.r);
        let g = overlay_channel(t_pixel.g, b_pixel.g);
        let b = overlay_channel(t_pixel.b, b_pixel.b);
        let pixel = Pixel { r, g, b };
        result.put_pixel(t_pixel.r as usize, t_pixel.g as usize, pixel);
    }
    result
}

fn overlay_channel(a: u8, b: u8) -> u8 {
    let a = a as f32 / 255.0;
    let b = b as f32 / 255.0;

    let c = if b <= 0.5 {
        2.0 * a * b
    } else {
        1.0 - 2.0 * (1.0 - a) * (1.0 - b)
    };

    (c * 255.0).round() as u8
}

fn screen(top: &RgbImage,bottom: &RgbImage)-> RgbImage{
    let mut result = top.clone();
    for t_pixel in &top.pixel_data {
        let b_pixel = bottom.get_pixel(t_pixel.r as usize, t_pixel.g as usize);
        let r = screen_channel(t_pixel.r, b_pixel.r);
        let g = screen_channel(t_pixel.g, b_pixel.g);
        let b = screen_channel(t_pixel.b, b_pixel.b);
        let pixel = Pixel { r, g, b };
        result.put_pixel(t_pixel.r as usize, t_pixel.g as usize, pixel);
    }
    result
}
fn screen_channel(a: u8, b: u8) -> u8 {
    let a = a as f32 / 255.0;
    let b = b as f32 / 255.0;

    let c = 1.0 - (1.0 - a) * (1.0 - b);

    (c * 255.0+ 0.5).round() as u8
}

fn change_channel(image: &RgbImage,multiplier: f32,channel:u8, r_offest: u8,g_offset:u8,b_offset:u8) -> RgbImage{
    let mut result = image.clone();
    for pixel in &mut result.pixel_data {
        let r = ((pixel.r as f32 * multiplier + r_offest as f32) as u32).min(255) as u8;
        let g = ((pixel.g as f32 * multiplier + g_offset as f32) as u32).min(255) as u8;
        let b = ((pixel.b as f32 * multiplier + b_offset as f32) as u32).min(255) as u8;
        match channel {
            0 => pixel.r = r,
            1 => pixel.g = g,
            2 => pixel.b = b,
            _ => {}
        }
    }
    result
}

fn list_files(dir: &str) -> Result<HashMap<String, RgbImage>, Box<dyn std::error::Error>> {
    let path = Path::new(dir);
    let mut fmap = HashMap::new();

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_name = entry.file_name().into_string().unwrap_or_default();
            if let Some(ext) = Path::new(&file_name).extension() {
                if ext == "tga" {
                    let image = RgbImage::from_file(&entry.path())?;
                    fmap.insert(file_name, image);
                }
            }
        }
    }
    Ok(fmap)
}
fn get(name: &str) -> RgbImage {
    let path = format!("input/{}.tga", name);
    read_file(&path).expect(&format!("Failed to load image: {}", name))
}

fn main() -> io::Result<()>{
    
    let part1 = multiply(&get("layer1.tga"), &get("pattern1"));
    write_file("part1", &part1).expect("Failed to write part1");

    let part2 = subtract(&get("layer2"), &get("car"));
    write_file("part2", &part2).expect("Failed to write part2");

    let mut part3 = multiply(&get("layer1"), &get("pattern2"));
    part3 = screen(&get("text"), &part3);
    write_file("part3", &part3).expect("Failed to write part3");

    let mut part4 = multiply(&get("layer2"), &get("circles"));
    part4 = subtract(&get("pattern2"), &part4);
    write_file("part4", &part4).expect("Failed to write part4");

    let part5 = overlay(&get("layer1"), &get("pattern1"));
    write_file("part5", &part5).expect("Failed to write part5");

    let part6 = change_channel(&get("car"), 1.0, 0, 0, 200, 0);
    write_file("part6", &part6).expect("Failed to write part6");

    let mut part7 = change_channel(&get("car"), 4.0, 1, 0, 0, 0);
    part7 = change_channel(&part7, 0.0, 2, 0, 0, 0);
    write_file("part7", &part7).expect("Failed to write part7");

    let mut part8_r = change_channel(&get("car"), 0.0, 1, 0, 0, 0);
    part8_r = change_channel(&part8_r, 0.0, 2, 0, 0, 0);
    write_file("part8_r", &part8_r).expect("Failed to write part8_r");
    
    let mut part8_g = change_channel(&get("car"), 0.0, 0, 0, 0, 0);
    part8_g = change_channel(&part8_g, 0.0, 2, 0, 0, 0);
    write_file("part8_g", &part8_g).expect("Failed to write part8_g");
    
    let mut part8_b = change_channel(&get("car"), 0.0, 0, 0, 0, 0);
    part8_b = change_channel(&part8_b, 0.0, 1, 0, 0, 0);
    write_file("part8_b", &part8_b).expect("Failed to write part8_b");

    let part9 = add(&add(&get("layer_red"), &get("layer_blue")), &get("layer_green"));
    write_file("part9", &part9).expect("Failed to write part9");

    let part10 = &get("text2");
    write_file("part10", &part10).expect("Failed to write part10");
    Ok(())
}
