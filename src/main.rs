use freetype::Library;
use freetype::face::LoadFlag;
use freetype::error::Error;
use freetype::freetype_sys::*;

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
    // Init the library
    let lib = Library::init().unwrap();

    // Load a font face
    let mut face = lib.new_memory_face(
        std::fs::read("font.ttf").unwrap(), 0).unwrap();
            
    // Get the number of fixed bitmap font sizes contained in this face
    let num_fixed_sizes = (*face.raw()).num_fixed_sizes;
    assert!(num_fixed_sizes > 0);
            
    // Select the bitmap font size to use by index, in this case, we pick the
    // first available fixed bitmap size by using `0`
    // This is an index into `face.available_sizes`
    let err: Error = unsafe { FT_Select_Size(face.raw_mut(), 0).into() };
    assert!(err == Error::Ok, "FT_Select_Size() error : {}", err);

    // Enumerate all glyphs in the character map
    unsafe {
        // Initialize the glyph index to zero
        let mut glyph_index = 0;

        // Get the first character code and glyph index
        let mut char_code =
            FT_Get_First_Char(face.raw_mut(), &mut glyph_index);
        while glyph_index != 0 {
            // The character code is the actual character, like 0x41 for an
            // 'A', the glyph index is arbitrary.
            //
            // Internally, freetype2 uses FT_Load_Glyph() inside of
            // FT_Load_Char(), which just uses FT_Get_Char_Index() to perform
            // the conversion. Thus, we really only care about the glyph index
            // when it comes to actually enumerating glyphs
            //print!("Character {} | Glyph {}\n", char_code, glyph_index);

            // This is what we're fuzzing, discard the results
            let _ = face.load_glyph(glyph_index, LoadFlag::DEFAULT);

            // Get the next character code and glyph index
            char_code =
                FT_Get_Next_Char(face.raw_mut(), char_code, &mut glyph_index);
        }
    }
}

