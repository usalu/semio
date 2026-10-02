//! 🚦️ Throttle: what one client address may ask of an instance, and what the instance carries at
//! once — the flood protection of a public edge, as pure bookkeeping.
//!
//! **Three kinds of bound.** *Rates* are token buckets per client address, one per [`RequestClass`]
//! (writes, reads, socket upgrades): an address spends a token per request and is refused with the
//! wait until its next token once the bucket is empty. *Allowances* ([`Allowance`]) are token
//! buckets per client address as well, named by the instance for what its commands **create** — a
//! registration, an upload — and refilled over hours rather than seconds: a command of such a class
//! spends a token on top of its request's, and gets it back when it changed nothing
//! ([`Throttle::spend`], [`Throttle::refund`]), so an allowance counts what an address made the
//! instance keep, not what it asked. *Caps* are counts of what is alive right now: sockets per
//! address, sockets in total and requests in flight. A rate bounds how fast one script may go, an
//! allowance how much of a finite store it may take, a cap how much memory and queueing all
//! clients together may cost.
//!
//! **Sized by the instance.** The numbers are the instance's ([`Limits`]): a rate has to carry
//! every honest client that shares one address — a lecture hall behind one NAT is one address — and
//! still leave the instance able to serve everybody else while one address runs at its rate.
//! [`Limits::default`] is that sizing for a class of three hundred; [`Limits::open`] bounds nothing.
//!
//! **Bounded itself.** At most [`Limits::clients`] addresses are tracked; an address is forgotten
//! once it has been idle for [`Limits::idle`], holds no socket and every bucket of it is full again
//! — so forgetting an address never hands it an allowance a second time — and when the table is
//! full anyway every further address shares one bucket set, so a caller minting addresses buys
//! nothing. An IPv6 address is keyed by its /48, the largest block a network commonly routes to one
//! subscriber: whoever holds such a block mints its 65536 networks at will, and is one client.
//!
//! No transport, no runtime, no clock: the gateway resolves the address and passes the instant in.
//!
//! @see <https://www.rfc-editor.org/rfc/rfc6177> — IPv6 assignments to end sites (/48 to /56)

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

//#region 🔖️Limits
/// ⏱️ A token bucket: `tokens` flow in every `per`, at most `burst` are held.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rate {
    pub tokens: u32,
    pub per: Duration,
    pub burst: u32,
}

impl Rate {
    /// ♾️ A rate nothing is refused by.
    pub const OPEN: Self = Self::per_second(u32::MAX, u32::MAX);

    /// ⚡️ `tokens` per second: the pace of requests.
    pub const fn per_second(tokens: u32, burst: u32) -> Self {
        Self { tokens, per: Duration::from_secs(1), burst }
    }

    /// 🕰️ `tokens` per hour: the pace of what an instance keeps.
    pub const fn per_hour(tokens: u32, burst: u32) -> Self {
        Self { tokens, per: Duration::from_secs(3600), burst }
    }

    /// 🔋️ How long an empty bucket takes to hold its whole burst again.
    pub fn refill(&self) -> Duration {
        let nanos = (u128::from(self.burst) * token(*self)).div_ceil(u128::from(self.tokens.max(1)));
        Duration::from_nanos(u64::try_from(nanos).unwrap_or(u64::MAX))
    }
}

/// 💰️ A named allowance: one more bucket per client address, for the commands an instance classes
/// under `name` because each of them makes the instance keep something. It is spent on top of the
/// address's write allowance and handed back by a command that changed nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Allowance {
    pub name: &'static str,
    pub rate: Rate,
}

