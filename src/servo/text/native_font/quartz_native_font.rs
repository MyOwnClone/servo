export QuartzNativeFont, with_test_native_font;

import glyph::GlyphIndex;

class QuartzNativeFont/& {
    let bogus: int;

    new() { self.bogus = 0; }

    fn glyph_index(_codepoint: char) -> option<GlyphIndex> {
        fail;
    }

    // FIXME: What unit is this returning? Let's have a custom type
    fn glyph_h_advance(_glyph: GlyphIndex) -> option<int> {
        fail;
    }
}

#[cfg(target_os = "linux")]
fn with_test_native_font(f: fn@(nf: &NativeFont)) {
    fail
}