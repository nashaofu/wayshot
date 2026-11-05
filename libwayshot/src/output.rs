use std::fmt::Display;

use wayland_client::protocol::{wl_output, wl_output::WlOutput};

use crate::region::{LogicalRegion, Size};

/// Represents an accessible wayland output.
///
/// Do not instantiate, instead use [`crate::WayshotConnection::get_all_outputs`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutputInfo {
    pub wl_output: WlOutput,
    pub name: String,
    pub description: String,
    pub transform: wl_output::Transform,
    pub physical_size: Size,
    pub logical_region: LogicalRegion,
}

impl Display for OutputInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{name} ({description})",
            name = self.name,
            description = self.description
        )
    }
}

impl OutputInfo {
    pub(crate) fn rotated_physical_size(&self) -> Size {
        match self.transform {
            wl_output::Transform::_90
            | wl_output::Transform::_270
            | wl_output::Transform::Flipped90
            | wl_output::Transform::Flipped270 => Size {
                width: self.physical_size.height,
                height: self.physical_size.width,
            },
            _ => self.physical_size,
        }
    }

    pub(crate) fn scale(&self) -> f64 {
        if self.logical_region.inner.size.height == 0 {
            return 1.0;
        }
        self.rotated_physical_size().height as f64 / self.logical_region.inner.size.height as f64
    }
}
