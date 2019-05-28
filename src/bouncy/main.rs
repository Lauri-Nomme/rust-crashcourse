use std::fmt::{Display, Formatter, Error};
use std::thread::sleep;

#[derive(Debug, Copy, Clone)]
enum VertDir {
    Up,
    Down,
}

#[derive(Debug, Copy, Clone)]
enum HorizDir {
    Left,
    Right,
}

struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Frame {
    width: u32,
    height: u32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Game {
        Game {
            frame: Frame {
                width: 60,
                height: 30,
            },
            ball: Ball {
                x: 2,
                y: 4,
                vert_dir: VertDir::Up,
                horiz_dir: HorizDir::Left,
            },
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut top_bottom = |f: &mut Formatter| {
            write!(f, "+");
            for _ in 0..self.frame.width {
                write!(f, "-");
            }
            write!(f, "+\n")
        };

        top_bottom(f);

        for row in 0..self.frame.height {
            write!(f, "|");
            for column in 0..self.frame.width {
                write!(f, "{}", if self.ball.x == column && self.ball.y == row { 'o' } else { ' ' });
            }
            write!(f, "|\n");
        }

        top_bottom(f)
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        self.horiz_dir = if self.x == 0 {
            HorizDir::Right
        } else if self.x == frame.width - 1 {
            HorizDir::Left
        } else {
            self.horiz_dir.clone()
        };

        self.vert_dir = if self.y == 0 {
            VertDir::Down
        } else if self.y == frame.height - 1 {
            VertDir::Up
        } else {
            self.vert_dir.clone()
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        };

        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

fn main() {
    let mut game = Game::new();
    let sleep_duration = std::time::Duration::from_millis(33);

    loop {
        println!("{}", game);
        game.step();
        sleep(sleep_duration);
    }
}