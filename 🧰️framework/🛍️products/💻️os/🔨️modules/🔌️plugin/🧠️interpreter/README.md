# Interpreter Source Notes

## 🔌️owned-semio-boundary/🦀️.rs

🤝️ The owned twin of `reactor.channel-version`: the guest's compiled app-channel version, which a host admits before any frame.
🧬️ OS-HUB-COLLABORATION-AI-END-TO-END (TC3b): the owned twin of `world actor`'s `codec`
interface — the four pure document-kind functions a headless host calls on a package whose Rust
codec it does not link.
📜️ The owned twin of `codec.replay-envelopes`, the hub's Check In fold.
📏️ The largest authored scalar export carries its three-scalar prefix and nineteen-scalar authority.
🪢️ The immutable module witness used by original phased opening.
👓️ Writes the same retained borrowed VM after its original observed measurement.
📏️ Retains the original VM borrow and scalar extent without allocating checkpoint storage.

## 🏛️instance-state/🦀️.rs

▶️ The actual declared start function, without opening an instance or cloning its body.
🧯️ Names a shadow-stack underflow for what it is. A `--stack-first` guest keeps its shadow
stack in `[0, __stack_pointer_init)` and grows it down, so an overrun addresses below zero and
surfaces as an ordinary out-of-bounds access at a huge unsigned address — indistinguishable,
in the bare trap text, from a wild pointer. Reported once, at the only place that still holds
the guest's call depth and globals (ticket 26/09/18 slice A2: a 1 MiB-linked component
overflowing at call depth 26 read as "runaway recursion" for a full slice).

## 🏗️instance/🧪️tests/🦀️.rs

🧪️ Capacity denial keeps the actual partial owner; bounded initialization produces the same memory, table and global as Wasmtime.
▶️ Real instruction cost remains distinct from physical admission and a borrowed completed result survives until paid closure.
🛑️ Cancellation and a foreign same-named issuer preserve the actual partially born storage and its source until paid return.
📥️ A borrowed input copies into already born guest memory only under its genuine incoming source and copy grant.
📞️ Nested calls preserve their denied operands and return each funded physical frame separately from eight actual VM instructions.
📥️ Actual memory load/store instructions quote all guest-byte and machine mutations before changing address, operands or PC.

## 🧮️numeric-execution/🦀️.rs

🧮️ Numeric failures remain fixed scalar facts until an ordinary caller chooses diagnostic text.
🔢️ The shared numeric evaluator uses two stack scalars and returns a fixed failure without heap effects.
🧮️ The independent VM agrees with the shared fixed evaluator and actual original instruction machine, including zero-heap traps.

## 🦀️.rs

🪪️ The interpreter's authored source: the half of the owned engine identity that names every instruction a guest executes.
🧮️ The heap witness the crate's allocation laws read through `semio_framework_trace::observe_heap_allocations_on_this_thread`.

## ⚙️instruction-execution/🦀️.rs

🎯️ What a taken branch needs of its target once the results are in place.
📥️ Leaves exactly `types` on top of the operand stack at `stack_height`: the same checks, in the same
order and with the same traps, as [`take_results`] followed by re-pushing its values — without the
intermediate copy, which every block end, branch and return used to allocate.

## 📸️checkpoint-codec/🦀️.rs

👓️ Visits original borrowed checkpoint fields without creating intermediate storage.
📤️ Measures borrowed state before original allocation and returns every failure owner to its caller slot.

### Codec origin ownership

🧊️ A codec call's use of its compiled guest's origin: the origin, the fuel THIS call spent assembling it (the part of its
budget it used), and the fuel it already reported as progress — the assembly's own, or the one it relayed while it waited
for another call's — so its operation's progress continues from there instead of starting over.

🧯️ One origin assembly in flight: however it ends — with an origin, a fault, a cancellation or a panic — the cell
settles (ready, or absent again) and every waiting call wakes.

🧠️ One component compiled for the owned interpreter: its parsed artifact, and the origin its codec calls start from
once one of them assembled it. A clone shares both, so every holder of one compiled guest starts its codec calls from the
same origin.


### Guest quarantine and channel admission

☠️ Set by the first guest trap. A trapped guest keeps whatever linear memory, shadow-stack
pointer and allocator state the trap left behind, so every later call starts from a lower
stack pointer and traps again 112 bytes further down — ticket 26/09/18 slice A1 read that
drift as the defect itself. There is no recovery short of re-instantiation, so this refuses.
    
🔖️ Reads a fresh owned instance's `semio_owned_channel_version_v1` once and admits it through the one kernel admission
([`semio_os_kernel::admit_guest_channel_version`]) before the instance serves anything — an actor turn or a codec call —
so a guest of another channel is refused as `PluginHostError::Refused` (`plugin.channel-mismatch`).


### Codec observation

🧊️ `owned`'s codec origin for one call ([`OwnedCodecOriginUse`]). While another call assembles it, this one waits,
relays that assembly's fuel through `progress` and observes `cancellation`; if that assembly ends without an origin,
this call assembles it.
    
📜️ `codec.replay-envelopes` — the hub's Check In fold, with the same stall-bound fuel
observations as [`Self::codec_genesis_observed`]: a long ledger tail is the longest guest call
a Check In makes, so its fuel progress is what its stall bound watches.
    
🧬️ A throwaway instance of `compiled` in its codec origin's state, for a caller that drives its own codec operation on
it (a retained SQLite snapshot call), with the fuel this call spent assembling the origin ([`Self::codec_origin`]).
    

### Runtime and clock layout

🏃️ The owned interpreter as a guest runtime: compiles components, instantiates actors and runs their turns, jobs,
checkpoints and pure `codec` exports under instruction fuel. Holds no engine; instance ids are its only state.

🕰️ `wasi:clocks/wall-clock.now`'s `datetime` record (`seconds: u64` at 0, `nanoseconds: u32` at 8,
padded to 16), read from the host's real wall clock — a guest Store stamps history with it.

