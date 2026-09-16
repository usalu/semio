/** 📬️ `BoundedMailbox` — the TypeScript twin of the Rust `Mailbox` in `🎭️actor/🦀️.rs`'s
 * `📬️Mailbox` region: same four-lane bounded ring (so `popNext` honors lane priority for free), same
 * latest-wins coalescing scan on `enqueue`, same lowest-priority-nonempty-lane eviction before a hard
 * reject.
 *
 * 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P1, relocated by coordinator "sol"'s follow-up):
 * originally landed in the domain-neutral `🧰️framework/📦️packages/🟦️typescript/🟦️.ts`, but this
 * is the actor module's own vocabulary twin and its main consumer (T-P4's turn scheduler) lives in
 * this same `@semio-tech/framework-actor` package — "if code is repeated, it MUST be close to each
 * other" (CLAUDE.md) argues for living beside `🧵️shard-client.ts`, not in the generic base package,
 * which also cannot reach this module's `🤖️generated/🎭️actor/🟦️.ts` without inverting the layering.
 */

//#region 🔌️WireTypes
/** ⚖️ `Lane`/`CoalesceKey` are taken from the owned-schema mirror — they are real wire types (no
 * `#[serde(rename_all)]` on Rust `Lane`, so its wire form is PascalCase, e.g. `"Interactive"`, not
 * `"interactive"`) — redeclaring them locally would silently drift from the wire the moment either
 * side changes. `Backpressure` is declared fresh below instead: the generated mirror's `Backpressure`
 * is `{ "kind": "dropped" } & Lane`, an object-intersected-with-a-string-literal-union type that no
 * value can ever satisfy. The owned schema fixes that historical tuple-variant projection. */
import type { Lane, CoalesceKey } from "../🤖️generated/🎭️actor/🟦️.ts";
export type { Lane, CoalesceKey };
//#endregion 🔌️WireTypes

//#region 📬️BoundedMailbox
const MAILBOX_LANE_ORDER: readonly Lane[] = ["Interactive", "UserVisible", "Background", "Maintenance"];

function laneRank(lane: Lane): number {
  return MAILBOX_LANE_ORDER.indexOf(lane);
}

/** @emoji 🚦 What {@link BoundedMailbox.enqueue} reports back. `rejected` must always surface to the
 * UI as a busy signal — it must never be treated as a silent drop of a user action.
 * `Rejected` — muss der UI immer als Beschäftigt-Signal angezeigt werden, niemals als stilles Verwerfen. */
export type Backpressure = { readonly kind: "accept" } | { readonly kind: "coalesced" } | { readonly kind: "dropped"; readonly lane: Lane } | { readonly kind: "rejected" };

/** @emoji ✉️ One message offered to a {@link BoundedMailbox}: its scheduling lane, payload, an
 * optional coalescing key that lets a newer envelope replace an older queued one in place, and an
 * optional causal `order` key (see {@link createBoundedMailbox} `## causal order`). */
export interface MailboxEnvelope<T> {
  readonly lane: Lane;
  readonly coalesce?: CoalesceKey;
  /** 🔗️ Causal dequeue key within the lane — the caller's own monotonic space (the input ledger's
   * `causedBy ?? inputSeq`). Absent ⇒ plain arrival order, exactly as before this field existed. */
  readonly order?: number;
  readonly payload: T;
}

/** @emoji 📬️ Bounded ring per actor: one FIFO queue per {@link Lane} (so `popNext` honors lane
 * priority for free), a coalescing scan on `enqueue`, and eviction of the lowest-priority nonempty
 * lane before a hard `rejected`. */
export interface BoundedMailbox<T> {
  enqueue(envelope: MailboxEnvelope<T>): Backpressure;
  popNext(): MailboxEnvelope<T> | undefined;
  readonly length: number;
  readonly isEmpty: boolean;
}

