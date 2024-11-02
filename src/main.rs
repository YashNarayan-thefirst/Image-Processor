#[derive(Clone)]
struct Header {
    id_length: u8, color_map_type: u8, image_type_code: u8,
    color_map_origin: [u8; 2], color_map_length: [u8; 2],
    color_map_depth: u8, x_origin: [u8; 2], y_origin: [u8; 2],
    width: [u8; 2], height: [u8; 2], bits_per_pixel: u8, image_descriptor: u8,
}

#[derive(Clone, Copy)]
struct Pixel { r: u8, g: u8, b: u8 }
#[derive(Clone)]
struct RgbImage { header: Header, pixel_data: Vec<Pixel> }

fn read_file(filepath: &str) -> Vec<u8> { std::fs::read(filepath).expect("File read error") }

fn create_image(image_bytes: &[u8], output_path: &str) { std::fs::write(output_path, image_bytes).expect("Write error") }

fn get_bytes(rgb_image: RgbImage) -> Vec<u8> {
    let mut bytes = vec![
        rgb_image.header.id_length, rgb_image.header.color_map_type, rgb_image.header.image_type_code,
        rgb_image.header.color_map_origin[0], rgb_image.header.color_map_origin[1],
        rgb_image.header.color_map_length[0], rgb_image.header.color_map_length[1],
        rgb_image.header.color_map_depth, rgb_image.header.x_origin[0], rgb_image.header.x_origin[1],
        rgb_image.header.y_origin[0], rgb_image.header.y_origin[1], rgb_image.header.width[0], 
        rgb_image.header.width[1], rgb_image.header.height[0], rgb_image.header.height[1], 
        rgb_image.header.bits_per_pixel, rgb_image.header.image_descriptor
    ];
    rgb_image.pixel_data.iter().for_each(|p| bytes.extend([p.r, p.g, p.b]));
    bytes
}

fn get_rgb_image_data(input_path: &str) -> RgbImage {
    let data = read_file(input_path);
    let header = Header {
        id_length: data[0], color_map_type: data[1], image_type_code: data[2],
        color_map_origin: [data[3], data[4]], color_map_length: [data[5], data[6]],
        color_map_depth: data[7], x_origin: [data[8], data[9]], y_origin: [data[10], data[11]],
        width: [data[12], data[13]], height: [data[14], data[15]],
        bits_per_pixel: data[16], image_descriptor: data[17],
    };
    let pixel_data = data[18..].chunks(3).map(|c| Pixel { r: c[0], g: c[1], b: c[2] }).collect();
    RgbImage { header, pixel_data }
}

fn add(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    for i in 0..top.pixel_data.len() {
        let t_pixel = top.pixel_data[i];
        let b_pixel = bottom.pixel_data[i];
        result.pixel_data[i] = Pixel {
            r: (t_pixel.r+b_pixel.r).min(255),
            g: (t_pixel.g+b_pixel.g).min(255),
            b: (t_pixel.b+b_pixel.b).min(255),
        };
    }
    result
}

fn multiply(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    for i in 0..top.pixel_data.len() {
        let t_pixel = top.pixel_data[i];
        let b_pixel = bottom.pixel_data[i];
        result.pixel_data[i] = Pixel {
            r: (t_pixel.r as u16 * b_pixel.r as u16 / 255) as u8,
            g: (t_pixel.g as u16 * b_pixel.g as u16 / 255) as u8,
            b: (t_pixel.b as u16 * b_pixel.b as u16 / 255) as u8,
        };
    }
    result
}

fn subtract(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    for i in 0..top.pixel_data.len() {
        let t_pixel = top.pixel_data[i];
        let b_pixel = bottom.pixel_data[i];
        result.pixel_data[i] = Pixel {
            r: b_pixel.r.saturating_sub(t_pixel.r),
            g: b_pixel.g.saturating_sub(t_pixel.g),
            b: b_pixel.b.saturating_sub(t_pixel.b),
        };
    }
    result
}

fn screen(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    for i in 0..top.pixel_data.len() {
        let t_pixel = top.pixel_data[i];
        let b_pixel = bottom.pixel_data[i];
        result.pixel_data[i] = Pixel {
            r: 255 - ((255 - t_pixel.r) as u16 * (255 - b_pixel.r) as u16 / 255) as u8,
            g: 255 - ((255 - t_pixel.g) as u16 * (255 - b_pixel.g) as u16 / 255) as u8,
            b: 255 - ((255 - t_pixel.b) as u16 * (255 - b_pixel.b) as u16 / 255) as u8,
        };
    }
    result
}

