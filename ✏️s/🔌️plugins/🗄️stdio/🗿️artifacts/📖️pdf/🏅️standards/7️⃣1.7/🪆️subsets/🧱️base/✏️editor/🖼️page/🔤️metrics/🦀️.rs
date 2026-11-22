//! 🔤️ Glyph advances from the PDF's declared font resources.

use super::{PdfFont, PdfSnapshot};
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts::NativePdfArtifactResources;
use semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts::FontCodec;
use std::collections::HashMap;

pub struct FontMeasurements<'a> {
    snapshot: &'a PdfSnapshot,
    resources: NativePdfArtifactResources<'a>,
    codecs: HashMap<String, Option<FontCodec>>,
}

impl<'a> FontMeasurements<'a> {
    pub fn new(snapshot: &'a PdfSnapshot) -> Self {
        Self { snapshot, resources: NativePdfArtifactResources::from_objects(&snapshot.objects), codecs: HashMap::new() }
    }

    pub fn codec(&mut self, name: &str) -> Option<&FontCodec> {
        if !self.codecs.contains_key(name) {
            let codec = self.snapshot.font(name).and_then(|font| self.font_codec(font));
            self.codecs.insert(name.to_owned(), codec);
        }
        self.codecs.get(name)?.as_ref()
    }

    pub fn font_codec(&self, font: &PdfFont) -> Option<FontCodec> {
        FontCodec::new(font, &self.resources).ok()
    }

    pub fn encoded_codes(&mut self, name: &str, text: &str) -> Option<Vec<u32>> {
        let codec = self.codec(name)?;
        let bytes = codec.encode(text)?;
        Some(codec.character_codes(&bytes).collect())
    }

    pub fn measure(&mut self, name: &str, text: Option<&str>, codes: &[u32]) -> Option<(String, f64, usize, usize)> {
        let codec = self.codec(name)?;
        let glyphs = if let Some(text) = text { codec.decode(&codec.encode(text)?) } else { codec.decode_codes(codes) };
        let width = glyphs.iter().map(|glyph| glyph.width).sum::<f64>() * codec.type3_matrix.map_or(0.001, |matrix| matrix[0]);
        let spaces = glyphs.iter().filter(|glyph| glyph.is_space_char).count();
        let count = glyphs.len();
        let text = if let Some(text) = text { text.to_owned() } else {
            glyphs.into_iter().fold(String::new(), |mut text, glyph| {
                if let Some(characters) = glyph.text { text.push_str(&characters); }
                else { text.push(char::from_u32(glyph.code).unwrap_or(char::REPLACEMENT_CHARACTER)); }
                text
            })
        };
        Some((text, width, count, spaces))
    }

}
