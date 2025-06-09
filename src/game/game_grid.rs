use uuid::Uuid;

use bevy::prelude::*;
use grid::*;

use rand::distr::{Distribution, StandardUniform};
use rand::prelude::*;

use crate::game::cell_line::CellLine;

pub struct GameGridPlugin;

pub const GRID_WIDTH: usize = 9;
pub const GRID_HEIGHT: usize = 9;

impl Plugin for GameGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InitializeGridEvent>()
            .add_event::<PopCellEvent>()
            .insert_resource(GameGrid::new());
    }
}

/// Call this to (re)initialize a GameGrid resource
#[derive(Event, Debug, Default)]
pub struct InitializeGridEvent;

#[derive(Event, Debug, Default)]
pub struct PopCellEvent(pub Vec2);

/// This is a list of available colors for pieces

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PieceColor {
    Pink,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
    Cyan,
    Red,
}

impl PieceColor {
    pub fn to_color(&self) -> Color {
        match self {
            PieceColor::Pink => Color::srgb(1.0, 0.00, 0.50), // #FF1493
            PieceColor::Green => Color::srgb(0.22, 1.0, 0.08), // #39FF14
            PieceColor::Blue => Color::srgb(0.3, 0.3, 1.0),   // #4D4DFF
            PieceColor::Yellow => Color::srgb(1.0, 1.0, 0.0), // #FFFF00
            PieceColor::Orange => Color::srgb(1.0, 0.6, 0.2), // #FF9933
            PieceColor::Purple => Color::srgb(1.5, 0.00, 1.6), // #C71585
            PieceColor::Cyan => Color::srgb(0.0, 1.0, 1.0),   // #00FFFF
            PieceColor::Red => Color::srgb(1.0, 0.06, 0.24),  // #FF103C
        }
    }
}

impl Distribution<PieceColor> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceColor {
        match rng.random_range(0..=7) {
            0 => PieceColor::Pink,
            1 => PieceColor::Green,
            2 => PieceColor::Blue,
            3 => PieceColor::Yellow,
            4 => PieceColor::Orange,
            5 => PieceColor::Purple,
            6 => PieceColor::Cyan,
            7 => PieceColor::Red,
            _ => PieceColor::Red,
        }
    }
}

// This is a list of valid shapes for game pieces
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum PieceShape {
    Circle,
    Square,
    Triangle,
    X,
    Plus,
    Diamond,
    Bomb,
}

impl Distribution<PieceShape> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceShape {
        match rng.random_range(0..=5) {
            0 => PieceShape::Circle,
            1 => PieceShape::Square,
            2 => PieceShape::Triangle,
            3 => PieceShape::X,
            4 => PieceShape::Plus,
            5 => PieceShape::Diamond,
            _ => PieceShape::Diamond,
        }
    }
}

// This is a list of valid notes for game pieces
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum PieceSound {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Distribution<PieceSound> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceSound {
        match rng.random_range(0..=5) {
            0 => PieceSound::A,
            1 => PieceSound::B,
            2 => PieceSound::C,
            3 => PieceSound::D,
            4 => PieceSound::E,
            5 => PieceSound::F,
            _ => PieceSound::G,
        }
    }
}

///A game piece containing its iproperties
#[derive(Debug, Clone, Copy)]
pub struct GamePiece {
    pub color: PieceColor,
    pub shape: PieceShape,
    pub sound: PieceSound,
    id: Uuid,
}

