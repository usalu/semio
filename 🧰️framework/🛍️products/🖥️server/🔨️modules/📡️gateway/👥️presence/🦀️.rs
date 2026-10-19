//! 👥️ The presence roster and the presence rooms with their settings, readers and admission.

use semio_framework_async::ShardedMap;
use semio_framework_server_contract::{PresenceEntry, PresenceFrame};
use semio_framework_server_throttle::ClientKey;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::hash::{BuildHasher, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex as StdMutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::watch;
use tokio::time::Instant;

//#region 🔖️Presence
/// 🎨️ One actor's held palette slot in a scope, ref-counted across that actor's concurrently open
/// sockets so a second window never steals a third colour.
#[derive(Clone, Debug, PartialEq, Eq)]
struct ColourLease {
    pub index: u8,
    refs: u32,
}

/// 🌈️ One scope's live palette leases. Never persisted, rebuilt from nothing after a restart.
#[derive(Default)]
struct ScopeColours {
    by_actor: BTreeMap<String, ColourLease>,
}

/// 👤️ One connected actor's presence in a scope. `peer` stays opaque — the gateway stores and
/// forwards it, never decodes it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceSession {
    /// 🙋️ Who is present.
    pub actor: String,
    /// 🪟️ Which surface the actor joined from.
    pub surface: String,
    /// 🎨️ The palette slot leased for the life of the session.
    pub colour: u8,
    /// 📦️ The last opaque presence payload the actor published, if any.
    pub peer: Option<Vec<u8>>,
}

/// 👥️ The ephemeral presence registry: who is in which scope, and which of the 256 palette slots
/// each of them holds.
#[derive(Default)]
pub struct Presence {
    colours: ShardedMap<String, ScopeColours>,
    pub sessions: ShardedMap<(String, String), PresenceSession>,
}

impl Presence {
    /// 🐣️ An empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🎨️ The lowest palette slot not currently held in `scope`, or the actor's existing slot with
    /// one more reference on it. Wraps once all 256 slots are taken.
    pub fn acquire_colour(&self, scope: &str, actor: &str) -> u8 {
        self.colours.mutate_or_default(scope.to_string(), |colours| {
            if let Some(lease) = colours.by_actor.get_mut(actor) {
                lease.refs += 1;
                return lease.index;
            }
            let mut taken = [false; 256];
            for lease in colours.by_actor.values() {
                taken[usize::from(lease.index)] = true;
            }
            let index = taken.iter().position(|taken| !taken).unwrap_or(colours.by_actor.len() % 256) as u8;
            colours.by_actor.insert(actor.to_string(), ColourLease { index, refs: 1 });
            index
        })
    }

    /// 🧹️ Drop one reference on the actor's slot, freeing it on the last disconnect.
    pub fn release_colour(&self, scope: &str, actor: &str) {
        self.colours.with_mut(scope, |colours| {
            let Some(colours) = colours else { return };
            let exhausted = match colours.by_actor.get_mut(actor) {
                Some(lease) => {
                    lease.refs = lease.refs.saturating_sub(1);
                    lease.refs == 0
                }
                None => false,
            };
            if exhausted {
                colours.by_actor.remove(actor);
            }
        });
    }

    /// 🔎️ The slot this actor currently holds in `scope`, if any.
    pub fn colour_of(&self, scope: &str, actor: &str) -> Option<u8> {
        self.colours.with(scope, |colours| colours.and_then(|colours| colours.by_actor.get(actor).map(|lease| lease.index)))
    }

    /// 🚪️ Register a session and lease it a colour.
    pub fn join(&self, scope: &str, actor: &str, surface: &str) -> u8 {
        let colour = self.acquire_colour(scope, actor);
        let session = PresenceSession { actor: actor.to_string(), surface: surface.to_string(), colour, peer: None };
        self.sessions.insert((scope.to_string(), actor.to_string()), session);
        colour
    }

    /// 📤️ Record the opaque payload an actor last published.
    pub fn publish_peer(&self, scope: &str, actor: &str, peer: Vec<u8>) {
        self.sessions.with_mut(&(scope.to_string(), actor.to_string()), |session| {
            if let Some(session) = session {
                session.peer = Some(peer);
            }
        });
    }

    /// 🚶️ Remove the session and release its colour.
    pub fn leave(&self, scope: &str, actor: &str) {
        self.sessions.remove(&(scope.to_string(), actor.to_string()));
        self.release_colour(scope, actor);
    }

