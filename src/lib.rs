use wasm_bindgen::prelude::*;
extern crate raster;

fn get_pixel(width: i32, height: i32, bytes: &[u8], x: i32, y: i32) -> raster::Color {
  let rgba = 4;
  let start = (y * width) + x;
  let start = start * rgba;
  let end = start + rgba;
  let len = bytes.len();
  if start as usize > len || end as usize > len {
    raster::Color {
      r: 0,
      g: 0,
      b: 0,
      a: 0,
    }
  } else {
    //let slice : &[u8; 4] = bytes[start as usize..end as usize];
    raster::Color {
      r: bytes[start as usize],
      g: bytes[(start + 1) as usize],
      b: bytes[(start + 2) as usize],
      a: bytes[(start + 3) as usize],
    }
  }
}

fn set_pixel(width: i32, height: i32, vec: &mut Vec<u8>, x: i32, y: i32, color: &raster::Color) {
  let rgba = 4; // length
  let start = (y * width) + x;
  let start = start * rgba;
  if x >= width || y >= height {
  } else if start < 0 {
  } else {
    vec[start as usize] = color.r;
    vec[start as usize + 1] = color.g;
    vec[start as usize + 2] = color.b;
    vec[start as usize + 3] = color.a;
  }
}

#[wasm_bindgen]
pub fn horizontally_flip(w: i32, h: i32, bytes: &[u8]) -> Vec<u8> {
  let mut vec: Vec<u8> = vec![0];
  vec.resize((w * h) as usize * 4, 0);
  for x in 0..w {
    let src_x = x;
    let dest_x = w - x - 1;
    if dest_x <= src_x {
      break;
    }
    for y in 0..h {
      let pixel_left = get_pixel(w, h, bytes, src_x, y);
      let pixel_right = get_pixel(w, h, bytes, dest_x, y);

      set_pixel(w, h, &mut vec, dest_x, y, &pixel_left);
      set_pixel(w, h, &mut vec, src_x, y, &pixel_right);
    }
  }

  vec
}

#[wasm_bindgen]
pub fn vertically_flip(w: i32, h: i32, bytes: &[u8]) -> Vec<u8> {
  let mut vec: Vec<u8> = vec![0];
  vec.resize((w * h) as usize * 4, 0);

  for y in 0..h {
    let src_y = y;
    let dest_y = h - y - 1;
    if dest_y <= src_y {
      break;
    }
    for x in 0..w {
      let pixel_top = get_pixel(w, h, bytes, x, src_y);
      let pixel_bottom = get_pixel(w, h, bytes, x, dest_y);

      set_pixel(w, h, &mut vec, x, dest_y, &pixel_top);
      set_pixel(w, h, &mut vec, x, src_y, &pixel_bottom);
    }
  }
  vec
}
