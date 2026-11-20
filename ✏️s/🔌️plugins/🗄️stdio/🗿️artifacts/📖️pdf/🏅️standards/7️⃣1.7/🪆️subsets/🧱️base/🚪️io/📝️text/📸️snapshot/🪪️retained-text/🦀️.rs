//! 🪪️ Native known string roles bind once into retained semantic text and date values.
use crate::standards::v1_7::subsets::base::io::text_string::PdfTextScalarCursor;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;

fn role(key:&str)->Option<u8> {
    if ["CreationDate","ModDate","LastModified","M"].contains(&key) {Some(1)}
    else if key=="URI" {Some(2)}
    else if ["Title","Author","Subject","Keywords","Creator","Producer","Lang","T","TU","TM","V","DV","RC","RV","Contents","Subj","NM","State","StateModel","Desc","F","UF","DOS","Mac","Unix","D","Dest","JS","Name","Registry","Ordering","OutputCondition","OutputConditionIdentifier","RegistryName","Info","Names","Opt","Fields","FontFamily"].contains(&key) {Some(0)}else{None}
}
/// 🧾️ Admits known roles while retaining one logical object for each indirect identity.
pub fn admit_retained_text(
    objects: &mut [PdfIndirectObject], trailer: &mut [PdfDictEntry], position: impl Fn(ObjRef) -> Option<usize>,
) -> Result<(), &'static str> {
    let mut cursor = RetainedTextCursor::new(objects.len());
    let grant = RetainedTextGrant { work: 1, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
    let result = loop {
        match cursor.advance(objects, trailer, &position, grant, false).status {
            Ok(RetainedTextStatus::More) => {}
            Ok(RetainedTextStatus::Complete) => return Ok(()),
            Err(reason) => break Err(reason),
            _ => break Err("unexpected retained text state under reference grants"),
        }
    };
    cursor.begin_close();
    while cursor.close_step(grant).status == Ok(RetainedTextStatus::More) {}
    result
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RetainedTextGrant { pub work: usize, pub allocation_bytes: usize, pub release_bytes: usize }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedTextStatus {
    More, NeedsAllocation { bytes: usize }, NeedsRelease { bytes: usize }, Complete, Cancelled, Closed,
}

pub struct RetainedTextReceipt {
    pub status: Result<RetainedTextStatus, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

#[derive(Clone, Copy)]
enum Stage { ReserveMarks, InitializeMarks, CollectObjects, CollectTrailer, Propagate, BindObjects, BindTrailer, Retire { cancelled: bool }, Complete, Closed }

#[derive(Clone, Copy)]
struct Frame { next_child: usize, entered: bool, role: Option<u8> }

impl Frame {
    const fn new(role: Option<u8>) -> Self { Self { next_child: 0, entered: false, role } }
}

enum Visit { Value { role: Option<u8>, reference: Option<ObjRef>, string: bool }, Move, Done }

struct Walk {
    frames: [Frame; 64],
    depth: usize,
}

impl Default for Walk {
    fn default() -> Self { Self { frames: [Frame::new(None); 64], depth: 0 } }
}

impl Walk {
    fn value<'a>(&self, mut value: &'a PdfObject) -> Option<&'a PdfObject> {
        for frame in &self.frames[..self.depth] { value = child(value, frame.next_child - 1)?.0; }
        Some(value)
    }

    fn value_mut<'a>(&self, mut value: &'a mut PdfObject) -> Option<&'a mut PdfObject> {
        for frame in &self.frames[..self.depth] {
            let index = frame.next_child - 1;
            value = match value {
                PdfObject::Array(items) => items.get_mut(index)?,
                PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => &mut entries.get_mut(index)?.value,
                _ => return None,
            };
        }
        Some(value)
    }

    fn step(&mut self, root: &PdfObject, context: Option<u8>) -> Option<Visit> {
        let value = self.value(root)?;
        let depth = self.depth;
        let frame = &mut self.frames[depth];
        if !frame.entered {
            if depth == 0 { frame.role = context; }
            frame.entered = true;
            return Some(Visit::Value { role: frame.role, reference: value.as_ref(), string: matches!(value, PdfObject::Str(_)) });
        }
        if depth + 1 < self.frames.len() {
            let frame = &mut self.frames[depth];
            if let Some((_, key)) = child(value, frame.next_child) {
                let inherited = match key { Some(key) => role(key), None => frame.role };
                frame.next_child += 1;
                self.depth += 1;
                self.frames[self.depth] = Frame::new(inherited);
                return Some(Visit::Move);
            }
        }
        if depth == 0 {
            self.frames[0] = Frame::new(None);
            Some(Visit::Done)
        } else {
            self.depth -= 1;
            Some(Visit::Move)
        }
    }
}

