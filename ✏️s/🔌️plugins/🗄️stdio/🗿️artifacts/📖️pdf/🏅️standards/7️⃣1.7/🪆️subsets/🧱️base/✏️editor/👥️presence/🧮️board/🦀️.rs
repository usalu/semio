//! 🧮️ The collaborator board: the fold of presence events into one bounded table of peers, with away marking, expiry and reconnect, and the overlay projected from it.
//! [Presence in collaborative editing](https://www.w3.org/TR/activitystreams-core/)

use crate::PdfPresence;
use semio_ui_locale::Locale;
use serde_json::{json, Value};
use std::collections::BTreeMap;

//#region 🧮️Board
/// ⏱️ A peer unheard for this long is shown as away; it equals the framework's presence time to live.
pub const AWAY_AFTER_MS: u64 = 4_000;
/// 🗑️ A peer unheard for this long, or a connection lost for this long, is forgotten.
pub const EXPIRE_AFTER_MS: u64 = 30_000;
/// 🔁️ After a reconnect a peer that has not been heard again within this long is forgotten.
pub const RECONNECT_GRACE_MS: u64 = 10_000;
/// 📏️ The most peers the board keeps, as many as the framework's peer table.
pub const PEER_SLOTS: usize = 64;
/// 🎨️ Colour-blind safe palette (Okabe-Ito); the peer's colour index selects an entry, so the same collaborator has the same colour on every screen.
pub const PALETTE: [[u8; 3]; 8] = [[0, 114, 178], [213, 94, 0], [0, 158, 115], [204, 121, 167], [230, 159, 0], [86, 180, 233], [240, 228, 66], [153, 153, 153]];

/// 👤️ What one heartbeat of one collaborator carries: who, in which colour, and the lane presence plus the objects selected on the framework's typed interaction channel.
#[derive(Clone, Debug, PartialEq)]
pub struct PeerSample {
    pub actor: String,
    pub label: String,
    pub color: Option<u8>,
    pub presence: PdfPresence,
    pub selection: Vec<String>,
}

/// 📨️ Every fact the board folds.
#[derive(Clone, Debug, PartialEq)]
pub enum PresenceEvent {
    Heard { at_ms: u64, sample: PeerSample },
    Left { actor: String },
    Disconnected { at_ms: u64 },
    Reconnected { at_ms: u64 },
    Tick { at_ms: u64 },
}

#[derive(Clone, Debug, PartialEq)]
struct Entry {
    sample: PeerSample,
    heard_ms: u64,
}

/// 🧮️ The folded table of peers; folding is deterministic and order-insensitive for stale heartbeats.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PeerBoard {
    entries: BTreeMap<String, Entry>,
    offline_since: Option<u64>,
    reconnected: Option<(u64, u64)>,
}

impl PeerBoard {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn offline(&self) -> bool {
        self.offline_since.is_some()
    }

    pub fn fold(&mut self, event: PresenceEvent) {
        match event {
            PresenceEvent::Heard { at_ms, sample } => {
                if let Some(since) = self.offline_since {
                    self.reconnect(at_ms.max(since));
                }
                if self.entries.get(&sample.actor).is_some_and(|entry| entry.heard_ms > at_ms) {
                    return;
                }
                if !self.entries.contains_key(&sample.actor) && self.entries.len() >= PEER_SLOTS {
                    if let Some(oldest) = self.entries.iter().min_by_key(|(actor, entry)| (entry.heard_ms, (*actor).clone())).map(|(actor, _)| actor.clone()) {
                        self.entries.remove(&oldest);
                    }
                }
                self.entries.insert(sample.actor.clone(), Entry { sample, heard_ms: at_ms });
            }
            PresenceEvent::Left { actor } => {
                self.entries.remove(&actor);
            }
            PresenceEvent::Disconnected { at_ms } => {
                self.offline_since.get_or_insert(at_ms);
                self.reconnected = None;
            }
            PresenceEvent::Reconnected { at_ms } => self.reconnect(at_ms),
            PresenceEvent::Tick { at_ms } => self.tick(at_ms),
        }
    }

    fn reconnect(&mut self, at_ms: u64) {
        if self.offline_since.take().is_some() {
            self.reconnected = Some((at_ms, at_ms + RECONNECT_GRACE_MS));
        }
    }

