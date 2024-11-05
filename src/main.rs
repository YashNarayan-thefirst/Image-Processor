use std::{
    fs::File,
    io::{Read, Write},
};

/*
FAQs:
1. How is the formatting so good?
    I used cargo fmt, which automatically formats my code in a rust-intened fashion. It is needed to look at the abominations I have made.
2. How is testing done?
    I use the example files as a reference and compare my generated images using the boolean eq(==) operator. It's that simple.
3. Why is the width and height a vec?
    The width and height fields in the Header struct being vec is due to the design choice for storing binary data in a flexible form
4. What is the time complexity?
    I have not implemented any optimization, so the time complexity is likely O(n^2).
5. Why are some of the colors swapped around such as in part 7?
    This is a mistake I should have corrected earlier, but I realized midway into the project that the order of storing pixels is (b,g,r) not (r,g,b)
    I will correct this in the future.
What has been done:
    1. Read/write files ✅
    2. Image manipulation functions ✅
    3. Performing all required tasks ✅

Future expansions:
    1. Correct the rgb mistake
    2. Implement optimization
    3. Implement more image manipulation functions
*/


#[derive(Clone, Debug)]
struct Header {
    //Rust doesn't have classes, so structs need to be used
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
    image_descriptor: u8,
}

#[derive(Clone, Copy, Debug)]
struct Pixel {
    blue: u8,
    green: u8,
    red: u8,
}

#[derive(Clone, Debug)]
struct RgbImage {
    header: Header,
    pixel_data: Vec<Pixel>,
}

