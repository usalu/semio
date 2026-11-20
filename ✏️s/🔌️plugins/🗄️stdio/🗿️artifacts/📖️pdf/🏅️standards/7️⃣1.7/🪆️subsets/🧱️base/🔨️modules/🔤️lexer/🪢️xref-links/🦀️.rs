//! 🪢️ Follows typed revision links in a PDF trailer without rescanning the dictionary.

use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfDictEntry;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct XrefLinks {
    pub previous: Option<usize>,
    pub hybrid: Option<usize>,
}

#[derive(Default)]
pub struct XrefLinkCursor {
    position: usize,
    seen: [bool; 2],
    links: XrefLinks,
}

impl XrefLinkCursor {
    /// 🔎️ Checks at most one dictionary entry per unit before exposing the current revision links.
    pub fn advance(&mut self, entries: &[PdfDictEntry], work: usize) -> (usize, Option<XrefLinks>) {
        let end = self.position.saturating_add(work).min(entries.len());
        let start = self.position;
        for entry in &entries[start..end] {
            let (slot, link) = match entry.key.as_str() {
                "Prev" => (0, &mut self.links.previous),
                "XRefStm" => (1, &mut self.links.hybrid),
                _ => continue,
            };
            if !self.seen[slot] {
                self.seen[slot] = true;
                *link = entry.value.as_i64().and_then(|number| usize::try_from(number).ok());
            }
        }
        self.position = end;
        (end - start, (end == entries.len()).then_some(self.links))
    }
}

/// 🔄️ Detects a cycle in the immutable Prev chain with Brent's constant-space schedule.
#[derive(Default)]
pub struct XrefPrimaryCycle {
    anchor: Option<usize>,
    distance: usize,
    span: usize,
}

impl XrefPrimaryCycle {
    pub fn revisits(&mut self, offset: usize) -> bool {
        if self.anchor == Some(offset) { return true; }
        if self.distance == self.span {
            self.anchor = Some(offset);
            self.distance = 0;
            self.span = self.span.saturating_mul(2).max(1);
        }
        self.distance = self.distance.saturating_add(1);
        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrefTarget { Primary(usize), Companion(usize) }

/// 🪢️ Visits each primary xref followed by its companion before following Prev.
pub struct XrefRevisionSchedule {
    previous: Option<usize>,
    companion: Option<usize>,
}

impl XrefRevisionSchedule {
    pub fn new(start: Option<usize>) -> Self { Self { previous: start, companion: None } }

    pub fn next_target(&mut self) -> Option<XrefTarget> {
        self.companion.take().map(XrefTarget::Companion).or_else(|| self.previous.take().map(XrefTarget::Primary))
    }

    pub fn finish_primary(&mut self, links: XrefLinks) {
        self.previous = links.previous;
        self.companion = links.hybrid;
    }

    pub fn awaits_companion(&self) -> bool { self.companion.is_some() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{CosGrant, CosLimits, CosStatus, PdfCosCursor};

    #[test]
    fn revision_schedule_admits_the_hybrid_companion_before_older_sections() {
        let mut schedule = XrefRevisionSchedule::new(Some(400));
        assert_eq!(schedule.next_target(), Some(XrefTarget::Primary(400)));
        schedule.finish_primary(XrefLinks { previous: Some(200), hybrid: Some(300) });
        assert!(schedule.awaits_companion());
        assert_eq!(schedule.next_target(), Some(XrefTarget::Companion(300)));
        assert_eq!(schedule.next_target(), Some(XrefTarget::Primary(200)));
        schedule.finish_primary(XrefLinks::default());
        assert_eq!(schedule.next_target(), None);
    }

    #[test]
    fn primary_cycle_detection_keeps_unsigned_offsets_and_finds_a_late_cycle() {
        let mut cycle = XrefPrimaryCycle::default();
        for offset in [usize::MAX, 800, 51, 900, 600, 50, 22] {
            assert!(!cycle.revisits(offset));
        }
        assert!([900, 600, 50, 22].into_iter().cycle().take(20).any(|offset| cycle.revisits(offset)));

        let mut cycle = XrefPrimaryCycle::default();
        assert!(!cycle.revisits(32));
        assert!(cycle.revisits(32));
    }

    #[test]
    fn genuine_incremental_trailer_exposes_the_previous_revision() {
        let source = include_bytes!("../../../🧪️tests/🧫️fixtures/🔗️incremental-free/📑️table.pdf");
        let offset = source.windows(7).rposition(|w| w == b"trailer").unwrap() + 7;
        let mut trailer = PdfCosCursor::new(offset, CosLimits::default());
        let grant = CosGrant { work: 1, allocation_bytes: 8192, release_bytes: 8192 };
        loop {
            if matches!(trailer.advance(source, grant, false).status.unwrap(), CosStatus::Complete { .. }) { break }
        }
        let value = trailer.take_value().unwrap();
        let entries = value.as_dict().unwrap();
        let mut links = XrefLinkCursor::default();
        for (i, _) in entries.iter().enumerate() {
            let (_, result) = links.advance(entries, 1);
            assert_eq!(result, (i + 1 == entries.len()).then_some(XrefLinks { previous: Some(703), hybrid: None }));
        }
    }
}
