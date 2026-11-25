//! 📜️ Borrows page program bytes directly or joins the authored content streams exactly once.

use crate::standards::v1_7::subsets::base::schema::{graph_source::ObjectSource,snapshot::{ObjRef,PdfObject}};

#[path = "🧭️controlled/🦀️.rs"]
mod controlled;
pub use controlled::{PageContentBarrier, PageContentCursor, PageContentEvent, PageContentReceipt};

#[path = "🧺️program/🦀️.rs"]
mod program;
pub use program::{operator_bytes, PageProgramCursor};

enum Member { Reference(ObjRef), Inline(usize) }

/// 📜️ Projects a single borrowed stream or an ordered array of page content streams.
pub fn project_content_bytes<T>(source: &mut dyn ObjectSource, contents: Option<&PdfObject>, project: impl FnOnce(&[u8]) -> T) -> T {
    let mut joined = Vec::new();
    match contents {
        Some(PdfObject::Stream { data, .. }) => return project(&data.to_vec_owner()),
        Some(PdfObject::Array(items)) => {
            for item in items {
                match item {
                    PdfObject::Stream { data, .. } => append(&mut joined, &data.to_vec_owner()),
                    PdfObject::Ref(reference) => append_reference(source, *reference, &mut joined),
                    _ => {}
                }
            }
        }
        Some(PdfObject::Ref(reference)) => {
            let mut project = Some(project);
            let mut direct = None;
            let mut members = Vec::new();
            source.inspect(*reference, &mut |value| match value {
                Some(PdfObject::Stream { data, .. }) => {
                    direct = project.take().map(|project| project(&data.to_vec_owner()));
                }
                Some(PdfObject::Array(items)) => {
                    members.extend(items.iter().enumerate().filter_map(|(index, item)| match item {
                        PdfObject::Ref(reference) => Some(Member::Reference(*reference)),
                        PdfObject::Stream { .. } => Some(Member::Inline(index)),
                        _ => None,
                    }));
                }
                _ => {}
            });
            if let Some(direct) = direct { return direct; }
            for member in members {
                match member {
                    Member::Reference(reference) => append_reference(source, reference, &mut joined),
                    Member::Inline(index) => source.inspect(*reference, &mut |value| {
                        if let Some(PdfObject::Stream { data, .. }) = value.and_then(PdfObject::as_array).and_then(|items| items.get(index)) {
                            append(&mut joined, &data.to_vec_owner());
                        }
                    }),
                }
            }
            return project.expect("unconsumed page content projection")(&joined);
        }
        _ => {}
    }
    project(&joined)
}

fn append_reference(source: &mut dyn ObjectSource, reference: ObjRef, joined: &mut Vec<u8>) {
    source.inspect(reference, &mut |value| {
        if let Some(PdfObject::Stream { data, .. }) = value { append(joined, &data.to_vec_owner()); }
    });
}

fn append(joined: &mut Vec<u8>, data: &[u8]) {
    if !joined.is_empty() { joined.push(b'\n'); }
    joined.extend_from_slice(data);
}