    /// 📋️ Everyone currently present in `scope`, ordered by actor.
    pub fn roster(&self, scope: &str) -> Vec<PresenceSession> {
        let mut roster = Vec::new();
        self.sessions.for_each(|(entry_scope, _), session| {
            if entry_scope == scope {
                roster.push(session.clone());
            }
        });
        roster.sort_by(|left, right| left.actor.cmp(&right.actor));
        roster
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceRoom
/// 🧍️ The resource presence admission is evaluated on, at [`PolicyPoint::Subscription`] with the
/// actions [`PRESENCE_JOIN`] and [`PRESENCE_PUBLISH`].
pub const PRESENCE_RESOURCE: &str = "presence";

/// 🚪️ The action admitting a principal into a presence room.
pub const PRESENCE_JOIN: &str = "join";

/// 📣️ The action admitting a principal to share a state in a presence room.
pub const PRESENCE_PUBLISH: &str = "publish";

/// 👀️ The action admitting a principal to watch a presence room it has not joined, read-only.
pub const PRESENCE_WATCH: &str = "watch";

/// 🔌️ The websocket subprotocol of the presence socket.
pub const PRESENCE_PROTOCOL_V1: &str = "semio.presence.v1";

/// 🚫️ The `refused` reason of a client frame larger than [`PresenceSettings::max_state_bytes`].
pub const REFUSED_TOO_LARGE: &str = "state-too-large";

/// 🚫️ The `refused` reason of a frame that is neither a `state` nor a `watch` frame.
pub const REFUSED_INVALID: &str = "frame-invalid";

/// 🚫️ The `refused` reason of a state from a principal policy does not let publish, and — followed
/// by the scope — of a watch naming a scope policy does not let it watch.
pub const REFUSED_FORBIDDEN: &str = "forbidden";

/// 🚫️ The `refused` reason of a watch naming more than [`PresenceSettings::max_watch_scopes`] scopes.
pub const REFUSED_WATCH_TOO_MANY: &str = "watch-too-many";

/// ⏱️ The knobs of the presence socket. The defaults are the protocol's (design §15).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PresenceSettings {
    /// 🕰️ How often, at most, a member is sent what changed in its room: one `batch` per tick.
    pub tick: Duration,
    /// 💤️ How long a socket may stay silent (no frame, no pong) before it is closed.
    pub idle: Duration,
    /// 🏓️ How often the server pings every socket.
    pub keepalive: Duration,
    /// 📏️ The largest client frame (`state` or `watch`) accepted, in bytes.
    pub max_state_bytes: usize,
    /// 🚦️ How many client frames per second a session may send; the excess is dropped.
    pub max_states_per_second: u32,
    /// 🏷️ The longest `surface` a session may join with, in characters.
    pub max_surface_chars: usize,
    /// 👀️ The most scopes one `watch` may name.
    pub max_watch_scopes: usize,
    /// 🐢️ The slowest interval a watcher may ask for; the fastest is [`tick`](Self::tick).
    pub max_watch_interval: Duration,
    /// 🖼️ The most bytes of entries and departures one server frame carries: what a room sends one
    /// member per [`tick`](Self::tick), and what a socket is sent per watch interval over everything
    /// it watches. A roster or a burst of changes larger than that continues in the next frames. An
    /// entry is never split, so a frame holding one larger entry exceeds it.
    pub max_frame_bytes: usize,
    /// 📡️ The most bytes of entries and departures all rooms of the instance together send per
    /// second, a quarter of a second's worth at once; `None` bounds nothing. The budget is dealt
    /// fairly among the client addresses whose sockets read: an address is never held back while it
    /// takes less than an even split, and one that asks for more than its fair share — through
    /// however many sockets — waits by itself, each of its sockets longer for its next frame, which
    /// then carries the newest states. Only the `welcome` of a joining session is sent at once (and
    /// counted): the rate sessions join at is bounded by the instance's edge ([`Limits::upgrades`]).
    pub max_bytes_per_second: Option<u64>,
    /// 🪦️ How many departures a room remembers. A reader that missed more is sent the roster anew.
    pub departures_kept: usize,
}

impl PresenceSettings {
    /// 📏️ The most the presence socket's protocol layer takes of one inbound message: twice the
    /// largest state, so a frame somewhat over the limit is still read and answered
    /// [`REFUSED_TOO_LARGE`], while anything larger fails the socket before it is buffered.
    pub fn inbound_bytes(&self) -> usize {
        self.max_state_bytes.saturating_mul(2)
    }
}

impl Default for PresenceSettings {
    fn default() -> Self {
        Self {
            tick: Duration::from_millis(100),
            idle: Duration::from_secs(60),
            keepalive: Duration::from_secs(20),
            max_state_bytes: 2048,
            max_states_per_second: 30,
            max_surface_chars: 64,
            max_watch_scopes: 16,
            max_watch_interval: Duration::from_secs(60),
            max_frame_bytes: 4096,
            max_bytes_per_second: Some(16 * 1024 * 1024),
            departures_kept: 256,
        }
    }
}

/// 🌍️ The instance's answer to "may a browser page served from this origin open a socket here".
/// Checked for every presence socket that presents an `Origin`; an instance that sets none admits
/// every origin, which is right only where nothing but loopback can reach the process.
pub type OriginAdmission = Arc<dyn Fn(&str) -> bool + Send + Sync>;

/// 🧍️ One member of a presence room: its session and its entry as every reader is sent it.
#[derive(Debug)]
pub struct Member {
    pub session: String,
    pub wire: Box<str>,
}

/// 🏠️ One room: its members on numbered seats, the clock every join, state and departure
/// advances, the departures it still remembers (and the clock of the newest one it forgot), and
/// how many sockets read it.
///
/// A member keeps its seat while it is in the room, and a vacated seat is the next one taken.
/// `changed` is a tournament over the seats — the leaves hold the clock each seat last changed at
/// (`0` while vacant), every node above the newer of its two halves — so the next seat that
/// changed after a given clock is found in a number of steps logarithmic in the room's size
/// ([`Room::next_changed`]), which is what lets every reader go round a room of any size at the
/// cost of what it is sent.
#[derive(Debug)]
pub struct Room {
    pub seats: Vec<Option<Member>>,
    pub changed: Vec<u64>,
    vacant: Vec<usize>,
    pub seated: HashMap<String, usize>,
    departed: VecDeque<(u64, String)>,
    forgotten: u64,
    pub clock: u64,
    pub readers: usize,
    changes: watch::Sender<u64>,
}

impl Default for Room {
    fn default() -> Self {
        Self { seats: Vec::new(), changed: Vec::new(), vacant: Vec::new(), seated: HashMap::new(), departed: VecDeque::new(), forgotten: 0, clock: 0, readers: 0, changes: watch::channel(0).0 }
    }
}

impl Room {
    /// ⏭️ Advance the clock and wake every reader waiting for a change.
    fn advance(&mut self) -> u64 {
        self.clock += 1;
        self.changes.send_replace(self.clock);
        self.clock
    }

