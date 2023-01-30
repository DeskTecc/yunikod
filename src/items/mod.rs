pub mod onyxheart;
pub mod dragonsoul;
pub mod wood;
pub mod stick;
pub mod stone;
pub mod grass;
pub mod coal;
pub mod gold;
pub mod iron;
pub mod pickaxe;
pub mod axe;
pub mod hand;

use crate::entities::{EntityKind, Direction};
use dragonsoul::DragonSoul;
use onyxheart::OnyxHeart;
use tui::text::Span;
use wood::Wood;

use self::{gold::Gold, iron::Iron, coal::Coal, grass::Grass, stick::Stick, stone::Stone, pickaxe::Pickaxe, axe::Axe, hand::Hand};

pub enum ItemKind {
    OH(OnyxHeart),
    DS(DragonSoul),
    Wood(Wood),
    Gold(Gold),
    Iron(Iron),
    Stone(Stone),
    Coal(Coal),
    Grass(Grass),
    Stick(Stick),
    Pickaxe(Pickaxe),
    Axe(Axe),
    Hand(Hand)
}

impl ItemKind {
    pub fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind> {
        match self {
            ItemKind::OH(i) => i.utilize(coords),
            ItemKind::DS(i) => i.utilize(coords),
            ItemKind::Stick(i) => i.utilize(coords),
            ItemKind::Stone(i) => i.utilize(coords),
            ItemKind::Gold(i) => i.utilize(coords),
            ItemKind::Grass(i) => i.utilize(coords),
            ItemKind::Wood(i) => i.utilize(coords),
            ItemKind::Iron(i) => i.utilize(coords),
            ItemKind::Coal(i) => i.utilize(coords),
            ItemKind::Pickaxe(i) => i.utilize(coords),
            ItemKind::Axe(i) => i.utilize(coords),
            ItemKind::Hand(i) => i.utilize(coords),
        }
    }

    pub fn shape<'a>(&self) -> Span<'a> {
        match self {
            ItemKind::OH(_) => OnyxHeart::shape(),
            ItemKind::DS(_) => DragonSoul::shape(),
            ItemKind::Stick(_) => Stick::shape(),
            ItemKind::Stone(_) => Stone::shape(),
            ItemKind::Gold(_) => Gold::shape(),
            ItemKind::Grass(_) => Grass::shape(),
            ItemKind::Wood(_) => Wood::shape(),
            ItemKind::Iron(_) => Iron::shape(),
            ItemKind::Coal(_) => Coal::shape(),
            ItemKind::Pickaxe(_) => Pickaxe::shape(),
            ItemKind::Axe(_) => Axe::shape(),
            ItemKind::Hand(_) => Hand::shape(),
        }
    }

    pub fn name<'a>(&self) -> &str {
        match self {
            ItemKind::OH(_) => OnyxHeart::name(),
            ItemKind::DS(_) => DragonSoul::name(),
            ItemKind::Stick(_) => Stick::name(),
            ItemKind::Stone(_) => Stone::name(),
            ItemKind::Gold(_) => Gold::name(),
            ItemKind::Grass(_) => Grass::name(),
            ItemKind::Wood(_) => Wood::name(),
            ItemKind::Iron(_) => Iron::name(),
            ItemKind::Coal(_) => Coal::name(),
            ItemKind::Pickaxe(_) => Pickaxe::name(),
            ItemKind::Axe(_) => Axe::name(),
            ItemKind::Hand(_) => Hand::name(),
        }
    }

    pub fn quantity(&self) -> i8 {
        match self {
            ItemKind::OH(i) => i.quantity(),
            ItemKind::DS(i) => i.quantity(),
            ItemKind::Stick(i) => i.quantity(),
            ItemKind::Stone(i) => i.quantity(),
            ItemKind::Gold(i) => i.quantity(),
            ItemKind::Grass(i) => i.quantity(),
            ItemKind::Wood(i) => i.quantity(),
            ItemKind::Iron(i) => i.quantity(),
            ItemKind::Coal(i) => i.quantity(),
            ItemKind::Pickaxe(i) => i.quantity(),
            ItemKind::Axe(i) => i.quantity(),
            ItemKind::Hand(i) => i.quantity(),
        }
    }
    pub fn change_quantity(&mut self, amount: i8) -> i8 {
        match self {
            ItemKind::OH(i) => i.change_quantity(amount),
            ItemKind::DS(i) => i.change_quantity(amount),
            ItemKind::Stick(i) => i.change_quantity(amount),
            ItemKind::Stone(i) => i.change_quantity(amount),
            ItemKind::Gold(i) => i.change_quantity(amount),
            ItemKind::Grass(i) => i.change_quantity(amount),
            ItemKind::Wood(i) => i.change_quantity(amount),
            ItemKind::Iron(i) => i.change_quantity(amount),
            ItemKind::Coal(i) => i.change_quantity(amount),
            ItemKind::Pickaxe(i) => i.change_quantity(amount),
            ItemKind::Axe(i) => i.change_quantity(amount),
            ItemKind::Hand(i) => i.change_quantity(amount),
        }
    }
}

pub trait Item {
    fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind>;
    fn shape<'a>() -> Span<'a>;
    fn name<'a>() -> &'a str;
    fn quantity(&self) -> i8;
    fn max_quantity(&self) -> i8;
    fn change_quantity(&mut self, amount: i8) -> i8;
}
