use image::DynamicImage;
use wayland_client::protocol::wl_output::Transform;

use crate::region::Size;

#[tracing::instrument(skip(image))]
pub(crate) fn rotate_image_buffer(
    image: DynamicImage,
    transform: Transform,
    logical_size: Size,
    max_scale: f64,
) -> DynamicImage {
    let rotated_image = match transform {
        Transform::_90 => image::imageops::rotate90(&image).into(),
        Transform::_180 => image::imageops::rotate180(&image).into(),
        Transform::_270 => image::imageops::rotate270(&image).into(),
        Transform::Flipped => image::imageops::flip_horizontal(&image).into(),
        Transform::Flipped90 => {
            let flipped_buffer = image::imageops::flip_horizontal(&image);
            image::imageops::rotate90(&flipped_buffer).into()
        }
        Transform::Flipped180 => {
            let flipped_buffer = image::imageops::flip_horizontal(&image);
            image::imageops::rotate180(&flipped_buffer).into()
        }
        Transform::Flipped270 => {
            let flipped_buffer = image::imageops::flip_horizontal(&image);
            image::imageops::rotate270(&flipped_buffer).into()
        }
        _ => image,
    };

    if logical_size.width == 0 {
        return rotated_image;
    }

    let scale = rotated_image.width() as f64 / logical_size.width as f64;
    // The amount of scaling left to perform.
    let scaling_left = max_scale / scale;
    if scaling_left <= 1.0 {
        tracing::debug!("No scaling left to do");
        return rotated_image;
    }

    tracing::debug!("Scaling left to do: {scaling_left}");
    let new_width = (rotated_image.width() as f64 * scaling_left).round() as u32;
    let new_height = (rotated_image.height() as f64 * scaling_left).round() as u32;
    tracing::debug!("Resizing image to {new_width}x{new_height}");
    image::imageops::resize(
        &rotated_image,
        new_width,
        new_height,
        image::imageops::FilterType::Gaussian,
    )
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    #[test]
    fn test_rotate_90_scaling() {
        // Native 1920x1080 buffer, rotated 90 degrees
        let img = DynamicImage::ImageRgba8(RgbaImage::new(1920, 1080));
        // Logical size in Wayland has transform already applied (1080x1920)
        let logical_size = Size {
            width: 1080,
            height: 1920,
        };
        let rotated = rotate_image_buffer(img, Transform::_90, logical_size, 1.0);
        assert_eq!(rotated.width(), 1080);
        assert_eq!(rotated.height(), 1920);
    }

    #[test]
    fn test_rotate_270_scaling() {
        let img = DynamicImage::ImageRgba8(RgbaImage::new(1920, 1080));
        let logical_size = Size {
            width: 1080,
            height: 1920,
        };
        let rotated = rotate_image_buffer(img, Transform::_270, logical_size, 1.0);
        assert_eq!(rotated.width(), 1080);
        assert_eq!(rotated.height(), 1920);
    }

    #[test]
    fn test_rotate_180_scaling() {
        let img = DynamicImage::ImageRgba8(RgbaImage::new(1920, 1080));
        let logical_size = Size {
            width: 1920,
            height: 1080,
        };
        let rotated = rotate_image_buffer(img, Transform::_180, logical_size, 1.0);
        assert_eq!(rotated.width(), 1920);
        assert_eq!(rotated.height(), 1080);
    }
}
