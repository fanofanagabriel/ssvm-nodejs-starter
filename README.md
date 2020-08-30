# 基于 ASSEMBLY 的图像处理 WEB 服务

Fork 自 [second-state/ssvm-nodejs-starter](https://github.com/second-state/ssvm-nodejs-starter)

## 使用 raster 作为图像处理库


这中间遇到 raster 0.2.0 依赖编译不过去的问题，需要 [Overriding Dependencies](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html) ，用 raster github  当前的分支。

```toml
[dependencies]
raster = "*"
wasm-bindgen = "=0.2.61"

[patch.crates-io]
raster = { path = "/root/workspace/raster" }
```

WASI 不允许使用 `std::fs`， 据说 [#343](https://github.com/wasmerio/wasmer/pull/343) 解决了, 但 ssvmup 最高只支持 "=0.2.61" 的 wasm-bindgen, 所以只能用 nodejs 将图像的像素传给 wasm 接口

```js
  new jimp("uploads/" + file, function (err, image) {
    var w = image.bitmap.width;
    var h = image.bitmap.height;
    var bitmap_data = image.bitmap.data;
    [...]

    var uintc8 = new Uint8ClampedArray(w * h * 4);
    if (type == 1) {
      uintc8 = horizontally_flip(w, h, bitmap_data);
    }
    else if (type == 2) {
      uintc8 = vertically_flip(w, h, bitmap_data);
    }
    [...]

```

对此还需要改装一下 raster 的 `flip` 接口

```rust
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
```

因为 wasm 不支持 `&mut [u8]` 参数类型，所以 Rust 内部还需要一笔拷贝的花销。


## 效果

用 Docker 搭建服务, [点击前往](http://141.164.37.152:8080/)


![效果](docs/img/demo.png)

## 