/**
 * @emoji 📬️ Creates a {@link BoundedMailbox} of `capacity` envelopes total across all four lanes —
 * the TypeScript twin of Rust `Mailbox::new`/`Mailbox::enqueue`/`Mailbox::pop_next`.
 *
 * `enqueue` first does a latest-wins coalescing scan within the incoming envelope's own lane
 * (replacing an existing envelope with the same {@link CoalesceKey} in place, preserving its queue
 * position — a hot key must not jump the line): `coalesced`. Otherwise, if the ring is full, it
 * evicts the single lowest-priority nonempty lane strictly below the incoming lane (never the
 * incoming lane itself or anything higher-priority): `dropped(lane)`. If there is nothing
 * lower-priority to evict, the envelope is `rejected` outright rather than silently discarded.
 * Otherwise: `accept`.
 *
 * ## causal order (`MailboxEnvelope.order`, INPUT-CAUSALITY-LEDGER §2 B, law L2)
 * Within ONE lane the dequeue position is decided at `enqueue` time by this rule, and by nothing
 * else — `popNext` stays a plain head pop:
 * - An envelope WITHOUT `order` is appended at the tail (pure arrival order, byte-for-byte the old
 *   behaviour: a mailbox that never sees `order` cannot observe this feature).
 * - An envelope WITH `order` is inserted immediately BEFORE the earliest-queued envelope of the same
 *   lane that carries a strictly larger `order`; if there is none it is appended at the tail. So the
 *   ordered envelopes of a lane are kept sorted by `(order, arrival)` — equal `order` keeps arrival
 *   order — and an ordered envelope overtakes exactly the strictly-larger ordered envelope it lands
 *   in front of plus everything (ordered or not) that was queued behind that one.
 * - Unordered envelopes never reorder among themselves, and an ordered envelope never overtakes an
 *   unordered envelope queued ahead of every larger-ordered one. This is the "ordered-only"
 *   alternative rather than the mixed key `(order ?? arrivalSeq, arrivalSeq)`: an arrival counter
 *   is mailbox-internal, so no caller could ever make its own `order` space comparable with it,
 *   which would have made the mixed rule's position of unordered envelopes arbitrary. The runtime's
 *   use is covered exactly: inputs and their guest follow-ups all carry the ledger's causal key
 *   (`causedBy ?? inputSeq`), so a follow-up of input N lands before the already-queued input N+1
 *   and ahead of any internal maintenance turn queued behind it; unordered envelopes are those
 *   maintenance turns, which may legitimately wait.
 * - Coalescing keeps the ORIGINAL envelope's queue position (a hot key must not jump the line, in
 *   either direction); the replacement takes the newer envelope's `order` when it carries one and
 *   otherwise retains the original's, so the slot stays addressable by later ordered insertions.
 *   That is the one place the sorted-by-`order` invariant may be relaxed — position wins.
 * - `dropped`/`rejected` are untouched: eviction still takes the HEAD of the victim lane (whatever
 *   the rule put there), and capacity is counted per envelope exactly as before.
 * - `order` is TypeScript-only for now: the Rust twin's `Envelope` has `seq`/`deadline_ms` but no
 *   causal key; lifting it there is the design's native follow-up, not this module's concern.
 */
export function createBoundedMailbox<T>(capacity: number): BoundedMailbox<T> {
  const lanes: MailboxEnvelope<T>[][] = MAILBOX_LANE_ORDER.map(() => []);
  let len = 0;

  /** 🔗️ Applies the `## causal order` rule: tail for unordered, otherwise before the earliest
   * queued envelope with a strictly larger `order` (ties and unordered neighbours are not passed). */
  function insertByOrder(lane: MailboxEnvelope<T>[], envelope: MailboxEnvelope<T>): void {
    const order = envelope.order;
    if (order !== undefined) {
      for (let index = 0; index < lane.length; index++) {
        const queued = lane[index]!.order;
        if (queued !== undefined && queued > order) {
          lane.splice(index, 0, envelope);
          return;
        }
      }
    }
    lane.push(envelope);
  }

  return {
    enqueue(envelope: MailboxEnvelope<T>): Backpressure {
      const incomingRank = laneRank(envelope.lane);
      if (envelope.coalesce !== undefined) {
        const lane = lanes[incomingRank]!;
        const existingIndex = lane.findIndex((queued) => queued.coalesce === envelope.coalesce);
        if (existingIndex !== -1) {
          const existing = lane[existingIndex]!;
          lane[existingIndex] = envelope.order === undefined && existing.order !== undefined ? { ...envelope, order: existing.order } : envelope;
          return { kind: "coalesced" };
        }
      }
      if (len >= capacity) {
        let victimRank = -1;
        for (let rank = MAILBOX_LANE_ORDER.length - 1; rank > incomingRank; rank--) {
          if (lanes[rank]!.length > 0) {
            victimRank = rank;
            break;
          }
        }
        if (victimRank === -1) return { kind: "rejected" };
        lanes[victimRank]!.shift();
        len -= 1;
        insertByOrder(lanes[incomingRank]!, envelope);
        len += 1;
        return { kind: "dropped", lane: MAILBOX_LANE_ORDER[victimRank]! };
      }
      insertByOrder(lanes[incomingRank]!, envelope);
      len += 1;
      return { kind: "accept" };
    },
    popNext(): MailboxEnvelope<T> | undefined {
      for (const lane of lanes) {
        if (lane.length > 0) {
          len -= 1;
          return lane.shift();
        }
      }
      return undefined;
    },
    get length(): number {
      return len;
    },
    get isEmpty(): boolean {
      return len === 0;
    },
  };
}
//#endregion 📬️BoundedMailbox

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️createboundedmailbox/🟦️.ts");
  await registerTests1(import.meta.vitest, { createBoundedMailbox }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests
