/** 🔀️ The merge policy, conflict, unsolicited report and fault decoding tests. */
import { expect, it } from "vitest";
import { AppChannelClient, AppChannelRequestSequence, decodeConflictsFromWire, decodeDispatchReportFromWire, decodeMergeReportFromWire, encodeAppCommand, faultMessages } from "@semio-tech/framework/app-channel";
import { decodePackValue, encodePackValue } from "@semio-tech/framework/packs";
import { source, type AppCommandValue, type AppFrameValue } from "../../🛠️backbone-wire-helpers/🟦️.ts";
import { fakeHandle } from "../🧰️app-channel-client-fixtures/🟦️.ts";
import type { Conflict, DispatchReport, MergeReport } from "../🧰️app-channel-client-fixtures/🟦️.ts";

/** 🧯 Registers the merge policy and conflict vectors, the unsolicited report frames and the fault and wire decoders. */
export function registerMergePolicyAndFaultDecodingTests(): void {
  /**
   * 🔗️ Rust↔TS parity for the C8 merge/conflict surface, driven through {@link AppChannelClient}'s
   * OWN public methods rather than the raw `encodeAppCommand`/`decodeAppFrame` codec functions the
   * `AppChannelCodec` suite already asserts against the same two files — this is the "does the
   * CLIENT layer itself send/surface the new commands and frames correctly" half of the parity
   * story (contract-freeze §C8/§C9, ticket
   * `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`). Four throwaway
   * `configure({})` calls burn seq 1-4 so `setMergePolicy`/`resolveConflict`/`readConflicts` land on
   * the exact seq (5/6/7) the golden vectors were baked against.
   */
  it("setMergePolicy()/resolveConflict()/readConflicts() match the shared cross-language merge command vectors, byte-for-byte", async () => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const here = dirname(fileURLToPath(source.url));
    const commandVectors = JSON.parse(readFileSync(join(here, "🧪️tests/🧫️fixtures", "📡️channel", "🔀️app-command-merge.json"), "utf8")) as Record<string, string>;
    const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");

    const seen: AppCommandValue[] = [];
    const handle = fakeHandle((_instanceId, commands) => {
      seen.push(...commands);
      return [{ Done: { in_reply_to: Object.values(commands[0]!)[0]!.seq } }];
    });
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    await client.configure({});
    await client.configure({});
    await client.configure({});
    await client.configure({});
    await client.setMergePolicy("Normal");
    await client.resolveConflict("conflict-1", "accept");
    await client.readConflicts();

    expect(seen[4]).toEqual({ setMergePolicy: { seq: 5, policy: 1 } });
    expect(seen[5]).toEqual({ resolveConflict: { seq: 6, conflict_id: "conflict-1", resolution: 0 } });
    expect(seen[6]).toEqual({ readConflicts: { seq: 7 } });
    expect(hex(encodeAppCommand(seen[4]!)), "AppCommand::SetMergePolicy").toBe(commandVectors.SetMergePolicy);
    expect(hex(encodeAppCommand(seen[5]!)), "AppCommand::ResolveConflict").toBe(commandVectors.ResolveConflict);
    expect(hex(encodeAppCommand(seen[6]!)), "AppCommand::ReadConflicts").toBe(commandVectors.ReadConflicts);
  });
  it("command() surfaces unsolicited MergeReport/Conflicts frames and the extended Invocation.messages/Error.report fields verbatim", async () => {
    const handle = fakeHandle(() => [
      { Invocation: { in_reply_to: 1, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [9], mutations: [], inverse_group: [] } },
      { MergeReport: { in_reply_to: null, report: [1] } },
      { Conflicts: { in_reply_to: null, conflicts: [2] } },
    ]);
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
    const frames = await client.command(new Uint8Array([1]), {});
    expect(frames).toHaveLength(3);
    const invocation = frames.find((frame): frame is Extract<AppFrameValue, { readonly Invocation: unknown }> => "Invocation" in frame);
    const mergeReport = frames.find((frame): frame is Extract<AppFrameValue, { readonly MergeReport: unknown }> => "MergeReport" in frame);
    const conflicts = frames.find((frame): frame is Extract<AppFrameValue, { readonly Conflicts: unknown }> => "Conflicts" in frame);
    expect(invocation?.Invocation.messages).toEqual([9]);
    expect(mergeReport?.MergeReport.report).toEqual([1]);
    expect(conflicts?.Conflicts.conflicts).toEqual([2]);
  });
  /**
   * 🔗️ Round-trips the frozen TS shapes ({@link DispatchReport}/{@link MergeReport}/{@link
   * Conflict}) through {@link encodePackValue}/the new `decode*FromWire` helpers — proving the
   * field-name mapping this lane derived from Rust's `#[serde(rename_all = "camelCase")]`
   * (`policy`/`worst`/`messages`, `insertionIndex`, `editId`→`edit_id` NOT renamed inside
   * `ConflictKind`'s struct variants, `MergePolicy`'s bare un-camelCased variant names) actually
   * decodes the way the wire's `store::pack_rt::encode_wire_value`-backed `encode_wire_serialized`
   * would produce it, since no live Rust `DispatchReport`/`MergeReport`/`Conflict` pack bytes are
   * checked into a fixture yet (only the outer `AppFrame` framing is, in `📢️app-frame-merge.json`).
   */
  it("faultMessages()/decodeDispatchReportFromWire()/decodeMergeReportFromWire()/decodeConflictsFromWire() decode the frozen TS report shapes", () => {
    const dispatchReport: DispatchReport = {
      policy: "Vigilant",
      worst: "warning",
      messages: [{ level: "warning", code: "mutation.clamped", message: "value clamped to range" }],
    };
    const reportBytes = Array.from(encodePackValue(dispatchReport));
    expect(decodeDispatchReportFromWire(reportBytes, decodePackValue)).toEqual(dispatchReport);
    expect(faultMessages(reportBytes, decodePackValue)).toEqual(dispatchReport.messages);
    expect(faultMessages([], decodePackValue)).toEqual([]);

    const mergeReport: MergeReport = {
      policy: "Normal",
      accepted: true,
      insertionIndex: 3,
      replayed: [{ edit_id: "e1", messages: [{ level: "info", code: "mutation.cascade", message: "cascaded" }] }],
      worst: "info",
      conflict: null,
    };
    expect(decodeMergeReportFromWire(Array.from(encodePackValue(mergeReport)), decodePackValue)).toEqual(mergeReport);
    expect(decodeMergeReportFromWire([], decodePackValue)).toBeNull();

    const conflicts: readonly Conflict[] = [
      {
        id: "conflict-abc",
        kind: { kind: "degraded", edit_ids: ["e1"] },
        status: "open",
        messages: [{ level: "error", code: "mutation.target-missing", message: "target missing" }],
        actors: ["actor-1"],
        timestamp: { actor: 1, physical_ms: 100, logical: 0 },
      },
    ];
    expect(decodeConflictsFromWire(Array.from(encodePackValue(conflicts)), decodePackValue)).toEqual(conflicts);
    expect(decodeConflictsFromWire([], decodePackValue)).toEqual([]);
  });
}