impl GamePiece {
    pub fn new(color: PieceColor, shape: PieceShape, sound: PieceSound) -> Self {
        GamePiece {
            color,
            shape,
            sound,
            id: Uuid::new_v4(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    fn random() -> Self {
        let mut rng = rand::rng();
        GamePiece {
            color: rng.random(),
            shape: rng.random(),
            sound: rng.random(),
            id: Uuid::new_v4(),
        }
    }

    fn reroll(&mut self) {
        let mut rng = rand::rng();
        self.color = rng.random();
        self.shape = rng.random();
    }

    fn compare(&self, other: Option<GamePiece>) -> usize {
        let mut similarity = 0;
        if let Some(op) = other {
            if self.shape == op.shape {
                similarity += 1;
            }

            if self.sound == op.sound {
                similarity += 1;
            }

            if self.color == op.color {
                similarity += 1;
            }
        }

        similarity
    }
}

impl Default for GamePiece {
    fn default() -> Self {
        GamePiece::random()
    }
}

// GameGrid holds the individual game pieces
#[derive(Resource, Debug, Clone)]
pub struct GameGrid {
    pub cells: Grid<Option<GamePiece>>,
}

impl Default for GameGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl GameGrid {
    pub fn new() -> Self {
        fn initialize_positions(gg: &mut GameGrid) {
            for cell in gg.cells.iter_mut() {
                let mut rng = rand::rng();

                if cell.is_none() {
                    *cell = Some(GamePiece::new(rng.random(), rng.random(), rng.random()));
                }
            }
        }

        let mut grid = GameGrid {
            cells: Grid::new(GRID_HEIGHT, GRID_WIDTH),
        };
        initialize_positions(&mut grid);
        grid
    }

    pub fn get_position(&self, source: &GamePiece) -> Vec2 {
        let mut retpos: Vec2 = Vec2::default();
        for (i, target) in self.cells.indexed_iter() {
            if let Some(piece) = target
                && piece.id() == source.id()
            {
                retpos = Vec2::new(i.0 as f32, i.1 as f32);
            }
        }
        retpos
    }
    pub fn get_piece(&self, position: Vec2) -> &Option<GamePiece> {
        if let Some(piece) = self.cells.get(position.x as usize, position.y as usize) {
            piece
        } else {
            &None
        }
    }

    pub fn get_mut_piece(&mut self, position: Vec2) -> Option<&mut GamePiece> {
        let x = position.x as usize;
        let y = position.y as usize;

        if x >= self.cells.cols() || y >= self.cells.rows() {
            return None;
        }

        self.cells.get_mut(y, x)?.as_mut()
    }

    pub fn collapse_column(grid: Grid<Option<GamePiece>>, col: usize) -> Vec<Option<GamePiece>> {
        let mut col_vec: Vec<Option<GamePiece>> = vec![];
        for cell in grid.iter_row(col) {
            if cell.is_some() {
                col_vec.push(*cell);
            }
        }

        let diff = grid.cols() - col_vec.len();

        for _i in 0..diff {
            let mut rng = rand::rng();
            let piece = Some(GamePiece::new(rng.random(), rng.random(), rng.random()));
            col_vec.push(piece);
        }
        col_vec
    }

    pub fn pop_cell(&mut self, target: Vec2) {
        let row = target.y as usize;
        let col = target.x as usize;

        if let Some(cell) = self.cells.get_mut(row, col) {
            *cell = None
        }
    }

    pub fn check_neighbors(&self, target: Vec2, cell_line: CellLine, grid: &GameGrid) -> bool {
        let mut neighbors = vec![];
        //Top Left
        if let Some(tl) = self.get_piece(target + Vec2::new(-1.0, 1.0)) {
            neighbors.push(tl)
        }
        //Top Mid
        if let Some(tl) = self.get_piece(target + Vec2::new(0.0, 1.0)) {
            neighbors.push(tl)
        }
        //Top Right
        if let Some(tl) = self.get_piece(target + Vec2::new(1.0, 1.0)) {
            neighbors.push(tl)
        }
        //Left
        if let Some(tl) = self.get_piece(target + Vec2::new(-1.0, 0.0)) {
            neighbors.push(tl)
        }
        //Right
        if let Some(tl) = self.get_piece(target + Vec2::new(1.0, 0.0)) {
            neighbors.push(tl)
        }
        //Bottom Left
        if let Some(tl) = self.get_piece(target + Vec2::new(-1.0, -1.0)) {
            neighbors.push(tl)
        }
        //Bottom Mid
        if let Some(tl) = self.get_piece(target + Vec2::new(0.0, -1.0)) {
            neighbors.push(tl)
        }
        //Bottom Right
        if let Some(tl) = self.get_piece(target + Vec2::new(1.0, -1.0)) {
            neighbors.push(tl)
        }

        let mut retval = false;
        for neighbor in neighbors {
            if cell_line.validate(neighbor, self.get_piece(target).as_ref().unwrap(), grid) {
                retval = true
            }
        }

        retval
    }
}
