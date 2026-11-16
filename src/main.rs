use libpng::{LibpngManager, PngImage, ColorType, PngChunk, ChunkType};

fn main() {
    let mut mgr = LibpngManager::new("libpng");
    
    let image = PngImage {
        id: "sample.png".to_string(),
        width: 800,
        height: 600,
        bit_depth: 8,
        color_type: ColorType::RGBA,
        interlaced: false,
        chunks: vec![
            PngChunk {
                chunk_type: ChunkType::IHDR,
                data: vec![],
                crc: 0,
            },
        ],
        data: vec![],
    };
    
    if let Err(e) = mgr.add_image(image) {
        eprintln!("Failed to add image: {}", e);
        return;
    }
    
    println!("SigmaOS-libpng operational: {} images", mgr.count());
}