    /// 🫙️ Whether nobody is in the room and nobody reads it.
    fn deserted(&self) -> bool {
        self.seated.is_empty() && self.readers == 0
    }

    /// 🪑️ Seat `member` (or replace the entry on the seat its session holds) at the next clock.
    pub fn seat(&mut self, member: Member) {
        let seat = match self.seated.get(&member.session) {
            Some(seat) => *seat,
            None => {
                if self.vacant.is_empty() {
                    self.grow();
                }
                let seat = self.vacant.pop().unwrap_or_default();
                self.seated.insert(member.session.clone(), seat);
                seat
            }
        };
        self.seats[seat] = Some(member);
        let clock = self.advance();
        self.mark(seat, clock);
    }

    /// 🧳️ Vacate `session`'s seat at the next clock and remember the departure, forgetting the
    /// oldest ones past `kept`; `false` when the session holds no seat.
    pub fn vacate(&mut self, session: &str, kept: usize) -> bool {
        let Some(seat) = self.seated.remove(session) else { return false };
        self.seats[seat] = None;
        self.vacant.push(seat);
        self.mark(seat, 0);
        let clock = self.advance();
        self.departed.push_back((clock, session.to_string()));
        while self.departed.len() > kept {
            if let Some((clock, _)) = self.departed.pop_front() {
                self.forgotten = clock;
            }
        }
        true
    }

