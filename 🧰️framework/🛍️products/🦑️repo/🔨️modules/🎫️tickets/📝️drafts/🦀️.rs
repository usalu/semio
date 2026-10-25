use super::*;

pub(super) struct DraftPlan { source: String, moves: Vec<(String, String)> }

fn bounded(path: &str) -> TicketResult<()> {
    if path.encode_utf16().count() > 256 || path.contains('\0') { return Err(TicketError::invalid("bounded draft path required")); }
    Ok(())
}

pub(super) fn segment(value: &str) -> bool {
    if value.is_empty() || value == "." || value == ".." || value.ends_with('.') || value.ends_with(' ') || value.chars().any(|value| value < '\u{20}' || value == '\u{7f}' || "/\\:<>\"|?*".contains(value)) { return false; }
    let stem = value.split('.').next().unwrap_or_default().to_ascii_uppercase();
    !matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL" | "COM¹" | "COM²" | "COM³" | "LPT¹" | "LPT²" | "LPT³") && !(stem.len() == 4 && (stem.starts_with("COM") || stem.starts_with("LPT")) && matches!(stem.as_bytes()[3], b'1'..=b'9'))
}

pub(super) fn owned_ticket_path(layout: &TicketLayout, id: &TicketId) -> TicketResult<String> {
    if !(0..=99).contains(&id.year) || !(1..=12).contains(&id.month) || !(1..=31).contains(&id.day) || id.slug.split('/').any(|value| !segment(value)) { return Err(TicketError::invalid("safe ticket identity required")); }
    let path = layout.ticket_dir(id); bounded(&path)?; bounded(&layout.important_path(id))?; Ok(path)
}

pub(super) fn existing_ancestry<S: TicketStore>(store: &S, path: &str) -> TicketResult<()> {
    let mut current = path.to_owned();
    loop { bounded(&current)?; if let Some(facts) = store.metadata(&current) { if facts.kind != NodeKind::Directory { return Err(TicketError::invalid("physical ticket ancestry required")); } }
        if std::path::Path::new(&current).parent().is_none() { return Ok(()); }
        let parent = dir_name(&current); if parent == current || parent == "." || parent.ends_with(':') { return Ok(()); } current = parent;
    }
}

fn ancestry<S: TicketStore>(store: &S, path: &str) -> TicketResult<()> {
    let mut current = path.to_owned();
    loop {
        bounded(&current)?;
        if current != "/" && current != "." && !store.is_directory(&current) { return Err(TicketError::invalid(format!("physical draft ancestry required: {current}"))); }
        if std::path::Path::new(&current).parent().is_none() { return Ok(()); }
        let parent = dir_name(&current);
        if parent == current || parent == "." || parent.ends_with(':') { return Ok(()); }
        current = parent;
    }
}

fn inspect<S: TicketStore>(store: &S, source: &str, destination: &str, depth: usize, count: &mut usize) -> TicketResult<()> {
    bounded(source)?; bounded(destination)?;
    *count += 1;
    if *count > 4096 || depth > 32 { return Err(TicketError::invalid("bounded draft tree required")); }
    let facts = store.metadata(source).ok_or_else(|| TicketError::not_found(format!("draft entry missing: {source}")))?;
    match facts.kind {
        NodeKind::Symlink => Err(TicketError::invalid("draft symlink refused")),
        NodeKind::File => Ok(()),
        NodeKind::Directory => {
            for entry in store.entries(source)? {
                if !segment(&entry.name) { return Err(TicketError::invalid("safe draft entry name required")); }
                inspect(store, &join_path(source, &entry.name), &join_path(destination, &entry.name), depth + 1, count)?;
            }
            Ok(())
        }
    }
}

pub(super) fn projection<S: TicketStore>(store: &S, source: &str, destination: &str) -> TicketResult<()> { let mut count = 0; inspect(store, source, destination, 0, &mut count) }

fn collision_name(name: &str, suffix: usize) -> String {
    if suffix == 1 { return name.to_owned(); }
    match name.rfind('.') { Some(index) => format!("{}_{}{}", &name[..index], suffix, &name[index..]), None => format!("{name}_{suffix}") }
}

pub(super) fn prepare<S: TicketStore>(store: &S, layout: &TicketLayout, id: &TicketId, draft: &str) -> TicketResult<Option<DraftPlan>> {
    if draft.is_empty() { return Ok(None); }
    if draft.split('/').any(|value| !segment(value)) { return Err(TicketError::invalid("relative draft identity required")); }
    let target = owned_ticket_path(layout, id)?;
    existing_ancestry(store, &target)?;
    let source = join_path(&join_path(layout.repo_meta_dir(), "📝️drafts"), draft);
    bounded(&source)?;
    if !store.exists(&source) { return Err(TicketError::not_found(format!("draft not found: {draft}"))); }
    ancestry(store, &source)?;
    let entries = store.entries(&source)?;
    if entries.len() > 1024 { return Err(TicketError::invalid("bounded immediate draft entries required")); }
    let mut selected = BTreeSet::from([layout.document_path(id), layout.important_dir(id)]);
    let mut moves = Vec::new();
    let mut count = 1;
    for entry in entries {
        if !segment(&entry.name) { return Err(TicketError::invalid("safe draft entry name required")); }
        let mut destination = None;
        for suffix in 1..=1024 {
            let candidate = join_path(&target, &collision_name(&entry.name, suffix)); bounded(&candidate)?;
            if !selected.contains(&candidate) && !store.exists(&candidate) { destination = Some(candidate); break; }
        }
        let destination = destination.ok_or_else(|| TicketError::conflict("draft collision limit exhausted"))?;
        let child = join_path(&source, &entry.name);
        inspect(store, &child, &destination, 1, &mut count)?;
        selected.insert(destination.clone()); moves.push((child, destination));
    }
    Ok(Some(DraftPlan { source, moves }))
}

pub(super) fn attach<S: TicketStore>(store: &S, plan: Option<DraftPlan>) -> TicketResult<()> {
    let Some(plan) = plan else { return Ok(()) };
    let mut count = 1;
    for (source, destination) in plan.moves {
        ancestry(store, &plan.source)?;
        existing_ancestry(store, &dir_name(&destination))?;
        inspect(store, &source, &destination, 1, &mut count)?;
        if store.exists(&destination) { return Err(TicketError::conflict("draft destination changed")); }
        store.rename(&source, &destination)?;
    }
    store.remove_directory(&plan.source)
}