fn child(value: &PdfObject, index: usize) -> Option<(&PdfObject, Option<&str>)> {
    match value {
        PdfObject::Array(items) => Some((items.get(index)?, None)),
        PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => {
            let field = entries.get(index)?;
            Some((&field.value, Some(&field.key)))
        }
        _ => None,
    }
}

enum ConvertStage { Measure, Allocate, Encode, Replace }

struct Conversion {
    stage: ConvertStage,
    role: u8,
    cursor: PdfTextScalarCursor,
    length: usize,
    text: String,
}

impl Conversion {
    fn new(bytes: &[u8], role: u8) -> Self {
        Self { stage: ConvertStage::Measure, role, cursor: scalars(bytes, role), length: 0, text: String::new() }
    }

    fn advance(&mut self, value: &mut PdfObject, grant: RetainedTextGrant) -> (RetainedTextReceipt, bool) {
        use RetainedTextStatus as Status;
        let PdfObject::Str(bytes) = value else { return (failure("retained PDF text source changed"), false); };
        match self.stage {
            ConvertStage::Measure => {
                if let Some(scalar) = self.cursor.next(bytes) {
                    let Some(length) = self.length.checked_add(scalar.len_utf8()) else {
                        return (failure("retained PDF text length overflow"), false);
                    };
                    self.length = length;
                } else { self.stage = ConvertStage::Allocate; }
            }
            ConvertStage::Allocate => {
                if self.length > grant.allocation_bytes { return (report(Status::NeedsAllocation { bytes: self.length }, 0), false); }
                if self.text.try_reserve_exact(self.length).is_err() { return (failure("unable to allocate retained PDF text"), false); }
                self.cursor = scalars(bytes, self.role);
                self.stage = ConvertStage::Encode;
                return (RetainedTextReceipt { allocated_bytes: self.text.capacity(), ..more() }, false);
            }
            ConvertStage::Encode => {
                if let Some(scalar) = self.cursor.next(bytes) {
                    if self.text.len() + scalar.len_utf8() > self.length { return (failure("retained PDF text source changed"), false); }
                    self.text.push(scalar);
                } else {
                    if self.text.len() != self.length { return (failure("retained PDF text source changed"), false); }
                    self.stage = ConvertStage::Replace;
                }
            }
            ConvertStage::Replace => {
                let date = if self.role == 1 { super::date::parse_pdf_date(&self.text) } else { None };
                let released_bytes = bytes.capacity() + if date.is_some() { self.text.capacity() } else { 0 };
                if released_bytes > grant.release_bytes { return (report(Status::NeedsRelease { bytes: released_bytes }, 0), false); }
                *value = match date {
                    Some(date) => { self.text = String::new(); PdfObject::Date(date) }
                    None => PdfObject::Text(std::mem::take(&mut self.text)),
                };
                return (RetainedTextReceipt { released_bytes, ..more() }, true);
            }
        }
        (more(), false)
    }
}

fn scalars(bytes: &[u8], role: u8) -> PdfTextScalarCursor {
    if role == 2 { PdfTextScalarCursor::uri() } else { PdfTextScalarCursor::text(bytes) }
}

/// 🪪️ Discovers native text roles over a stable retained graph and converts each string under separate grants.
pub struct RetainedTextCursor {
    stage: Stage,
    maximum_objects: usize,
    index: usize,
    walk: Walk,
    marks: Vec<u8>,
    queue: Vec<(usize, u8)>,
    replacement: Option<Vec<(usize, u8)>>,
    pending_role: Option<(usize, u8)>,
    conversion: Option<Conversion>,
}

impl RetainedTextCursor {
    pub fn new(maximum_objects: usize) -> Self {
        Self { stage: Stage::ReserveMarks, maximum_objects, index: 0, walk: Walk::default(), marks: Vec::new(), queue: Vec::new(), replacement: None, pending_role: None, conversion: None }
    }