    /// 📈️ Double the seats (one, in an empty room) and rebuild the tournament over them.
    fn grow(&mut self) {
        let seats = (self.seats.len() * 2).max(1);
        let mut changed = vec![0; seats * 2];
        changed[seats..seats + self.seats.len()].copy_from_slice(&self.changed[self.seats.len()..]);
        for node in (1..seats).rev() {
            changed[node] = changed[node * 2].max(changed[node * 2 + 1]);
        }
        self.vacant.extend((self.seats.len()..seats).rev());
        self.seats.resize_with(seats, || None);
        self.changed = changed;
    }

    /// 📍️ Record that `seat` changed at `clock` (`0`: it is vacant).
    fn mark(&mut self, seat: usize, clock: u64) {
        let mut node = self.seats.len() + seat;
        self.changed[node] = clock;
        while node > 1 {
            node /= 2;
            self.changed[node] = self.changed[node * 2].max(self.changed[node * 2 + 1]);
        }
    }

    /// 🔎️ The first seat at or after `from` that changed after `clock`.
    pub fn next_changed(&self, from: usize, clock: u64) -> Option<usize> {
        let seats = self.seats.len();
        if from >= seats {
            return None;
        }
        let mut node = seats + from;
        loop {
            if self.changed[node] > clock {
                while node < seats {
                    node = if self.changed[node * 2] > clock { node * 2 } else { node * 2 + 1 };
                }
                return Some(node - seats);
            }
            while node % 2 == 1 {
                node /= 2;
            }
            if node == 0 {
                return None;
            }
            node += 1;
        }
    }
}

/// ⏲️ How often the rooms' budget is dealt anew among the client addresses that read: long enough
/// to measure what an address takes, short enough that a flood is met within a second.
pub const SHARE_EPOCH: Duration = Duration::from_millis(250);

/// 🍰️ What one client address may still take of the rooms' budget: its equal part and its fair
/// part — two levels of bytes like the whole's ([`Outflow`]) —, how many of its readers are alive,
/// and what the next deal goes by: the bytes it drew since the last deal, the rate measured up to
/// that deal, and whether a page was refused it since the last deal or during the one before.
#[derive(Debug)]
pub struct Share {
    pub readers: usize,
    equal: i128,
    fair: i128,
    filled: Instant,
    pub drawn: u64,
    pub rate: u64,
    pub held: bool,
    was_held: bool,
}

/// 🚰️ What all rooms together may still send, and what each client address may take of it.
///
/// The whole is a level of bytes — counted in billionths, so that a nanosecond of flow is a whole
/// number — that fills at the instance's presence budget up to a quarter of a second's worth and
/// that every page draws what it carries from. Every address that reads holds two such levels of
/// its own ([`Share`]). Its **equal part** fills at the budget divided by the number of addresses
/// reading, and a page is never refused while it is above nothing — whatever the other addresses
/// take. Its **fair part** fills at the rate the budget was last dealt at, and past its equal part
/// an address reads only while both its fair part and the whole are above nothing.
///
/// The budget is dealt max-min fairly every [`SHARE_EPOCH`]: an address that was refused no page
/// is given what it took (the mean of its last two measurements), and what those leave is split
/// evenly among the addresses that were refused one. An address that takes less than an even split
/// is never refused, so what is measured of it is what it asked for; an address that asks for more
/// than its split waits by itself, however many sockets it reads through. Over any stretch the
/// rooms send the budget, half a second's worth and at most one page per reader more.
#[derive(Debug)]
pub struct Outflow {
    pub per_second: u64,
    level: i128,
    filled: Instant,
    pub shares: HashMap<ClientKey, Share>,
    fair: u64,
    dealt: Instant,
}

impl Outflow {
    const BYTE: i128 = 1_000_000_000;

    /// 🏦️ The whole budget, undrawn, read by nobody, as of `now`.
    pub fn new(per_second: u64, now: Instant) -> Self {
        Self { per_second, level: Self::brim(per_second), filled: now, shares: HashMap::new(), fair: per_second, dealt: now }
    }

    /// 🥛️ The most a level filling at `per_second` holds: a quarter of a second's worth.
    fn brim(per_second: u64) -> i128 {
        i128::from(per_second) * Self::BYTE / 4
    }

    /// 🟰️ The rate every address's equal part fills at.
    fn even(&self) -> u64 {
        self.per_second / (self.shares.len() as u64).max(1)
    }

