//! # Art
//!
//! A library for modeling artistic concepts.

// Export this as the crate
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
  /// The primary colors according to the RYB color model.
  pub enum PrimaryColor {
    Red,
    Yellow,
    Blue,
  }

  /// The secondary colors according to the RYB color model.
  #[derive(Debug, PartialEq)]
  pub enum SecondaryColor {
    Orange,
    Green,
    Purple,
  }
}

pub mod utils {
  use crate::kinds::*;

  /// Combines two primary colors in equal amounts to create
  /// a secondary color.
  ///
  /// ```
  /// use chapter_14_cargo_and_crates_io::kinds::PrimaryColor;
  /// use chapter_14_cargo_and_crates_io::kinds::SecondaryColor;
  /// assert_eq!(chapter_14_cargo_and_crates_io::utils::mix(PrimaryColor::Red, PrimaryColor::Yellow), SecondaryColor::Orange)
  /// ```
  pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
    // --snip--
    SecondaryColor::Orange
  }
}

fn main() {}