/// 🎚️ Everything an instance bounds at its edge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Limits {
    /// ✍️ Requests that change state (commands), per client address.
    pub writes: Rate,
    /// 📖️ Requests that read (queries, event pages, the instance document), per client address.
    pub reads: Rate,
    /// 🔌️ Websocket upgrades, per client address.
    pub upgrades: Rate,
    /// 🧧️ The allowances this instance names, per client address; none unless stated.
    pub allowances: Vec<Allowance>,
    /// 🧵️ Websockets one client address may hold open at once.
    pub sockets_per_client: usize,
    /// 🧶️ Websockets open at once, over every client.
    pub sockets: usize,
    /// 🌊️ Requests being served at once, over every client; one more is turned away.
    pub in_flight: usize,
    /// 📦️ The largest request body accepted, in bytes; `None` keeps the transport's own limit.
    pub body_bytes: Option<usize>,
    /// ⌛️ How long a request is given to deliver its body before it is turned away.
    pub body_patience: Duration,
    /// 🧮️ How many client addresses are tracked at once.
    pub clients: usize,
    /// 💤️ How long an address without a request or a socket is remembered at least; it is remembered
    /// for longer while a bucket of it is not full again.
    pub idle: Duration,
}

impl Default for Limits {
    /// 🏫️ The production sizing: a class of three hundred behind one address (a few commands per
    /// second each while answering, polled and crowd queries, two to three sockets each, everyone
    /// reconnecting within seconds after a network drop) passes untouched, and one address running
    /// at these rates leaves the instance serving the others.
    fn default() -> Self {
        Self {
            writes: Rate::per_second(300, 900),
            reads: Rate::per_second(600, 1800),
            upgrades: Rate::per_second(100, 1800),
            allowances: Vec::new(),
            sockets_per_client: 2048,
            sockets: 8192,
            in_flight: 2048,
            body_bytes: Some(16 * 1024),
            body_patience: Duration::from_secs(10),
            clients: 16_384,
            idle: Duration::from_secs(120),
        }
    }
}

impl Limits {
    /// 🚪️ Limits that bound nothing: the shape of an instance that sits behind an edge of its own.
    pub fn open() -> Self {
        Self { writes: Rate::OPEN, reads: Rate::OPEN, upgrades: Rate::OPEN, allowances: Vec::new(), sockets_per_client: usize::MAX, sockets: usize::MAX, in_flight: usize::MAX, body_bytes: None, body_patience: Duration::MAX, clients: 1, idle: Duration::from_secs(1) }
    }

    fn rate(&self, class: RequestClass) -> Rate {
        match class {
            RequestClass::Write => self.writes,
            RequestClass::Read => self.reads,
            RequestClass::Upgrade => self.upgrades,
        }
    }

    /// 🗝️ Where the allowance called `name` stands among [`allowances`](Self::allowances), with its rate.
    fn allowance(&self, name: &str) -> Option<(usize, Rate)> {
        self.allowances.iter().position(|allowance| allowance.name == name).map(|index| (index, self.allowances[index].rate))
    }
}

/// 🗂️ What a request costs against: the bucket it spends from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequestClass {
    Write,
    Read,
    Upgrade,
}

/// 🚫️ Why the throttle turned a request away, with the wait after which it is worth retrying.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// 🐇️ This client address spent its allowance.
    Throttled { retry_after: Duration },
    /// 🌊️ The instance as a whole is at a cap.
    Overloaded { retry_after: Duration },
}

/// ⏳️ The wait a caller turned away at a cap is told: caps free up as requests finish and sockets
/// close, which the throttle cannot foresee.
pub const OVERLOAD_RETRY: Duration = Duration::from_secs(1);
//#endregion 🔖️Limits

//#region 🔖️Client
/// 🏷️ The address a client is throttled as: an IPv4 address, or the /48 of an IPv6 address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClientKey {
    V4([u8; 4]),
    V6([u8; 6]),
}

impl ClientKey {
    /// 🧭️ The key of `address`; an IPv4 address carried inside IPv6 is its IPv4 self.
    pub fn of(address: IpAddr) -> Self {
        match address {
            IpAddr::V4(address) => Self::V4(address.octets()),
            IpAddr::V6(address) => match address.to_ipv4_mapped() {
                Some(mapped) => Self::V4(mapped.octets()),
                None => {
                    let mut site = [0u8; 6];
                    site.copy_from_slice(&address.octets()[..6]);
                    Self::V6(site)
                }
            },
        }
    }
}