    /// 🫴️ Count one more reader of `client`, whose share starts full.
    pub fn attend(&mut self, client: ClientKey) {
        if let Some(share) = self.shares.get_mut(&client) {
            share.readers += 1;
            return;
        }
        let even = self.per_second / (self.shares.len() as u64 + 1);
        self.shares.insert(client, Share { readers: 1, equal: Self::brim(even), fair: Self::brim(self.fair), filled: self.filled, drawn: 0, rate: 0, held: false, was_held: false });
    }

    /// 🫳️ Count one reader of `client` less, forgetting the share with its last.
    pub fn depart(&mut self, client: ClientKey) {
        let Some(share) = self.shares.get_mut(&client) else { return };
        share.readers = share.readers.saturating_sub(1);
        if share.readers == 0 {
            self.shares.remove(&client);
        }
    }

    /// 🃏️ Deal the budget anew once an epoch has passed since the last deal.
    fn deal(&mut self, now: Instant) {
        let span = now.saturating_duration_since(self.dealt);
        if span < SHARE_EPOCH {
            return;
        }
        self.dealt = now;
        let mut modest = Vec::new();
        let mut open = 0u64;
        for share in self.shares.values_mut() {
            let rate = u64::try_from(u128::from(share.drawn) * 1_000_000_000 / span.as_nanos().max(1)).unwrap_or(u64::MAX);
            open += 1;
            if !(share.held || share.was_held) {
                modest.push(rate / 2 + share.rate / 2);
            }
            (share.rate, share.drawn, share.was_held, share.held) = (rate, 0, share.held, false);
        }
        modest.sort_unstable();
        let (mut left, mut largest) = (self.per_second, 0);
        for demand in modest {
            if u128::from(demand) * u128::from(open) > u128::from(left) {
                break;
            }
            (left, open, largest) = (left - demand, open - 1, demand);
        }
        self.fair = left.checked_div(open).unwrap_or(left + largest);
    }

    /// 🔓️ Fill up to `now` and say whether `client` may read a page; a refusal is remembered for
    /// the next deal.
    pub fn open(&mut self, client: ClientKey, now: Instant) -> bool {
        self.deal(now);
        let flowed = now.saturating_duration_since(self.filled).as_nanos() as i128 * i128::from(self.per_second);
        self.level = (self.level + flowed).min(Self::brim(self.per_second));
        self.filled = self.filled.max(now);
        let (even, fair, whole) = (self.even(), self.fair, self.level > 0);
        let Some(share) = self.shares.get_mut(&client) else { return whole };
        let elapsed = now.saturating_duration_since(share.filled).as_nanos() as i128;
        share.equal = (share.equal + elapsed * i128::from(even)).min(Self::brim(even));
        share.fair = (share.fair + elapsed * i128::from(fair)).min(Self::brim(fair));
        share.filled = share.filled.max(now);
        let open = share.equal > 0 || (share.fair > 0 && whole);
        share.held |= !open;
        open
    }

    /// 📉️ Draw the bytes a page carried to `client`.
    pub fn draw(&mut self, client: ClientKey, bytes: usize) {
        self.level -= bytes as i128 * Self::BYTE;
        if let Some(share) = self.shares.get_mut(&client) {
            share.equal -= bytes as i128 * Self::BYTE;
            share.fair -= bytes as i128 * Self::BYTE;
            share.drawn += bytes as u64;
        }
    }
}

/// 👥️ Every presence room of the instance, keyed by presence lane. Latest-state: a room keeps each
/// session's newest entry only and queues nothing for anybody — a socket reads it through its own
/// [`RoomReader`]. A room lives while it has a member or a reader.
pub struct PresenceRooms {
    pub rooms: ShardedMap<String, Room>,
    pub departures_kept: usize,
    pub outflow: Option<StdMutex<Outflow>>,
}

impl Default for PresenceRooms {
    fn default() -> Self {
        Self::new(&PresenceSettings::default(), Instant::now())
    }
}

impl PresenceRooms {
    /// 🌱️ No rooms, as of `now`: each room will remember [`PresenceSettings::departures_kept`]
    /// departures, and all of them together send [`PresenceSettings::max_bytes_per_second`].
    pub fn new(settings: &PresenceSettings, now: Instant) -> Self {
        let outflow = settings.max_bytes_per_second.map(|per_second| StdMutex::new(Outflow::new(per_second, now)));
        Self { rooms: ShardedMap::new(), departures_kept: settings.departures_kept, outflow }
    }

