//! ⏱️ The idle streak of a retained command, counted by the turn that steps it.

/// 🛑️ The fault code a retained command carries when it stayed pending for [`command_idle_turn_limit`] consecutive turns that consumed no work and moved no retained stock; the shell names and localizes it (`⏱️command-stall`).
pub const COMMAND_STALLED_FAULT_CODE:&str="plugin.command-stalled";

/// ⏳️ The consecutive quiet turns a retained command may spend before the reactor faults it: a spinning guest answers `more-work` thousands of times per second, a host that is only waiting on a job paces its turns at 25 ms or more and never reaches the limit inside the shell's 300 s command wall.
pub const COMMAND_IDLE_TURN_LIMIT:u32=16_384;

thread_local!{
    static COMMAND_IDLE_STREAK:std::cell::Cell<(u32,u64,u32)>=const{std::cell::Cell::new((0,0,0))};
    static COMMAND_IDLE_LIMIT:std::cell::Cell<u32>=const{std::cell::Cell::new(COMMAND_IDLE_TURN_LIMIT)};
}

/// 🔢️ The idle limit this thread's reactor enforces; laws lower it through [`set_command_idle_turn_limit`].
pub fn command_idle_turn_limit()->u32{COMMAND_IDLE_LIMIT.with(|limit|limit.get())}

/// 🛑️ Whether a streak of consecutive quiet turns exhausts this thread's idle limit, so the reactor faults the command as stalled.
#[doc(hidden)]
pub fn command_stalled(streak:u32)->bool{streak>=command_idle_turn_limit()}

/// 🧪️ Lowers or restores the idle limit of this thread's reactor so a law can reach it without spinning for minutes.
#[doc(hidden)]
pub fn set_command_idle_turn_limit(limit:u32){COMMAND_IDLE_LIMIT.with(|slot|slot.set(limit))}

/// 📏️ Counts one turn of the command `seq` of `instance`: a quiet turn extends the streak, any other turn (or another command) restarts it; returns the streak.
#[doc(hidden)]
pub fn note_command_turn(instance:u32,seq:u64,quiet:bool)->u32{COMMAND_IDLE_STREAK.with(|slot|{let (i,s,n)=slot.get();let n=if (i,s)==(instance,seq){n}else{0};let n=if quiet{n.saturating_add(1)}else{0};slot.set((instance,seq,n));n})}
