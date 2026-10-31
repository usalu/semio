//! 🪜️ Grant-driven incremental fold of one directory page: the projection is copied row by row and the page folded event by event, each unit quoted before it runs, so no turn's work grows with the directory. See [`super`].

use super::{directory_event_space, directory_event_user, HomeDirectoryProjection};
use std::collections::BTreeMap;
use std::ops::Bound;
use std::sync::Arc;

pub(super) const SPACE_ENTRY_BYTES: usize = std::mem::size_of::<(Arc<str>, Arc<store::os_directory::DirectorySpace>)>() + 64;
pub(super) const USER_ENTRY_BYTES: usize = 128;
const EVENT_BASE_BYTES: usize = 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Spaces,
    Users,
    Events,
    Done,
}

/// 🧮️ The cost of the work one `step` did, in the same byte units as the quote of each unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HomeDirectoryFoldStep {
    pub charged_bytes: usize,
    pub units: usize,
    pub done: bool,
}

/// 🪜️ One page folding into a copy of the base projection under a byte budget per step.
pub struct HomeDirectoryFold {
    base: Arc<HomeDirectoryProjection>,
    page: store::os_directory::DirectoryEventPageV1,
    next: HomeDirectoryProjection,
    users: BTreeMap<String, store::os_directory::UserView>,
    phase: Phase,
    space_cursor: Option<Arc<str>>,
    user_cursor: Option<String>,
    event: usize,
}

impl HomeDirectoryFold {
    /// 🌱️ An origin page rebuilds from nothing (no rows to copy); any other page continues the base.
    pub fn new(base: Arc<HomeDirectoryProjection>, page: store::os_directory::DirectoryEventPageV1) -> Self {
        let origin = page.after_seq_exclusive == 0;
        let next = HomeDirectoryProjection { cursor: if origin { 0 } else { base.cursor }, ..HomeDirectoryProjection::default() };
        Self { base, page, next, users: BTreeMap::new(), phase: if origin { Phase::Events } else { Phase::Spaces }, space_cursor: None, user_cursor: None, event: 0 }
    }

    /// 📏️ The exact quote of the next indivisible unit, or `None` once the fold is complete.
    pub fn next_quote(&self) -> Option<usize> {
        match self.phase {
            Phase::Spaces => self.next_space().map(|(key, _)| SPACE_ENTRY_BYTES + key.len()).or(Some(0)),
            Phase::Users => self.next_user().map(|(key, _)| USER_ENTRY_BYTES + key.len()).or(Some(0)),
            Phase::Events => self.page.events.get(self.event).map(|event| EVENT_BASE_BYTES + self.event_row_bytes(event)).or(Some(0)),
            Phase::Done => None,
        }
    }

    /// ⏩️ Runs whole units while their quotes fit `budget_bytes`; a unit that does not fit alone makes no progress (`units == 0`, not done).
    pub fn step(&mut self, budget_bytes: usize) -> HomeDirectoryFoldStep {
        let mut charged = 0usize;
        let mut units = 0usize;
        while let Some(quote) = self.next_quote() {
            if quote > budget_bytes.saturating_sub(charged) {
                break;
            }
            charged += quote;
            units += 1;
            self.run_unit();
        }
        HomeDirectoryFoldStep { charged_bytes: charged, units, done: self.phase == Phase::Done }
    }

    /// 📄️ The folded projection once every unit ran.
    pub fn finish(self) -> Option<HomeDirectoryProjection> {
        if self.phase != Phase::Done {
            return None;
        }
        let mut next = self.next;
        next.users = Arc::new(self.users);
        next.cursor = self.page.through_seq_inclusive;
        next.session_binding_sha256 = self.page.session_binding_sha256.clone();
        next.authorization_generation = self.page.authorization_generation;
        next.receipt_sha256 = self.page.receipt_sha256.clone();
        Some(next)
    }

    fn next_space(&self) -> Option<(&Arc<str>, &Arc<store::os_directory::DirectorySpace>)> {
        let after = self.space_cursor.as_ref().map_or(Bound::Unbounded, Bound::Excluded);
        self.base.spaces.range::<Arc<str>, _>((after, Bound::Unbounded)).next()
    }

    fn next_user(&self) -> Option<(&String, &store::os_directory::UserView)> {
        let after = self.user_cursor.as_ref().map_or(Bound::Unbounded, Bound::Excluded);
        self.base.users.range::<String, _>((after, Bound::Unbounded)).next()
    }

    fn event_row_bytes(&self, event: &store::os_directory::DirectoryEvent) -> usize {
        directory_event_space(&event.body).and_then(|id| self.next.spaces.get(id)).map_or(0, |row| super::row_bytes(row))
    }

    fn run_unit(&mut self) {
        match self.phase {
            Phase::Spaces => match self.next_space().map(|(key, row)| (key.clone(), row.clone())) {
                Some((key, row)) => {
                    self.next.spaces.insert(key.clone(), row);
                    self.space_cursor = Some(key);
                }
                None => self.phase = Phase::Users,
            },
            Phase::Users => match self.next_user().map(|(key, user)| (key.clone(), user.clone())) {
                Some((key, user)) => {
                    self.users.insert(key.clone(), user);
                    self.user_cursor = Some(key);
                }
                None => self.phase = Phase::Events,
            },
            Phase::Events => match self.page.events.get(self.event) {
                Some(event) => {
                    self.fold_event(event.clone());
                    self.event += 1;
                }
                None => self.phase = Phase::Done,
            },
            Phase::Done => {}
        }
    }

    fn fold_event(&mut self, event: store::os_directory::DirectoryEvent) {
        let space_id = directory_event_space(&event.body);
        let user_id = directory_event_user(&event.body);
        let mut model = store::os_directory::DirectoryReadModel { spaces: BTreeMap::new(), cursor: self.next.cursor, users: BTreeMap::new() };
        if let Some((id, row)) = space_id.and_then(|id| self.next.spaces.remove(id).map(|row| (id, row))) {
            model.spaces.insert(id.to_owned(), Arc::unwrap_or_clone(row));
        }
        if let Some(user) = user_id.and_then(|id| self.users.get(id)) {
            model.users.insert(user.id.clone(), user.clone());
        }
        let folded = store::os_directory::fold(model, &event);
        self.next.cursor = folded.cursor;
        for (id, space) in folded.spaces {
            self.next.spaces.insert(Arc::from(id.as_str()), Arc::new(space));
        }
        if let store::os_directory::DirectoryEventBody::UserCreated { user_id, .. } = &event.body {
            if let Some(user) = folded.users.get(user_id) {
                self.users.insert(user_id.clone(), user.clone());
            }
        }
    }
}
