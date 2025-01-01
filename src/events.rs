use crate::{
    app_state::{ApplicationData, GameScreen},
    ds::Board,
};
use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode, rect::Point};

enum BoardDirection {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

impl BoardDirection {
    fn random() -> BoardDirection {
        let mut rng = rand::thread_rng();
        return match rng.gen_range(0..4) {
            0 => BoardDirection::UP,
            1 => BoardDirection::DOWN,
            2 => BoardDirection::LEFT,
            3 => BoardDirection::RIGHT,
            _ => unreachable!(),
        };
    }
}

impl Board {
    fn get_selected_tile(&self) -> Option<usize> {
        for (index, tile) in self.cells.iter().enumerate() {
            if tile.selected {
                return Some(index);
            }
        }
        return None;
    }

    fn get_tile_at_address(&self, address: &(i32, i32)) -> Option<usize> {
        for (index, tile) in self.cells.iter().enumerate() {
            if tile.address.0 == address.0 && tile.address.1 == address.1 {
                return Some(index);
            }
        }
        return None;
    }

    fn move_tile(&mut self, dir: BoardDirection) -> bool {
        let selected_tile_index = match self.get_selected_tile() {
            Some(index) => index,
            None => return false,
        };

        let (x, y) = self.cells[selected_tile_index].address;

        let target_address = match dir {
            BoardDirection::UP => (x, y - 1),
            BoardDirection::DOWN => (x, y + 1),
            BoardDirection::LEFT => (x - 1, y),
            BoardDirection::RIGHT => (x + 1, y),
        };

        let target_tile_index = match self.get_tile_at_address(&target_address) {
            Some(index) => index,
            None => return false,
        };

        // borrow two mutable references to dodge borrow checker palava
        if selected_tile_index < target_tile_index {
            let (left, right) = self.cells.split_at_mut(target_tile_index);
            return left[selected_tile_index].swap_value(&mut right[0]);
        } else {
            let (left, right) = self.cells.split_at_mut(selected_tile_index);
            return right[0].swap_value(&mut left[target_tile_index]);
        }
    }

    pub fn scatter(&mut self) {
        // first move the first tile up
        self.move_tile(BoardDirection::DOWN);

        // now shuffle all the tiles at the bottom;
        let mut count = 1000;

        while count != 0 {
            'enforce_movement: loop {
                // if we actually moved a tile and the tile is not arranged, unto the next
                if self.move_tile(BoardDirection::random()) && !self.check_arranged() {
                    break 'enforce_movement;
                }
            }
            count -= 1;
        }
    }
}

fn handle_events_home(context: &mut ApplicationData, event: Event) {
    match event {
        Event::KeyDown { keycode, .. } => {
            if let Some(Keycode::Return) = keycode {
                let mut board = Board::new(3, Point::new(200, 150));
                board.scatter();

                context.board = Some(board);
                context.current_screen = GameScreen::Play;
            }
        }
        _ => {}
    }
}

fn handle_events_play(context: &mut ApplicationData, event: Event) {
    match event {
        Event::KeyDown { keycode, .. } => {
            if let Some(code) = keycode {
                match code {
                    Keycode::ESCAPE => {
                        context.current_screen = GameScreen::Home;
                    }
                    Keycode::UP => {
                        let board = context.board.take();
                        let mut board = board.unwrap();
                        board.move_tile(BoardDirection::DOWN);

                        context.board = Some(board);
                    }
                    Keycode::DOWN => {
                        let board = context.board.take();
                        let mut board = board.unwrap();
                        board.move_tile(BoardDirection::UP);

                        context.board = Some(board);
                    }
                    Keycode::LEFT => {
                        let board = context.board.take();
                        let mut board = board.unwrap();
                        board.move_tile(BoardDirection::RIGHT);

                        context.board = Some(board);
                    }
                    Keycode::RIGHT => {
                        let board = context.board.take();
                        let mut board = board.unwrap();
                        board.move_tile(BoardDirection::LEFT);

                        context.board = Some(board);
                    }
                    Keycode::H => {
                        let board = context.board.take();
                        let mut board = board.unwrap();
                        board.show_help = !board.show_help;
                        context.board = Some(board);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

pub fn handle_events(context: &mut ApplicationData, event: Event) -> Result<(), String> {
    // check if the user needs to quit the window
    if let Event::Quit { .. } = event {
        context.running = false;
        return Ok(());
    }

    // further handling
    match context.current_screen {
        GameScreen::Home => {
            handle_events_home(context, event);
        }
        GameScreen::Play => {
            handle_events_play(context, event);
        }
    }

    return Ok(());
}
