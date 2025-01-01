use sdl2::{
    render::{Texture, TextureCreator, WindowCanvas},
    ttf::Font,
    video::WindowContext,
};

use crate::ds::Board;

pub struct ApplicationData<'a> {
    pub running: bool,
    pub canvas: WindowCanvas,
    pub current_screen: GameScreen,
    pub title_font: Font<'a, 'static>,
    pub tile_font: Font<'a, 'static>,
    pub texture_creator: TextureCreator<WindowContext>,
    pub board: Option<Board>,
    pub board_image: Texture<'a>,
}

pub enum GameScreen {
    Home,
    Play,
}
