use crate::{
    app_state::{ApplicationData, GameScreen},
    ds::{Board, Tile, TILE_SIZE, TILE_SPACING},
};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};

impl Tile {
    fn render(&self, context: &mut ApplicationData, board: &Board) -> Result<(), String> {
        if self.selected {
            context.canvas.set_draw_color(Color::WHITE);
            context.canvas.draw_rect(Rect::new(
                self.position.x,
                self.position.y,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ))?;
        } else {
            context.canvas.set_draw_color(if self.value.is_none() {
                Color::GRAY
            } else {
                Color::BLUE
            });

            context.canvas.fill_rect(Rect::new(
                self.position.x,
                self.position.y,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ))?;

            if self.value.is_some() {
                if self.image_src_rect.is_some() && self.image_dest_rect.is_some() {
                    let img_rect = self.image_src_rect.unwrap();
                    let img_dest = self.image_dest_rect.unwrap();
                    context
                        .canvas
                        .copy(&context.board_image, img_rect, img_dest)?;
                }

                // draw a number on the tile
                if board.show_help {
                    let text_surf = context
                        .tile_font
                        .render(self.value.unwrap().to_string().as_str())
                        .blended(Color::BLACK)
                        .map_err(|e| e.to_string())?;

                    let mut rect = text_surf.rect();
                    rect.set_x(self.position.x + TILE_SIZE / 2);
                    rect.set_y(self.position.y + TILE_SIZE / 2);

                    let texture = text_surf.as_texture(&context.texture_creator).unwrap();
                    context.canvas.copy(&texture, None, Some(rect))?;
                }
            }
        }

        // let mut surf = Surface::new(TILE_SIZE as u32, TILE_SIZE as u32, PixelFormatEnum::RGB24)
        //     .map_err(|e| e.to_string())?;

        // surf.fill_rect(
        //     None,
        //     if self.value.is_none() {
        //         Color::GRAY
        //     } else {
        //         Color::BLUE
        //     },
        // )?;

        // let texture = surf
        //     .as_texture(&context.texture_creator)
        //     .map_err(|e| e.to_string())?;

        // let mut draw_pos = surf.rect();

        // draw_pos.set_x(self.position.x);
        // draw_pos.set_y(self.position.y);

        // context.canvas.copy(&texture, None, Some(draw_pos))?;

        return Ok(());
    }
}

impl Board {
    fn render(&self, context: &mut ApplicationData) -> Result<(), String> {
        for tile in &self.cells {
            tile.render(context, self)?;
        }
        return Ok(());
    }
}

fn render_home(context: &mut ApplicationData) -> Result<(), String> {
    let message = "Press enter to continue";
    let font_surface = context
        .title_font
        .render(&message)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();

    let texture = context
        .texture_creator
        .create_texture_from_surface(&font_surface)
        .map_err(|e| e.to_string())?;

    // lets see if we can position the text at the center
    let canvas_size = context.canvas.window().size();
    let mut pos = font_surface.rect();

    pos.center_on(Point::new(
        canvas_size.0 as i32 / 2,
        canvas_size.1 as i32 / 2,
    ));

    context.canvas.copy(&texture, None, Some(pos))?;
    return Ok(());
}

fn render_play(context: &mut ApplicationData) -> Result<(), String> {
    let board = context.board.take();

    if let Some(board) = board {
        let board_valid = board.check_arranged();
        board.render(context)?;

        // render passed
        let text_surf = context
            .title_font
            .render(if board_valid {
                "You won!"
            } else {
                "Arrange the tile to win"
            })
            .blended(if board_valid {
                Color::GREEN
            } else {
                Color::GRAY
            })
            .unwrap();

        // help text
        let mut text_rect = text_surf.rect();
        text_rect.set_x(200);
        text_rect.set_y(100);
        let texture = text_surf.as_texture(&context.texture_creator).unwrap();
        context.canvas.copy(&texture, None, Some(text_rect))?;

        // draw full image by the side
        let col = board.col;
        let row = board.row;

        context.canvas.copy(
            &context.board_image,
            Rect::new(
                0,
                0,
                (TILE_SIZE * row) as u32 + (TILE_SPACING * row) as u32,
                (TILE_SIZE * (col - 1)) as u32 + (TILE_SPACING * (col - 1)) as u32,
            ),
            Rect::new(
                100 + (TILE_SIZE * row) + (TILE_SPACING * (row)) + board.position.x,
                board.position.y + TILE_SIZE + TILE_SPACING,
                (TILE_SIZE * row) as u32 + (TILE_SPACING * row) as u32,
                (TILE_SIZE * (col - 1)) as u32 + (TILE_SPACING * (col - 1)) as u32,
            ),
        )?;

        // return the board value
        context.board = Some(board);
    }

    return Ok(());
}

pub fn render_canvas(context: &mut ApplicationData) -> Result<(), String> {
    match context.current_screen {
        GameScreen::Home => {
            return render_home(context);
        }
        GameScreen::Play => {
            return render_play(context);
        }
    }
}