fn read_file_vec(filepath: &str) -> Result<Vec<u8>, std::io::Error> {
    //I wrote this with reference from: https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
    let mut file = File::open(filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn create_image(image_bytes: &[u8], output_path: &str) -> Result<(), std::io::Error> {
    //Same thing, but from: https://doc.rust-lang.org/std/fs/struct.File.html
    if let Some(parent) = std::path::Path::new(output_path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    File::create(output_path)?.write_all(image_bytes)
}

fn multiply(a: &RgbImage, b: &RgbImage) -> RgbImage {
    //Iterates through each px, normailizing then multiply their values. +0.5 to handle rounding truncations.
    RgbImage {
        header: a.header.clone(),
        pixel_data: a
            .pixel_data
            .iter()
            .zip(&b.pixel_data)
            .map(|(a, b)| {
                let (ar, ag, ab) = (
                    a.red as f64 / 255.0,
                    a.green as f64 / 255.0,
                    a.blue as f64 / 255.0,
                );
                let (br, bg, bb) = (b.red as f64, b.green as f64, b.blue as f64);
                Pixel {
                    red: (ar * br + 0.5) as u8,
                    green: (ag * bg + 0.5) as u8,
                    blue: (ab * bb + 0.5) as u8,
                }
            })
            .collect(),
    }
}

fn subtract(top: &RgbImage, bottom: &RgbImage) -> RgbImage {
    //Subtracts each pixel, pretty self explanatory
    RgbImage {
        header: bottom.header.clone(),
        pixel_data: top
            .pixel_data
            .iter()
            .zip(&bottom.pixel_data)
            .map(|(t, b)| Pixel {
                red: if t.red <= b.red { b.red - t.red } else { 0 },
                green: if t.green <= b.green {
                    b.green - t.green
                } else {
                    0
                },
                blue: if t.blue <= b.blue { b.blue - t.blue } else { 0 },
            })
            .collect(),
    }
}

fn screen(a: &RgbImage, b: &RgbImage) -> RgbImage {
    //Uses the provided screen formula
    RgbImage {
        header: a.header.clone(),
        pixel_data: a
            .pixel_data
            .iter()
            .zip(&b.pixel_data)
            .map(|(a, b)| {
                let (ar, ag, ab) = (
                    a.red as f64 / 255.0,
                    a.green as f64 / 255.0,
                    a.blue as f64 / 255.0,
                );
                let (br, bg, bb) = (
                    b.red as f64 / 255.0,
                    b.green as f64 / 255.0,
                    b.blue as f64 / 255.0,
                );
                Pixel {
                    red: 255 - ((1.0 - ar) * (1.0 - br) * 255.0 + 0.5) as u8,
                    green: 255 - ((1.0 - ag) * (1.0 - bg) * 255.0 + 0.5) as u8,
                    blue: 255 - ((1.0 - ab) * (1.0 - bb) * 255.0 + 0.5) as u8,
                }
            })
            .collect(),
    }
}

fn overlay(a: &RgbImage, b: &RgbImage) -> RgbImage {
    //Uses the overlay pixel formula and maps it to the image
    RgbImage {
        header: a.header.clone(),
        pixel_data: a
            .pixel_data
            .iter()
            .zip(&b.pixel_data)
            .map(|(a, b)| overlay_pixels(a, b))
            .collect(),
    }
}
fn overlay_pixels(a: &Pixel, b: &Pixel) -> Pixel {
    //Uses the overlay formula: b<=0.5: (c=2ab)/ b>0.5 (c=1-2(1-a)(1-b))                                    This was a nightmare to write
    let (ar, ag, ab) = (
        a.red as f64 / 255.0,
        a.green as f64 / 255.0,
        a.blue as f64 / 255.0,
    );
    let (br, bg, bb) = (
        b.red as f64 / 255.0,
        b.green as f64 / 255.0,
        b.blue as f64 / 255.0,
    );
    Pixel {
        red: ((if br <= 0.5 {
            ar * 2.0 * br
        } else {
            1.0 - 2.0 * (1.0 - ar) * (1.0 - br)
        }) * 255.0
            + 0.5) as u8,
        green: ((if bg <= 0.5 {
            ag * 2.0 * bg
        } else {
            1.0 - 2.0 * (1.0 - ag) * (1.0 - bg)
        }) * 255.0
            + 0.5) as u8,
        blue: ((if bb <= 0.5 {
            ab * 2.0 * bb
        } else {
            1.0 - 2.0 * (1.0 - ab) * (1.0 - bb)
        }) * 255.0
            + 0.5) as u8,
    }
}
fn generate_rgb_image_bytes(rgb_image: RgbImage) -> Vec<u8> {
    //Generates a Vec of all data in sequence, for ease of access
    let mut bytes = vec![
        rgb_image.header.id_length,
        rgb_image.header.color_map_type,
        rgb_image.header.image_type_code,
    ];
    bytes.extend(&rgb_image.header.color_map_origin);
    bytes.extend(&rgb_image.header.color_map_length);
    bytes.push(rgb_image.header.color_map_depth);
    bytes.extend(&rgb_image.header.x_origin);
    bytes.extend(&rgb_image.header.y_origin);
    bytes.extend(&rgb_image.header.width);
    bytes.extend(&rgb_image.header.height);
    bytes.push(rgb_image.header.bits_per_pixel);
    bytes.push(rgb_image.header.image_descriptor);

    for p in rgb_image.pixel_data {
        bytes.push(p.red);
        bytes.push(p.green);
        bytes.push(p.blue);
    }
    bytes
}
fn get_rgb_image_data(input_path: &str) -> RgbImage {
    //Generates an RgbImage given a path for "officially" accessing files
    let raw = read_file_vec(input_path).expect("Failed to read file");

    let header = Header {
        id_length: raw[0],
        color_map_type: raw[1],
        image_type_code: raw[2],
        color_map_origin: raw[3..5].try_into().unwrap(),
        color_map_length: raw[5..7].try_into().unwrap(),
        color_map_depth: raw[7],
        x_origin: raw[8..10].try_into().unwrap(),
        y_origin: raw[10..12].try_into().unwrap(),
        width: raw[12..14].try_into().unwrap(),
        height: raw[14..16].try_into().unwrap(),
        bits_per_pixel: raw[16],
        image_descriptor: raw[17],
    };

    let pixels: Vec<Pixel> = raw[18..]
        .chunks_exact(3)
        .map(|chunk| Pixel {
            red: chunk[0],
            green: chunk[1],
            blue: chunk[2],
        })
        .collect();

    RgbImage {
        header,
        pixel_data: pixels,
    }
}

fn print_test(rgb_image: RgbImage, s: &str, no: u8) {
    //Compres the generated user image to the example images.                                               I even put the unicode checkmark and cross for no reason
    let generated_bytes = generate_rgb_image_bytes(rgb_image);
    let output_path = format!("output/{}.tga", s);
    create_image(&generated_bytes, &output_path).expect("Cannot write file");

    let test_bytes = read_file_vec(&format!("examples/EXAMPLE_{}.tga", s).to_string())
        .expect("Failed to read output file");

    println!(
        "Part #{} Test: {}",
        no,
        if generated_bytes == test_bytes {
            "✅"
        } else {
            "❌"
        }
    );
}

fn main() {
    //Assign filenames to the appropriate variable so we can access their data. I was planning to use a hashmap, but I was too lazy to write hashmap.unwrap().get(k)
    let car = get_rgb_image_data("input/car.tga");
    let circles = get_rgb_image_data("input/circles.tga");
    let layer_blue = get_rgb_image_data("input/layer_blue.tga");
    let layer_green = get_rgb_image_data("input/layer_green.tga");
    let layer_red = get_rgb_image_data("input/layer_red.tga");
    let layer1 = get_rgb_image_data("input/layer1.tga");
    let layer2 = get_rgb_image_data("input/layer2.tga");
    let pattern2 = get_rgb_image_data("input/pattern2.tga");
    let pattern1 = get_rgb_image_data("input/pattern1.tga");
    let text = get_rgb_image_data("input/text.tga");
    let text2 = get_rgb_image_data("input/text2.tga");


    //Creating and testing images
    print_test(multiply(&layer1, &pattern1), "part1", 1);
    print_test(subtract(&layer2, &car), "part2", 2);
    print_test(screen(&text, &multiply(&layer1, &pattern2)), "part3", 3);
    print_test(
        subtract(&pattern2, &multiply(&layer2, &circles)),
        "part4",
        4,
    );
    print_test(overlay(&layer1, &pattern1), "part5", 5);
    
    //Iterates through pxs and constructs new pxs with g+200
    let mut part6 = car.clone();
    for p in &mut part6.pixel_data {
        p.green = (p.green as u16 + 200).min(255) as u8;
    }
    print_test(part6, "part6", 6);
    //Same here, but multiplies red with a max of 255 and removes blue 
    let mut part7 = car.clone();
    for p in &mut part7.pixel_data {
        p.blue = (p.blue as u16 * 4).min(255) as u8;
        p.green = p.green as u8;
        p.red = (p.red * 0) as u8;
    }
    print_test(part7, "part7", 7);

    //Iterates through pxs and constructs new pxs with each color value
    {
        let part8_b = RgbImage {
            header: car.header.clone(),
            pixel_data: car
                .pixel_data
                .iter()
                .map(|p| Pixel {
                    red: p.red,
                    green: p.red,
                    blue: p.red,
                })
                .collect(),
        };
        print_test(part8_b, "part8_b", 83);
    }
    {
        let part8_g = RgbImage {
            header: car.header.clone(),
            pixel_data: car
                .pixel_data
                .iter()
                .map(|p| Pixel {
                    red: p.green,
                    green: p.green,
                    blue: p.green,
                })
                .collect(),
        };
        print_test(part8_g, "part8_g", 82);
    }
    {
        let part8_r = RgbImage {
            header: car.header.clone(),
            pixel_data: car
                .pixel_data
                .iter()
                .map(|p| Pixel {
                    red: p.blue,
                    green: p.blue,
                    blue: p.blue,
                })
                .collect(),
        };
        print_test(part8_r, "part8_r", 81);
    }
    //Merges the rgb values of each layer and stores it
    let part9 = RgbImage {
        header: layer_red.header.clone(),
        pixel_data: {
            let mut pixels = Vec::new();
            for i in 0..layer_red.pixel_data.len() {
                pixels.push(Pixel {
                    red: layer_blue.pixel_data[i].red,
                    green: layer_green.pixel_data[i].green,
                    blue: layer_red.pixel_data[i].blue,
                });
            }
            pixels
        },
    };

    print_test(part9, "part9", 9);

    //Flips pixels using the reverse method, since they are stored in a vec
    let part10 = RgbImage {
        header: text2.header.clone(),
        pixel_data: {
            let mut reversed_pixels = text2.pixel_data.clone();
            reversed_pixels.reverse();
            reversed_pixels
        },
    };

    print_test(part10, "part10", 10);

    //The extra credit nightmare, please look away from this mess
    let mut header = car.header.clone();
    let width = u16::from_le_bytes(car.header.width.try_into().unwrap()) * 2; //We need an image twice the length and height each, so we do this calculation to get it
    let height = u16::from_le_bytes(car.header.height.try_into().unwrap()) * 2; //
    header.width = width.to_le_bytes().to_vec();
    header.height = height.to_le_bytes().to_vec();

    let row_width = width as usize / 2; //Used to determine the stopping point
    let row_height = height as usize / 2;
    let mut combined_pixels = Vec::with_capacity(width as usize * height as usize);

    for y in 0..row_height {
        let start = y * row_width;
        let end = (y + 1) * row_width;
        combined_pixels.extend_from_slice(&text.pixel_data[start..end]);
        combined_pixels.extend_from_slice(&pattern1.pixel_data[start..end]);
    }

    for y in 0..row_height {
        let start = y * row_width;
        let end = (y + 1) * row_width;
        combined_pixels.extend_from_slice(&car.pixel_data[start..end]);
        combined_pixels.extend_from_slice(&circles.pixel_data[start..end]);
    }

    let ec = RgbImage {
        header,
        pixel_data: combined_pixels,
    };

    print_test(ec, "extracredit", 11);
}
