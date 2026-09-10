/** @emoji 🧮️ The guest linear-memory budget, on the HOST side of the boundary.
 *
 * 🧨️ The twin of `🧮️memory/🦀️.rs`; both are pinned to `🧮️memory/🧬️schema/🔣️.json`, and both must
 * be, because the party that lowers a payload into a guest is the only party that can keep it inside
 * the bound. A block the guest allocator refuses is NOT a fault the actor can report: `cabi_realloc`
 * answers a null with `handle_alloc_error` → `abort_internal` → `unreachable`, an unrecoverable trap
 * raised before one instruction of guest code runs (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */

/** @emoji 📏️ Largest linear memory a plugin guest may grow to — `maximumBytes`. */
export const GUEST_LINEAR_MEMORY_MAXIMUM_BYTES = 536_870_912;

/** @emoji 📏️ Largest CONTIGUOUS block a routine per-command or per-turn guest path may request —
 * `contiguousRequestCeilingBytes`, one wasm page and the guest allocator's growth granularity. */
export const GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES = 65_536;

/** @emoji 📏️ Largest ASSEMBLED answer the host may deliver into a guest for ONE outstanding request —
 * `hostAnswerCeilingBytes`. Past this the guest answers with a typed fault instead of allocating. */
export const GUEST_HOST_ANSWER_CEILING_BYTES = 8_388_608;

/** @emoji 📄️ Cuts one host answer into the prologue pages a guest may accept plus the terminal page
 * its completion carries — the twin of `guest_host_answer_pages` in `🧮️memory/🦀️.rs`.
 *
 * Every page is at most {@link GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES}, and the LAST element is
 * always the terminal page (empty only when `answer` itself is empty), so a caller delivers
 * `prologue` as chunk events against the request and `terminal` on its completion. */
export function guestAnswerPages(answer: Uint8Array): { readonly prologue: readonly Uint8Array[]; readonly terminal: Uint8Array } {
  if (answer.byteLength <= GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES) return { prologue: [], terminal: answer };
  const prologue: Uint8Array[] = [];
  let cut = 0;
  while (answer.byteLength - cut > GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES) {
    prologue.push(answer.subarray(cut, cut + GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES));
    cut += GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
  }
  return { prologue, terminal: answer.subarray(cut) };
}
