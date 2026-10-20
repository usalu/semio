/** 🟠️ The pins app channel version tests of the @semio-tech/framework-os AppChannelCodec suite. */
import { expect, it } from "vitest";
import { APP_CHANNEL_VERSION, CHANNEL_MISMATCH_CODE, admitGuestChannelVersion, decodeAppCommand, decodeAppFrame, encodeAppCommand, encodeAppFrame } from "@semio-tech/framework/app-channel";
import { source, type AppCommandValue, type AppFrameValue } from "../../🛠️backbone-wire-helpers/🟦️.ts";

/** 🟡 Registers the pins app channel version tests of the @semio-tech/framework-os AppChannelCodec suite. */
export function registerPinsAppChannelVersionTests(): void {
  /**
   * 🔗️ Cross-language drift guard for the M2 transaction variants (tags 22-26/19-22): both this
   * suite and `protocol_channel`'s `channel_transaction_fixtures_match_shared_cross_language_json_vectors`
   * Rust test load the SAME two JSON files under `🧫️fixtures/📡️channel/` — there is exactly one
   * committed hex string per label, not a copy per language, so a codec change that shifts these
   * bytes on either side fails here or there, never silently in both at once.
   */
  it("pins APP_CHANNEL_VERSION against the shared cross-language channel version", async () => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const here = dirname(fileURLToPath(source.url));
    const pin = JSON.parse(readFileSync(join(here, "🔨️modules/🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json"), "utf8")) as { channelVersion: number };
    expect(APP_CHANNEL_VERSION).toBe(pin.channelVersion);
  });
  it("admits only a guest of the host's own channel version, naming both versions when it refuses (shared handshake corpus)", async () => {
    const { readFileSync } = await import("node:fs");
    const law = JSON.parse(readFileSync(new URL("./🔨️modules/📡️spr/🧵️channel/🧪️tests/🧫️fixtures/🧫️channel-handshake/🔣️.json", source.url), "utf8")) as { code: string; cases: { name: string; guestOffset: number; hostOffset?: number; admitted: boolean }[] };
    expect(law.code).toBe(CHANNEL_MISMATCH_CODE);
    for (const row of law.cases) {
      const guest = APP_CHANNEL_VERSION + row.guestOffset;
      const host = APP_CHANNEL_VERSION + (row.hostOffset ?? 0);
      expect(row.admitted, row.name).toBe(guest === host);
      const refusal = admitGuestChannelVersion(guest, host);
      if (row.admitted) expect(refusal, row.name).toBeNull();
      else expect([refusal?.code, refusal?.origin, refusal?.params], row.name).toEqual([CHANNEL_MISMATCH_CODE, "framework", { guest: String(guest), host: String(host) }]);
    }
  });
  it("matches the shared cross-language transaction fixture vectors, byte-for-byte", async () => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const here = dirname(fileURLToPath(source.url));
    const channelFixturesDir = join(here, "🧪️tests/🧫️fixtures", "📡️channel");
    const commandVectors = JSON.parse(readFileSync(join(channelFixturesDir, "🧾️app-command-transaction.json"), "utf8")) as Record<string, string>;
    const frameVectors = JSON.parse(readFileSync(join(channelFixturesDir, "📨️app-frame-transaction.json"), "utf8")) as Record<string, string>;
    const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");

    const commandCases: Readonly<Record<string, AppCommandValue>> = {
      TransactionPrepareOwner: { transactionPrepare: { seq: 1, txn_id: "t", mutation_id: "m", payload: [9], prepared_ops: [], origin: [], prepared_child_ops: [] } },
      TransactionPreparePrePlanned: { transactionPrepare: { seq: 2, txn_id: "t", mutation_id: "", payload: [], prepared_ops: [[1], [2, 2]], origin: [9], prepared_child_ops: [] } },
      TransactionPreparePrePlannedChildren: { transactionPrepare: { seq: 7, txn_id: "t", mutation_id: "", payload: [], prepared_ops: [[1]], origin: [9], prepared_child_ops: [5, 6] } },
      TransactionCommit: { transactionCommit: { seq: 3, txn_id: "t" } },
      TransactionRollback: { transactionRollback: { seq: 4, txn_id: "t" } },
      TransactionUndo: { transactionUndo: { seq: 5, group_id: "g" } },
      TransactionRedo: { transactionRedo: { seq: 6, group_id: "g" } },
    };
    const frameCases: Readonly<Record<string, AppFrameValue>> = {
      TransactionProposal: { transactionProposal: { in_reply_to: 1, proposal_id: "p", local_ops: [[1]], foreign: [] } },
      TransactionPrepared: { transactionPrepared: { txn_id: "t", foreign: [[1]], rejection: [] } },
      TransactionCommitted: { transactionCommitted: { txn_id: "t", edit_id: "e" } },
      TransactionRolledBack: { transactionRolledBack: { txn_id: "t" } },
    };

    expect(Object.keys(commandVectors).sort()).toEqual(Object.keys(commandCases).sort());
    expect(Object.keys(frameVectors).sort()).toEqual(Object.keys(frameCases).sort());
    for (const [label, value] of Object.entries(commandCases)) {
      expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandVectors[label]);
      expect(decodeAppCommand(new Uint8Array(Buffer.from(commandVectors[label]!, "hex")))).toEqual(value);
    }
    for (const [label, value] of Object.entries(frameCases)) {
      expect(hex(encodeAppFrame(value)), `AppFrame::${label}`).toBe(frameVectors[label]);
      expect(decodeAppFrame(new Uint8Array(Buffer.from(frameVectors[label]!, "hex")))).toEqual(value);
    }
  });
  /**
   * 🔗️ Cross-language drift guard for the C3 opening variants (tags 27-29): both this suite and
   * `protocol_channel`'s `channel_opening_fixtures_match_shared_cross_language_json_vectors` Rust
   * test load the SAME JSON file under `🧫️fixtures/📡️channel/` — no `AppFrame` variants were added
   * for opening, so only the command-side vector file exists.
   */
  it("matches the shared cross-language opening fixture vectors, byte-for-byte", async () => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const here = dirname(fileURLToPath(source.url));
    const channelFixturesDir = join(here, "🧪️tests/🧫️fixtures", "📡️channel");
    const commandVectors = JSON.parse(readFileSync(join(channelFixturesDir, "🚪️app-command-opening.json"), "utf8")) as Record<string, string>;
    const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");

    const commandCases: Readonly<Record<string, AppCommandValue>> = {
      OpenArtifactResolve: { openArtifact: { seq: 1, artifact_ref: "s.cad.cad@1/*#viewer", role: 0, plugin_id: "", app_id: "" } },
      OpenArtifactExplicit: { openArtifact: { seq: 2, artifact_ref: "s.cad.cad@1/*#editor", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
      SetDefaultApp: { setDefaultApp: { seq: 3, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
      ClearDefaultApp: { clearDefaultApp: { seq: 4, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 0 } },
    };

    expect(Object.keys(commandVectors).sort()).toEqual(Object.keys(commandCases).sort());
    for (const [label, value] of Object.entries(commandCases)) {
      expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandVectors[label]);
      expect(decodeAppCommand(new Uint8Array(Buffer.from(commandVectors[label]!, "hex")))).toEqual(value);
    }
  });
  /**
   * 🔗️ Cross-language drift guard for the C8 merge-policy/conflict variants (tags 30-32/23-24)
   * plus the extended `Invocation`/`Error` frames: both this suite and `protocol_channel`'s
   * `channel_merge_fixtures_match_shared_cross_language_json_vectors` Rust test load the SAME two
   * JSON files under `🧫️fixtures/📡️channel/` — see contract-freeze.md §C8 of
   * `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/`.
   */
  it("matches the shared cross-language merge fixture vectors, byte-for-byte", async () => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const here = dirname(fileURLToPath(source.url));
    const channelFixturesDir = join(here, "🧪️tests/🧫️fixtures", "📡️channel");
    const commandVectors = JSON.parse(readFileSync(join(channelFixturesDir, "🔀️app-command-merge.json"), "utf8")) as Record<string, string>;
    const frameVectors = JSON.parse(readFileSync(join(channelFixturesDir, "📢️app-frame-merge.json"), "utf8")) as Record<string, string>;
    const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");

    const commandCases: Readonly<Record<string, AppCommandValue>> = {
      SetMergePolicy: { setMergePolicy: { seq: 5, policy: 1 } },
      ResolveConflict: { resolveConflict: { seq: 6, conflict_id: "conflict-1", resolution: 0 } },
      ReadConflicts: { readConflicts: { seq: 7 } },
    };
    const frameCases: Readonly<Record<string, AppFrameValue>> = {
      MergeReport: { MergeReport: { in_reply_to: 1, report: [1] } },
      Conflicts: { Conflicts: { in_reply_to: null, conflicts: [2] } },
      Invocation: { Invocation: { in_reply_to: 1, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [9], mutations: [10], inverse_group: [11] } },
      Error: { Error: { in_reply_to: null, fault: [99], report: [7] } },
    };

    expect(Object.keys(commandVectors).sort()).toEqual(Object.keys(commandCases).sort());
    expect(Object.keys(frameVectors).sort()).toEqual(Object.keys(frameCases).sort());
    for (const [label, value] of Object.entries(commandCases)) {
      expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandVectors[label]);
      expect(decodeAppCommand(new Uint8Array(Buffer.from(commandVectors[label]!, "hex")))).toEqual(value);
    }
    for (const [label, value] of Object.entries(frameCases)) {
      expect(hex(encodeAppFrame(value)), `AppFrame::${label}`).toBe(frameVectors[label]);
      expect(decodeAppFrame(new Uint8Array(Buffer.from(frameVectors[label]!, "hex")))).toEqual(value);
    }
  });
}
