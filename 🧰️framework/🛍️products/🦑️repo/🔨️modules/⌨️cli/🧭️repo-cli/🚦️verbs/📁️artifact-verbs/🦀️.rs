//! 📁️ Artifact verbs: folders, files, sections, move, integrate, extract, rename and sync.

use super::{flag_or_arg, print_lines, resolve_section_name};
use semio_framework_repo_cli_command_framework::{CliError, Invocation};
use semio_framework_repo_graphql::RepoContext;
use serde_json::json;

fn path_mutation(invocation: &mut Invocation<'_>, query: &str) -> Result<(), CliError> {
    let path = flag_or_arg(invocation, "path", 0);
    if path.is_empty() {
        return Err("missing path".into());
    }
    invocation.graphql(query, json!({ "path": path }))
}

fn move_mutation(invocation: &mut Invocation<'_>, query: &str) -> Result<(), CliError> {
    let source = flag_or_arg(invocation, "source", 0);
    let target = flag_or_arg(invocation, "target", 1);
    if source.is_empty() || target.is_empty() {
        return Err("missing source or target".into());
    }
    invocation.graphql(query, json!({ "src": source, "dst": target }))
}

pub fn folder_create(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    path_mutation(invocation, "mutation FolderCreate($path: String!) { folderCreate(path: $path) { id path name uri } }")
}

pub fn folder_move(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    move_mutation(invocation, "mutation FolderMove($src: String!, $dst: String!) { folderMove(src: $src, dst: $dst) { id path name uri } }")
}

pub fn folder_delete(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    path_mutation(invocation, "mutation FolderDelete($path: String!) { folderDelete(path: $path) }")
}

pub fn folder_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    invocation.graphql("query Folders { repo { folders { id path name kind } } }", json!({}))
}

pub fn file_create(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    path_mutation(invocation, "mutation FileCreate($path: String!) { fileCreate(path: $path) { id path name uri } }")
}

pub fn file_move(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    move_mutation(invocation, "mutation FileMove($src: String!, $dst: String!) { fileMove(src: $src, dst: $dst) { id path name uri } }")
}

pub fn file_delete(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    path_mutation(invocation, "mutation FileDelete($path: String!) { fileDelete(path: $path) }")
}

pub fn file_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    invocation.graphql("query Files { repo { files { id path name kind extension } } }", json!({}))
}

pub fn section_create(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let file = flag_or_arg(invocation, "file", 0);
    let name = flag_or_arg(invocation, "name", 1);
    let parent = flag_or_arg(invocation, "parent", 2);
    if file.is_empty() || name.is_empty() {
        return Err("missing file or name".into());
    }
    let mut variables = json!({ "file": file, "name": name });
    if !parent.is_empty() {
        variables["parent"] = json!(parent);
    }
    invocation.graphql("mutation SectionCreate($file: String!, $name: String!, $parent: String) { sectionCreate(file: $file, name: $name, parent: $parent) { id name range { start end } } }", variables)
}

pub fn section_move(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let file = flag_or_arg(invocation, "file", 0);
    let old = flag_or_arg(invocation, "old", 1);
    let new = flag_or_arg(invocation, "new", 2);
    if file.is_empty() || old.is_empty() || new.is_empty() {
        return Err("missing file or names".into());
    }
    invocation.graphql("mutation SectionMove($file: String!, $oldName: String!, $newName: String!) { sectionMove(file: $file, oldName: $oldName, newName: $newName) { id name range { start end } } }", json!({ "file": file, "oldName": old, "newName": new }))
}

pub fn section_delete(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let file = flag_or_arg(invocation, "file", 0);
    let name = flag_or_arg(invocation, "name", 1);
    if file.is_empty() || name.is_empty() {
        return Err("missing file or name".into());
    }
    invocation.graphql("mutation SectionDelete($file: String!, $name: String!) { sectionDelete(file: $file, name: $name) }", json!({ "file": file, "name": name }))
}

pub fn section_integrate(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let source = flag_or_arg(invocation, "source", 0);
    let target_section = flag_or_arg(invocation, "target-section", 1);
    let target_file = flag_or_arg(invocation, "target-file", 2);
    let target_parent = flag_or_arg(invocation, "target-parent", 3);
    if source.is_empty() || target_section.is_empty() || target_file.is_empty() {
        return Err("missing source, target section, or target file".into());
    }
    let mut variables = json!({ "source": source, "targetSection": target_section, "targetFile": target_file });
    if !target_parent.is_empty() {
        variables["targetParent"] = json!(target_parent);
    }
    invocation.graphql("mutation Integrate($source: String!, $targetSection: String!, $targetFile: String!, $targetParent: String) { integrate(source: $source, targetSection: $targetSection, targetFile: $targetFile, targetParent: $targetParent) { id } }", variables)
}

pub fn section_extract(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let source_file = flag_or_arg(invocation, "source-file", 0);
    let source_section = flag_or_arg(invocation, "source-section", 1);
    let target_file = flag_or_arg(invocation, "target-file", 2);
    if source_file.is_empty() || source_section.is_empty() || target_file.is_empty() {
        return Err("missing source file, source section, or target file".into());
    }
    invocation.graphql("mutation Extract($sourceFile: String!, $sourceSection: String!, $targetFile: String!) { extract(sourceFile: $sourceFile, sourceSection: $sourceSection, targetFile: $targetFile) { id } }", json!({ "sourceFile": source_file, "sourceSection": source_section, "targetFile": target_file }))
}

