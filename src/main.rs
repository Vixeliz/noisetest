extern crate nannou;
use nannou::{
    color::RgbHue,
    image,
    noise::{self, BasicMulti, MultiFractal, NoiseFn, OpenSimplex, Worley},
    prelude::*,
};

fn main() {
    nannou::sketch(view).loop_mode(LoopMode::loop_once()).run();
}

// curved rects making a circle gets super rotated and distorted far out
fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let circles = 11;
    let start_circle = 4;
    // let noise: Worley = noise::Worley::new()
    //     .set_frequency(0.01251051)
    //     .enable_range(true)
    //     .set_range_function(noise::RangeFunction::Euclidean)
    //     .set_displacement(1.0);
    let noise: Worley = noise::Worley::default()
        .set_frequency(0.01251051)
        .enable_range(true)
        .set_range_function(noise::RangeFunction::Euclidean);
    // .set_displacement(1.0);
    // let noise_b = noise::Worley::new()
    //     .set_frequency(0.02251051)
    //     .enable_range(true)
    //     .set_range_function(noise::RangeFunction::Euclidean)
    //     .set_displacement(15.0);
    // let sampler = noise::BasicMulti::new();
    // let noise_final: noise::Blend<[f64; 2]> = noise::Blend::new(&noise, &noise_b, &sampler);
    // let noise = noise::Billow::new().set_frequency(0.01251051);
    let image = image::ImageBuffer::from_fn(1000, 1000, |x, y| {
        let n = noise.get([x as f64, y as f64]).abs() * 256.0;
        nannou::image::Rgba([n as u8, n as u8, n as u8, std::u8::MAX])
    });
    let texture = wgpu::TextureBuilder::new()
        .size([1000, 1000])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(app.window(app.window_id()).unwrap().device());
    let flat_samples = image.as_flat_samples();

    texture.upload_data(
        app.window(app.window_id()).unwrap().device(),
        &mut frame.command_encoder(),
        &flat_samples.as_slice(),
    );
    draw.texture(&texture);
    // for x in -1000..1000 {
    //     for y in -1000..1000 {
    //         let val = noise.get([x as f64, y as f64]) * 10.0;
    //         draw.ellipse()
    //             .color(Rgb::new(val as f32, val as f32, val as f32))
    //             .w(1.0)
    //             .h(1.0)
    //             .xy(Vec2::new(x as f32, y as f32));
    //     }
    // }
    draw.background().color(WHITE);
    draw.to_frame(app, &frame).unwrap();
}
