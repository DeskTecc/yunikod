use tui::{    
    style::{Color, Style},
    text::Span, widgets::canvas::Context,
};
use crate::entities::{Direction, Entity};

pub struct Bullet {
    x: f64,
    y: f64,
    looking: Direction,
}

impl<'a> Bullet {
    pub fn new(x: f64, y: f64, direction: Direction) -> Self {
        Bullet {
            x,
            y,
            looking: direction
        }
    }
}

impl<'a> Entity<'a> for Bullet {
    fn shape(&self) -> Span<'a> {
        Span::styled("*", Style::default().fg(Color::Yellow))
    }

    fn draw<'b>(&'a self, ctx: &mut Context<'b>) {
        ctx.print(self.x, self.y, self.shape())
    }

    fn on_tick(&mut self) {
        match self.looking {
            Direction::Up => self.y += 1.0,
            Direction::Down => self.y -= 1.0,
            Direction::Left => self.x -= 1.0,
            Direction::Right => self.x += 1.0,
        }
    }

    fn looking(&mut self) -> Direction {
        self.looking.to_owned()
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}