    pub fn advance(
        &mut self, objects: &mut [PdfIndirectObject], trailer: &mut [PdfDictEntry], position: impl Fn(ObjRef) -> Option<usize>,
        grant: RetainedTextGrant, cancelled: bool,
    ) -> RetainedTextReceipt {
        use RetainedTextStatus as Status;
        if cancelled { self.begin_close(); }
        match self.stage {
            Stage::Complete => return report(Status::Complete, 0),
            Stage::Closed => return report(Status::Closed, 0),
            Stage::Retire { cancelled: true } => return report(Status::Cancelled, 0),
            _ if grant.work == 0 => return report(Status::More, 0),
            _ => {}
        }
        if objects.len() != self.maximum_objects { return failure("retained PDF text graph changed"); }
        if self.pending_role.is_some() { return self.enqueue(grant); }
        match self.stage {
            Stage::ReserveMarks => {
                let step = reserve(&mut self.marks, self.maximum_objects, grant);
                if step.status == Ok(Status::More) { self.stage = Stage::InitializeMarks; }
                step
            }
            Stage::InitializeMarks => {
                if self.marks.len() == self.maximum_objects { self.stage = Stage::CollectObjects; }
                else { self.marks.push(0); }
                more()
            }
            Stage::CollectObjects | Stage::CollectTrailer | Stage::Propagate => self.collect(objects, trailer, position),
            Stage::BindObjects | Stage::BindTrailer => self.bind(objects, trailer, grant),
            Stage::Retire { cancelled: false } => self.retire(grant, false),
            _ => unreachable!(),
        }
    }

    fn collect(&mut self, objects: &[PdfIndirectObject], trailer: &[PdfDictEntry], position: impl Fn(ObjRef) -> Option<usize>) -> RetainedTextReceipt {
        let (value, context) = match self.stage {
            Stage::CollectObjects => {
                let Some(object) = objects.get(self.index) else { return self.transition(Stage::CollectTrailer); };
                (&object.value, None)
            }
            Stage::CollectTrailer => {
                let Some(entry) = trailer.get(self.index) else { return self.transition(Stage::Propagate); };
                (&entry.value, role(&entry.key))
            }
            Stage::Propagate => {
                let Some(&(index, context)) = self.queue.get(self.index) else { return self.transition(Stage::BindObjects); };
                (&objects[index].value, Some(context))
            }
            _ => unreachable!(),
        };
        match self.walk.step(value, context) {
            Some(Visit::Value { role: Some(role), reference: Some(reference), .. }) => {
                if let Some(index) = position(reference).filter(|&index| objects.get(index).is_some_and(|object| object.id == reference)) {
                    let mask = 1 << role;
                    if self.marks[index] & mask == 0 {
                        self.marks[index] |= mask;
                        self.pending_role = Some((index, role));
                    }
                }
            }
            Some(Visit::Done) => self.index += 1,
            Some(_) => {}
            None => return failure("retained PDF text traversal changed"),
        }
        more()
    }

    fn enqueue(&mut self, grant: RetainedTextGrant) -> RetainedTextReceipt {
        let length = self.queue.len();
        if length < self.queue.capacity() {
            self.queue.push(self.pending_role.take().expect("discovered PDF text role"));
            return more();
        }
        if let Some(replacement) = self.replacement.as_mut() {
            if let Some(&entry) = self.queue.get(replacement.len()) {
                replacement.push(entry);
                return more();
            }
            let step = release(&mut self.queue, grant);
            if step.status == Ok(RetainedTextStatus::More) { self.queue = self.replacement.take().expect("complete PDF text role queue"); }
            return step;
        }
        let Some(maximum) = self.maximum_objects.checked_mul(3) else { return failure("PDF text role queue length overflow"); };
        let capacity = length.saturating_mul(2).max(16).min(maximum);
        if capacity <= length { return failure("PDF text role queue exceeds the admitted object count"); }
        let mut replacement = Vec::new();
        let step = reserve(&mut replacement, capacity, grant);
        if step.status == Ok(RetainedTextStatus::More) { self.replacement = Some(replacement); }
        step
    }