    /// 🪣️ The rooms' budget, when the instance states one.
    pub fn outflow(&self) -> Option<std::sync::MutexGuard<'_, Outflow>> {
        self.outflow.as_ref().map(|outflow| outflow.lock().unwrap_or_else(|poisoned| poisoned.into_inner()))
    }

    /// 🟢️ Whether the rooms' budget and `client`'s share of it allow reading another page at `now`.
    fn may_send(&self, client: ClientKey, now: Instant) -> bool {
        self.outflow().is_none_or(|mut outflow| outflow.open(client, now))
    }

    /// ➖️ Count the bytes a page carried to `client` against the rooms' budget and its share.
    pub fn sent(&self, client: ClientKey, bytes: usize) {
        if let Some(mut outflow) = self.outflow() {
            outflow.draw(client, bytes);
        }
    }

    /// 📖️ Start reading a room from nothing on behalf of `client`, the address whose share of the
    /// rooms' budget the pages are drawn from: the first page replaces whatever the reader's client
    /// held and is led by `lead`'s entry, when it names a member. Dropping the reader ends the read.
    pub fn read(self: &Arc<Self>, lane: &str, lead: Option<&str>, client: ClientKey) -> RoomReader {
        let changes = self.rooms.mutate_or_default(lane.to_string(), |room| {
            room.readers += 1;
            room.changes.subscribe()
        });
        if let Some(mut outflow) = self.outflow() {
            outflow.attend(client);
        }
        RoomReader { rooms: Arc::clone(self), lane: lane.to_string(), lead: lead.map(str::to_string), client, changes, seen: 0, target: 0, resume: 0, departures: 0, fresh: true, behind: true }
    }

    /// 🚪️ Add `entry`'s session to a room.
    pub fn join(&self, lane: &str, entry: &PresenceEntry) {
        let Some(wire) = wire(entry) else { return };
        self.rooms.mutate_or_default(lane.to_string(), |room| room.seat(Member { session: entry.session.clone(), wire }));
    }

    /// ✏️ Replace a member's entry; a session that is not in the room changes nothing.
    pub fn update(&self, lane: &str, entry: &PresenceEntry) {
        let Some(wire) = wire(entry) else { return };
        self.rooms.with_mut(lane, |room| {
            if let Some(room) = room.filter(|room| room.seated.contains_key(&entry.session)) {
                room.seat(Member { session: entry.session.clone(), wire });
            }
        });
    }

    /// 🚶️ Remove a session and remember its departure.
    pub fn leave(&self, lane: &str, session: &str) {
        self.rooms.with_mut(lane, |room| room.is_some_and(|room| room.vacate(session, self.departures_kept)));
        self.rooms.remove_if(lane, Room::deserted);
    }

    /// 📋️ Every member of a room, ordered by session.
    pub fn roster(&self, lane: &str) -> Vec<PresenceEntry> {
        let mut roster: Vec<PresenceEntry> = self.rooms.with(lane, |room| room.map(|room| room.seats.iter().flatten().filter_map(|member| serde_json::from_str(&member.wire).ok()).collect()).unwrap_or_default());
        roster.sort_by(|left, right| left.session.cmp(&right.session));
        roster
    }

    /// 🔢️ How many rooms have a member or a reader.
    pub fn rooms(&self) -> usize {
        self.rooms.len()
    }

    fn release(&self, lane: &str, client: ClientKey) {
        self.rooms.with_mut(lane, |room| {
            if let Some(room) = room {
                room.readers = room.readers.saturating_sub(1);
            }
        });
        self.rooms.remove_if(lane, Room::deserted);
        if let Some(mut outflow) = self.outflow() {
            outflow.depart(client);
        }
    }
}

/// 🧾️ `entry` as every reader is sent it.
pub fn wire(entry: &PresenceEntry) -> Option<Box<str>> {
    serde_json::to_string(entry).ok().map(String::into_boxed_str)
}

/// 🔤️ `text` as a JSON string.
fn quoted(text: &str) -> String {
    serde_json::Value::from(text).to_string()
}

/// 🔗️ Append one JSON value to a comma-joined list.
pub fn join(list: &mut String, value: &str) {
    if !list.is_empty() {
        list.push(',');
    }
    list.push_str(value);
}

