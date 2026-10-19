//! 🪡️ The `Graft` that patches the retained object graph in place, with the resource section helpers.

use super::*;

//#region 🔖️Encode
/// 🪡 A graft in progress: the retained graph being patched and its trailer, the next free
/// object number, and every value a patch displaced — collected at the end once nothing reaches
/// it, so the objects a lane stopped owning leave the file while unrelated orphans stay.
pub struct Graft<'a> {
    pub typed: &'a PdfSnapshot,
    pub graph: BTreeMap<u32, PdfIndirectObject>,
    pub trailer: Vec<PdfDictEntry>,
    pub next: u32,
    displaced: Vec<PdfObject>,
}
impl<'a> Graft<'a> {
    pub fn new(typed: &'a PdfSnapshot) -> Self {
        let graph: BTreeMap<u32, PdfIndirectObject> = typed.objects.iter().map(|object| (object.id.num, object.clone())).collect();
        let next = graph.keys().next_back().copied().unwrap_or(0) + 1;
        Self { typed, graph, trailer: typed.trailer.clone(), next, displaced: Vec::new() }
    }

    pub fn read(&self) -> PResult<Reading> {
        Reading::of(GraphView(&self.graph), &self.trailer, &self.typed.declared_version)
    }

    /// 🔁 One graft round against `reading` (what the graph spells now); `original` is the
    /// untouched graph's reading, the only source a resource may be retained from. `false` when a
    /// moved lane is beyond what a graft can express.
    pub fn round(&mut self, reading: &Reading, original: &Reading, moved: &Moved) -> PResult<bool> {
        let pages = match moved.pages {
            true => match self.graft_pages(reading)? {
                Some(pages) => pages,
                None => return Ok(false),
            },
            false => reading.page_refs.clone(),
        };
        if moved.info {
            self.graft_info();
        }
        if !moved.catalog.is_empty() {
            self.graft_catalog(reading, &pages, &moved.catalog)?;
        }
        Ok(!moved.resources || self.retain_resources(original, &pages))
    }

    /// 🏁 Collects what the grafts displaced and hands back the graph and its trailer.
    pub fn finish(mut self) -> PResult<(Cow<'a, [PdfIndirectObject]>, DocumentTrailer)> {
        self.collect_displaced();
        let trailer = retained_trailer(self.typed, &self.trailer)?;
        Ok((Cow::Owned(self.graph.into_values().collect()), trailer))
    }

    pub fn reserve(&mut self) -> ObjRef {
        let reference = ObjRef { num: self.next, gen: 0 };
        self.next += 1;
        reference
    }

    pub fn value(&self, reference: ObjRef) -> Option<&PdfObject> {
        self.graph.get(&reference.num).map(|object| &object.value)
    }

    pub fn resolve(&self, value: &PdfObject) -> Option<PdfObject> {
        match value {
            PdfObject::Ref(reference) => self.value(*reference).cloned(),
            other => Some(other.clone()),
        }
    }

    pub fn dict(&self, reference: ObjRef) -> Vec<PdfDictEntry> {
        self.value(reference).and_then(PdfObject::as_dict).map(<[PdfDictEntry]>::to_vec).unwrap_or_default()
    }

    fn set_dict(&mut self, reference: ObjRef, entries: Vec<PdfDictEntry>) {
        self.graph.insert(reference.num, PdfIndirectObject { id: reference, value: PdfObject::Dict(entries) });
    }

    fn catalog(&self) -> Option<ObjRef> {
        dict_get(&self.trailer, "Root").and_then(PdfObject::as_ref)
    }

    fn page_tree_root(&self) -> Option<ObjRef> {
        dict_get(&self.dict(self.catalog()?), "Pages").and_then(PdfObject::as_ref)
    }

    /// 🧬 Moves the lowered objects `value` reaches out of `scratch` into the graph under fresh
    /// numbers (one number per scratch object across a batch, recorded in `adopted`) and returns
    /// `value` with its references rewritten.
    fn adopt(&mut self, value: &PdfObject, scratch: &HashMap<u32, PdfObject>, adopted: &mut HashMap<u32, ObjRef>) -> PdfObject {
        let mut pending = Vec::new();
        referenced(value, &[], &mut pending);
        let mut fresh = Vec::new();
        while let Some(num) = pending.pop() {
            if adopted.contains_key(&num) {
                continue;
            }
            let Some(inner) = scratch.get(&num) else { continue };
            let target = self.reserve();
            adopted.insert(num, target);
            referenced(inner, &[], &mut pending);
            fresh.push(num);
        }
        for num in fresh {
            let target = adopted[&num];
            self.graph.insert(target.num, PdfIndirectObject { id: target, value: rewrite_refs(&scratch[&num], adopted) });
        }
        rewrite_refs(value, adopted)
    }

