use serde::Deserialize;
use std::collections::BTreeSet;
use std::io::{self, Read};

/// 🧾️ A portable reference to one completed component and its verified descriptor.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct NativeRuntimeModule {
    pub plugin_id: String,
    pub wasm_sha256: String,
    pub wasm_path: String,
    pub descriptor_path: String,
}

/// 📋️ The native runtime's compact, profile-specific artifact manifest.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NativeRuntimeManifest {
    version: u32,
    variant: String,
    profile: String,
    pub modules: Vec<NativeRuntimeModule>,
}

impl NativeRuntimeManifest {
    /// 🔎️ Rejects ambiguous identities and nonportable paths before any component is loaded.
    pub fn read(reader: impl Read, variant: &str) -> Result<Self, String> {
        let mut reader = reader.take(1024 * 1024 + 1);
        let value: Self = serde_json::from_reader(&mut reader).map_err(|error| format!("Native runtime manifest: {error}"))?;
        if reader.limit() == 0 {
            return Err("Native runtime manifest exceeds 1 MiB".into());
        }
        if value.version != 1 || value.variant != variant || !matches!(value.profile.as_str(), "dev" | "release") || value.modules.is_empty() || value.modules.len() > 1024 {
            return Err("Native runtime manifest identity mismatch".into());
        }
        let mut ids = BTreeSet::new();
        for module in &value.modules {
            let valid_id = module.plugin_id.split('-').all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()));
            let valid_hash = module.wasm_sha256.len() == 64 && module.wasm_sha256.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
            let valid_paths = [&module.wasm_path, &module.descriptor_path].into_iter().all(|path| !path.is_empty() && path.chars().count() <= 4096 && !path.starts_with('/') && !path.chars().any(|character| character <= '\u{1f}' || "\\:<>\"|?*".contains(character)) && path.split('/').all(|part| !part.is_empty() && part != "."));
            if !valid_id || !valid_hash || !valid_paths || !ids.insert(&module.plugin_id) {
                return Err("Invalid native runtime module".into());
            }
        }
        Ok(value)
    }
}

/// 📄️ Supplies borrowed payload pages to a JSON reader without collecting another byte buffer.
pub(crate) struct NativeJsonPages<'a, I: Iterator<Item = &'a [u8]>> {
    pages: I,
    current: &'a [u8],
}

impl<'a, I: Iterator<Item = &'a [u8]>> NativeJsonPages<'a, I> {
    pub fn new(pages: I) -> Self {
        Self { pages, current: &[] }
    }
}

impl<'a, I: Iterator<Item = &'a [u8]>> Read for NativeJsonPages<'a, I> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if output.is_empty() {
            return Ok(0);
        }
        while self.current.is_empty() {
            match self.pages.next() {
                Some(page) => self.current = page,
                None => return Ok(0),
            }
        }
        let length = output.len().min(self.current.len());
        output[..length].copy_from_slice(&self.current[..length]);
        self.current = &self.current[length..];
        Ok(length)
    }
}
