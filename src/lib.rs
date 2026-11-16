//! SigmaOS-libpng: libpng PNG image format model.
//!
//! Models PNG images, chunks, color types, compression,
//! filtering, and metadata handling.

use std::collections::BTreeMap;
use std::fmt;

/// PNG color types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    Grayscale = 0,
    RGB = 2,
    Palette = 3,
    GrayscaleAlpha = 4,
    RGBA = 6,
}

impl ColorType {
    pub fn channels(self) -> u8 {
        match self {
            ColorType::Grayscale => 1,
            ColorType::RGB => 3,
            ColorType::Palette => 1,
            ColorType::GrayscaleAlpha => 2,
            ColorType::RGBA => 4,
        }
    }
}

/// PNG chunk types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkType {
    IHDR,  // Image header
    PLTE,  // Palette
    IDAT,  // Image data
    IEND,  // Image end
    TExt,  // Text
    ZTxt,  // Compressed text
    ITxt,  // International text
    Other,
}

/// A PNG chunk.
#[derive(Debug, Clone)]
pub struct PngChunk {
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
    pub crc: u32,
}

/// A PNG image.
#[derive(Debug, Clone)]
pub struct PngImage {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: ColorType,
    pub interlaced: bool,
    pub chunks: Vec<PngChunk>,
    pub data: Vec<u8>,
}

/// Errors for the libpng domain.
#[derive(Debug, PartialEq)]
pub enum LibpngError {
    EmptyId,
    InvalidDimensions,
    InvalidBitDepth,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for LibpngError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyId => write!(f, "image id cannot be empty"),
            Self::InvalidDimensions => write!(f, "width and height must be non-zero"),
            Self::InvalidBitDepth => write!(f, "bit depth must be 1, 2, 4, 8, or 16"),
            Self::Duplicate(id) => write!(f, "duplicate image id: {id}"),
            Self::NotFound(id) => write!(f, "image not found: {id}"),
        }
    }
}

impl std::error::Error for LibpngError {}

/// libpng manager.
pub struct LibpngManager {
    pub name: String,
    images: BTreeMap<String, PngImage>,
}

impl LibpngManager {
    /// Create a new manager.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            images: BTreeMap::new(),
        }
    }

    /// Add an image with validation.
    pub fn add_image(&mut self, image: PngImage) -> Result<(), LibpngError> {
        if image.id.is_empty() {
            return Err(LibpngError::EmptyId);
        }
        if image.width == 0 || image.height == 0 {
            return Err(LibpngError::InvalidDimensions);
        }
        if !matches!(image.bit_depth, 1 | 2 | 4 | 8 | 16) {
            return Err(LibpngError::InvalidBitDepth);
        }
        if self.images.contains_key(&image.id) {
            return Err(LibpngError::Duplicate(image.id));
        }
        self.images.insert(image.id.clone(), image);
        Ok(())
    }

    /// Retrieve an image by id.
    pub fn get_image(&self, id: &str) -> Option<&PngImage> {
        self.images.get(id)
    }

    /// Count of images.
    pub fn count(&self) -> usize {
        self.images.len()
    }

    /// Remove an image.
    pub fn remove_image(&mut self, id: &str) -> Result<(), LibpngError> {
        self.images
            .remove(id)
            .ok_or_else(|| LibpngError::NotFound(id.to_string()))
            .map(|_| ())
    }

    /// Clear all images.
    pub fn clear(&mut self) {
        self.images.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_image(id: &str, width: u32, height: u32) -> PngImage {
        PngImage {
            id: id.to_string(),
            width,
            height,
            bit_depth: 8,
            color_type: ColorType::RGBA,
            interlaced: false,
            chunks: vec![],
            data: vec![],
        }
    }

    #[test]
    fn test_create_manager() {
        let mgr = LibpngManager::new("libpng");
        assert_eq!(mgr.name, "libpng");
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_add_image() {
        let mut mgr = LibpngManager::new("libpng");
        let img = make_image("img1", 100, 100);
        assert!(mgr.add_image(img).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_id_rejected() {
        let mut mgr = LibpngManager::new("libpng");
        let img = make_image("", 100, 100);
        assert_eq!(mgr.add_image(img), Err(LibpngError::EmptyId));
    }

    #[test]
    fn test_invalid_dimensions_rejected() {
        let mut mgr = LibpngManager::new("libpng");
        let img = make_image("img1", 0, 100);
        assert_eq!(mgr.add_image(img), Err(LibpngError::InvalidDimensions));
    }

    #[test]
    fn test_invalid_bit_depth_rejected() {
        let mut mgr = LibpngManager::new("libpng");
        let mut img = make_image("img1", 100, 100);
        img.bit_depth = 7;
        assert_eq!(mgr.add_image(img), Err(LibpngError::InvalidBitDepth));
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut mgr = LibpngManager::new("libpng");
        let img = make_image("img1", 100, 100);
        mgr.add_image(img.clone()).unwrap();
        assert_eq!(mgr.add_image(img), Err(LibpngError::Duplicate("img1".to_string())));
    }

    #[test]
    fn test_get_image() {
        let mut mgr = LibpngManager::new("libpng");
        let img = make_image("img1", 100, 100);
        mgr.add_image(img).unwrap();
        assert!(mgr.get_image("img1").is_some());
        assert!(mgr.get_image("none").is_none());
    }

    #[test]
    fn test_remove_image() {
        let mut mgr = LibpngManager::new("libpng");
        let img = make_image("img1", 100, 100);
        mgr.add_image(img).unwrap();
        assert!(mgr.remove_image("img1").is_ok());
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.remove_image("none"), Err(LibpngError::NotFound("none".to_string())));
    }

    #[test]
    fn test_clear() {
        let mut mgr = LibpngManager::new("libpng");
        mgr.add_image(make_image("img1", 100, 100)).unwrap();
        mgr.clear();
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_color_type_channels() {
        assert_eq!(ColorType::RGB.channels(), 3);
        assert_eq!(ColorType::RGBA.channels(), 4);
        assert_eq!(ColorType::Grayscale.channels(), 1);
    }
}
