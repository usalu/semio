//! 🔗️ Reference lookup over the owned PDF graph.
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef,PdfObject,PdfIndirectObject};
use std::collections::HashMap;

/// 🧭️ Follows references in a first-party owned graph.
pub trait ObjectSource {
    /// 🎯 The object a reference points at (`None` when unresolvable).
    fn get(&mut self, reference: ObjRef) -> Option<PdfObject>;
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
    by_number: HashMap<u32, &'a PdfObject>,
}

impl<'a> GraphSource<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(objects: &'a [PdfIndirectObject]) -> Self {
        Self { by_number: objects.iter().map(|object| (object.id.num, &object.value)).collect() }
    }
}

impl ObjectSource for GraphSource<'_> {
    fn get(&mut self, reference: ObjRef) -> Option<PdfObject> {
        self.by_number.get(&reference.num).map(|value| (*value).clone())
    }
}