/// 🔖️ Where one socket stands in one room: a few numbers, whatever the room's size. A reader goes
/// round the room's seats and is sent every entry that changed since its last round began — the
/// newest state, once per round however often it changed — so when a round does not fit one page,
/// every changed session still gets its turn before any gets a second. A page costs what it
/// carries, not what the room holds.
///
/// A room's departures are remembered up to [`PresenceSettings::departures_kept`]; a reader that
/// missed one that was forgotten starts over with a page that replaces what its client held.
pub struct RoomReader {
    pub rooms: Arc<PresenceRooms>,
    pub lane: String,
    lead: Option<String>,
    pub client: ClientKey,
    changes: watch::Receiver<u64>,
    pub seen: u64,
    pub target: u64,
    pub resume: usize,
    departures: u64,
    pub fresh: bool,
    behind: bool,
}

impl RoomReader {
    /// 📄️ Read what to send next at `now`, at most `budget` bytes of it — or one entry or
    /// departure, when the first alone is larger — and nothing while the rooms are at their budget
    /// or the reader's address is past its share of it ([`PresenceSettings::max_bytes_per_second`]):
    /// what is unsent then stays unsent. The first page of a member (a reader with a `lead`) does
    /// not wait for the budget, which it is counted against like any other: a session learns at
    /// once that it joined, and how many sessions join is for the edge to bound.
    pub fn page(&mut self, budget: usize, now: Instant) -> Page {
        let Self { rooms, lane, lead, client, changes, seen, target, resume, departures, fresh, behind } = self;
        if !(*fresh && lead.is_some()) && !rooms.may_send(*client, now) {
            *behind = true;
            return Page::default();
        }
        changes.borrow_and_update();
        let page = rooms.rooms.with(lane.as_str(), |room| {
            let mut page = Page::default();
            let Some(room) = room else {
                *behind = false;
                return page;
            };
            let leading = lead.as_deref().and_then(|session| room.seated.get(session)).copied();
            if *fresh || *departures < room.forgotten {
                (*seen, *target, *resume, *departures, *fresh) = (0, 0, 0, room.clock, false);
                page.replace = true;
                if let Some(member) = leading.and_then(|seat| room.seats[seat].as_ref()) {
                    page.entries.push_str(&member.wire);
                }
            }
            let unsent = room.departed.partition_point(|(clock, _)| *clock <= *departures);
            for (clock, session) in room.departed.range(unsent..) {
                let session = quoted(session);
                if !page.fits(session.len(), budget) {
                    break;
                }
                join(&mut page.left, &session);
                *departures = *clock;
            }
            let mut round_done = true;
            if *seen < room.clock {
                if *target <= *seen {
                    (*target, *resume) = (room.clock, 0);
                }
                while let Some(seat) = room.next_changed(*resume, *seen) {
                    if let Some(member) = room.seats[seat].as_ref().filter(|_| *seen > 0 || leading != Some(seat)) {
                        if !page.fits(member.wire.len(), budget) {
                            round_done = false;
                            break;
                        }
                        join(&mut page.entries, &member.wire);
                    }
                    *resume = seat + 1;
                }
                if round_done {
                    (*seen, *resume) = (*target, 0);
                }
            }
            *behind = !round_done || *seen < room.clock || room.departed.back().is_some_and(|(clock, _)| *clock > *departures);
            page
        });
        rooms.sent(*client, page.bytes());
        page
    }

    /// 🔔️ Wait until the room has something this reader was not sent: at once while the last page
    /// left something behind, else when the room next changes.
    pub async fn unsent(&mut self) {
        if self.behind {
            return;
        }
        if self.changes.changed().await.is_err() {
            std::future::pending::<()>().await;
        }
        self.behind = true;
    }

    /// 👁️ Whether the room has something this reader was not sent, without waiting.
    pub fn has_unsent(&self) -> bool {
        self.behind || self.changes.has_changed().unwrap_or(false)
    }

    /// 🆕️ Whether this reader has not read its first page yet.
    pub fn fresh(&self) -> bool {
        self.fresh
    }

    /// 🏃️ Whether this reader is still on the round that sends it the whole roster.
    pub fn arriving(&self) -> bool {
        self.behind && self.seen == 0
    }
}

impl Drop for RoomReader {
    fn drop(&mut self) {
        self.rooms.release(&self.lane, self.client);
    }
}

