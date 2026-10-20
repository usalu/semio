/** 🔶️ The round trips app typed tests of the @semio-tech/framework-os AppChannelCodec suite. */
import { expect, it } from "vitest";
import { decodeAppCommand, decodeAppFrame, encodeAppCommand, encodeAppFrame } from "@semio-tech/framework/app-channel";
import { INVOCATION_RESULT_PACK_MAXIMUM_BYTES, decodeInvocationResultPacks, encodePackValue } from "@semio-tech/framework/packs";
import { decodePresencePeer, encodePresencePeer } from "@semio-tech/framework-replication";
import { documentArchive, type AppCommandValue, type AppFrameValue } from "../../🛠️backbone-wire-helpers/🟦️.ts";
import type { ArtifactPresencePeer } from "../🟢️shared/🟦️.ts";

/** 🔷 Registers the round trips app typed tests of the @semio-tech/framework-os AppChannelCodec suite. */
export function registerRoundTripsAppTypedTests(): void {
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
}
