use std::{fs, io};
use std::path::{Path, PathBuf};

/// 📥️ Records actual build-producer filesystem inputs independently of rustc dep-info.
#[derive(Default)]
pub struct ResourceObservationsV1 {
    rows: Vec<String>,
}

fn quoted(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            character if character < '\u{20}' => output.push_str(&format!("\\u{:04x}", character as u32)),
            character => output.push(character),
        }
    }
    output.push('"');
    output
}

fn path_text(path: &Path) -> String {
    quoted(path.to_str().expect("UTF-8 build resource path"))
}

impl ResourceObservationsV1 {
    pub fn read_dir(&mut self, path: &Path) -> io::Result<Vec<fs::DirEntry>> {
        let source = PathBuf::from(file!()); let source = if source.is_absolute() { source } else { std::env::current_dir()?.join(source) };
        let operation = format!("{{\"source\":{},\"line\":{},\"name\":\"read_dir\"}}", path_text(&source), line!()); let listing = fs::read_dir(path).and_then(|entries| entries.collect::<io::Result<Vec<_>>>());
        let entries = match listing { Ok(entries) => entries, Err(error) => { self.failed(path, &operation); return Err(error); } };
        let metadata = entries.iter().map(|entry| { let kind = entry.file_type()?; let name = if kind.is_symlink() { "symlink" } else if kind.is_dir() { "directory" } else if kind.is_file() { "file" } else { "other" }; let target = if kind.is_symlink() { path_text(&fs::read_link(entry.path())?) } else { "null".to_owned() }; Ok((entry.file_name(), format!("{{\"path\":{},\"kind\":{},\"symlinkTarget\":{}}}", path_text(&entry.path()), quoted(name), target))) }).collect::<io::Result<Vec<_>>>();
        let mut metadata = match metadata { Ok(metadata) => metadata, Err(error) => { self.failed(path, &operation); return Err(error); } };
        metadata.sort_by(|a, b| a.0.to_str().expect("UTF-8 build entry").as_bytes().cmp(b.0.to_str().expect("UTF-8 build entry").as_bytes()));
        self.rows.push(format!("{{\"kind\":\"directory\",\"path\":{},\"entries\":[{}],\"operation\":{}}}", path_text(path), metadata.into_iter().map(|(_, row)| row).collect::<Vec<_>>().join(","), operation));
        Ok(entries)
    }

    fn failed(&mut self, path: &Path, operation: &str) {
        self.rows.push(format!("{{\"kind\":\"failed\",\"path\":{},\"operation\":{}}}", path_text(path), operation));
    }

    pub fn copied(&mut self, source: &Path, output: &Path) {
        self.rows.push(format!("{{\"kind\":\"copy\",\"path\":{},\"output\":{}}}", path_text(source), path_text(output)));
    }

    pub fn read(&mut self, source: &Path, out_dir: &Path) -> io::Result<Vec<u8>> {
        let owner = PathBuf::from(file!()); let owner = if owner.is_absolute() { owner } else { std::env::current_dir()?.join(owner) };
        let operation = format!("{{\"source\":{},\"line\":{},\"name\":\"read\"}}", path_text(&owner), line!()); let bytes = match fs::read(source) { Ok(bytes) => bytes, Err(error) => { self.failed(source, &operation); return Err(error); } };
        let output = out_dir.join(format!("semio-runtime-resource-read-{}.bin", self.rows.len()));
        fs::write(&output, &bytes).expect("record actual build resource read");
        self.rows.push(format!("{{\"kind\":\"read\",\"path\":{},\"output\":{},\"operation\":{}}}", path_text(source), path_text(&output), operation));
        Ok(bytes)
    }

    pub fn finish(&self, out_dir: &Path) {
        fs::write(out_dir.join("semio-runtime-resource-inputs.jsonl"), self.rows.join("\n") + "\n").expect("record build resource observations");
    }
}
