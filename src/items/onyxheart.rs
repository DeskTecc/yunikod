use crate::{
    items::Item,
    entities::{
        EntityKind,
        Direction, onyxstone::OnyxStone,
    }
};
use tui::{text::Span, style::{Style, Color}};
use rust_i18n::t;
rust_i18n::i18n!("locales");

pub struct OnyxHeart {
    quantity: i8
}

impl Item for OnyxHeart {
    fn utilize(&self, coords: (i64, i64, Direction)) -> Option<EntityKind> {
        let (x, y, direction) = coords;
        Some(EntityKind::OnyxStone(OnyxStone::new(x, y, direction)))
    }

    fn shape<'a>() -> tui::text::Span<'a> {
        Span::styled(" ", Style::default().fg(Color::Red))
    }

    fn name(lang: String) -> String {
        rust_i18n::set_locale(&lang); //set language
        t!("game.res.onyx_heart")
    }

    fn damage(&self) -> u8 {
        1
    }

    fn quantity(&self) -> i8 {
        self.quantity
    }

    fn max_quantity(&self) -> i8 {
        1
    }

    fn change_quantity(&mut self, amount: i8) -> i8 {
        let prevision = self.quantity + amount;
        if prevision < 0 {
            self.quantity = 0;
            return prevision;
        } else if prevision > self.max_quantity() {
            self.quantity = self.max_quantity();
            return self.max_quantity() - prevision;
        } else {
            self.quantity = prevision;
            return 0;
        }
    }
}
