//! 📚️ A cancellable command inventory published independently of the first interactive frame.

use crate::command_tree::CommandNode;
use std::path::{Path, PathBuf};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver}};

/// 📡️ A discovery result consumed by the UI without waiting for the worker.
pub enum Update { Ready(Vec<(String, crate::command_tree::CommandLeaf)>, String), Finished, Failed(String) }

/// ⏳️ One background discovery generation with explicit cancellation.
pub struct Job { pub receiver: Receiver<Update>, pub cancelled: Arc<AtomicBool>, pub started: std::time::Instant }

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Snapshot { version: u32, root: PathBuf, tree: CommandNode }

fn cache_path(root: &Path) -> PathBuf { root.join(".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/commands.json") }

fn cached(root: &Path) -> Option<CommandNode> {
    let file = std::fs::File::open(cache_path(root)).ok()?;
    if file.metadata().ok()?.len() > 16 * 1024 * 1024 { return None; }
    let snapshot: Snapshot = serde_json::from_reader(std::io::BufReader::new(file)).ok()?;
    (snapshot.version == 1 && snapshot.root == root).then_some(snapshot.tree)
}

/// 🚀️ Uses a cached catalog while discovering current workspace targets off the UI thread.
pub fn start(root: PathBuf, refresh: bool) -> Job {
    let (sender, receiver) = mpsc::channel();
    let cancelled = Arc::new(AtomicBool::new(false));
    let signal = cancelled.clone();
    std::thread::spawn(move || {
        let seed = crate::command_tree::seed(&root);
        if signal.load(Ordering::Relaxed) || sender.send(Update::Ready(commands(&seed), "quick commands".into())).is_err() { return; }
        if !refresh { if let Some(tree) = cached(&root) { if sender.send(Update::Ready(commands(&tree), "cached commands".into())).is_err() { return; } } }
        let tree = crate::command_tree::discover_cancellable(&root, &signal);
        if signal.load(Ordering::Relaxed) { let _ = sender.send(Update::Finished); return; }
        let commands = commands(&tree);
        let path = cache_path(&root);
        let result = (|| -> std::io::Result<()> {
            std::fs::create_dir_all(path.parent().expect("inventory directory"))?;
            let temporary = path.with_extension(format!("{}-{}.tmp", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos()));
            let result = (|| -> std::io::Result<()> {
                let mut file = std::fs::File::create(&temporary)?;
                serde_json::to_writer(&mut file, &Snapshot { version: 1, root: root.clone(), tree }).map_err(std::io::Error::other)?;
                file.sync_data()?;
                std::fs::rename(&temporary, &path)
            })();
            if result.is_err() { let _ = std::fs::remove_file(temporary); }
            result
        })();
        if let Err(error) = result { let _ = sender.send(Update::Failed(format!("inventory cache: {error}"))); }
        let _ = sender.send(Update::Ready(commands, "commands ready".into()));
        let _ = sender.send(Update::Finished);
    });
    Job { receiver, cancelled, started: std::time::Instant::now() }
}

/// 🔎️ A flat command launcher retaining renderer and example labels without preference questions.
pub fn commands(tree: &CommandNode) -> Vec<(String, crate::command_tree::CommandLeaf)> {
    fn visit(node: &CommandNode, labels: &mut Vec<String>, output: &mut Vec<(String, crate::command_tree::CommandLeaf)>) {
        if node.key != "root" { labels.push(node.label.clone()); }
        if let Some(leaf) = &node.leaf { output.push((labels.join(" / "), leaf.clone())); }
        for child in &node.children { visit(child, labels, output); }
        if node.key != "root" { labels.pop(); }
    }
    let mut output = Vec::new(); visit(tree, &mut Vec::new(), &mut output); output
}

impl Drop for Job { fn drop(&mut self) { self.cancelled.store(true, Ordering::Relaxed); } }

#[cfg(test)]
mod tests {
    #[test]
    fn command_launcher_preserves_labels_and_direct_leaf_selection() {
        let tree = crate::command_tree::seed(std::path::Path::new("."));
        let commands = super::commands(&tree);
        assert!(commands.iter().all(|(label, _)| !label.contains("Language / Sprache") && !label.contains("Terminology / Begriffe")));
    }
}