    fn tick(&mut self, now: u64) {
        if let Some(since) = self.offline_since {
            if now.saturating_sub(since) >= EXPIRE_AFTER_MS {
                self.entries.clear();
            }
            return;
        }
        self.entries.retain(|_, entry| now.saturating_sub(entry.heard_ms) < EXPIRE_AFTER_MS);
        if let Some((at, until)) = self.reconnected {
            if now >= until {
                self.entries.retain(|_, entry| entry.heard_ms >= at);
                self.reconnected = None;
            }
        }
    }

    fn status(&self, entry: &Entry, now: u64) -> PeerStatus {
        let resyncing = self.reconnected.is_some_and(|(at, _)| entry.heard_ms < at);
        if self.offline_since.is_some() || resyncing || now.saturating_sub(entry.heard_ms) >= AWAY_AFTER_MS { PeerStatus::Away } else { PeerStatus::Live }
    }

    pub fn overlay(&self, now_ms: u64, local_page: u32) -> PeerOverlay {
        let mut overlay = PeerOverlay { page: local_page, ..Default::default() };
        for entry in self.entries.values() {
            let mark = PeerMark::of(&entry.sample, self.status(entry, now_ms));
            let presence = &entry.sample.presence;
            overlay.roster.push(PeerRow { mark: mark.clone(), page: presence.page, selected: entry.sample.selection.len() as u32, caret: !presence.caret_object.is_empty() });
            overlay.thumbnails.entry(presence.page).or_default().push(mark.clone());
            if presence.page != local_page {
                continue;
            }
            for object in &entry.sample.selection {
                overlay.selections.push(SelectionMark { mark: mark.clone(), object: object.clone() });
            }
            if mark.status != PeerStatus::Live {
                continue;
            }
            if presence.pointer_active {
                overlay.pointers.push(PointerMark { mark: mark.clone(), x: presence.pointer_x.min(1000), y: presence.pointer_y.min(1000) });
            }
            if presence.view_right > presence.view_left && presence.view_bottom > presence.view_top {
                overlay.viewports.push(ViewportMark { mark: mark.clone(), left: presence.view_left.min(1000), top: presence.view_top.min(1000), right: presence.view_right.min(1000), bottom: presence.view_bottom.min(1000) });
            }
            if !presence.caret_object.is_empty() {
                overlay.carets.push(CaretMark { mark, object: presence.caret_object.clone(), offset: presence.caret_offset, anchor: presence.caret_anchor });
            }
        }
        overlay
    }
}

/// 🟢️ Whether a peer is currently heard (`Live`) or only remembered (`Away`: unheard, offline or awaiting its first heartbeat after a reconnect).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeerStatus {
    Live,
    Away,
}

/// 🏷️ Who a mark belongs to: name, palette index and status. Colour is never the only carrier: every mark has the name and every rendering states it as text.
#[derive(Clone, Debug, PartialEq)]
pub struct PeerMark {
    pub actor: String,
    pub label: String,
    pub color: u8,
    pub status: PeerStatus,
}

impl PeerMark {
    fn of(sample: &PeerSample, status: PeerStatus) -> Self {
        let color = sample.color.unwrap_or_else(|| sample.actor.bytes().fold(0u8, |sum, byte| sum.wrapping_add(byte))) % PALETTE.len() as u8;
        Self { actor: sample.actor.clone(), label: if sample.label.is_empty() { sample.actor.clone() } else { sample.label.clone() }, color, status }
    }

    /// 🔤️ The name as text, with the away state spelled out, so a mark never depends on colour alone.
    pub fn caption(&self, locale: Locale) -> String {
        match (self.status, locale) {
            (PeerStatus::Live, _) => self.label.clone(),
            (PeerStatus::Away, Locale::En) => format!("{} (away)", self.label),
            (PeerStatus::Away, Locale::De) => format!("{} (abwesend)", self.label),
        }
    }