    /// 🔧 Re-states `key` in `entries`: `value` (adopted from `scratch`) replaces the entry,
    /// `None` drops it; the value it displaces is collected later.
    fn put(&mut self, entries: &mut Vec<PdfDictEntry>, key: &str, value: Option<PdfObject>, scratch: &HashMap<u32, PdfObject>, adopted: &mut HashMap<u32, ObjRef>) {
        let value = value.map(|value| self.adopt(&value, scratch, adopted));
        match (entries.iter().position(|entry| entry.key == key), value) {
            (Some(index), Some(value)) => {
                let old = std::mem::replace(&mut entries[index].value, value);
                self.displaced.push(old);
            }
            (Some(index), None) => {
                let old = entries.remove(index).value;
                self.displaced.push(old);
            }
            (None, Some(value)) => entries.push(PdfDictEntry::new(key, value)),
            (None, None) => {}
        }
    }

    /// 🧹 Drops every object only displaced values reached that nothing in the grafted document
    /// reaches any more (`/Parent` back-links do not keep an object in the candidate set).
    fn collect_displaced(&mut self) {
        let mut pending = Vec::new();
        for value in std::mem::take(&mut self.displaced) {
            referenced(&value, &["Parent"], &mut pending);
        }
        let mut candidates = HashSet::new();
        while let Some(num) = pending.pop() {
            if candidates.insert(num) {
                if let Some(object) = self.graph.get(&num) {
                    referenced(&object.value, &["Parent"], &mut pending);
                }
            }
        }
        if candidates.is_empty() {
            return;
        }
        let mut reachable = HashSet::new();
        let mut pending = Vec::new();
        self.trailer.iter().for_each(|entry| referenced(&entry.value, &[], &mut pending));
        while let Some(num) = pending.pop() {
            if reachable.insert(num) {
                if let Some(object) = self.graph.get(&num) {
                    referenced(&object.value, &[], &mut pending);
                }
            }
        }
        self.graph.retain(|num, _| !candidates.contains(num) || reachable.contains(num));
    }

    /// 📄 Grafts the page lane. A page the graph already spells keeps its object; a page whose
    /// fields moved is patched in place (only the moved entries re-stated); a new page is lowered
    /// fresh; and the page tree is re-stated flat under its root only when the page sequence
    /// itself changed. Returns the page references in typed order, `None` when a page carries
    /// what no graft can express.
    fn graft_pages(&mut self, reading: &Reading) -> PResult<Option<Vec<ObjRef>>> {
        let typed = self.typed;
        let read = &reading.lanes.pages;
        let Some(root) = self.page_tree_root() else { return Ok(None) };
        let mut source: Vec<Option<usize>> = (0..typed.pages.len()).map(|index| (index < read.len() && typed.pages[index] == read[index]).then_some(index)).collect();
        let mut used = vec![false; read.len()];
        source.iter().flatten().for_each(|slot| used[*slot] = true);
        for index in 0..typed.pages.len() {
            if source[index].is_none() {
                if let Some(slot) = (0..read.len()).find(|slot| !used[*slot] && read[*slot] == typed.pages[index]) {
                    used[slot] = true;
                    source[index] = Some(slot);
                }
            }
        }
        let mut patched = Vec::new();
        for index in 0..typed.pages.len() {
            if source[index].is_none() && index < read.len() && !used[index] {
                used[index] = true;
                source[index] = Some(index);
                patched.push(index);
            }
        }
        let mut pages = Vec::with_capacity(typed.pages.len());
        let mut inserted = Vec::new();
        for (index, slot) in source.iter().enumerate() {
            match slot {
                Some(slot) => pages.push(reading.page_refs[*slot]),
                None => {
                    inserted.push(index);
                    pages.push(self.reserve());
                }
            }
        }
        let removed: Vec<ObjRef> = (0..read.len()).filter(|slot| !used[*slot]).map(|slot| reading.page_refs[slot]).collect();
        let targets = reading.targets(&pages);
        for index in patched {
            if !self.patch_page(index, &read[index], pages[index], &targets)? {
                return Ok(None);
            }
        }
        for index in inserted {
            if !self.insert_page(index, pages[index], root, &targets)? {
                return Ok(None);
            }
        }
        if pages != reading.page_refs {
            self.restate_page_tree(root, &pages, &removed);
        }
        Ok(Some(pages))
    }