    fn bind(&mut self, objects: &mut [PdfIndirectObject], trailer: &mut [PdfDictEntry], grant: RetainedTextGrant) -> RetainedTextReceipt {
        let (root, context) = match self.stage {
            Stage::BindObjects => {
                let Some(object) = objects.get_mut(self.index) else { return self.transition(Stage::BindTrailer); };
                (&mut object.value, priority(self.marks[self.index]))
            }
            Stage::BindTrailer => {
                let Some(entry) = trailer.get_mut(self.index) else { return self.transition(Stage::Retire { cancelled: false }); };
                (&mut entry.value, role(&entry.key))
            }
            _ => unreachable!(),
        };
        if let Some(conversion) = self.conversion.as_mut() {
            let Some(value) = self.walk.value_mut(root) else { return failure("retained PDF transcode location changed"); };
            let (step, finished) = conversion.advance(value, grant);
            if finished { self.conversion = None; }
            return step;
        }
        match self.walk.step(root, context) {
            Some(Visit::Value { role: Some(role), string: true, .. }) => {
                let Some(PdfObject::Str(bytes)) = self.walk.value(root) else { return failure("retained PDF string stopped resolving"); };
                self.conversion = Some(Conversion::new(bytes, role));
            }
            Some(Visit::Done) => self.index += 1,
            Some(_) => {}
            None => return failure("retained PDF text binding location changed"),
        }
        more()
    }

    fn transition(&mut self, stage: Stage) -> RetainedTextReceipt {
        self.index = 0;
        self.stage = stage;
        more()
    }

    pub fn begin_close(&mut self) {
        if !matches!(self.stage, Stage::Complete | Stage::Closed) { self.stage = Stage::Retire { cancelled: true }; }
    }

    pub fn close_step(&mut self, grant: RetainedTextGrant) -> RetainedTextReceipt {
        if matches!(self.stage, Stage::Complete | Stage::Closed) { return report(RetainedTextStatus::Closed, 0); }
        if !matches!(self.stage, Stage::Retire { cancelled: true }) || grant.work == 0 { return report(RetainedTextStatus::More, 0); }
        self.retire(grant, true)
    }

    fn retire(&mut self, grant: RetainedTextGrant, cancelled: bool) -> RetainedTextReceipt {
        use RetainedTextStatus as Status;
        if let Some(conversion) = self.conversion.as_ref() {
            let bytes = conversion.text.capacity();
            if bytes > grant.release_bytes { return report(Status::NeedsRelease { bytes }, 0); }
            self.conversion = None;
            return RetainedTextReceipt { released_bytes: bytes, ..more() };
        }
        if let Some(replacement) = self.replacement.as_mut() {
            let step = release(replacement, grant);
            if step.status == Ok(Status::More) { self.replacement = None; }
            return step;
        }
        if self.queue.capacity() != 0 { return release(&mut self.queue, grant); }
        if self.marks.capacity() != 0 { return release(&mut self.marks, grant); }
        self.stage = if cancelled { Stage::Closed } else { Stage::Complete };
        report(if cancelled { Status::Closed } else { Status::Complete }, 1)
    }
}

fn priority(mark: u8) -> Option<u8> { [1, 0, 2].into_iter().find(|&role| mark & (1 << role) != 0) }

fn reserve<T>(buffer: &mut Vec<T>, length: usize, grant: RetainedTextGrant) -> RetainedTextReceipt {
    let Some(bytes) = length.checked_mul(std::mem::size_of::<T>()) else { return failure("PDF text scratch allocation overflow"); };
    if bytes > grant.allocation_bytes { return report(RetainedTextStatus::NeedsAllocation { bytes }, 0); }
    if buffer.try_reserve_exact(length).is_err() { return failure("unable to allocate the PDF text role scratch"); }
    RetainedTextReceipt { allocated_bytes: buffer.capacity() * std::mem::size_of::<T>(), ..more() }
}

fn release<T>(buffer: &mut Vec<T>, grant: RetainedTextGrant) -> RetainedTextReceipt {
    let bytes = buffer.capacity() * std::mem::size_of::<T>();
    if bytes > grant.release_bytes { return report(RetainedTextStatus::NeedsRelease { bytes }, 0); }
    *buffer = Vec::new();
    RetainedTextReceipt { released_bytes: bytes, ..more() }
}

fn report(status: RetainedTextStatus, work: usize) -> RetainedTextReceipt {
    RetainedTextReceipt { status: Ok(status), work, allocated_bytes: 0, released_bytes: 0 }
}

