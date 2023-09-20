//! Configuring image filtering.

/// How textures are filtered when rendered at other than their native size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterQuality {
    /// Low quality (i.e. nearest neighbor, blocky, pixel art)
    Low,
    /// Medium quality (i.e. bilinear)
    Medium,
    /// High quality (i.e. trilinear if available)
    High,
}

/// How textures are filtered when rendered at other than their native size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageFiltering {
    /// The filtering method to use when an image is scaled down.
    ///
    /// By default, it is 'Linear'.
    pub min_filter: FilterQuality,

    /// The filtering method to use when an image is scaled up.
    ///
    /// By default, it is 'Linear'.
    pub mag_filter: FilterQuality,

    /// The filtering method to use for mipmap generation.
    ///
    /// Only used for wgpu currently.
    ///
    /// By default, it is 'Linear'.
    pub mipmap_filter: FilterQuality,
}

impl Default for ImageFiltering {
    fn default() -> Self {
        Self {
            min_filter: FilterQuality::Medium,
            mag_filter: FilterQuality::Medium,
            mipmap_filter: FilterQuality::Medium,
        }
    }
}
