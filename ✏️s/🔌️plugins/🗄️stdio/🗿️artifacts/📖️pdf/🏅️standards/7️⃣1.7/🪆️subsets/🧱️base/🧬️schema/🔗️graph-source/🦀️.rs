//! 🔗️ Reference lookup over the owned PDF graph.
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef,PdfObject,PdfIndirectObject};
use std::collections::HashMap;
use crate::standards::v1_7::subsets::base::schema::stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfGraphPath, PdfStreamRoleValue, PdfStreamRoleKind};

/// 🧭️ Follows references in a first-party owned graph.
pub trait ObjectSource {
    /// 🎯 The object a reference points at (`None` when unresolvable).
    fn get(&mut self, reference: ObjRef) -> Option<PdfObject>;
    /// 🪪️ Finds a logical graph identity without parsing its representation.
    fn identity_of(&self, value: &PdfObject) -> Option<PdfGraphIdentity> { value.as_ref().map(|owner| PdfGraphIdentity { owner, path: Vec::new() }) }
    /// 🧵️ Names the transitive logical inputs consumed by native admission.
    fn role_dependencies(&self, identity: &PdfGraphIdentity) -> Vec<PdfGraphIdentity> { vec![identity.clone()] }
    /// 🧷️ Records semantic admission performed by native IO.
    fn record_role(&mut self, _value: &PdfObject, _role: PdfStreamRoleValue) {}
    /// 🧬️ Adds consumer-specific inputs to the intrinsic stream admission.
    fn record_role_dependencies(&mut self, _value: &PdfObject, _kind: PdfStreamRoleKind, _dependencies: &[PdfGraphIdentity]) {}
    /// 📚️ Returns native admissions already captured by this owned source.
    fn admitted_roles(&self) -> &[PdfAdmittedStreamRole] { &[] }
    /// 🎯️ Reads an already admitted semantic stream role.
    fn semantic_role(&mut self, _value: &PdfObject, _kind: PdfStreamRoleKind) -> Option<PdfStreamRoleValue> { None }
    /// 📄️ Reads a role whose consumer identity differs from its stream dependencies.
    fn semantic_role_at(&mut self, _identity: &PdfGraphIdentity, _kind: PdfStreamRoleKind) -> Option<PdfStreamRoleValue> { None }
    /// 🎯 Follows `value` if it is a reference, else returns it as is.
    fn deref(&mut self, value: &PdfObject) -> PdfObject {
        match value {
            PdfObject::Ref(reference) => self.get(*reference).unwrap_or(PdfObject::Null),
            other => other.clone(),
        }
    }
    /// 🎯 Follows a reference chain to a dictionary/stream entry.
    fn deref_key(&mut self, value: &PdfObject, key: &str) -> Option<PdfObject> {
        let owner = self.deref(value);
        let entry = owner.dict_get(key)?.clone();
        Some(self.deref(&entry))
    }
}

/// 🧭 A source over the retained `objects` lane (already normalized).
pub struct GraphSource<'a> {
    by_number: HashMap<ObjRef, &'a PdfObject>,
    objects: &'a [PdfIndirectObject],
    pub admitted_roles: Vec<PdfAdmittedStreamRole>,
    pub missing_roles: Vec<(PdfGraphIdentity, PdfStreamRoleKind)>,
}

