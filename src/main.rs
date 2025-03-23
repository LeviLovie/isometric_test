mod config;
mod sound;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::time::{Duration, Instant};
use std_utils::{errors::*, paths::rel_path, Error, Result};
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let config = config::Config::from_file("assets/config.yaml".into())
        .logmsg("Error while parsing \"config.yaml\"")?;
    info!("Config:\n{:#?}", config);

    let tiles_config_path = rel_path(&config.tiles.config.clone())?;
    let tiles = map::Tiles::from_file(tiles_config_path.clone())
        .logmsg(&format!("Error while parsing {:?}", tiles_config_path))?;
    info!("Tiles:\n{:#?}", tiles);

    return Ok(());

    //let target_fps = 30.0;
    //let frame_time = Duration::from_secs_f64(1.0 / target_fps);
    //
    //let mut sound = sound::Sound::new().expect("Failed to create Sound");
    //sound
    //    .load(vec!["Master", "Master.strings"])
    //    .expect("Failed to load banks");
    //
    //let sdl_context = sdl2::init().unwrap();
    //let video_subsystem = sdl_context.video().unwrap();
    //
    //let window = video_subsystem
    //    .window("Catties", 800, 600)
    //    .position_centered()
    //    .build()
    //    .unwrap();
    //
    //let mut canvas = window.into_canvas().build().unwrap();
    //canvas.set_draw_color(Color::RGB(0, 0, 0));
    //canvas.clear();
    //canvas.present();
    //
    //let mut event_pump = sdl_context.event_pump().unwrap();
    //let mut last_update = Instant::now();
    //let mut fps;
    //'running: loop {
    //    for event in event_pump.poll_iter() {
    //        match event {
    //            Event::Quit { .. }
    //            | Event::KeyDown {
    //                keycode: Some(Keycode::Escape),
    //                ..
    //            } => break 'running,
    //            Event::KeyDown {
    //                keycode: Some(Keycode::Space),
    //                ..
    //            } => {
    //                sound.start("Jump").expect("Failed to play event");
    //                sound.update().expect("Failed to update Sound");
    //            }
    //            _ => {}
    //        }
    //    }
    //
    //    canvas.set_draw_color(Color::RGB(0, 0, 0));
    //    canvas.clear();
    //
    //    canvas.present();
    //    let elapsed = last_update.elapsed();
    //    if elapsed < frame_time {
    //        std::thread::sleep(frame_time - elapsed);
    //    } else {
    //        debug!("Frame took too long: {:?}", elapsed);
    //    }
    //    let elapsed = last_update.elapsed();
    //    fps = 1_000_000 / elapsed.as_micros();
    //    println!("FPS: {}", fps);
    //    last_update = Instant::now();
    //}
    //
    //return Ok(());
}
