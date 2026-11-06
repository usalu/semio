# Design Notes

## Preserved Source Rationale

<a id="source-notes-18"></a>

### 🦀️.rs:18

⛽️ The ceiling one pure `codec` call runs under. The WIT states the contract these functions
honour — no storage, no window, no effect, one throwaway instance per call — so what bounds them
is wall time, not effect or frame counts. 60 s is the same order as [`headless_open_budget`],
because printing or applying against an 80 KB document pack is work of the same scale as bringing
an instance up, and a guest that needs longer is wedged rather than busy.
🧵️ The binding generation this gateway claims for a document backbone. One session opens one
binding per instance and never rebinds — a rebind is what a shell does when a window swaps
documents, which a headless channel with one bound artifact per plugin cannot do — so the
generation is a constant rather than a counter that would imply a history it does not have.

<a id="source-notes-91"></a>

### 🦀️.rs:91

🗂️ Every guest-backed codec this process has registered, in registration order.

🧭️ A process-global table exists because [`store::os_store::serialization::ArtifactCodec`]'s four operations are bare,
NON-CAPTURING `fn` pointers — an erasure table, by design, so that a schema-string-keyed caller
can print and apply without naming a concrete `P`/`Mutation`. A thunk therefore cannot close over
a component, and cannot be told which schema it was registered under either: the signatures carry
only bytes. So a thunk resolves its route by asking the components themselves, in order, and
taking the first that answers — see [`guest_print_mirror`] for why that is the WIT's own
discriminator rather than a guess.

<a id="source-notes-111"></a>

### 🦀️.rs:111

📥️ `(pack, spr) -> (dsl, ops)` through whichever registered component owns this pair.

🧭️ **The component IS the discriminator.** `interface codec`'s own contract states it: "a pair
the component cannot print back is not a document of this kind". So asking each registered
package in turn and taking the first that answers is not a guess at the schema — it is running
the exact fence the interface defines, once per candidate. In a real gateway session the
candidate list is the packages whose documents this agent has open, which is one or two.

<a id="source-notes-326"></a>

### 🦀️.rs:326

🗂️ Registers `artifact_schema`'s document codec from the component's OWN `codec` export, and
answers the pack-schema hash the component itself computed.

🪪️ `expected_pack_schema_hash` is the hub's declared hex hash for this document, and it is a
CROSS-CHECK, never the source: the hash that goes into the registry is the one
`codec.pack-schema-hash` returned from the component bytes the hub authorized and this process
verified by SHA-256. Registering the hub's own number would make `finish_connect_hub`'s
`local_schema_hash == authority.pack_schema_hash` compare the hub's value with itself, which is
not satisfying that check — it is deleting it.

🔁️ Idempotent per schema: a second registration of a schema this process already serves is a
no-op that still verifies the hash, so re-opening a document never stacks routes.

📦️ The generic pack container extension. A guest-backed codec never reaches the folder
text lane (see [`guest_compile_dsl`]), which is the only place this is read.
