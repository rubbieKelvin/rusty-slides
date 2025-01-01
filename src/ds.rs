use sdl2::rect::{Point, Rect};

pub const TILE_SIZE: i32 = 100;
pub const TILE_SPACING: i32 = 10;

pub struct Tile {
    pub selected: bool,
    pub value: Option<i32>,
    pub address: (i32, i32),
    pub position: Point,
    pub image_src_rect: Option<Rect>,
    pub image_dest_rect: Option<Rect>,
}

impl Tile {
    pub fn new(
        value: Option<i32>,
        pos: Point,
        address: (i32, i32),
        image_src_rect: Option<Rect>,
        image_dest_rect: Option<Rect>,
    ) -> Self {
        return Tile {
            value,
            address,
            position: pos,
            selected: false,
            image_src_rect,
            image_dest_rect,
        };
    }

    pub fn swap_value(&mut self, other: &mut Tile) -> bool {
        // returns true if the values where swaped
        // dont swap if the other tile is empty (block tiles)
        if other.value.is_none() {
            return false;
        }

        let other_selected = other.selected;
        let other_value = other.value.take();
        let other_img_src = other.image_src_rect;

        other.selected = self.selected;
        other.value = self.value;

        if self.image_src_rect.is_some() {
            other.image_src_rect = self.image_src_rect;
        }

        self.selected = other_selected;
        self.value = other_value;

        if other_img_src.is_some() {
            self.image_src_rect = other_img_src;
        }
        return true;
    }
}

#[allow(dead_code)]
pub struct Board {
    pub row: i32,
    pub col: i32,
    pub position: Point,
    pub cells: Vec<Tile>,
    pub show_help: bool
}

impl Board {
    pub fn new(size: i32, topleft: Point) -> Self {
        let mut cells = Vec::new();

        // add the empty tile; tile 0
        let mut empty_tile = Tile::new(
            Some(0),
            topleft.clone(),
            (0, 0),
            None,
            Some(Rect::new(
                topleft.x,
                topleft.y,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            )),
        );
        empty_tile.selected = true;
        cells.push(empty_tile);

        // add the immovable tiles at the top
        for i in 1..size {
            cells.push(Tile::new(
                None,
                Point::new((TILE_SIZE * i) + (TILE_SPACING * i) + topleft.x, topleft.y),
                (i, 0),
                None,
                None,
            ));
        }

        // now push main tiles
        for i in 0..(size.pow(2)) {
            let x = i % size;
            let y = (i / size) + 1; // adding one cuse we alreaady added a layer above

            cells.push(Tile::new(
                Some(i + 1),
                Point::new(
                    (TILE_SIZE * x) + (TILE_SPACING * x) + topleft.x,
                    (TILE_SIZE * y) + (TILE_SPACING * y) + topleft.y,
                ),
                (x, y),
                Some(Rect::new(
                    (TILE_SIZE * x) + (TILE_SPACING * x),
                    (TILE_SIZE * (y - 1)) + (TILE_SPACING * (y -1)),
                    TILE_SIZE as u32,
                    TILE_SIZE as u32,
                )),
                Some(Rect::new(
                    (TILE_SIZE * x) + (TILE_SPACING * x) + topleft.x,
                    (TILE_SIZE * y) + (TILE_SPACING * y) + topleft.y,
                    TILE_SIZE as u32,
                    TILE_SIZE as u32,
                )),
            ));
        }

        return Board {
            row: size,
            col: size + 1,
            cells,
            position: topleft,
            show_help: false
        };
    }

    pub fn check_arranged(&self) -> bool {
        let mut count = 0;
        for tile in &self.cells {
            if let Some(value) = tile.value {
                if count != value {
                    return false;
                }
                count += 1;
            }
        }
        return true;
    }
}
