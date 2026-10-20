/** 🔸️ The tags appframe variant per tests of the @semio-tech/framework-os AppChannelCodec suite. */
import { expect, it } from "vitest";
import { decodeAppCommand, decodeAppFrame, encodeAppCommand, encodeAppFrame } from "@semio-tech/framework/app-channel";
import { decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse } from "@semio-tech/framework/backbone";
import { decodeDocumentArchiveBytes, encodeDocumentArchiveBytes } from "@semio-tech/framework/wire-contracts";
import { source, documentArchive, type AppCommandValue, type AppFrameValue } from "../../🛠️backbone-wire-helpers/🟦️.ts";

/** 🔹 Registers the tags appframe variant per tests of the @semio-tech/framework-os AppChannelCodec suite. */
export function registerTagsAppframeVariantPerTests(): void {
  it("tags every AppFrame variant per the agreed contract order", () => {
    expect(encodeAppFrame({ Done: { in_reply_to: 0 } })[0]).toBe(0);
    expect(encodeAppFrame({ Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } })[0]).toBe(1);
    expect(encodeAppFrame({ Error: { in_reply_to: null, fault: [], report: [] } })[0]).toBe(9);
    expect(encodeAppFrame({ Ephemeral: { presence: [], presence_generation: 0, transient_generation: 0, interaction: [], tool_run: [], history_edit: [] } })[0]).toBe(13);
    expect(encodeAppFrame({ HistorySnapshot: { in_reply_to: 0, history_patch: [] } })[0]).toBe(14);
    expect(encodeAppFrame({ transactionProposal: { in_reply_to: 0, proposal_id: "", local_ops: [], foreign: [] } })[0]).toBe(15);
    expect(encodeAppFrame({ transactionPrepared: { txn_id: "", foreign: [], rejection: [] } })[0]).toBe(16);
    expect(encodeAppFrame({ transactionCommitted: { txn_id: "", edit_id: "" } })[0]).toBe(17);
    expect(encodeAppFrame({ transactionRolledBack: { txn_id: "" } })[0]).toBe(18);
    expect(encodeAppFrame({ MergeReport: { in_reply_to: null, report: [] } })[0]).toBe(19);
    expect(encodeAppFrame({ Conflicts: { in_reply_to: null, conflicts: [] } })[0]).toBe(20);
    expect(encodeAppFrame({ UiPatch: { in_reply_to: null, surface: "", kind: "", revision: 0, base_revision: 0, ops: [] } })[0]).toBe(21);
    expect(encodeAppFrame({ UiSnapshotEnd: { revision: 0 } })[0]).toBe(22);
    expect(encodeAppFrame({ WindowConfigs: { in_reply_to: 0, entries: [] } })[0]).toBe(24);
    expect(encodeAppFrame({ DocumentArchive: { in_reply_to: 0, archive: { parent_pack: [], parent_spr: [], members: [] } } })[0]).toBe(26);
    expect(encodeAppFrame({ DocumentArchiveLoad: { in_reply_to: 0, status: { operation: 1, state: "ready", completed: 2, total: 2, ahead: 0, fault: [] } } })[0]).toBe(27);
    expect(encodeAppFrame({ ChildHeads: { in_reply_to: 0, entries: [] } })[0]).toBe(32);
    expect(() => decodeAppFrame(new Uint8Array([32, 1, 0x81, 0x08]))).toThrow("decodeAppFrame: child head count exceeds the member authority");
  });
  it("rejects recursive document archives beyond their fixed member authority", () => {
    const oversized = {
      parent_pack: [],
      parent_spr: [],
      members: Array.from({ length: 1_025 }, (_, ordinal) => ({
        ordinal,
        reference: { artifact_id: `child-${ordinal}`, artifact_kind: "s.test.child", standard: "1", subset: "*" },
        owner: { parent: { artifact_id: "root-1", artifact_kind: "s.test.root", standard: "1", subset: "*" }, slot: "children", child_id: `child-${ordinal}` },
        envelope_pack: [ordinal & 0xff],
      })),
    };
    expect(() => encodeAppCommand({ LoadDocumentArchive: { seq: 1, archive: oversized } })).toThrow("1024-member authority");
    expect(() => decodeAppCommand(Uint8Array.from([32, 1, 0, 0, 0x81, 0x08]))).toThrow("1024-member authority");
  });
  it("persists the complete recursive document archive as one exact versioned byte owner", () => {
    const encoded = encodeDocumentArchiveBytes(documentArchive);
    expect(decodeDocumentArchiveBytes(encoded)).toEqual(documentArchive);
    expect(() => decodeDocumentArchiveBytes(Uint8Array.from([...encoded, 0]))).toThrow("trailing bytes");
    expect(() => decodeDocumentArchiveBytes(Uint8Array.from([2]))).toThrow("unsupported or missing version");
  });
  it("carries one exact recursive document archive through actor request and event wires", () => {
    const archive = Array.from(encodeDocumentArchiveBytes(documentArchive));
    const clientInstanceId = "12345678-1234-4123-8123-123456789abc";
    expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest({ kind: "send", documentId: "root-1", clientInstanceId, message: { kind: "localDocumentArchive", archive } }))).toEqual({ kind: "send", documentId: "root-1", clientInstanceId, message: { kind: "localDocumentArchive", archive } });
    expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse({ kind: "event", documentId: "root-1", clientInstanceId, event: { kind: "documentArchiveReplaced", archive } }))).toEqual({ kind: "event", documentId: "root-1", clientInstanceId, event: { kind: "documentArchiveReplaced", archive } });
    expect(() => encodeBackboneWorkerRequest({ kind: "send", documentId: "root-1", clientInstanceId, message: { kind: "localDocumentArchive", archive: [2] } })).toThrow("unsupported or missing version");
    expect(() => encodeBackboneWorkerResponse({ kind: "event", documentId: "root-1", clientInstanceId, event: { kind: "documentArchiveReplaced", archive: [2] } })).toThrow("unsupported or missing version");
  });
  /**
   * 🔒️ Cross-language drift guard: the exact same fixture values and golden hex committed in
   * `protocol_channel`'s own `🔖️Corpus` region (`🔨️modules/📡️protocol/🧵️channel/📦️packages/🦀️rust/📦️lib.rs`,
   * `channel_command_fixture_corpus`/`channel_command_fixture_hex` and their `AppFrame` twins) —
   * sourced by running the real `encode_app_command`/`encode_app_frame` and copying their
   * printed `[TRACE] AppCommand::<label> = <hex>` output (`cargo test -p semio-protocol-channel
   * -- --nocapture`), NOT hand-computed. Any future change to either codec that shifts these
   * bytes fails on exactly one side, forcing a deliberate update of both this table and the Rust
   * golden hex in the same change.
   */
  it("matches protocol_channel's own golden hex fixture corpus, byte-for-byte", () => {
          const commandFixtures: readonly (readonly [string, AppCommandValue])[] = [
      ["ConfigCommand", { ConfigCommand: { seq: 1, command: [9] } }],
      ["Command", { Command: { seq: 1, command: [1], view_state: [] } }],
      ["CommandText", { CommandText: { seq: 1, line: "go" } }],
      ["ContextMenu", { ContextMenu: { seq: 1, request: [1] } }],
      ["ArtifactCommand", { ArtifactCommand: { seq: 1, command: [1] } }],
      ["ApplyEnvelopes", { ApplyEnvelopes: { seq: 1, envelopes: [] } }],
      ["ReadDocument", { ReadDocument: { seq: 1 } }],
      ["LoadConfig", { LoadConfig: { seq: 1, pack: [1], spr: [2] } }],
      ["ReadConfig", { ReadConfig: { seq: 1 } }],
      ["LoadWindowConfig", { LoadWindowConfig: { seq: 1, entry: { window_id: "w1", window_kind_id: "graph", envelope_pack: [1, 2] } } }],
      ["ReadWindowConfigs", { ReadWindowConfigs: { seq: 1 } }],
      ["MediaIn", { MediaIn: { seq: 1, port: "p", descriptor: [1], data: [2] } }],
      ["MediaOut", { MediaOut: { seq: 1, port: "p", request: [1] } }],
      ["MediaFingerprint", { MediaFingerprint: { seq: 1, port: "p" } }],
      ["PureCommand", { PureCommand: { seq: 1, command: [1], head: [2] } }],
      ["LoadChildren", { LoadChildren: { seq: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1], owner: "" }] } }],
      ["LoadChildrenOfAMember", { LoadChildren: { seq: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1], owner: "content/forms-1" }] } }],
      ["ReadChildren", { ReadChildren: { seq: 1 } }],
      ["ReadChildHeads", { ReadChildHeads: { seq: 1 } }],
      ["ReadHistory", { ReadHistory: { seq: 1 } }],
    ];
          const commandGoldenHex: Readonly<Record<string, string>> = {
      ConfigCommand: "00010109",
      Command: "0101010100",
      CommandText: "020102676f",
      ContextMenu: "03010101",
      ArtifactCommand: "04010101",
      ApplyEnvelopes: "050100",
      ReadDocument: "0701",
      LoadConfig: "080101010102",
      ReadConfig: "0901",
      LoadWindowConfig: "1e01027731056772617068020102",
      ReadWindowConfigs: "1f01",
      MediaIn: "0a01017001010102",
      MediaOut: "0b0101700101",
      MediaFingerprint: "0c010170",
      PureCommand: "0d0101010102",
      LoadChildren: "0e0101017301630164010100",
      LoadChildrenOfAMember: "0e010101730163016401010f636f6e74656e742f666f726d732d31",
      ReadChildren: "0f01",
      ReadChildHeads: "2a01",
      ReadHistory: "1001",
    };
          const frameFixtures: readonly (readonly [string, AppFrameValue])[] = [
      ["Done", { Done: { in_reply_to: 1 } }],
      ["Invocation", { Invocation: { in_reply_to: 1, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } }],
      ["DocumentChanged", { DocumentChanged: { envelopes: [], origin: "o" } }],
      ["Document", { Document: { in_reply_to: 1, pack: [1], spr: [2], ops: "o" } }],
      ["Config", { Config: { in_reply_to: 1, pack: [1], spr: [2], ops: "c" } }],
      ["WindowConfigs", { WindowConfigs: { in_reply_to: 1, entries: [{ window_id: "w1", window_kind_id: "graph", envelope_pack: [1, 2] }, { window_id: "w2", window_kind_id: "graph", envelope_pack: [3] }] } }],
      ["ConfigChanged", { ConfigChanged: { envelopes: [], origin: "o" } }],
      ["ContextMenu", { ContextMenu: { in_reply_to: 1, items: [1] } }],
      ["Media", { Media: { in_reply_to: 1, port: "p", descriptor: [1], data: [2] } }],
      ["MediaFingerprint", { MediaFingerprint: { in_reply_to: 1, port: "p", fingerprint: [1] } }],
      ["Error", { Error: { in_reply_to: null, fault: [99], report: [] } }],
      ["Emit", { Emit: { in_reply_to: 1, document_ops: [1], config_ops: [], draft_ops: [], output: [2], diagnostics: [], child_ops: [] } }],
      ["Draft", { Draft: { in_reply_to: 1, pack: [1], spr: [2], ops: "d" } }],
      ["Children", { Children: { in_reply_to: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1], owner: "" }] } }],
      ["ChildrenOfAMember", { Children: { in_reply_to: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1], owner: "content/forms-1" }] } }],
      ["ChildHeads", { ChildHeads: { in_reply_to: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", head_pack: [1], owner: "" }] } }],
      ["ChildHeadsOfAMember", { ChildHeads: { in_reply_to: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", head_pack: [1], owner: "content/forms-1" }] } }],
      ["Ephemeral", { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [], tool_run: [], history_edit: [] } }],
      ["HistorySnapshot", { HistorySnapshot: { in_reply_to: 1, history_patch: [1] } }],
      ["UiPatch", { UiPatch: { in_reply_to: 1, surface: "1:body", kind: "window", revision: 3, base_revision: 2, ops: [9] } }],
      ["UiSnapshotEnd", { UiSnapshotEnd: { revision: 6 } }],
    ];
          const frameGoldenHex: Readonly<Record<string, string>> = {
      Done: "0001",
      Invocation: "01010101000000000000",
      DocumentChanged: "0200016f",
      Document: "030101010102016f",
      Config: "0401010101020163",
      WindowConfigs: "1801020277310567726170680201020277320567726170680103",
      ConfigChanged: "0500016f",
      ContextMenu: "06010101",
      Media: "0701017001010102",
      MediaFingerprint: "080101700101",
      Error: "0900016300",
      Emit: "0a010101000001020000",
      Draft: "0b01010101020164",
      Children: "0c0101017301630164010100",
      ChildrenOfAMember: "0c010101730163016401010f636f6e74656e742f666f726d732d31",
      ChildHeads: "200101017301630164010100",
      ChildHeadsOfAMember: "20010101730163016401010f636f6e74656e742f666f726d732d31",
      Ephemeral: "0d0201020304000000",
      HistorySnapshot: "0e010101",
      UiPatch: "15010106313a626f64790677696e646f7703020109",
      UiSnapshotEnd: "1606",
    };
    const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
    for (const [label, value] of commandFixtures) expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandGoldenHex[label]);
    for (const [label, value] of frameFixtures) expect(hex(encodeAppFrame(value)), `AppFrame::${label}`).toBe(frameGoldenHex[label]);
  });
  it("matches shared concrete-window config persistence wire vectors", async () => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "🧪️tests/🧫️fixtures", "📡️channel", "🪟️window-config.json"), "utf8")) as Record<string, string>;
    const hex = (bytes: Uint8Array) => Buffer.from(bytes).toString("hex");
    expect(hex(encodeAppCommand({ LoadWindowConfig: { seq: 1, entry: { window_id: "w1", window_kind_id: "graph", envelope_pack: [1, 2] } } }))).toBe(fixture.loadWindowConfig);
    expect(hex(encodeAppCommand({ ReadWindowConfigs: { seq: 1 } }))).toBe(fixture.readWindowConfigs);
    expect(
      hex(
        encodeAppFrame({
          WindowConfigs: {
            in_reply_to: 1,
            entries: [
              { window_id: "w1", window_kind_id: "graph", envelope_pack: [1, 2] },
              { window_id: "w2", window_kind_id: "graph", envelope_pack: [3] },
            ],
          },
        }),
      ),
    ).toBe(fixture.windowConfigs);
  });
}
