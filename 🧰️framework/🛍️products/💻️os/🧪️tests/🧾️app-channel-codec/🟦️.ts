/** 🧾️ App channel codec: channel version admission, app commands and frames, window config, media export wire.
 *
 * @see ../🛠️backbone-wire-helpers/🟦️.ts — the shared helpers
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { describe, expect, it } from "vitest";
import { APP_CHANNEL_VERSION, CHANNEL_MISMATCH_CODE, admitGuestChannelVersion, AppChannelClient, AppChannelRequestSequence, decodeAppCommand, decodeAppFrame, encodeAppCommand, encodeAppFrame } from "@semio-tech/framework/app-channel";
import { INVOCATION_RESULT_PACK_MAXIMUM_BYTES, decodeInvocationResultPacks, encodePackValue } from "@semio-tech/framework/packs";
import { decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse } from "@semio-tech/framework/backbone";
import { createTurnOutcomeBroadcast } from "@semio-tech/framework";
import { decodeDocumentArchiveBytes, encodeDocumentArchiveBytes } from "@semio-tech/framework/wire-contracts";
import { decodePresencePeer, encodePresencePeer } from "@semio-tech/framework-replication";
import { source, documentArchive, type AppChannelHandle, type AppCommandValue, type AppFrameValue, type TurnOutcome } from "../🛠️backbone-wire-helpers/🟦️.ts";

type ArtifactPresencePeer = import("@semio-tech/framework-replication").ArtifactPresencePeer;

describe("@semio-tech/framework-os AppChannelCodec", () => {
  it("round-trips the app-typed presence pack through the document-presence wire", () => {
    const peer: ArtifactPresencePeer = {
      actor: "actor-1",
      connectedAtMs: 42,
      label: "One",
      presencePack: [1, 2, 3],
      color: 4,
      surface: "s.space.home@1/*#editor",
      views: [{ windowId: "w1", space: "canvas", kind: { kind: "canvas", x: 1, y: 2, zoom: 1.5 }, size: [800, 600], pointer: [10, 20, 0] }],
      ui: { hoveredPath: "row[0]#a" },
    };
    expect(decodePresencePeer(new Uint8Array(encodePresencePeer(peer)), [0])).toEqual(peer);
  });

  const sampleCommands: readonly AppCommandValue[] = [
    { ConfigCommand: { seq: 1, command: [4, 5] } },
    { Command: { seq: 2, command: [1], view_state: [2, 3] } },
    { CommandText: { seq: 3, line: "move 1 2" } },
    { ContextMenu: { seq: 5, request: [9, 9] } },
    { ArtifactCommand: { seq: 6, command: [7] } },
    { ApplyEnvelopes: { seq: 7, envelopes: [] } },
    { ReadDocument: { seq: 9 } },
    { LoadDocumentArchive: { seq: 36, archive: documentArchive } },
    { MergeDocumentArchive: { seq: 43, archive: documentArchive } },
    { ReadDocumentArchive: { seq: 37 } },
    { PollDocumentArchiveLoad: { seq: 38, operation: 36 } },
    { CancelDocumentArchiveLoad: { seq: 39, operation: 36 } },
    { AcknowledgeDocumentArchiveLoad: { seq: 40, operation: 36 } },
    { LoadConfig: { seq: 10, pack: [1], spr: [2] } },
    { ReadConfig: { seq: 11 } },
    { LoadWindowConfig: { seq: 12, entry: { window_id: "w1", window_kind_id: "graph", envelope_pack: [1, 2] } } },
    { ReadWindowConfigs: { seq: 13 } },
    { MediaIn: { seq: 14, port: "in-1", descriptor: [1], data: [2, 3] } },
    { MediaOut: { seq: 15, port: "out-1", request: [4] } },
    { MediaFingerprint: { seq: 16, port: "fp-1" } },
    { PureCommand: { seq: 17, command: [1], head: [2] } },
    { PureCommand: { seq: 41, command: [1], head: [] } },
    { LoadChildren: { seq: 18, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1], owner: "" }, { slot: "s", child_id: "n", dialect: "d", envelope_pack: [], owner: "content/forms-1" }] } },
    { ReadChildren: { seq: 19 } },
    { ReadChildHeads: { seq: 42 } },
    { ReadHistory: { seq: 20 } },
    { transactionPrepare: { seq: 21, txn_id: "txn-1", mutation_id: "s.demo#kind", payload: [1, 2], prepared_ops: [], origin: [], prepared_child_ops: [] } },
    { transactionPrepare: { seq: 22, txn_id: "txn-1", mutation_id: "", payload: [], prepared_ops: [[1], [2, 2]], origin: [9], prepared_child_ops: [] } },
    { transactionPrepare: { seq: 36, txn_id: "txn-2", mutation_id: "", payload: [], prepared_ops: [], origin: [9], prepared_child_ops: [5, 6] } },
    { transactionCommit: { seq: 23, txn_id: "txn-1" } },
    { transactionRollback: { seq: 24, txn_id: "txn-1" } },
    { transactionUndo: { seq: 25, group_id: "grp-1" } },
    { transactionRedo: { seq: 26, group_id: "grp-1" } },
    { openArtifact: { seq: 27, artifact_ref: "s.cad.cad@1/*#viewer", role: 0, plugin_id: "", app_id: "" } },
    { openArtifact: { seq: 28, artifact_ref: "s.cad.cad@1/*#editor", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
    { setDefaultApp: { seq: 29, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
    { clearDefaultApp: { seq: 30, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 0 } },
    { setMergePolicy: { seq: 31, policy: 1 } },
    { resolveConflict: { seq: 32, conflict_id: "conflict-1", resolution: 0 } },
    { readConflicts: { seq: 33 } },
    { presence: { seq: 34, own_color: 3, peers: [[1, 2], [9]] } },
    { presence: { seq: 35, own_color: null, peers: [] } },
  ];

  const sampleFrames: readonly AppFrameValue[] = [
    { Done: { in_reply_to: 1 } },
    { Invocation: { in_reply_to: 2, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } },
    { Invocation: { in_reply_to: 2, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [9], mutations: [10], inverse_group: [11] } },
    { DocumentChanged: { envelopes: [[1, 2]], origin: "remote" } },
    { Document: { in_reply_to: 6, pack: [1, 2], spr: [3, 4], ops: "op-log" } },
    { DocumentArchive: { in_reply_to: 7, archive: documentArchive } },
    { DocumentArchiveLoad: { in_reply_to: 8, status: { operation: 36, state: "running", completed: 1, total: 3, ahead: 0, fault: [] } } },
    { WindowConfigs: { in_reply_to: 6, entries: [{ window_id: "w1", window_kind_id: "graph", envelope_pack: [1, 2] }] } },
    { ContextMenu: { in_reply_to: 7, items: [1, 2, 3] } },
    { Media: { in_reply_to: 8, port: "out-1", descriptor: [1], data: [2] } },
    { MediaFingerprint: { in_reply_to: 9, port: "fp-1", fingerprint: [1, 2, 3, 4] } },
    { Error: { in_reply_to: 10, fault: [1, 2, 3], report: [6] } },
    { Error: { in_reply_to: null, fault: [4, 5], report: [] } },
    { Emit: { in_reply_to: 11, document_ops: [1], config_ops: [2], draft_ops: [3], output: [4], diagnostics: [5], child_ops: [] } },
    { Emit: { in_reply_to: 11, document_ops: [], config_ops: [], draft_ops: [], output: [4], diagnostics: [], child_ops: [6, 7, 8] } },
    { Draft: { in_reply_to: 12, pack: [1], spr: [2], ops: "d" } },
    { Children: { in_reply_to: 13, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1], owner: "content/forms-1" }] } },
    { ChildHeads: { in_reply_to: 42, entries: [{ slot: "mesh", child_id: "child-1", dialect: "s.stdio.mesh@1/*", head_pack: [7, 8, 9], owner: "" }, { slot: "brep", child_id: "child-2", dialect: "s.stdio.brep@1/*", head_pack: [], owner: "content/forms-1" }] } },
    { ChildHeads: { in_reply_to: 43, entries: [] } },
    { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [7], tool_run: [8], history_edit: [9] } },
    { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [], tool_run: [], history_edit: [] } },
    { HistorySnapshot: { in_reply_to: 14, history_patch: [1] } },
    { transactionProposal: { in_reply_to: 15, proposal_id: "prop-1", local_ops: [[1]], foreign: [[2, 3]] } },
    { transactionPrepared: { txn_id: "txn-1", foreign: [[1]], rejection: [] } },
    { transactionPrepared: { txn_id: "txn-1", foreign: [], rejection: [1, 2] } },
    { transactionCommitted: { txn_id: "txn-1", edit_id: "edit-1" } },
    { transactionRolledBack: { txn_id: "txn-1" } },
    { MergeReport: { in_reply_to: 16, report: [1, 2] } },
    { MergeReport: { in_reply_to: null, report: [] } },
    { Conflicts: { in_reply_to: 17, conflicts: [3] } },
    { Conflicts: { in_reply_to: null, conflicts: [] } },
    { UiPatch: { in_reply_to: 18, surface: "1:body", kind: "window", revision: 2, base_revision: 1, ops: [3] } },
    { UiPatch: { in_reply_to: null, surface: "1:body", kind: "window", revision: 1, base_revision: 0, ops: [] } },
    { UiSnapshotEnd: { revision: 5 } },
  ];

  it("projects exact packed invocation mutations and inverse ownership without legacy field aliases", () => {
    const inverse = { targetMutation: "mutation-1", inverseDiff: { schema: "s.map.patch@1", payload: [9] }, baseVersion: 7, undoPolicy: "ExactBaseOnly" } as const;
    const mutation = {
      id: "mutation-1",
      document: "340282366920938463463374607431768211455",
      baseVersion: 7,
      invocationId: "invocation-1",
      diff: { schema: "s.map.patch@1", payload: [1, 2] },
      inverse,
      author: "actor-1",
      timestamp: { actor: 3, physical_ms: 5, logical: 1 },
    } as const;
    const inverseGroup = { invocationId: "invocation-1", mutations: ["mutation-1"], inverseMutations: [inverse], memberEdits: [{ document: mutation.document, editId: "edit-1" }] } as const;
    expect(decodeInvocationResultPacks({ mutations: encodePackValue([mutation]), inverse_group: encodePackValue(inverseGroup) })).toEqual({ mutations: [mutation], inverseGroup });
    expect(() => decodeInvocationResultPacks({ mutations: encodePackValue([{ ...mutation, artifact: 1 }]), inverse_group: encodePackValue(inverseGroup) })).toThrow("invalid exact fields");
    expect(() => decodeInvocationResultPacks({ mutations: encodePackValue([mutation]), inverse_group: [] })).toThrow("must be published together");
  });

  it("rejects invocation result packs outside the command transport byte authority", () => {
    const oversized = new Array<number>(INVOCATION_RESULT_PACK_MAXIMUM_BYTES + 1).fill(0);
    expect(() => encodeAppFrame({ Invocation: { in_reply_to: 1, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: oversized, inverse_group: [] } })).toThrow("exceeds command transport authority");
    expect(() => decodeInvocationResultPacks({ mutations: oversized, inverse_group: [] })).toThrow("exceeds command transport authority");
  });

  it.each(sampleCommands.map((cmd) => [cmd] as const))("round-trips AppCommand %j", (cmd) => {
    expect(decodeAppCommand(encodeAppCommand(cmd))).toEqual(cmd);
  });

  it.each(sampleFrames.map((frame) => [frame] as const))("round-trips AppFrame %j", (frame) => {
    expect(decodeAppFrame(encodeAppFrame(frame))).toEqual(frame);
  });

  it("tags every AppCommand variant per the agreed contract order", () => {
    expect(encodeAppCommand({ ConfigCommand: { seq: 0, command: [] } })[0]).toBe(0);
    expect(encodeAppCommand({ Command: { seq: 0, command: [], view_state: [] } })[0]).toBe(1);
    expect(encodeAppCommand({ ReadChildren: { seq: 0 } })[0]).toBe(15);
    expect(encodeAppCommand({ ReadChildHeads: { seq: 0 } })[0]).toBe(42);
    expect(encodeAppCommand({ MergeDocumentArchive: { seq: 0, archive: { parent_pack: [], parent_spr: [], members: [] } } })[0]).toBe(43);
    expect(encodeAppCommand({ ReadHistory: { seq: 0 } })[0]).toBe(16);
    expect(encodeAppCommand({ transactionPrepare: { seq: 0, txn_id: "", mutation_id: "", payload: [], prepared_ops: [], origin: [], prepared_child_ops: [] } })[0]).toBe(17);
    expect(encodeAppCommand({ transactionCommit: { seq: 0, txn_id: "" } })[0]).toBe(18);
    expect(encodeAppCommand({ transactionRollback: { seq: 0, txn_id: "" } })[0]).toBe(19);
    expect(encodeAppCommand({ transactionUndo: { seq: 0, group_id: "" } })[0]).toBe(20);
    expect(encodeAppCommand({ transactionRedo: { seq: 0, group_id: "" } })[0]).toBe(21);
    expect(encodeAppCommand({ openArtifact: { seq: 0, artifact_ref: "", role: 0, plugin_id: "", app_id: "" } })[0]).toBe(22);
    expect(encodeAppCommand({ setDefaultApp: { seq: 0, artifact_kind: "", standard: "", subset: "", role: 0, plugin_id: "", app_id: "" } })[0]).toBe(23);
    expect(encodeAppCommand({ clearDefaultApp: { seq: 0, artifact_kind: "", standard: "", subset: "", role: 0 } })[0]).toBe(24);
    expect(encodeAppCommand({ setMergePolicy: { seq: 0, policy: 0 } })[0]).toBe(25);
    expect(encodeAppCommand({ resolveConflict: { seq: 0, conflict_id: "", resolution: 0 } })[0]).toBe(26);
    expect(encodeAppCommand({ readConflicts: { seq: 0 } })[0]).toBe(27);
    expect(encodeAppCommand({ presence: { seq: 0, own_color: null, peers: [] } })[0]).toBe(28);
    expect(encodeAppCommand({ LocalInteractionQuery: { seq: 0, command: { kind: "read", requestId: "0" } } })[0]).toBe(29);
    expect(encodeAppCommand({ LoadWindowConfig: { seq: 0, entry: { window_id: "", window_kind_id: "", envelope_pack: [] } } })[0]).toBe(30);
    expect(encodeAppCommand({ ReadWindowConfigs: { seq: 0 } })[0]).toBe(31);
    expect(encodeAppCommand({ LoadDocumentArchive: { seq: 0, archive: { parent_pack: [], parent_spr: [], members: [] } } })[0]).toBe(32);
    expect(encodeAppCommand({ ReadDocumentArchive: { seq: 0 } })[0]).toBe(33);
    expect(encodeAppCommand({ PollDocumentArchiveLoad: { seq: 0, operation: 1 } })[0]).toBe(34);
    expect(encodeAppCommand({ CancelDocumentArchiveLoad: { seq: 0, operation: 1 } })[0]).toBe(35);
    expect(encodeAppCommand({ AcknowledgeDocumentArchiveLoad: { seq: 0, operation: 1 } })[0]).toBe(36);
    expect(() => decodeAppCommand(new Uint8Array([6, 8, 1, 1, 1, 2]))).toThrow("decodeAppCommand: unknown tag 6");
  });

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
});

describe("resumable media export wire", () => {
  it("routes media requests through exact receipts without admitting unrelated large sequences", async () => {
    const handle = { app_instance_id: 7, parent_document_id: "doc-α", operation_id: 9007199254740993n, base_revision: 9007199254740995n, generation: 18446744073709551615n };
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    const sent: AppCommandValue[] = [];
    const port: AppChannelHandle = { outcomes: broadcast.stream, enqueue: (_instance, packets) => sent.push(...packets.map(decodeAppCommand)) };
    const client = new AppChannelClient(port, new AppChannelRequestSequence(), 7, "fixture");
    try {
      const submitted = client.submitMediaExport("playback:out", handle.parent_document_id, handle.base_revision);
      expect(sent.at(-1)).toEqual({ SubmitMediaExport: { seq: 1n, port: "playback:out", expected_parent_document_id: handle.parent_document_id, expected_base_revision: handle.base_revision } });
      const valid: AppFrameValue = { MediaExportSubmitted: { in_reply_to: 1n, handle } };
      broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ MediaExportSubmitted: { in_reply_to: 9007199254740993n, handle } }), encodeAppFrame(valid)] });
      expect(await submitted).toEqual([valid]);
      for (const [index, operation] of ["poll", "cancel", "chunk"].entries()) {
        const seq = BigInt(index + 2);
        const pending = operation === "poll" ? client.pollMediaExport(handle) : operation === "cancel" ? client.cancelMediaExport(handle) : client.takeMediaExportChunk(handle);
        expect(sent.at(-1)).toEqual({ [operation === "poll" ? "PollMediaExport" : operation === "cancel" ? "CancelMediaExport" : "TakeMediaExportChunk"]: { seq, handle } });
        const frame: AppFrameValue = operation === "chunk" ? { MediaExportChunk: { in_reply_to: seq, handle, data: [], terminal: true } } : operation === "cancel" ? { Done: { in_reply_to: Number(seq) } } : { MediaExportStatus: { in_reply_to: seq, handle, state: "running", applied_progress: 2n, checkpoint_available: false, mime_type: "", total_bytes: 0n, detail: "" } };
        broadcast.push({ instanceId: 7, frames: [encodeAppFrame(frame)] });
        expect(await pending).toEqual([frame]);
      }
    } finally { client.dispose(); }
  });

  it("matches the shared Rust vectors and the independent LEB128 encoder without rounding", async () => {
    const { readFileSync } = await import("node:fs");
    const directory = "./🔨️modules/📡️spr/🧵️channel/🧪️tests/🧫️fixtures/🎬️media-export-wire/";
    const fixture = JSON.parse(readFileSync(new URL(directory + "🔣️.json", source.url), "utf8"));
    const oracleModule = "@webassemblyjs/leb128/lib/leb.js";
    const imported = await import(oracleModule);
    const u64 = (value: bigint): number[] => {
      const input = Buffer.alloc(8); input.writeBigUInt64LE(value);
      return Array.from(imported.default.encodeUIntBuffer(input));
    };
    const string = (value: string): number[] => { const bytes = Buffer.from(value); return [...u64(BigInt(bytes.length)), ...bytes]; };
    const f = fixture.handle;
    const handle = { app_instance_id: f.appInstanceId, parent_document_id: f.parentDocumentId, operation_id: BigInt(f.operationId), base_revision: BigInt(f.baseRevision), generation: BigInt(f.generation) };
    const handleBytes = [...u64(BigInt(handle.app_instance_id)), ...string(handle.parent_document_id), ...u64(handle.operation_id), ...u64(handle.base_revision), ...u64(handle.generation)];
    for (const [index, name] of ["submit", "poll", "cancel", "takeChunk"].entries()) {
      const row = fixture.commands[name], value = row.fields, seq = BigInt(value.seq);
      const key = ["SubmitMediaExport", "PollMediaExport", "CancelMediaExport", "TakeMediaExportChunk"][index]!;
      const command = { [key]: index === 0 ? { seq, port: value.port, expected_parent_document_id: value.expectedParentDocumentId, expected_base_revision: BigInt(value.expectedBaseRevision) } : { seq, handle } } as AppCommandValue;
      const oracle = [37 + index, ...u64(seq), ...(index === 0 ? [...string(value.port), ...string(value.expectedParentDocumentId), ...u64(BigInt(value.expectedBaseRevision))] : handleBytes)];
      expect(oracle).toEqual(row.bytes);
      expect(Buffer.from(row.bytes).toString("hex")).toBe(row.hex);
      expect([...encodeAppCommand(command)]).toEqual(row.bytes);
      expect(decodeAppCommand(Uint8Array.from(row.bytes))).toEqual(command);
    }
    for (const name of ["submitted", "status", "chunk", "terminalChunk"]) {
      const row = fixture.frames[name], value = row.fields, in_reply_to = BigInt(value.inReplyTo);
      const state = ["running", "complete", "cancelled", "failed"].indexOf(value.state);
      const frame = name === "submitted" ? { MediaExportSubmitted: { in_reply_to, handle } } : name === "status" ? { MediaExportStatus: { in_reply_to, handle, state: value.state, applied_progress: BigInt(value.appliedProgress), checkpoint_available: value.checkpointAvailable, mime_type: value.mimeType, total_bytes: BigInt(value.totalBytes), detail: value.detail } } : { MediaExportChunk: { in_reply_to, handle, data: value.data, terminal: value.terminal } };
      const tail = name === "submitted" ? [] : name === "status" ? [state, ...u64(BigInt(value.appliedProgress)), Number(value.checkpointAvailable), ...string(value.mimeType), ...u64(BigInt(value.totalBytes)), ...string(value.detail)] : [...u64(BigInt(value.data.length)), ...value.data, Number(value.terminal)];
      expect([name === "submitted" ? 28 : name === "status" ? 29 : 30, ...u64(in_reply_to), ...handleBytes, ...tail]).toEqual(row.bytes);
      expect([...encodeAppFrame(frame as AppFrameValue)]).toEqual(row.bytes);
      expect(decodeAppFrame(Uint8Array.from(row.bytes))).toEqual(frame);
    }
  });

  it("preserves exact large operation authority and rejects incomplete or extra bytes", () => {
    const handle = { app_instance_id: 7, parent_document_id: "Überblick", operation_id: 9007199254740993n, base_revision: 18446744073709551615n, generation: 9007199254740995n };
    const commands = [
      { SubmitMediaExport: { seq: 1n, port: "playback:out", expected_parent_document_id: handle.parent_document_id, expected_base_revision: handle.base_revision } },
      { PollMediaExport: { seq: 2n, handle } }, { CancelMediaExport: { seq: 3n, handle } }, { TakeMediaExportChunk: { seq: 4n, handle } },
    ];
    const frames = [
      { MediaExportSubmitted: { in_reply_to: 1n, handle } },
      ...(["running", "complete", "cancelled", "failed"] as const).map((state) => ({ MediaExportStatus: { in_reply_to: 2n, handle, state, applied_progress: 9007199254740997n, checkpoint_available: true, mime_type: "audio/mpeg", total_bytes: 9007199254740999n, detail: "Status" } })),
      { MediaExportChunk: { in_reply_to: 4n, handle, data: [0, 128, 255], terminal: false } },
      { MediaExportChunk: { in_reply_to: 5n, handle, data: [], terminal: true } },
    ];
    for (const [index, command] of commands.entries()) {
      const bytes = encodeAppCommand(command as AppCommandValue);
      expect(bytes[0]).toBe(37 + index);
      expect(decodeAppCommand(bytes)).toEqual(command);
      expect(() => decodeAppCommand(bytes.subarray(0, bytes.length - 1))).toThrow();
      expect(() => decodeAppCommand(Uint8Array.from([...bytes, 0]))).toThrow();
    }
    for (const frame of frames) {
      const bytes = encodeAppFrame(frame as AppFrameValue);
      expect(decodeAppFrame(bytes)).toEqual(frame);
      expect(() => decodeAppFrame(bytes.subarray(0, bytes.length - 1))).toThrow();
      expect(() => decodeAppFrame(Uint8Array.from([...bytes, 0]))).toThrow();
    }
  });
});
