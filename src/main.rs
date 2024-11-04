use std::path::Path;
#[derive(Clone, Default)]
struct Header {
    id_length: u8, color_map_type: u8, image_type_code: u8,
    color_map_origin: u16, color_map_length: u16,
    color_map_depth: u8, x_origin: u16, y_origin: u16,
    width: u16, height: u16, bits_per_pixel: u8, image_descriptor: u8,
}

#[derive(Clone, Copy, Default)]
struct Pixel { r: u8, g: u8, b: u8 }

#[derive(Clone)]
struct RgbImage { header: Header, pixel_data: Vec<Pixel> }

fn read_file(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = std::fs::read(filepath).expect("Cannot read file");
    Ok(data)
}

fn create_image(image_bytes: &[u8], filename: &str) {
    let x = format!("output/{}.tga", filename);
    let output_path = Path::new(&x);
    std::fs::write(&output_path, image_bytes).expect("Write error")
}
fn get_bytes(rgb_image: RgbImage) -> Vec<u8> {
    let mut bytes = vec![
        rgb_image.header.id_length, rgb_image.header.color_map_type, rgb_image.header.image_type_code,
        (rgb_image.header.color_map_origin & 0xFF) as u8, (rgb_image.header.color_map_origin >> 8) as u8,
        (rgb_image.header.color_map_length & 0xFF) as u8, (rgb_image.header.color_map_length >> 8) as u8,
        rgb_image.header.color_map_depth,
        (rgb_image.header.x_origin & 0xFF) as u8, (rgb_image.header.x_origin >> 8) as u8,
        (rgb_image.header.y_origin & 0xFF) as u8, (rgb_image.header.y_origin >> 8) as u8,
        (rgb_image.header.width & 0xFF) as u8, (rgb_image.header.width >> 8) as u8,
        (rgb_image.header.height & 0xFF) as u8, (rgb_image.header.height >> 8) as u8,
        rgb_image.header.bits_per_pixel, rgb_image.header.image_descriptor
    ];
    bytes.extend(rgb_image.pixel_data.iter().flat_map(|p| [p.r, p.g, p.b]));
    bytes
}

fn get_rgb_image_data(input_path: &str) -> RgbImage {
    let data = read_file(input_path).expect("Cannot read file");
    let header = Header {
        id_length: data[0],
        color_map_type: data[1],
        image_type_code: data[2],
        color_map_origin: u16::from_le_bytes([data[3], data[4]]),
        color_map_length: u16::from_le_bytes([data[5], data[6]]),
        color_map_depth: data[7],
        x_origin: u16::from_le_bytes([data[8], data[9]]),
        y_origin: u16::from_le_bytes([data[10], data[11]]),
        width: u16::from_le_bytes([data[12], data[13]]),
        height: u16::from_le_bytes([data[14], data[15]]),
        bits_per_pixel: data[16],
        image_descriptor: data[17],
    };

    let mut pixel_data = Vec::new();
    for i in (18..data.len()).step_by(3) {
        let pixel = Pixel {
            r: data[i],
            g: data[i+1],
            b: data[i+2],
        };
        pixel_data.push(pixel);
    }

    RgbImage { header, pixel_data }
}

fn add(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    result.pixel_data.iter_mut().zip(&bottom.pixel_data).for_each(|(t, b)| {
        t.r = t.r.saturating_add(b.r);
        t.g = t.g.saturating_add(b.g);
        t.b = t.b.saturating_add(b.b);
    });
    result
}

fn multiply(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    result.pixel_data.iter_mut().zip(&bottom.pixel_data).for_each(|(t, b)| {
        t.r = ((t.r as u16 * b.r as u16) / 255) as u8;
        t.g = ((t.g as u16 * b.g as u16) / 255) as u8;
        t.b = ((t.b as u16 * b.b as u16) / 255) as u8;
    });
    result
}

fn subtract(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    result.pixel_data.iter_mut().zip(&bottom.pixel_data).for_each(|(t, b)| {
        t.r = b.r.saturating_sub(t.r);
        t.g = b.g.saturating_sub(t.g);
        t.b = b.b.saturating_sub(t.b);
    });
    result
}

fn screen(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    result.pixel_data.iter_mut().zip(&bottom.pixel_data).for_each(|(t, b)| {
        t.r = 255 - ((255 - t.r as u16) * (255 - b.r as u16) / 255) as u8;
        t.g = 255 - ((255 - t.g as u16) * (255 - b.g as u16) / 255) as u8;
        t.b = 255 - ((255 - t.b as u16) * (255 - b.b as u16) / 255) as u8;
    });
    result
}

fn overlay(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    let mut result = top.clone();
    result.pixel_data.iter_mut().zip(&bottom.pixel_data).for_each(|(t, b)| {
        t.r = if b.r < 128 {
            ((t.r as u16 * b.r as u16) / 128) as u8
        } else {
            255 - ((2 * (255 - t.r) as u16 * (255 - b.r) as u16) / 255) as u8
        };
        t.g = if b.g < 128 {
            ((t.g as u16 * b.g as u16) / 128) as u8
        } else {
            255 - ((2 * (255 - t.g) as u16 * (255 - b.g) as u16) / 255) as u8
        };
        t.b = if b.b < 128 {
            ((t.b as u16 * b.b as u16) / 128) as u8
        } else {
            255 - ((2 * (255 - t.b) as u16 * (255 - b.b) as u16) / 255) as u8
        };
    });
    result
}

fn change_channel(image: &RgbImage, multiplier: f32, channel: u8, r_offset: u8, g_offset: u8, b_offset: u8) -> RgbImage {
    let mut result = image.clone();
    result.pixel_data.iter_mut().for_each(|p| {
        let r = ((p.r as f32 * multiplier + r_offset as f32).clamp(0.0, 255.0)) as u8;
        let g = ((p.g as f32 * multiplier + g_offset as f32).clamp(0.0, 255.0)) as u8;
        let b = ((p.b as f32 * multiplier + b_offset as f32).clamp(0.0, 255.0)) as u8;
        match channel {
            0 => p.r = r,
            1 => p.g = g,
            2 => p.b = b,
            _ => {}
        }
    });
    result
}
fn test_image(output: &[u8], example: &[u8]) -> bool { output.iter().zip(example).all(|(o, e)| o == e) }

fn print_test(rgb_image: RgbImage, s: &str, no: u8) {
    let generated = get_bytes(rgb_image);
    let example = read_file(&format!("examples/EXAMPLE_{}.RgbImage", s)).expect("Cannot read file");
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
