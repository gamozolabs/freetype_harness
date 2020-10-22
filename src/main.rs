use std::io;
use freetype::Library;
use freetype::face::LoadFlag;
use freetype::error::Error;
use freetype::freetype_sys::*;

/// Fuzz a font file with `contents`, returns the number of glyphs which
/// successfully parsed from it
fn fuzz(contents: &'static mut [u8]) -> Result<usize, Error> {
    // Count the number of successfully parsed glyphs
    let mut parsed_glyphs = 0usize;

    // Init the library
    let lib = Library::init()?;

    // Load a font face
    let as_vec = unsafe {
        Vec::from_raw_parts(contents.as_mut_ptr(),
            contents.len(), contents.len())
    };
    let mut face = lib.new_memory_face(as_vec, 0)?;

    // Get the number of fixed bitmap font sizes contained in this face
    let num_fixed_sizes = (*face.raw()).num_fixed_sizes;
    if num_fixed_sizes > 0 {
        // There's a fixed bitmap size, select the first one
    
        // Select the bitmap font size to use by index, in this case, we pick
        // the first available fixed bitmap size by using `0`. This is an index
        // into `face.available_sizes`
        let err: Error = unsafe { FT_Select_Size(face.raw_mut(), 0).into() };
        if err != Error::Ok { return Err(err); }
    } else {
        // Doesn't seem to be a bitmap font, just pick a size
        face.set_char_size(40 * 64, 0, 50, 0)?;
    }

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
            parsed_glyphs +=
                face.load_glyph(glyph_index, LoadFlag::DEFAULT)
                .is_ok() as usize;

            // Get the next character code and glyph index
            char_code =
                FT_Get_Next_Char(face.raw_mut(), char_code, &mut glyph_index);
        }
    }

    Ok(parsed_glyphs)
}

fn main() -> io::Result<()> {
    for filename in std::fs::read_dir("inputs")? {
        let filename = filename?.path();
        
        let status = fuzz(std::fs::read(&filename).unwrap().leak());
        print!("Status {:?} | {:?}\n", status, filename);
    }

    Ok(())
}