/// 📃️ What one reader is sent next of one room: entries and departed sessions, already JSON.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Page {
    /// 📇️ The entries, as JSON objects joined by commas.
    pub entries: String,
    /// 🚷️ The sessions that left, as JSON strings joined by commas.
    pub left: String,
    /// 🔁️ Whether this page replaces everything the reader was sent of the room before.
    pub replace: bool,
}

impl Page {
    /// 🕳️ Whether there is nothing to send.
    pub fn is_empty(&self) -> bool {
        !self.replace && self.bytes() == 0
    }

    /// ⚖️ The bytes of entries and departures this page carries.
    pub fn bytes(&self) -> usize {
        self.entries.len() + self.left.len()
    }

    /// 🧮️ Whether `more` bytes and their separator still fit `budget` — always, in an empty page.
    fn fits(&self, more: usize, budget: usize) -> bool {
        self.bytes() == 0 || self.bytes() + 1 + more <= budget
    }

    /// 👋️ This page as the `welcome` frame of `session`.
    pub fn welcome(&self, session: &str, colour: u8) -> String {
        format!(r#"{{"type":"welcome","session":{},"colour":{colour},"roster":[{}]}}"#, quoted(session), self.entries)
    }

    /// 📦️ This page as a `batch` frame.
    pub fn batch(&self) -> String {
        format!(r#"{{"type":"batch","entries":[{}],"left":[{}]}}"#, self.entries, self.left)
    }

    /// 🔭️ This page as the `watched` frame of `scope`, a `snapshot` when it replaces.
    pub fn watched(&self, scope: &str) -> String {
        format!(r#"{{"type":"watched","scope":{},"entries":[{}],"left":[{}]{}}}"#, quoted(scope), self.entries, self.left, if self.replace { r#","snapshot":true"# } else { "" })
    }
}

/// 🚦️ A session's state-frame allowance: at most `limit` frames per one-second window.
#[derive(Clone, Copy, Debug)]
pub struct StateBudget {
    window: Instant,
    pub spent: u32,
    pub limit: u32,
}

impl StateBudget {
    /// 🌱️ A fresh allowance of `limit` frames per second.
    pub fn new(limit: u32) -> Self {
        Self { window: Instant::now(), spent: 0, limit }
    }

    /// ✅️ Spend one frame at `now`; `false` when the window's allowance is exhausted.
    pub fn spend(&mut self, now: Instant) -> bool {
        if now.duration_since(self.window) >= Duration::from_secs(1) {
            self.window = now;
            self.spent = 0;
        }
        self.spent += 1;
        self.spent <= self.limit
    }
}

/// 🛃️ Judge one client text frame: `Ok(Some(frame))` — a `state` or a `watch` — to apply,
/// `Ok(None)` to drop silently (over the rate), `Err(reason)` to answer with a `refused` frame.
pub fn admit_frame(text: &str, settings: &PresenceSettings, budget: &mut StateBudget, now: Instant) -> Result<Option<PresenceFrame>, String> {
    if text.len() > settings.max_state_bytes {
        return Err(REFUSED_TOO_LARGE.to_string());
    }
    if !budget.spend(now) {
        return Ok(None);
    }
    match serde_json::from_str::<PresenceFrame>(text) {
        Ok(frame @ (PresenceFrame::State { .. } | PresenceFrame::Watch { .. })) => Ok(Some(frame)),
        _ => Err(REFUSED_INVALID.to_string()),
    }
}

/// 🐢️ The interval a watcher asked for, clamped between [`PresenceSettings::tick`] (a room never
/// changes faster) and [`PresenceSettings::max_watch_interval`].
pub fn watch_interval(interval_ms: u64, settings: &PresenceSettings) -> Duration {
    Duration::from_millis(interval_ms).min(settings.max_watch_interval).max(settings.tick)
}

/// 🎲️ A fresh presence session id: 128 bits from the process's randomly keyed hasher over a
/// counter and the clock, as 32 lowercase hex characters. Public, never a credential.
pub fn presence_session_id() -> String {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let keys = std::collections::hash_map::RandomState::new();
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    let count = NEXT.fetch_add(1, Ordering::Relaxed);
    let half = |salt: u64| {
        let mut hasher = keys.build_hasher();
        hasher.write_u64(salt);
        hasher.write_u64(count);
        hasher.write_u128(nanos);
        hasher.finish()
    };
    format!("{:016x}{:016x}", half(0x7072_6573_656e_6365), half(0x7365_7373_696f_6e73))
}
//#endregion 🔖️PresenceRoom

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