impl<'a> GraphSource<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(objects: &'a [PdfIndirectObject]) -> Self {
        Self { by_number: objects.iter().map(|object| (object.id, &object.value)).collect(), objects, admitted_roles: Vec::new(), missing_roles: Vec::new() }
    }
    /// 🛂️ Binds an immutable graph to its explicitly admitted semantic context.
    pub fn with_roles(objects: &'a [PdfIndirectObject], roles: &[PdfAdmittedStreamRole]) -> Self {
        Self { admitted_roles: roles.to_vec(), ..Self::new(objects) }
    }
    /// 🪪️ Enumerates exact owners for equal direct values without choosing an arbitrary alias.
    fn identities_of(&self, value: &PdfObject) -> Vec<PdfGraphIdentity> {
        if let Some(owner) = value.as_ref() { return vec![PdfGraphIdentity { owner, path: Vec::new() }]; }
        fn locate(value: &PdfObject, needle: &PdfObject, path: &mut Vec<PdfGraphPath>, depth: usize, found: &mut Vec<Vec<PdfGraphPath>>) {
            if value == needle { found.push(path.clone()); }
            if depth == 64 { return; }
            match value {
                PdfObject::Array(items) => for (index, item) in items.iter().enumerate() { path.push(PdfGraphPath::Item { index }); locate(item, needle, path, depth + 1, found); path.pop(); },
                PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => for entry in entries { path.push(PdfGraphPath::Entry { key: entry.key.clone() }); locate(&entry.value, needle, path, depth + 1, found); path.pop(); },
                _ => {}
            }
        }
        let mut identities = Vec::new();
        for object in self.objects { let mut paths = Vec::new(); locate(&object.value, value, &mut Vec::new(), 0, &mut paths); identities.extend(paths.into_iter().map(|path| PdfGraphIdentity { owner: object.id, path })); }
        identities
    }
    /// 🧵️ Captures exact structural inputs and transitive logical references consumed by admission.
    pub fn dependencies_of(&self, identity: &PdfGraphIdentity) -> Vec<PdfGraphIdentity> {
        fn references(value: &PdfObject, output: &mut Vec<ObjRef>) {
            match value {
                PdfObject::Ref(owner) => output.push(*owner),
                PdfObject::Array(items) => for item in items { references(item, output); },
                PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => for entry in entries { references(&entry.value, output); },
                _ => {}
            }
        }
        let mut dependencies = vec![identity.clone()];
        if identity.path == [PdfGraphPath::Entry { key: "Contents".into() }] {
            let mut page = identity.owner;
            let mut visited = Vec::new();
            while !visited.contains(&page) && visited.len() < 64 {
                visited.push(page);
                let Some(value) = self.by_number.get(&page) else { break; };
                if let Some(resources) = value.dict_get("Resources") {
                    let (owner, path, dictionary) = match resources {
                        PdfObject::Ref(owner) => (*owner, Vec::new(), self.by_number.get(owner).copied()),
                        value => (page, vec![PdfGraphPath::Entry { key: "Resources".into() }], Some(value)),
                    };
                    if dictionary.is_some_and(|value| value.dict_get("Font").is_some()) {
                        let mut path = path; path.push(PdfGraphPath::Entry { key: "Font".into() });
                        dependencies.push(PdfGraphIdentity { owner, path });
                    }
                    break;
                }
                let Some(parent) = value.dict_get("Parent").and_then(PdfObject::as_ref) else { break; }; page = parent;
            }
        }
        let mut index = 0;
        while index < dependencies.len() {
            let mut owners = Vec::new();
            if let Some(value) = crate::standards::v1_7::subsets::base::schema::stream_roles::resolve_identity(self.objects, &dependencies[index]) { references(value, &mut owners); }
            for owner in owners { let dependency = PdfGraphIdentity { owner, path: Vec::new() }; if !dependencies.contains(&dependency) { dependencies.push(dependency); } }
            index += 1;
        }
        dependencies
    }

}

impl ObjectSource for GraphSource<'_> {
    fn role_dependencies(&self, identity: &PdfGraphIdentity) -> Vec<PdfGraphIdentity> { self.dependencies_of(identity) }
    fn admitted_roles(&self) -> &[PdfAdmittedStreamRole] { &self.admitted_roles }
    fn get(&mut self, reference: ObjRef) -> Option<PdfObject> {
        self.by_number.get(&reference).map(|value| (*value).clone())
    }
    fn identity_of(&self, value: &PdfObject) -> Option<PdfGraphIdentity> {
        let identities = self.identities_of(value);
        (identities.len() == 1).then(|| identities[0].clone())
    }
    fn record_role(&mut self, value: &PdfObject, role: PdfStreamRoleValue) {
        for identity in self.identities_of(value) {
            let dependencies = self.dependencies_of(&identity);
            if let Some(current) = self.admitted_roles.iter_mut().find(|current| current.identity == identity && current.value.kind() == role.kind()) { current.value = role.clone(); current.dependencies = dependencies; }
            else { self.admitted_roles.push(PdfAdmittedStreamRole { dependencies, identity, value: role.clone() }); }
        }
    }
    fn record_role_dependencies(&mut self, value: &PdfObject, kind: PdfStreamRoleKind, dependencies: &[PdfGraphIdentity]) {
        let Some(identity) = self.identity_of(value) else { return; };
        if let Some(role) = self.admitted_roles.iter_mut().find(|role| role.identity == identity && role.value.kind() == kind) {
            for dependency in dependencies { if !role.dependencies.contains(dependency) { role.dependencies.push(dependency.clone()); } }
        }
    }
    fn semantic_role(&mut self, value: &PdfObject, kind: PdfStreamRoleKind) -> Option<PdfStreamRoleValue> {
        let identities = self.identities_of(value);
        if identities.is_empty() { return self.semantic_role_at(&PdfGraphIdentity { owner: ObjRef::default(), path: Vec::new() }, kind); }
        let mut result = None;
        for identity in identities {
            let role = self.semantic_role_at(&identity, kind)?;
            if result.as_ref().is_some_and(|current| *current != role) { self.missing_roles.push((identity, kind)); return None; }
            result = Some(role);
        }
        result
    }
    fn semantic_role_at(&mut self, identity: &PdfGraphIdentity, kind: PdfStreamRoleKind) -> Option<PdfStreamRoleValue> {
        let found = self.admitted_roles.iter().find(|role| role.identity == *identity && role.value.kind() == kind).map(|role| role.value.clone());
        if found.is_none() && !self.missing_roles.contains(&(identity.clone(), kind)) { self.missing_roles.push((identity.clone(), kind)); }
        found
    }
}