fn more() -> RetainedTextReceipt { report(RetainedTextStatus::More, 1) }
fn failure(reason: &'static str) -> RetainedTextReceipt { RetainedTextReceipt { status: Err(reason), ..more() } }

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (Vec<PdfIndirectObject>, Vec<PdfDictEntry>) {
        let reference = |num| ObjRef { num, gen: (num == 2) as u16 };
        let object = |num, value| PdfIndirectObject { id: reference(num), value };
        let field = |key: &str, num| PdfDictEntry::new(key, PdfObject::Ref(reference(num)));
        let bytes = |bytes: &[u8]| PdfObject::Str(bytes.to_vec());
        (
            vec![
                object(0, PdfObject::Dict(vec![field("Title", 2), field("CreationDate", 2), field("URI", 1), field("Names", 3), PdfDictEntry::new("Unknown", bytes(b"raw"))])),
                object(1, bytes("https://example.test/é".as_bytes())),
                object(2, bytes(b"D:20261120114500+01'00'")),
                object(3, PdfObject::Array(vec![PdfObject::Ref(reference(4)), bytes(&[0x93])])),
                object(4, PdfObject::Array(vec![PdfObject::Ref(reference(3)), bytes(&[0xFE, 0xFF, 0x00, 0x47, 0xD8, 0x3D, 0xDE, 0x00])])),
                object(5, bytes(b"wrong generation")),
            ],
            vec![PdfDictEntry::new("Title", PdfObject::Ref(ObjRef { num: 5, gen: 8 }))],
        )
    }

    #[test]
    fn one_unit_text_admission_preserves_role_priority_and_reference_generation() {
        let (mut objects, mut trailer) = fixture();
        let mut cursor = RetainedTextCursor::new(objects.len());
        let grant = RetainedTextGrant { work: 1, allocation_bytes: 4096, release_bytes: 4096 };
        let position = |reference: ObjRef| Some(reference.num as usize);
        for _ in 0..100_000 {
            let step = cursor.advance(&mut objects, &mut trailer, position, grant, false);
            assert!(step.work <= 1);
            match step.status.unwrap() {
                RetainedTextStatus::More => {}
                RetainedTextStatus::Complete => {
                    assert!(matches!(objects[2].value, PdfObject::Date(_)));
                    assert_eq!(objects[0].value.dict_get("Unknown").unwrap(), &PdfObject::Str(b"raw".to_vec()));
                    assert_eq!(objects[1].value, PdfObject::Text("https://example.test/é".into()));
                    assert_eq!(objects[3].value.as_array().unwrap()[1], PdfObject::Text("ﬁ".into()));
                    assert_eq!(objects[4].value.as_array().unwrap()[1], PdfObject::Text("G😀".into()));
                    assert_eq!(objects[5].value, PdfObject::Str(b"wrong generation".to_vec()));
                    return;
                }
                status => panic!("unexpected text admission status {status:?}"),
            }
        }
        panic!("text admission did not settle");
    }

    #[test]
    fn cancelling_transcode_leaves_original_string_and_returns_scratch_capacity() {
        let (mut objects, mut trailer) = fixture();
        let mut cursor = RetainedTextCursor::new(objects.len());
        let grant = RetainedTextGrant { work: 1, allocation_bytes: 4096, release_bytes: 4096 };
        let position = |reference: ObjRef| Some(reference.num as usize);
        let mut retained = 0usize;
        let mut started = false;
        for _ in 0..100_000 {
            let step = cursor.advance(&mut objects, &mut trailer, position, grant, false);
            retained += step.allocated_bytes;
            retained -= step.released_bytes;
            if cursor.conversion.as_ref().is_some_and(|conversion| !conversion.text.is_empty()) { started = true; break; }
        }
        assert!(started);
        assert_eq!(cursor.advance(&mut objects, &mut trailer, position, grant, true).status.unwrap(), RetainedTextStatus::Cancelled);
        while cursor.close_step(RetainedTextGrant { release_bytes: 0, ..grant }).status.unwrap() == RetainedTextStatus::More {}
        for _ in 0..100_000 {
            let step = cursor.close_step(grant);
            retained -= step.released_bytes;
            if step.status.unwrap() == RetainedTextStatus::Closed {
                assert_eq!(retained, 0);
                assert_eq!(objects[1].value, PdfObject::Str("https://example.test/é".as_bytes().to_vec()));
                return;
            }
        }
        panic!("redacted text scratch did not retire");
    }
}