    pub fn rgb(&self) -> [u8; 3] {
        PALETTE[usize::from(self.color) % PALETTE.len()]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PointerMark {
    pub mark: PeerMark,
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ViewportMark {
    pub mark: PeerMark,
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CaretMark {
    pub mark: PeerMark,
    pub object: String,
    pub offset: u32,
    pub anchor: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectionMark {
    pub mark: PeerMark,
    pub object: String,
}

/// 📋️ One roster line: a peer, its page and how much it selects.
#[derive(Clone, Debug, PartialEq)]
pub struct PeerRow {
    pub mark: PeerMark,
    pub page: u32,
    pub selected: u32,
    pub caret: bool,
}

/// 🖼️ Everything a screen draws of the collaborators: marks on the local page, a badge per page for the thumbnail strip and the roster.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PeerOverlay {
    pub page: u32,
    pub pointers: Vec<PointerMark>,
    pub viewports: Vec<ViewportMark>,
    pub carets: Vec<CaretMark>,
    pub selections: Vec<SelectionMark>,
    pub thumbnails: BTreeMap<u32, Vec<PeerMark>>,
    pub roster: Vec<PeerRow>,
}

impl PeerOverlay {
    pub fn is_empty(&self) -> bool {
        self.roster.is_empty()
    }

    /// 🔤️ The marks on `page`, for the thumbnail strip row of that page.
    pub fn on_page(&self, page: u32) -> &[PeerMark] {
        self.thumbnails.get(&page).map_or(&[], Vec::as_slice)
    }

    /// ♿️ One sentence per peer for a live region and the thumbnail rows, in the requested language.
    pub fn describe(&self, locale: Locale) -> Vec<String> {
        self.roster.iter().map(|row| describe_row(row, locale)).collect()
    }

    /// 🧾️ The language-neutral JSON form the fixtures compare.
    pub fn to_json(&self) -> Value {
        let mark = |mark: &PeerMark| json!({ "actor": mark.actor, "label": mark.label, "color": mark.color, "status": status_name(mark.status) });
        json!({
            "page": self.page,
            "pointers": self.pointers.iter().map(|m| json!({ "peer": mark(&m.mark), "x": m.x, "y": m.y })).collect::<Vec<_>>(),
            "viewports": self.viewports.iter().map(|m| json!({ "peer": mark(&m.mark), "rect": [m.left, m.top, m.right, m.bottom] })).collect::<Vec<_>>(),
            "carets": self.carets.iter().map(|m| json!({ "peer": mark(&m.mark), "object": m.object, "offset": m.offset, "anchor": m.anchor })).collect::<Vec<_>>(),
            "selections": self.selections.iter().map(|m| json!({ "peer": mark(&m.mark), "object": m.object })).collect::<Vec<_>>(),
            "thumbnails": self.thumbnails.iter().map(|(page, marks)| (page.to_string(), Value::Array(marks.iter().map(mark).collect()))).collect::<serde_json::Map<_, _>>(),
            "roster": self.roster.iter().map(|r| json!({ "peer": mark(&r.mark), "page": r.page, "selected": r.selected, "caret": r.caret })).collect::<Vec<_>>(),
        })
    }
}

fn status_name(status: PeerStatus) -> &'static str {
    match status {
        PeerStatus::Live => "live",
        PeerStatus::Away => "away",
    }
}

fn describe_row(row: &PeerRow, locale: Locale) -> String {
    let page = row.page + 1;
    let away = row.mark.status == PeerStatus::Away;
    match locale {
        Locale::De => {
            let mut text = format!("{} ist auf Seite {page}", row.mark.label);
            if row.selected > 0 { text.push_str(&format!(", {} {} ausgewählt", row.selected, if row.selected == 1 { "Objekt" } else { "Objekte" })); }
            if row.caret { text.push_str(", Textcursor gesetzt"); }
            if away { text.push_str(" (abwesend)"); }
            text
        }
        Locale::En => {
            let mut text = format!("{} is on page {page}", row.mark.label);
            if row.selected > 0 { text.push_str(&format!(", {} {} selected", row.selected, if row.selected == 1 { "object" } else { "objects" })); }
            if row.caret { text.push_str(", text caret placed"); }
            if away { text.push_str(" (away)"); }
            text
        }
    }
}

impl PresenceEvent {
    /// 📥️ Parses the language-neutral JSON form of an event.
    pub fn from_json(value: &Value) -> Option<Self> {
        let at = || value["atMs"].as_u64();
        Some(match value["event"].as_str()? {
            "heard" => Self::Heard { at_ms: at()?, sample: PeerSample::from_json(&value["peer"])? },
            "left" => Self::Left { actor: value["actor"].as_str()?.to_string() },
            "disconnected" => Self::Disconnected { at_ms: at()? },
            "reconnected" => Self::Reconnected { at_ms: at()? },
            "tick" => Self::Tick { at_ms: at()? },
            _ => return None,
        })
    }
}

impl PeerSample {
    /// 📥️ Parses the language-neutral JSON form of a heartbeat.
    pub fn from_json(value: &Value) -> Option<Self> {
        let u = |key: &str| value["presence"][key].as_u64().unwrap_or(0) as u32;
        Some(Self {
            actor: value["actor"].as_str()?.to_string(),
            label: value["label"].as_str().unwrap_or("").to_string(),
            color: value["color"].as_u64().map(|color| color as u8),
            presence: PdfPresence { page: u("page"), pointer_active: value["presence"]["pointerActive"].as_bool().unwrap_or(false), pointer_x: u("pointerX"), pointer_y: u("pointerY"), view_left: u("viewLeft"), view_top: u("viewTop"), view_right: u("viewRight"), view_bottom: u("viewBottom"), caret_object: value["presence"]["caretObject"].as_str().unwrap_or("").to_string(), caret_offset: u("caretOffset"), caret_anchor: u("caretAnchor") },
            selection: value["selection"].as_array().map(|ids| ids.iter().filter_map(|id| id.as_str().map(str::to_string)).collect()).unwrap_or_default(),
        })
    }
}

/// 📄️ The zero-based page a page-scoped id such as `p3.o12` or `p3:paper` names, for ids that carry their page.
pub fn page_of_object(id: &str) -> Option<u32> {
    let digits: String = id.strip_prefix('p')?.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

/// 🕹️ The collaborators the framework's typed interaction channel already delivers to every render: who selects which objects of `domain`. A peer is placed on the page of its first selected object; a peer that selects nothing says nothing about its page and is left out until its presence heartbeat arrives.
pub fn interaction_samples(interaction: &semio_plugin::app::InteractionView<'_>, domain: &str, page_of: &dyn Fn(&str) -> Option<u32>) -> Vec<PeerSample> {
    interaction.peers.iter().filter_map(|(actor, peer)| {
        let selection = peer.interaction.as_ref()?.domains.iter().find(|candidate| candidate.domain == domain)?.selected.clone();
        let page = selection.iter().find_map(|id| page_of(id))?;
        Some(PeerSample { actor: actor.to_string(), label: String::new(), color: peer.color, presence: PdfPresence { page, ..Default::default() }, selection })
    }).collect()
}

/// 👥️ The collaborators of one render with their typed presence: [`interaction_samples`] for who selects what and in which colour, overlaid with each peer's own [`PdfPresence`] (page, pointer, viewport, caret); a peer known only by its presence has no selection.
pub fn typed_samples<'a>(interaction: &semio_plugin::app::InteractionView<'_>, domain: &str, page_of: &dyn Fn(&str) -> Option<u32>, typed: impl Iterator<Item = (&'a str, &'a PdfPresence)>) -> Vec<PeerSample> {
    let mut samples: BTreeMap<String, PeerSample> = interaction_samples(interaction, domain, page_of).into_iter().map(|sample| (sample.actor.clone(), sample)).collect();
    for (actor, presence) in typed {
        let color = interaction.peers.iter().find(|(candidate, _)| *candidate == actor).and_then(|(_, peer)| peer.color);
        let sample = samples.entry(actor.to_string()).or_insert_with(|| PeerSample { actor: actor.to_string(), label: String::new(), color, presence: PdfPresence::default(), selection: Vec::new() });
        sample.presence = presence.clone();
    }
    samples.into_values().collect()
}

impl PeerBoard {
    /// 🌱️ The board of one render: every sample heard just now.
    pub fn of_samples(samples: impl IntoIterator<Item = PeerSample>) -> Self {
        let mut board = Self::default();
        for sample in samples {
            board.fold(PresenceEvent::Heard { at_ms: 0, sample });
        }
        board
    }
}
//#endregion 🧮️Board
