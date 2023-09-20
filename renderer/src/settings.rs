use crate::core::{Font, Pixels};
use crate::graphics::{Antialiasing, ImageFiltering};

/// The settings of a Backend.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// The default [`Font`] to use.
    pub default_font: Font,

    /// The default size of text.
    ///
    /// By default, it will be set to `16.0`.
    pub default_text_size: Pixels,

    /// The antialiasing strategy that will be used for triangle primitives.
    ///
    /// By default, it is `None`.
    pub antialiasing: Option<Antialiasing>,

    /// The filtering method to use when an image is presented at its non-native size.
    pub image_filtering: ImageFiltering,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            default_font: Font::default(),
            default_text_size: Pixels(16.0),
            antialiasing: None,
            image_filtering: ImageFiltering::default(),
        }
    }
}
