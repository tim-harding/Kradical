mod decompositions;
mod memberships;

pub use decompositions::*;
pub use memberships::*;

/// The constituent radicals for a kanji
pub struct Decomposition {
    /// The kanji
    kanji: &'static str,

    /// The radicals contained in the kanji
    radicals: &'static [&'static str],
}

/// The kanji that contain a radical
pub struct Membership {
    /// The radical
    radical: &'static str,

    /// The kanjis that contain the radical
    kanji: &'static [&'static str],

    /// The number of strokes to draw the radical
    strokes: u8,

    /// Alternate representations for the radical
    alternate: Alternate,
}

/// Alternate representations for a radical
pub enum Alternate {
    /// Another glyph that better depicts the radical
    Glyph(&'static str),

    /// The name of an image that the WWWJDIC server uses in place of the glyph
    Image(&'static str),

    /// No alternate representation
    None,
}