fn overlay(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    for i in 0..top.pixel_data.len() {
        let t_pixel = top.pixel_data[i];
        let b_pixel = bottom.pixel_data[i];
        result.pixel_data[i] = Pixel {
            r: if b_pixel.r < 128 {
                (t_pixel.r as u16 * b_pixel.r as u16 / 128) as u8
            } else {
                255 - (2 * (255 - t_pixel.r) as u16 * (255 - b_pixel.r) as u16 / 255) as u8
            },
            g: if b_pixel.g < 128 {
                (t_pixel.g as u16 * b_pixel.g as u16 / 128) as u8
            } else {
                255 - (2 * (255 - t_pixel.g) as u16 * (255 - b_pixel.g) as u16 / 255) as u8
            },
            b: if b_pixel.b < 128 {
                (t_pixel.b as u16 * b_pixel.b as u16 / 128) as u8
            } else {
                255 - (2 * (255 - t_pixel.b) as u16 * (255 - b_pixel.b) as u16 / 255) as u8
            },
        };
    }
    result
}
fn change_channel(image: &RgbImage, multiplier: f32, channel: u8, r_offset: u8, g_offset: u8, b_offset: u8) -> RgbImage {
    let mut result = image.clone();
    for pixel in &mut result.pixel_data {
        let r = ((pixel.r as f32 * multiplier + r_offset as f32) as u32).min(255) as u8;
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

fn test_image(output: &[u8], example: &[u8]) -> bool { output.iter().zip(example).all(|(o, e)| o == e) }

fn print_test(rgb_image: RgbImage, s: &str, no: u8) {
    let generated = get_bytes(rgb_image);
    let example = read_file(&format!("examples/EXAMPLE_{}.RgbImage", s));
    create_image(&generated, &format!("output/{}.RgbImage", s));
    println!("Task #{} Test: {}", no, test_image(&generated, &example));
}

fn main() {
    let part1 = multiply(&get_rgb_image_data("layer1.tga"), &get_rgb_image_data("pattern1"));
    create_image(&get_bytes(part1), "part1");

    let part2 = subtract(&get_rgb_image_data("layer2"), &get_rgb_image_data("car"));
    create_image(&get_bytes(part2), "part2");

    let mut part3 = multiply(&get_rgb_image_data("layer1"), &get_rgb_image_data("pattern2"));
    part3 = screen(&get_rgb_image_data("text"), &part3);
    create_image(&get_bytes(part3), "part3");

    let mut part4 = multiply(&get_rgb_image_data("layer2"), &get_rgb_image_data("circles"));
    part4 = subtract(&get_rgb_image_data("pattern2"), &part4);
    create_image(&get_bytes(part4), "part4");

    let part5 = overlay(&get_rgb_image_data("layer1"), &get_rgb_image_data("pattern1"));
    create_image(&get_bytes(part5), "part5");

    let part6 = change_channel(&get_rgb_image_data("car"), 1.0, 0, 0, 200, 0);
    create_image(&get_bytes(part6), "part6");

    let mut part7 = change_channel(&get_rgb_image_data("car"), 4.0, 1, 0, 0, 0);
    part7 = change_channel(&part7, 0.0, 2, 0, 0, 0);
    create_image(&get_bytes(part7), "part7");

    let mut part8_r = change_channel(&get_rgb_image_data("car"), 0.0, 1, 0, 0, 0);
    part8_r = change_channel(&part8_r, 0.0, 2, 0, 0, 0);
    create_image(&get_bytes(part8_r), "part8_r");

    let mut part8_g = change_channel(&get_rgb_image_data("car"), 0.0, 0, 0, 0, 0);
    part8_g = change_channel(&part8_g, 0.0, 2, 0, 0, 0);
    create_image(&get_bytes(part8_g), "part8_g");

    let mut part8_b = change_channel(&get_rgb_image_data("car"), 0.0, 0, 0, 0, 0);
    part8_b = change_channel(&part8_b, 0.0, 1, 0, 0, 0);
    create_image(&get_bytes(part8_b), "part8_b");

    let part9 = add(&add(&get_rgb_image_data("layer_red"), &get_rgb_image_data("layer_blue")), &get_rgb_image_data("layer_green"));
    create_image(&get_bytes(part9), "part9");

    let part10 = get_rgb_image_data("text2");
    create_image(&get_bytes(part10), "part10");
}
