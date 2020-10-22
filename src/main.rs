extern crate freetype;

#[no_mangle]
pub extern fn FT_Get_MM_Var() -> ! {
    unimplemented!();
}

#[no_mangle]
pub extern fn FT_Done_MM_Var() -> ! {
    unimplemented!();
}

#[no_mangle]
pub extern fn FT_Get_Var_Blend_Coordinates() -> ! {
    unimplemented!();
}

#[no_mangle]
pub extern fn FT_Set_Var_Blend_Coordinates() -> ! {
    unimplemented!();
}

fn main() {
    use freetype::Library;

    // Init the library
    let lib = Library::init().unwrap();

    // Load a font face
    let face = lib.new_memory_face(
        std::fs::read("font.ttf").unwrap(), 0).unwrap();
    
    // Set the font size
    face.set_char_size(40 * 64, 0, 50, 0).unwrap();

    // Load a character
    face.load_char('A' as usize, freetype::face::LoadFlag::RENDER).unwrap();

    // Get the glyph instance
    let glyph = face.glyph();
}