/// 🔎️ The client address a terminating proxy wrote into a forwarded-for header value: the last
/// entry of the list — the one the proxy itself appended, whatever a client put in front of it —
/// as a bare address, a bracketed IPv6 address or either with a port. `None` when it is no address.
pub fn forwarded_address(value: &str) -> Option<IpAddr> {
    let last = value.rsplit(',').next()?.trim();
    last.parse::<IpAddr>().ok().or_else(|| last.parse::<SocketAddr>().ok().map(|socket| socket.ip())).or_else(|| last.strip_prefix('[')?.strip_suffix(']')?.parse().ok())
}
//#endregion 🔖️Client

//#region 🔖️Throttle
/// 🪙️ One token of `rate`, in the units its bucket counts in: a nanosecond of flow is `rate.tokens`
/// units and a token as many units as its period has nanoseconds, so every level is a whole number
/// and no schedule depends on rounding.
fn token(rate: Rate) -> u128 {
    rate.per.as_nanos().max(1)
}

#[derive(Clone, Copy, Debug)]
struct Bucket {
    level: u128,
    filled: Instant,
}

impl Bucket {
    fn full(rate: Rate, now: Instant) -> Self {
        Self { level: u128::from(rate.burst) * token(rate), filled: now }
    }

    fn fill(&mut self, rate: Rate, now: Instant) {
        let now = now.max(self.filled);
        let flowed = now.duration_since(self.filled).as_nanos() * u128::from(rate.tokens);
        self.level = (self.level + flowed).min(u128::from(rate.burst) * token(rate));
        self.filled = now;
    }

    fn spend(&mut self, rate: Rate, now: Instant) -> Result<(), Duration> {
        self.fill(rate, now);
        if self.level >= token(rate) {
            self.level -= token(rate);
            return Ok(());
        }
        let wait = (token(rate) - self.level).div_ceil(u128::from(rate.tokens.max(1)));
        Err(Duration::from_nanos(u64::try_from(wait).unwrap_or(u64::MAX)))
    }

    fn refund(&mut self, rate: Rate, now: Instant) {
        self.fill(rate, now);
        self.level = (self.level + token(rate)).min(u128::from(rate.burst) * token(rate));
    }

    fn is_full(&self, rate: Rate, now: Instant) -> bool {
        let flowed = now.saturating_duration_since(self.filled).as_nanos() * u128::from(rate.tokens);
        self.level + flowed >= u128::from(rate.burst) * token(rate)
    }
}

#[derive(Clone, Debug)]
struct Client {
    writes: Bucket,
    reads: Bucket,
    upgrades: Bucket,
    allowances: Box<[Bucket]>,
    sockets: usize,
    seen: Instant,
}

impl Client {
    fn fresh(limits: &Limits, now: Instant) -> Self {
        Self { writes: Bucket::full(limits.writes, now), reads: Bucket::full(limits.reads, now), upgrades: Bucket::full(limits.upgrades, now), allowances: limits.allowances.iter().map(|allowance| Bucket::full(allowance.rate, now)).collect(), sockets: 0, seen: now }
    }

    fn bucket(&mut self, class: RequestClass) -> &mut Bucket {
        match class {
            RequestClass::Write => &mut self.writes,
            RequestClass::Read => &mut self.reads,
            RequestClass::Upgrade => &mut self.upgrades,
        }
    }

    /// 🫗️ Whether this entry is what a fresh one would be: no socket, idle for the limit, and
    /// every bucket full again.
    fn forgettable(&self, limits: &Limits, now: Instant) -> bool {
        self.sockets == 0
            && now.saturating_duration_since(self.seen) >= limits.idle
            && self.writes.is_full(limits.writes, now)
            && self.reads.is_full(limits.reads, now)
            && self.upgrades.is_full(limits.upgrades, now)
            && self.allowances.iter().zip(&limits.allowances).all(|(bucket, allowance)| bucket.is_full(allowance.rate, now))
    }
}