    /// ✏️ Patches the retained page at `reference` so it spells typed page `index`: the entries of
    /// every moved field are re-stated from the page lowered afresh, every other entry (content
    /// stream, resources, annotations, keys no field owns) stays as the graph holds it.
    fn patch_page(&mut self, index: usize, read: &PdfPage, reference: ObjRef, targets: &HashMap<(Category, String), ObjRef>) -> PResult<bool> {
        let snapshot = self.typed;
        let typed = &snapshot.pages[index];
        let content = typed.content != read.content;
        let annotations = typed.annotations != read.annotations;
        if annotations && (carries_widget(typed) || carries_widget(read)) {
            return Ok(false);
        }
        let mut probe = typed.clone();
        if !content {
            probe.content.clear();
        }
        if !annotations {
            probe.annotations.clear();
        }
        let (objects, _) = lower_page_standalone(snapshot, &probe, index, reference, reference, self.next, LowerOptions::default(), targets, &HashMap::new())?;
        let mut scratch: HashMap<u32, PdfObject> = objects.into_iter().map(|object| (object.id.num, object.value)).collect();
        let lowered = scratch.remove(&reference.num).and_then(|value| value.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default();
        let fields: [(bool, &str); 16] = [
            (typed.media_box != read.media_box, "MediaBox"),
            (typed.crop_box != read.crop_box, "CropBox"),
            (typed.bleed_box != read.bleed_box, "BleedBox"),
            (typed.trim_box != read.trim_box, "TrimBox"),
            (typed.art_box != read.art_box, "ArtBox"),
            (typed.rotate != read.rotate, "Rotate"),
            (typed.user_unit != read.user_unit, "UserUnit"),
            (content, "Contents"),
            (annotations, "Annots"),
            (typed.group != read.group, "Group"),
            (typed.thumbnail != read.thumbnail, "Thumb"),
            (typed.struct_parents != read.struct_parents, "StructParents"),
            (typed.transition != read.transition, "Trans"),
            (typed.duration != read.duration, "Dur"),
            (typed.metadata != read.metadata, "Metadata"),
            (typed.additional_actions != read.additional_actions, "AA"),
        ];
        let mut keys: Vec<&str> = fields.iter().filter(|(moved, _)| *moved).map(|(_, key)| *key).collect();
        if typed.extra != read.extra {
            keys.extend(read.extra.iter().chain(&typed.extra).map(|entry| entry.key.as_str()));
        }
        let mut seen = HashSet::new();
        keys.retain(|key| seen.insert(*key));
        let mut dict = self.dict(reference);
        let mut adopted = HashMap::new();
        for key in keys {
            let value = match (key, dict_get(&lowered, key)) {
                ("Rotate", None) => Some(PdfObject::Int(typed.rotate as i64)),
                (_, value) => value.cloned(),
            };
            self.put(&mut dict, key, value, &scratch, &mut adopted);
        }
        if content && !self.bind_resources(&mut dict, &typed.content, targets) {
            return Ok(false);
        }
        self.set_dict(reference, dict);
        Ok(true)
    }

    /// ➕ Lowers typed page `index` fresh at the reserved `reference` under `parent`.
    pub fn insert_page(&mut self, index: usize, reference: ObjRef, parent: ObjRef, targets: &HashMap<(Category, String), ObjRef>) -> PResult<bool> {
        let snapshot = self.typed;
        let page = &snapshot.pages[index];
        let references = content_references(&page.content);
        if carries_widget(page) || resource_sections(&references).iter().any(|(_, category, ids)| ids.iter().any(|id| !targets.contains_key(&(*category, id.clone())) && self.defines(*category, id))) {
            return Ok(false);
        }
        let (objects, _) = lower_page_standalone(snapshot, page, index, reference, parent, self.next, LowerOptions::default(), targets, &HashMap::new())?;
        let mut scratch: HashMap<u32, PdfObject> = objects.into_iter().map(|object| (object.id.num, object.value)).collect();
        let lowered = scratch.remove(&reference.num).unwrap_or(PdfObject::Dict(Vec::new()));
        let value = self.adopt(&lowered, &scratch, &mut HashMap::new());
        self.graph.insert(reference.num, PdfIndirectObject { id: reference, value });
        Ok(true)
    }

    /// 🆔 Whether a typed resource collection defines `id` in `category`.
    fn defines(&self, category: Category, id: &str) -> bool {
        let typed = self.typed;
        match category {
            Category::Font => typed.font(id).is_some(),
            Category::XObject => typed.image(id).is_some() || typed.form(id).is_some(),
            Category::ExtGState => typed.ext_g_state(id).is_some(),
            Category::Shading => typed.shading(id).is_some(),
            Category::Pattern => typed.pattern(id).is_some(),
            Category::ColorSpace => typed.color_spaces.iter().any(|space| space.name == id),
            Category::Properties => typed.properties.iter().any(|properties| properties.name == id),
            _ => false,
        }
    }

    /// 📚 Makes the page's resources bind every id its new content names: the resources it
    /// already sees (its own or inherited) are kept whole, and an id they do not bind to the
    /// graph's object for it is bound on a page-own copy. `false` when the content names a typed
    /// resource the graph holds no object for.
    fn bind_resources(&mut self, dict: &mut Vec<PdfDictEntry>, ops: &[PdfOp], targets: &HashMap<(Category, String), ObjRef>) -> bool {
        let effective = self.effective_resources(dict);
        let mut resources = effective.clone();
        for (key, category, ids) in resource_sections(&content_references(ops)) {
            let mut section = dict_get(&resources, key).and_then(|value| self.resolve(value)).and_then(|value| value.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default();
            let mut bound = false;
            for id in ids {
                match targets.get(&(category, id.clone())) {
                    Some(target) if dict_get(&section, &id) != Some(&PdfObject::Ref(*target)) => {
                        section.retain(|entry| entry.key != id);
                        section.push(PdfDictEntry::new(id, PdfObject::Ref(*target)));
                        bound = true;
                    }
                    None if self.defines(category, &id) && dict_get(&section, &id).is_none() => return false,
                    _ => {}
                }
            }
            if bound {
                resources.retain(|entry| entry.key != key);
                resources.push(PdfDictEntry::new(key, PdfObject::Dict(section)));
            }
        }
        if resources != effective {
            let own = PdfObject::Dict(resources);
            match dict.iter_mut().find(|entry| entry.key == "Resources") {
                Some(entry) => self.displaced.push(std::mem::replace(&mut entry.value, own)),
                None => dict.push(PdfDictEntry::new("Resources", own)),
            }
        }
        true
    }

    /// 📚 The resource dictionary a page sees: its own, else the nearest ancestor's.
    fn effective_resources(&self, dict: &[PdfDictEntry]) -> Vec<PdfDictEntry> {
        self.inherited(dict, "Resources", None).and_then(|value| self.resolve(&value)).and_then(|value| value.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default()
    }

    /// 🧬 The inherited attribute `key` of a page dictionary: its own entry, else the nearest
    /// ancestor's below `stop`.
    pub fn inherited(&self, dict: &[PdfDictEntry], key: &str, stop: Option<ObjRef>) -> Option<PdfObject> {
        if let Some(value) = dict_get(dict, key) {
            return Some(value.clone());
        }
        let mut parent = dict_get(dict, "Parent").and_then(PdfObject::as_ref);
        for _ in 0..64 {
            let node = parent.filter(|node| Some(node.num) != stop.map(|stop| stop.num))?;
            let entries = self.dict(node);
            if let Some(value) = dict_get(&entries, key) {
                return Some(value.clone());
            }
            parent = dict_get(&entries, "Parent").and_then(PdfObject::as_ref);
        }
        None
    }

    /// 🌳 Re-states the page tree flat under `root` (§7.7.3.2): every page a direct kid in typed
    /// order, carrying the attributes it inherited from the intermediate nodes it leaves; the
    /// intermediate nodes and the removed pages are displaced.
    fn restate_page_tree(&mut self, root: ObjRef, pages: &[ObjRef], removed: &[ObjRef]) {
        for page in pages {
            let mut dict = self.dict(*page);
            for key in INHERITED_PAGE_KEYS {
                if dict_get(&dict, key).is_none() {
                    if let Some(value) = self.inherited(&dict, key, Some(root)) {
                        dict.push(PdfDictEntry::new(key, value));
                    }
                }
            }
            dict.retain(|entry| entry.key != "Parent");
            dict.push(PdfDictEntry::new("Parent", PdfObject::Ref(root)));
            self.set_dict(*page, dict);
        }
        let mut tree = self.dict(root);
        let kids = PdfObject::Array(pages.iter().map(|page| PdfObject::Ref(*page)).collect());
        match tree.iter_mut().find(|entry| entry.key == "Kids") {
            Some(entry) => self.displaced.push(std::mem::replace(&mut entry.value, kids)),
            None => tree.push(PdfDictEntry::new("Kids", kids)),
        }
        tree.retain(|entry| entry.key != "Count");
        tree.push(PdfDictEntry::new("Count", PdfObject::Int(pages.len() as i64)));
        self.set_dict(root, tree);
        self.displaced.extend(removed.iter().map(|page| PdfObject::Ref(*page)));
    }

    /// ℹ️ Grafts the information dictionary: re-stated in place when the graph holds one, added
    /// when it does not, dropped from the trailer when the typed record is empty.
    fn graft_info(&mut self) {
        let snapshot = self.typed;
        let info = &snapshot.info;
        let current = dict_get(&self.trailer, "Info").and_then(PdfObject::as_ref).filter(|reference| self.graph.contains_key(&reference.num));
        if info.is_empty() {
            if let Some(index) = self.trailer.iter().position(|entry| entry.key == "Info") {
                let old = self.trailer.remove(index).value;
                self.displaced.push(old);
            }
            return;
        }
        let reference = current.unwrap_or_else(|| self.reserve());
        if let Some(old) = self.graph.insert(reference.num, PdfIndirectObject { id: reference, value: PdfObject::Dict(lower_info(info)) }) {
            self.displaced.push(old.value);
        }
        self.trailer.retain(|entry| entry.key != "Info");
        self.trailer.push(PdfDictEntry::new("Info", PdfObject::Ref(reference)));
    }

    /// 🗂️ Grafts the moved catalog lanes: the catalog is lowered afresh against the graph (its
    /// destinations resolve to the graph's pages) and only the entries the moved lanes own are
    /// re-stated in the retained catalog; named destinations re-state the `/Names` tree's `/Dests`
    /// (superseding a PDF 1.1 catalog `/Dests`), extra entries re-state exactly the keys they carry.
    fn graft_catalog(&mut self, reading: &Reading, pages: &[ObjRef], lanes: &[CatalogLane]) -> PResult<()> {
        let (Some(catalog_ref), Some(root)) = (self.catalog(), self.page_tree_root()) else { return Ok(()) };
        let (lowered, objects) = lower_catalog_standalone(self.typed, self.next, LowerOptions::default(), &reading.targets(pages), root)?;
        let scratch: HashMap<u32, PdfObject> = objects.into_iter().map(|object| (object.id.num, object.value)).collect();
        let mut catalog = self.dict(catalog_ref);
        let mut adopted = HashMap::new();
        for lane in lanes {
            match lane {
                CatalogLane::NamedDestinations => {
                    self.put(&mut catalog, "Dests", None, &scratch, &mut adopted);
                    let dests = dict_get(&lowered, "Names").and_then(|names| names.dict_get("Dests")).cloned();
                    self.put_named_destinations(&mut catalog, dests, &scratch, &mut adopted);
                }
                CatalogLane::Extra => {
                    for entry in &reading.lanes.catalog_extra {
                        self.put(&mut catalog, &entry.key, None, &scratch, &mut adopted);
                    }
                    catalog.extend(self.typed.catalog_extra.iter().cloned());
                }
                lane => self.put(&mut catalog, lane.key(), dict_get(&lowered, lane.key()).cloned(), &scratch, &mut adopted),
            }
        }
        self.set_dict(catalog_ref, catalog);
        Ok(())
    }

    /// 🎯 Re-states `/Names /Dests`, editing the `/Names` dictionary where the graph keeps it and
    /// dropping it once it holds nothing.
    fn put_named_destinations(&mut self, catalog: &mut Vec<PdfDictEntry>, dests: Option<PdfObject>, scratch: &HashMap<u32, PdfObject>, adopted: &mut HashMap<u32, ObjRef>) {
        let held = dict_get(catalog, "Names").cloned();
        let mut names = held.as_ref().and_then(|value| self.resolve(value)).and_then(|value| value.as_dict().map(<[PdfDictEntry]>::to_vec)).unwrap_or_default();
        self.put(&mut names, "Dests", dests, scratch, adopted);
        match (held.as_ref().and_then(PdfObject::as_ref), names.is_empty()) {
            (Some(reference), false) if self.graph.contains_key(&reference.num) => self.set_dict(reference, names),
            (_, true) => self.put(catalog, "Names", None, scratch, adopted),
            (_, false) => self.put(catalog, "Names", Some(PdfObject::Dict(names)), scratch, adopted),
        }
    }

    /// 🗃️ Keeps every typed resource alive and in typed order through the page tree root's
    /// `/Resources` (§7.7.3.4) — the only way a graft can restate a resource collection whose
    /// values are the graph's own when a page edit changed which pages reach them, or in what
    /// order. Pages that inherited the root's resources get them as their own first. `false` when
    /// a typed resource is not the original graph's object.
    fn retain_resources(&mut self, original: &Reading, pages: &[ObjRef]) -> bool {
        fn section<T: PartialEq>(key: &'static str, category: Category, typed: &[T], read: &[T], id: impl Fn(&T) -> &str, refs: &HashMap<(Category, String), ObjRef>) -> Option<(&'static str, Vec<PdfDictEntry>)> {
            let entries = typed.iter().map(|item| read.contains(item).then(|| refs.get(&(category, id(item).to_string())).map(|reference| PdfDictEntry::new(id(item), PdfObject::Ref(*reference)))).flatten()).collect::<Option<Vec<_>>>()?;
            Some((key, entries))
        }
        let (typed, read, refs) = (self.typed, &original.lanes, &original.refs);
        let x_objects: Option<Vec<PdfDictEntry>> = section("XObject", Category::XObject, &typed.images, &read.images, |image| image.id.as_str(), refs).zip(section("XObject", Category::XObject, &typed.forms, &read.forms, |form| form.id.as_str(), refs)).map(|((_, images), (_, forms))| images.into_iter().chain(forms).collect());
        let sections = [
            section("Font", Category::Font, &typed.fonts, &read.fonts, |font| font.id.as_str(), refs),
            x_objects.map(|entries| ("XObject", entries)),
            section("ExtGState", Category::ExtGState, &typed.ext_g_states, &read.ext_g_states, |state| state.id.as_str(), refs),
            section("Shading", Category::Shading, &typed.shadings, &read.shadings, |shading| shading.id.as_str(), refs),
            section("Pattern", Category::Pattern, &typed.patterns, &read.patterns, |pattern| pattern.id.as_str(), refs),
            section("ColorSpace", Category::ColorSpace, &typed.color_spaces, &read.color_spaces, |space| space.name.as_str(), refs),
            section("Properties", Category::Properties, &typed.properties, &read.properties, |properties| properties.name.as_str(), refs),
        ];
        let Some(sections) = sections.into_iter().collect::<Option<Vec<_>>>() else { return false };
        let Some(root) = self.page_tree_root() else { return false };
        let resources = PdfObject::Dict(sections.into_iter().filter(|(_, entries)| !entries.is_empty()).map(|(key, entries)| PdfDictEntry::new(key, PdfObject::Dict(entries))).collect());
        let mut tree = self.dict(root);
        let inherited = dict_get(&tree, "Resources").cloned().unwrap_or(PdfObject::Dict(Vec::new()));
        for page in pages {
            let mut dict = self.dict(*page);
            if self.inherited(&dict, "Resources", Some(root)).is_none() {
                dict.push(PdfDictEntry::new("Resources", inherited.clone()));
                self.set_dict(*page, dict);
            }
        }
        tree.retain(|entry| entry.key != "Resources");
        tree.push(PdfDictEntry::new("Resources", resources));
        self.set_dict(root, tree);
        true
    }
}
/// 📚 A content stream's resource names by resource dictionary section (§7.8.3).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resource_sections(references: &ContentReferences) -> [(&'static str, Category, Vec<String>); 7] {
    [
        ("Font", Category::Font, references.fonts.clone()),
        ("XObject", Category::XObject, references.x_objects.clone()),
        ("ExtGState", Category::ExtGState, references.ext_g_states.clone()),
        ("Shading", Category::Shading, references.shadings.clone()),
        ("Pattern", Category::Pattern, references.patterns.clone()),
        ("ColorSpace", Category::ColorSpace, references.color_spaces.clone()),
        ("Properties", Category::Properties, references.properties.clone()),
    ]
}
/// 🧷 Whether a page carries an interactive-form widget, whose field back-links only a whole
/// interactive-form write can re-state.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn carries_widget(page: &PdfPage) -> bool {
    page.annotations.iter().any(|annotation| matches!(annotation.kind, PdfAnnotationKind::Widget { .. }))
}
//#endregion 🔖️Encode