pub fn section_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let file = if invocation.args().is_empty() { invocation.text("file") } else { invocation.arg(0) };
    invocation.graphql("query Sections($path: String!) { file(path: $path) { sections { id path name range { start end } definitions { name } children { name } } } }", json!({ "path": file }))
}

/// 🚚️ Moves one artifact onto another by the kinds its two references name.
pub fn move_artifact(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let context = invocation.context();
    let root = context.root().to_path_buf();
    let source = semio_framework_repo_identity::parse_artifact_ref(&invocation.arg(0));
    let target = semio_framework_repo_identity::parse_artifact_ref(&invocation.arg(1));
    let lines = match (source.kind.as_str(), target.kind.as_str()) {
        ("file", "file") => {
            context.file_move(&source.path, &target.path).map_err(|error| CliError::Message(error.message))?;
            vec![format!("\n📄️Moved file: {} → {}", source.path, target.path)]
        }
        ("folder", "folder") => {
            context.folder_move(&source.path, &target.path).map_err(|error| CliError::Message(error.message))?;
            vec![format!("\n📁️Moved folder: {} → {}", source.path, target.path)]
        }
        ("section", "section") if source.path == target.path => {
            let (Some(old), Some(new)) = (source.section_parts.last(), target.section_parts.last()) else { return Err("missing section path".into()) };
            context.move_section(&source.path, &resolve_section_name(&root, &source.path, old), &resolve_section_name(&root, &target.path, new)).map_err(|error| CliError::Message(error.message))?
        }
        ("file", "section") => {
            let parts = &target.section_parts;
            let name = parts.last().map(|slug| resolve_section_name(&root, &target.path, slug)).unwrap_or_default();
            let parent = if parts.len() > 1 { resolve_section_name(&root, &target.path, &parts[parts.len() - 2]) } else { String::new() };
            let lines = context.integrate_file(&source.path, &name, &target.path, &parent).map_err(|error| CliError::Message(error.message))?;
            context.file_delete(&source.path).map_err(|error| CliError::Message(error.message))?;
            lines
        }
        ("section", "file") => {
            let Some(slug) = source.section_parts.last() else { return Err("missing section path".into()) };
            context.extract_section(&source.path, &resolve_section_name(&root, &source.path, slug), &target.path).map_err(|error| CliError::Message(error.message))?
        }
        (from, to) => return Err(CliError::Message(format!("unsupported move: {from} → {to}"))),
    };
    print_lines(invocation, lines);
    Ok(())
}

pub fn integrate(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let context = invocation.context();
    let root = context.root().to_path_buf();
    if invocation.args().len() == 2 {
        let source = semio_framework_repo_identity::parse_artifact_ref(&invocation.arg(0));
        let target = semio_framework_repo_identity::parse_artifact_ref(&invocation.arg(1));
        if source.kind == "file" && target.kind == "section" {
            let parts = &target.section_parts;
            let name = parts.last().map(|slug| resolve_section_name(&root, &target.path, slug)).unwrap_or_default();
            let parent = if parts.len() > 1 { resolve_section_name(&root, &target.path, &parts[parts.len() - 2]) } else { String::new() };
            let lines = context.integrate_file(&source.path, &name, &target.path, &parent).map_err(|error| CliError::Message(error.message))?;
            print_lines(invocation, lines);
            return Ok(());
        }
    }
    let file = flag_or_arg(invocation, "file", 0);
    let (target_file, target_section, parent) = (invocation.text("target-file"), invocation.text("target-section"), invocation.text("parent-section"));
    if file.is_empty() || target_file.is_empty() || target_section.is_empty() {
        return Err("missing file, target-file, or target-section".into());
    }
    let lines = context.integrate_file(&file, &target_section, &target_file, &parent).map_err(|error| CliError::Message(error.message))?;
    print_lines(invocation, lines);
    Ok(())
}

pub fn extract(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let context = invocation.context();
    let root = context.root().to_path_buf();
    if invocation.args().len() == 2 {
        let source = semio_framework_repo_identity::parse_artifact_ref(&invocation.arg(0));
        let target = semio_framework_repo_identity::parse_artifact_ref(&invocation.arg(1));
        if source.kind == "section" && target.kind == "file" {
            let Some(slug) = source.section_parts.last() else { return Err("missing section path".into()) };
            let lines = context.extract_section(&source.path, &resolve_section_name(&root, &source.path, slug), &target.path).map_err(|error| CliError::Message(error.message))?;
            print_lines(invocation, lines);
            return Ok(());
        }
    }
    let file = flag_or_arg(invocation, "file", 0);
    let (section, target_file) = (invocation.text("section"), invocation.text("target-file"));
    if file.is_empty() || section.is_empty() || target_file.is_empty() {
        return Err("missing file, section, or target-file".into());
    }
    let lines = context.extract_section(&file, &section, &target_file).map_err(|error| CliError::Message(error.message))?;
    print_lines(invocation, lines);
    Ok(())
}

pub fn rename(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let lines = invocation.context().rename_token(&invocation.arg(0), &invocation.arg(1), &invocation.arg(2)).map_err(|error| CliError::Message(error.message))?;
    print_lines(invocation, lines);
    Ok(())
}

pub fn sync(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    if !invocation.args().is_empty() {
        return Err(CliError::Message(format!("unknown sync target {:?}", invocation.arg(0))));
    }
    let help = invocation.parsed.selected().help();
    invocation.print(&help);
    Ok(())
}

pub fn sync_management(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    invocation.graphql("mutation SyncManagement { syncManagement }", json!({}))
}
