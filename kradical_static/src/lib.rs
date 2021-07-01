//! Contains the contents of `kradfile`, `kradfile2`, `radkfile`, and `radkfile2`
//! in a format that can be easily `use`d and compiled into and Rust program.

mod decompositions;
mod memberships;

pub use decompositions::*;
pub use memberships::*;

/// The constituent radicals for a kanji
pub struct Decomposition {
    /// The kanji
    pub kanji: &'static str,

    /// The radicals contained in the kanji
    pub radicals: &'static [&'static str],
}

/// The kanji that contain a radical
pub struct Membership {
    /// The radical
    pub radical: &'static str,

    /// The kanjis that contain the radical
    pub kanji: &'static [&'static str],

    /// The number of strokes to draw the radical
    pub strokes: u8,
}