struct Clients {
    tracked: HashMap<ClientKey, Client>,
    crowd: Client,
    swept: Instant,
}

impl Clients {
    /// 🧹️ Forget every address that is what a fresh one would be, at most twice per idle span.
    fn sweep(&mut self, limits: &Limits, now: Instant, forced: bool) {
        if !forced && now.saturating_duration_since(self.swept) < limits.idle / 2 {
            return;
        }
        self.swept = now;
        self.tracked.retain(|_, client| !client.forgettable(limits, now));
    }

    /// 🪑️ The bookkeeping of `key`: its own entry, or — once the table is full of live addresses —
    /// the one entry every untracked address shares.
    fn seat(&mut self, limits: &Limits, key: ClientKey, now: Instant) -> &mut Client {
        self.sweep(limits, now, false);
        if !self.tracked.contains_key(&key) && self.tracked.len() >= limits.clients {
            self.sweep(limits, now, true);
        }
        if !self.tracked.contains_key(&key) && self.tracked.len() >= limits.clients {
            return &mut self.crowd;
        }
        let client = self.tracked.entry(key).or_insert_with(|| Client::fresh(limits, now));
        client.seen = now;
        client
    }
}

/// 🚦️ The live bookkeeping of one instance's [`Limits`].
pub struct Throttle {
    limits: Limits,
    clients: Mutex<Clients>,
    in_flight: AtomicUsize,
    sockets: AtomicUsize,
}

impl Throttle {
    /// 🆕️ A throttle over `limits`, nothing spent and nothing alive, as of `now`.
    pub fn new(limits: Limits, now: Instant) -> Arc<Self> {
        let clients = Mutex::new(Clients { tracked: HashMap::new(), crowd: Client::fresh(&limits, now), swept: now });
        Arc::new(Self { limits, clients, in_flight: AtomicUsize::new(0), sockets: AtomicUsize::new(0) })
    }

    /// 🎚️ The limits this throttle enforces.
    pub fn limits(&self) -> &Limits {
        &self.limits
    }

    /// 🎟️ Spend one token of `client`'s `class` bucket at `now`, or learn how long to wait.
    pub fn admit(&self, client: ClientKey, class: RequestClass, now: Instant) -> Result<(), Refusal> {
        let rate = self.limits.rate(class);
        if rate == Rate::OPEN {
            return Ok(());
        }
        let mut clients = self.clients.lock().unwrap_or_else(PoisonError::into_inner);
        clients.seat(&self.limits, client, now).bucket(class).spend(rate, now).map_err(|retry_after| Refusal::Throttled { retry_after })
    }

    /// 🧾️ Spend one token of `client`'s allowance called `allowance` at `now`, or learn how long to
    /// wait for the next one. An allowance this instance did not size refuses nothing.
    pub fn spend(&self, client: ClientKey, allowance: &str, now: Instant) -> Result<(), Refusal> {
        let Some((index, rate)) = self.limits.allowance(allowance) else { return Ok(()) };
        let mut clients = self.clients.lock().unwrap_or_else(PoisonError::into_inner);
        clients.seat(&self.limits, client, now).allowances[index].spend(rate, now).map_err(|retry_after| Refusal::Throttled { retry_after })
    }

    /// ↩️ Hand `client` back the token a [`spend`](Self::spend) of `allowance` took: what it was
    /// spent for changed nothing.
    pub fn refund(&self, client: ClientKey, allowance: &str, now: Instant) {
        let Some((index, rate)) = self.limits.allowance(allowance) else { return };
        let mut clients = self.clients.lock().unwrap_or_else(PoisonError::into_inner);
        clients.seat(&self.limits, client, now).allowances[index].refund(rate, now);
    }

