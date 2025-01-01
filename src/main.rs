use std::path::Path;

use app_state::{ApplicationData, GameScreen};
use events::handle_events;
use process::process_data;
use render::render_canvas;
use sdl2::{image::LoadTexture, pixels::Color, render::WindowCanvas, Sdl};

mod app_state;
mod ds;
mod events;
mod process;
mod render;

fn init(title: &str, width: u32, height: u32) -> Result<(Sdl, WindowCanvas), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    return Ok((sdl_context, canvas));
}

fn main() -> Result<(), String> {
    if let Ok((context, canvas)) = init("Slides", 1200, 900) {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let image_texture_creatore = canvas.texture_creator();

        let mut app_data = ApplicationData {
            canvas,
            running: true,
            texture_creator,
            current_screen: GameScreen::Home,
            board: None,
            board_image: image_texture_creatore.load_texture("assets/img/me.jpg")?,
            title_font: ttf_context.load_font(Path::new("assets/fonts/pixelify.ttf"), 16)?,
            tile_font: ttf_context
                .load_font(Path::new("assets/fonts/jb-mono-nerd-bold.ttf"), 22)?,
        };

        let mut pump = context.event_pump()?;

        while app_data.running {
            app_data.canvas.set_draw_color(Color::BLACK);
            app_data.canvas.clear();

            for event in pump.poll_iter() {
                handle_events(&mut app_data, event)?;
            }

            process_data(&mut app_data);
            render_canvas(&mut app_data)?;

            app_data.canvas.present();
        }
    } else {
        println!("Error initializing sdl2");
    }
    return Ok(());
}
