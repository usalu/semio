/** 📬️ `BoundedMailbox` — the TypeScript twin of the Rust `Mailbox` in `🎭️actor/🦀️component.rs`'s
 * `📬️Mailbox` region: same four-lane bounded ring (so `popNext` honors lane priority for free), same
 * latest-wins coalescing scan on `enqueue`, same lowest-priority-nonempty-lane eviction before a hard
 * reject.
 *
 * 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P1, relocated by coordinator "sol"'s follow-up):
 * originally landed in the domain-neutral `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`, but this
 * is the actor module's own vocabulary twin and its main consumer (T-P4's turn scheduler) lives in
 * this same `@semio-tech/framework-actor` package — "if code is repeated, it MUST be close to each
 * other" (CLAUDE.md) argues for living beside `🧵️shard-client.ts`, not in the generic base package,
 * which also cannot reach this module's `🤖️generated/🟦️actor.ts` without inverting the layering.
 */

//#region 🔌️WireTypes
/** ⚖️ `Lane`/`CoalesceKey` are taken from the ts-rs-generated mirror — they are real wire types (no
 * `#[serde(rename_all)]` on Rust `Lane`, so its wire form is PascalCase, e.g. `"Interactive"`, not
 * `"interactive"`) — redeclaring them locally would silently drift from the wire the moment either
 * side changes. `Backpressure` is declared fresh below instead: the generated mirror's `Backpressure`
 * is `{ "kind": "dropped" } & Lane`, an object-intersected-with-a-string-literal-union type that no
 * value can ever satisfy (a ts-rs limitation on tuple-variant enums combined with
 * `rename_all_fields`), so importing it would be unusable, not just inconvenient. */
import type { Lane, CoalesceKey } from "../../🤖️generated/🟦️actor.ts";
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

/** @emoji ✉️ One message offered to a {@link BoundedMailbox}: its scheduling lane, payload, and an
 * optional coalescing key that lets a newer envelope replace an older queued one in place. */
export interface MailboxEnvelope<T> {
  readonly lane: Lane;
  readonly coalesce?: CoalesceKey;
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
 */
export function createBoundedMailbox<T>(capacity: number): BoundedMailbox<T> {
  const lanes: MailboxEnvelope<T>[][] = MAILBOX_LANE_ORDER.map(() => []);
  let len = 0;

  return {
    enqueue(envelope: MailboxEnvelope<T>): Backpressure {
      const incomingRank = laneRank(envelope.lane);
      if (envelope.coalesce !== undefined) {
        const lane = lanes[incomingRank]!;
        const existingIndex = lane.findIndex((queued) => queued.coalesce === envelope.coalesce);
        if (existingIndex !== -1) {
          lane[existingIndex] = envelope;
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
        lanes[incomingRank]!.push(envelope);
        len += 1;
        return { kind: "dropped", lane: MAILBOX_LANE_ORDER[victimRank]! };
      }
      lanes[incomingRank]!.push(envelope);
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
  const { describe, expect, it } = import.meta.vitest;

  describe("createBoundedMailbox", () => {
    it("overflow is rejected (not a silent drop) when nothing lower-priority exists to evict", () => {
      const mailbox = createBoundedMailbox<string>(2);
      expect(mailbox.enqueue({ lane: "Maintenance", payload: "a" })).toEqual({ kind: "accept" });
      expect(mailbox.enqueue({ lane: "Maintenance", payload: "b" })).toEqual({ kind: "accept" });
      expect(mailbox.enqueue({ lane: "Maintenance", payload: "c" })).toEqual({ kind: "rejected" });
      expect(mailbox.length).toBe(2);
    });

    it("coalescing collapses same-key entries latest-wins, preserving queue position", () => {
      const mailbox = createBoundedMailbox<number>(10);
      for (let i = 0; i < 200; i++) {
        const backpressure = mailbox.enqueue({ lane: "Interactive", coalesce: "pointer-move", payload: i });
        expect(backpressure.kind === "accept" || backpressure.kind === "coalesced").toBe(true);
      }
      expect(mailbox.length).toBe(1);
      expect(mailbox.popNext()?.payload).toBe(199);
    });

    it("lane priority beats FIFO order on popNext", () => {
      const mailbox = createBoundedMailbox<string>(10);
      mailbox.enqueue({ lane: "Maintenance", payload: "low" });
      mailbox.enqueue({ lane: "Background", payload: "mid" });
      mailbox.enqueue({ lane: "Interactive", payload: "high" });
      expect(mailbox.popNext()?.lane).toBe("Interactive");
      expect(mailbox.popNext()?.lane).toBe("Background");
      expect(mailbox.popNext()?.lane).toBe("Maintenance");
      expect(mailbox.isEmpty).toBe(true);
    });

    it("dropped backpressure reports the evicted lane, admitting the higher-priority incomer", () => {
      const mailbox = createBoundedMailbox<string>(2);
      mailbox.enqueue({ lane: "Maintenance", payload: "a" });
      mailbox.enqueue({ lane: "Background", payload: "b" });
      const backpressure = mailbox.enqueue({ lane: "Interactive", payload: "c" });
      expect(backpressure).toEqual({ kind: "dropped", lane: "Maintenance" });
      expect(mailbox.length).toBe(2);
      expect(mailbox.popNext()?.payload).toBe("c");
      expect(mailbox.popNext()?.payload).toBe("b");
    });
  });
}
//#endregion 🧪️Tests
