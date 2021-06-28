mod decompositions;
mod expansions;

pub use decompositions::*;
pub use expansions::*;

pub struct Decomposition {
    kanji: &'static str,
    radicals: &'static [&'static str],
}

pub struct Expansion {
    radical: &'static str,
    kanji: &'static [&'static str],
    strokes: u8,
    alternate: Alternate,
}

pub enum Alternate {
    Glyph(&'static str),
    Image(&'static str),
    None,
}
