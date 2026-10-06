//! 🌲️ Traverses unique physical directories with caller-owned matching and work control.
use std::collections::BTreeSet;
use std::io;

pub(crate) struct DiscoveryEntry<P> {
    pub(crate) value: P,
    pub(crate) directory: bool,
}

pub(crate) struct Discovery<P> {
    pub(crate) files: Vec<P>,
    pub(crate) directories: usize,
    pub(crate) cancelled: bool,
}

pub(crate) fn discover<P, K: Ord>(
    root: P,
    mut identity: impl FnMut(&P) -> io::Result<K>,
    mut entries: impl FnMut(&P) -> io::Result<Vec<DiscoveryEntry<P>>>,
    matches: impl Fn(&P) -> bool,
    mut control: impl FnMut(usize, &P) -> bool,
) -> io::Result<Discovery<P>> {
    let mut pending = vec![root];
    let mut visited = BTreeSet::new();
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        if !control(visited.len(), &directory) {
            return Ok(Discovery { files, directories: visited.len(), cancelled: true });
        }
        if !visited.insert(identity(&directory)?) {
            continue;
        }
        for entry in entries(&directory)? {
            if entry.directory {
                pending.push(entry.value);
            } else if matches(&entry.value) {
                files.push(entry.value);
            }
        }
    }
    Ok(Discovery { files, directories: visited.len(), cancelled: false })
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