    /// 🚪️ Count one more request in flight until the returned guard is dropped, or turn it away
    /// when the instance already serves as many as it may.
    pub fn enter(self: &Arc<Self>) -> Result<InFlight, Refusal> {
        claim(&self.in_flight, self.limits.in_flight).map(|()| InFlight { throttle: Arc::clone(self) }).ok_or(Refusal::Overloaded { retry_after: OVERLOAD_RETRY })
    }

    /// 🔌️ Count one more open socket of `client` until the returned permit is dropped, or turn it
    /// away at the client's own cap ([`Refusal::Throttled`]) or the instance's ([`Refusal::Overloaded`]).
    pub fn open_socket(self: &Arc<Self>, client: ClientKey, now: Instant) -> Result<SocketPermit, Refusal> {
        if self.limits.sockets_per_client == usize::MAX {
            claim(&self.sockets, self.limits.sockets).ok_or(Refusal::Overloaded { retry_after: OVERLOAD_RETRY })?;
            return Ok(SocketPermit { throttle: Arc::clone(self), seat: Seat::Uncounted });
        }
        let mut clients = self.clients.lock().unwrap_or_else(PoisonError::into_inner);
        let seat = clients.seat(&self.limits, client, now);
        if seat.sockets >= self.limits.sockets_per_client {
            return Err(Refusal::Throttled { retry_after: OVERLOAD_RETRY });
        }
        claim(&self.sockets, self.limits.sockets).ok_or(Refusal::Overloaded { retry_after: OVERLOAD_RETRY })?;
        seat.sockets += 1;
        let seat = if clients.tracked.contains_key(&client) { Seat::Own(client) } else { Seat::Crowd };
        Ok(SocketPermit { throttle: Arc::clone(self), seat })
    }

    /// 🌊️ How many requests are in flight.
    pub fn in_flight(&self) -> usize {
        self.in_flight.load(Ordering::Relaxed)
    }

    /// 🧶️ How many sockets are open.
    pub fn sockets(&self) -> usize {
        self.sockets.load(Ordering::Relaxed)
    }

    /// 🧮️ How many client addresses are tracked.
    pub fn clients(&self) -> usize {
        self.clients.lock().unwrap_or_else(PoisonError::into_inner).tracked.len()
    }
}

/// ➕️ Count one more toward `cap`, unless the count is already there.
fn claim(count: &AtomicUsize, cap: usize) -> Option<()> {
    let mut held = count.load(Ordering::Acquire);
    while held < cap {
        match count.compare_exchange_weak(held, held + 1, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => return Some(()),
            Err(actual) => held = actual,
        }
    }
    None
}

/// 🎫️ One request in flight; dropping it frees its place.
pub struct InFlight {
    throttle: Arc<Throttle>,
}

impl Drop for InFlight {
    fn drop(&mut self) {
        self.throttle.in_flight.fetch_sub(1, Ordering::AcqRel);
    }
}

/// 🪑️ Where a socket was counted: on its client's own entry, on the entry untracked addresses
/// share, or — when clients are not capped — nowhere but the instance's total.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seat {
    Own(ClientKey),
    Crowd,
    Uncounted,
}

/// 🎫️ One open socket of one client; dropping it frees both the client's and the instance's place.
pub struct SocketPermit {
    throttle: Arc<Throttle>,
    seat: Seat,
}

impl Drop for SocketPermit {
    fn drop(&mut self) {
        self.throttle.sockets.fetch_sub(1, Ordering::AcqRel);
        if self.seat == Seat::Uncounted {
            return;
        }
        let mut clients = self.throttle.clients.lock().unwrap_or_else(PoisonError::into_inner);
        let seat = match self.seat {
            Seat::Own(client) => clients.tracked.get_mut(&client),
            Seat::Crowd | Seat::Uncounted => Some(&mut clients.crowd),
        };
        if let Some(seat) = seat {
            seat.sockets = seat.sockets.saturating_sub(1);
        }
    }
}
//#endregion 🔖️Throttle

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
