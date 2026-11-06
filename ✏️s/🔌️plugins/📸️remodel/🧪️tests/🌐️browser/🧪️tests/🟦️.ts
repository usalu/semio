import corruptCases from "../🧫️fixtures/⛔️corrupt.json";
import snapshotCases from "../🧫️fixtures/📸️snapshot.json";
import snapshotSchema from "../🧬️schema/📸️snapshot.json";
import snapshotUndoCases from "../🧫️fixtures/↩️snapshot.json";
import snapshotUndoSchema from "../🧬️schema/↩️snapshot.json";
import coldCases from "../🧫️fixtures/❄️cold.json";
import coldSchema from "../🧬️schema/❄️cold.json";
import windowCases from "../🧫️fixtures/🪟️windows.json";
import windowSchema from "../🧬️schema/🪟️windows.json";
import corruptSchema from "../🧬️schema/⛔️corrupt.json";
import archiveCases from "../🧫️fixtures/🗄️archive.json";
import archiveSchema from "../🧬️schema/🗄️archive.json";
import lifecycleCases from "../🧫️fixtures/↩️lifecycle.json";
import lifecycleSchema from "../🧬️schema/↩️lifecycle.json";
import finalizationCases from "../🧫️fixtures/🏁️finalization.json";
import finalizationSchema from "../🧬️schema/🏁️finalization.json";
import { DatabaseSync } from "node:sqlite";
import mediaCases from "../🧫️fixtures/📥️media-observations.json";
import mediaSchema from "../🧬️schema/📥️media-observations.json";
import { inspectRemodelOpenedV1 } from "../🧭️journey/🟦️.ts";
import { existsSync } from "node:fs";
import { resolve } from "node:path";
import { expect, test } from "vitest";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import scenarios from "../🧫️fixtures/🔣️.json";
import { corruptImportObservedV1, archiveBytesEqualV1, archiveWitnessV1, abortObservedV1, undoRedoObservedV1, finalizationObservedV1, mediaImportObservedV1, initialRemodelBrowserReportV1, remodelEditorPresentV1, remodelBoundEditorV1, remodelAcceptanceSatisfiedV1, serializeRemodelBrowserReportV1, validateRemodelTwoHumanArgumentsV1, remodelInputsV1, remodelHostReadyV1 } from "../📋️report/🟦️.ts";

test("the window observation plan covers three distinct kinds and controls", () => {
  expect(new Ajv().validate(windowSchema, windowCases)).toBe(true);
  const db = new DatabaseSync(":memory:");
  try {
    const oracle = db.prepare("SELECT count(DISTINCT json_extract(value,'$.kind')) kinds, count(DISTINCT json_extract(value,'$.measure')) measures FROM json_each(?1)").get(JSON.stringify(windowCases)) as { kinds: number; measures: number };
    expect(oracle.kinds).toBe(3);
    expect(oracle.measures).toBe(3);
    expect(new Set(windowCases.map(row => row.kind)).size).toBe(oracle.kinds);
    expect(new Set(windowCases.map(row => row.measure)).size).toBe(oracle.measures);
  } finally { db.close(); }
});

test("snapshot witness admits only the declared download and binary envelope", async () => {
  const { snapshotWitnessV1 } = await import("../📋️report/🟦️.ts");
  expect(new Ajv().validate(snapshotSchema, snapshotCases)).toBe(true);
  const db = new DatabaseSync(":memory:");
  try {
    const query = db.prepare(`SELECT ?1='remodeling-snapshot.pack' AND ?2='application/vnd.semio.pack' AND hex(substr(?3,1,8))='8953454D0D0A1A0A' AND hex(substr(?3,9,4))='1D000000' AND substr(?3,13,29)=cast('remodeling.remodeling.pack v1' AS blob) AND length(?3)>41 AS ok`);
    for (const row of snapshotCases) {
      const bytes = Buffer.from(row.hex, "hex"), oracle = query.get(row.name, row.mime, bytes) as { ok: number | null };
      expect(Boolean(oracle.ok)).toBe(row.valid);
      if (row.valid) expect((await snapshotWitnessV1(row.name, bytes, row.mime, new AbortController().signal)).bytes).toBe(bytes.length);
      else await expect(snapshotWitnessV1(row.name, bytes, row.mime, new AbortController().signal)).rejects.toThrow();
    }
    const controller = new AbortController(), reason = Error("snapshot cancelled"), valid = snapshotCases[0]!;
    controller.abort(reason);
    await expect(snapshotWitnessV1(valid.name, Buffer.from(valid.hex, "hex"), valid.mime, controller.signal)).rejects.toBe(reason);
  } finally { db.close(); }
});

test("snapshot undo and redo restore the exact two observed heads", async () => {
  const { snapshotUndoRedoObservedV1 } = await import("../📋️report/🟦️.ts");
  expect(new Ajv().validate(snapshotUndoSchema, snapshotUndoCases)).toBe(true);
  const db = new DatabaseSync(":memory:");
  try {
    const query = db.prepare("SELECT ?1<>?2 AND ?1=?3 AND ?2=?4 AS ok");
    for (const row of snapshotUndoCases) {
      const values = [row.before, row.final, row.undo, row.redo].map(value => Buffer.from(value, "hex")) as [Buffer, Buffer, Buffer, Buffer];
      const oracle = query.get(...values) as { ok: number };
      expect(Boolean(oracle.ok)).toBe(row.pass);
      expect(snapshotUndoRedoObservedV1(...values)).toBe(Boolean(oracle.ok));
    }
  } finally { db.close(); }
});

test("cold archive comparison never passes an empty or different document", async () => {
  const { coldArchiveStatusV1 } = await import("../📋️report/🟦️.ts");
  expect(new Ajv().validate(coldSchema, coldCases)).toBe(true);
  const db = new DatabaseSync(":memory:");
  try {
    const query = db.prepare("SELECT CASE WHEN length(?1)=0 OR length(?2)=0 THEN 'unexecuted' WHEN ?1<>?2 THEN 'failed' WHEN ?3>0 THEN 'passed' ELSE 'unexecuted' END AS status");
    for (const row of coldCases) {
      const left = Buffer.from(row.left, "hex"), right = Buffer.from(row.right, "hex");
      const oracle = query.get(left, right, row.members) as { status: string };
      expect(oracle.status).toBe(row.status);
      expect(coldArchiveStatusV1(left, right, row.members)).toBe(oracle.status);
    }
  } finally { db.close(); }
});

test("every Remodel browser scenario remains unexecuted until real UI evidence exists", () => {
  const oracle = new Ajv({ strict: true });
  expect(oracle.validate(schema, scenarios)).toBe(true);
  expect(scenarios.find(row => row.id === "import-images")?.assets).toHaveLength(3);
  for (const id of ["import-jpeg", "import-corrupt", "import-append"]) expect(scenarios.some(row => row.id === id)).toBe(true);
  expect(scenarios.find(row => row.id === "import-video")?.description).toContain("45");
  expect(oracle.validate(schema, scenarios.map(row => row.id === "import-images" ? { id: row.id, description: row.description } : row))).toBe(false);
  expect(oracle.validate(schema, scenarios.slice(1))).toBe(false);
  expect(oracle.validate(schema, [...scenarios.slice(0, -1), { ...scenarios[0]!, description: "different" }])).toBe(false);
  const report = initialRemodelBrowserReportV1("http://localhost:6063");
  expect(report.scenarios.map(row => row.id)).toEqual(scenarios.map(row => row.id));
  expect(report.scenarios.every(row => row.status === "unexecuted")).toBe(true);
  expect(report.scenarios.every(row => row.evidence.length === 0)).toBe(true);
});


test("acceptance refuses wrong identity, missing evidence and incomplete checklists", () => {
  expect(remodelEditorPresentV1([{ app: "s.remodel.remodeling@1/*#viewer", role: "viewer" }])).toBe(false);
  expect(remodelEditorPresentV1([{ app: "s.remodel.remodeling@1/*#editor", role: "editor" }])).toBe(true);
  const report = initialRemodelBrowserReportV1("http://localhost:6063");
  report.scenarios[0]!.evidence.push(JSON.stringify({ editorChip: true, unrelatedWindowBody: true }));
  expect(report.scenarios[0]!.status).toBe("unexecuted");
  expect(remodelAcceptanceSatisfiedV1(report)).toBe(false);
  expect(JSON.parse(serializeRemodelBrowserReportV1(report))).toEqual(report);
  for (const row of report.scenarios) { row.status = "passed"; row.evidence.push("observed"); }
  expect(remodelAcceptanceSatisfiedV1(report)).toBe(true);
  report.scenarios[0]!.evidence.length = 0;
  expect(remodelAcceptanceSatisfiedV1(report)).toBe(false);
  report.scenarios[0]!.evidence.push("observed");
  report.scenarios.pop();
  expect(remodelAcceptanceSatisfiedV1(report)).toBe(false);
  report.faults.push("pageerror");
  expect(remodelAcceptanceSatisfiedV1(report)).toBe(false);
});


test("shared browser acceptance requires explicit scoped journey arguments", () => {
  const good = ["--hub", "http://localhost:7000", "--serve", "http://localhost:6063", "--out", "🗑️generated/report", "--locale", "en", "--kinds", "3d.remodeling"];
  expect(() => validateRemodelTwoHumanArgumentsV1(good)).not.toThrow();
  for (const bad of [good.slice(0, -2), [...good, "--hub", "http://localhost:1"], [...good.slice(0, 7), "fr", ...good.slice(8)], [...good.slice(0, -1), "a,b"], ["--hub", "http://user:pass@localhost", ...good.slice(2)], [...good, "--tag", "../unsafe"], [...good, "--unknown", "x"], [...good, "--journey", "viewer"], [...good.slice(0, 5), "outside", ...good.slice(6)]]) expect(() => validateRemodelTwoHumanArgumentsV1(bad)).toThrow();
});

test("the import input plan uses the three existing repository images in place", () => {
  const root = resolve(import.meta.dirname, "../../../../../..");
  const inputs = remodelInputsV1(root, "import-images");
  expect(inputs).toHaveLength(3);
  expect(inputs.every(path => path.startsWith(root) && path.length <= 256 && existsSync(path))).toBe(true);
  expect(inputs.map(path => path.split(/[\\/]/u).at(-1))).toEqual(["🎞️frame-00.png", "🎞️frame-01.png", "🎞️frame-02.png"]);
  for (const id of ["import-jpeg", "import-corrupt", "import-video"]) expect(remodelInputsV1(root, id).every(path => path.length <= 256 && existsSync(path))).toBe(true);
});


test("only an actual ready beacon admits host interactions", () => {
  expect(remodelHostReadyV1(null)).toBe(false);
  expect(remodelHostReadyV1("error:broken")).toBe(false);
  expect(remodelHostReadyV1("not-found:missing")).toBe(false);
  expect(remodelHostReadyV1("ready:yes")).toBe(true);
});

test("identity requires a visible populated body owned by the editor and a real scope", () => {
  const row = { app: "s.remodel.remodeling@1/*#editor", role: "editor", instance: "7", window: "model-1", kind: "model", space: "space", document: "artifact", visible: true, populated: true };
  expect(remodelBoundEditorV1([row])).toBe(true);
  for (const change of [{ app: "other" }, { role: "viewer" }, { instance: null }, { window: null }, { kind: null }, { space: null }, { document: null }, { visible: false }, { populated: false }]) expect(remodelBoundEditorV1([{ ...row, ...change }])).toBe(false);
});

test("opened journey refuses mismatched documents and leaves all scenarios incomplete", async () => {
  const attrs = { "data-surface-app-id": "s.remodel.remodeling@1/*#editor", "data-surface-role": "editor", "data-surface-session-instance-id": "1", "data-surface-window-id": "model", "data-surface-window-kind-id": "model", "data-surface-space-id": "space", "data-surface-document-id": "wrong" };
  let clicks = 0;
  const ui = { canvas: async () => null, observe: async (selector: string) => selector.includes("[data-ui-node-key]") ? [{ text: "content", visible: true, attributes: {} }] : [{ text: "content", visible: true, attributes: attrs }], click: async () => { clicks++; }, press: async () => {}, chooseFiles: async () => {}, download: async () => ({ name: "", mime: null, bytes: new Uint8Array() }) };
  const rows = await inspectRemodelOpenedV1({ spaceId: "space", documentId: "expected", locale: "en", signal: new AbortController().signal, a: ui, b: ui }, process.cwd());
  expect(rows).toHaveLength(15);
  expect(rows.find(row => row.id === "identity")?.status).toBe("failed");
  expect(rows.filter(row => row.id !== "identity").every(row => row.status === "unexecuted")).toBe(true);
  expect(clicks).toBe(0);
});

test("a matched scope does not turn absent controls or untouched scenarios into passes", async () => {
  const attrs = { "data-surface-app-id": "s.remodel.remodeling@1/*#editor", "data-surface-role": "editor", "data-surface-session-instance-id": "1", "data-surface-window-id": "model", "data-surface-window-kind-id": "model", "data-surface-space-id": "space", "data-surface-document-id": "doc" };
  let clicks = 0;
  const ui = { canvas: async () => null, observe: async (selector: string) => selector.includes("[data-ui-node-key]:not") ? [{ text: "content", visible: true, attributes: {} }] : selector === "[data-surface-app-id][data-surface-window-id]" ? [{ text: "", visible: true, attributes: attrs }] : [], click: async () => { clicks++; }, press: async () => {}, chooseFiles: async () => {}, download: async () => ({ name: "", mime: null, bytes: new Uint8Array() }) };
  const rows = await inspectRemodelOpenedV1({ spaceId: "space", documentId: "doc", locale: "en", signal: new AbortController().signal, a: ui, b: ui }, process.cwd());
  expect(rows).toHaveLength(15);
  expect(rows.filter(row => row.status === "passed")).toEqual([]);
  expect(rows.find(row => row.id === "import-cancel")?.status).toBe("unexecuted");
  expect(clicks).toBe(0);
});

test("media observation requires exact peer counts and a new retained frame stream", () => {
  expect(new Ajv({ strict: true }).validate(mediaSchema, mediaCases)).toBe(true);
  const db = new DatabaseSync(":memory:");
  try {
    const query = db.prepare(`WITH old AS (SELECT value id FROM json_each(?1)), streams AS (SELECT json_extract(value, '$.id') id, json_extract(value, '$.text') text FROM json_each(?2))
      SELECT NOT EXISTS(SELECT 1 FROM old WHERE length(trim(id))=0 OR NOT EXISTS(SELECT 1 FROM streams WHERE streams.id=old.id))
      AND NOT EXISTS(SELECT 1 FROM streams WHERE length(trim(id))=0)
      AND (SELECT count(*) FROM old)=(SELECT count(DISTINCT id) FROM old)
      AND (SELECT count(*) FROM streams)=(SELECT count(DISTINCT id) FROM streams)
      AND EXISTS(SELECT 1 FROM streams WHERE NOT EXISTS(SELECT 1 FROM old WHERE old.id=streams.id) AND text LIKE ?3) AS ok`);
    for (const row of mediaCases) {
      const oracle = query.get(JSON.stringify(row.prior), JSON.stringify(row.afterStreams), `% ${row.frames} frames,%`) as { ok: number };
      const counts = row.after[0] === row.before[0]! + 1 && row.after[1] === row.before[1]! + row.frames && row.peer[0] === row.after[0] && row.peer[1] === row.after[1];
      expect(Boolean(oracle.ok) && counts).toBe(row.pass);
      expect(mediaImportObservedV1(row.before, row.after, row.peer, row.frames, row.prior, row.afterStreams)).toBe(row.pass);
    }
  } finally { db.close(); }
});

test("finalization requires unchanged baseline and matching non-placeholder results", () => {
  expect(new Ajv({ strict: true }).validate(finalizationSchema, finalizationCases)).toBe(true);
  const db = new DatabaseSync(":memory:");
  try {
    const query = db.prepare("SELECT json_array_length(?1)=5 AND json_array_length(?2)=5 AND json_array_length(?4)=5 AND ?1=?2 AND ?2=?3 AND ?2<>?4 AND ?4=?5 AND ?6=1 AND lower(?7) LIKE '%reconstructed%' AND lower(?7) LIKE '% 3 vertices%' AS ok");
    for (const row of finalizationCases) {
      const oracle = query.get(JSON.stringify(row.baseline), JSON.stringify(row.before), JSON.stringify(row.peerBefore), JSON.stringify(row.after), JSON.stringify(row.peerAfter), Number(row.finalized), row.after[0]) as { ok: number };
      expect(Boolean(oracle.ok)).toBe(row.pass);
      expect(finalizationObservedV1(row.baseline, row.before, row.peerBefore, row.after, row.peerAfter, row.finalized)).toBe(row.pass);
    }
  } finally { db.close(); }
});

test("abort and undo receipts require the observed shared products", () => {
  expect(new Ajv({ strict: true }).validate(lifecycleSchema, lifecycleCases)).toBe(true);
  const db = new DatabaseSync(":memory:");
  try {
    for (const row of lifecycleCases) {
      const values = row.values.map(value => JSON.stringify(value));
      const oracle = row.kind === "abort"
        ? db.prepare("SELECT ?1=1 AND ?2=1 AND ?3=1 AND json_array_length(?4)=5 AND ?4=?5 AND ?5=?6 AS ok").get(...row.flags.map(Number), ...values)
        : db.prepare("SELECT ?1=1 AND json_array_length(?2)=5 AND json_array_length(?3)=5 AND ?2<>?3 AND ?3=?4 AND ?2=?5 AND ?5=?6 AS ok").get(...row.flags.map(Number), ...values);
      expect(Boolean((oracle as { ok: number }).ok)).toBe(row.pass);
      const actual = row.kind === "abort"
        ? abortObservedV1(row.flags[0]!, row.flags[1]!, row.flags[2]!, row.values[0]!, row.values[1]!, row.values[2]!)
        : undoRedoObservedV1(row.flags[0]!, row.values[0]!, row.values[1]!, row.values[2]!, row.values[3]!, row.values[4]!);
      expect(actual).toBe(row.pass);
    }
  } finally { db.close(); }
});

test("public archive witness validates its wire and compares exact bytes", () => {
  expect(new Ajv({ strict: true }).validate(archiveSchema, archiveCases)).toBe(true);
  const db = new DatabaseSync(":memory:");
  try {
    const compare = db.prepare("SELECT ?1=?2 AS ok");
    for (const row of archiveCases) {
      const left = Uint8Array.from(row.left), right = Uint8Array.from(row.right);
      const oracle = compare.get(left, right) as { ok: number };
      expect(Boolean(oracle.ok)).toBe(row.equal);
      expect(archiveBytesEqualV1(left, right)).toBe(row.equal);
    }
    const good = archiveWitnessV1("test.semio-archive", Uint8Array.from(archiveCases[0]!.left), null);
    expect(good.parentBytes).toBe(1);
    expect(good.members).toBe(0);
    expect(() => archiveWitnessV1("test.semio-archive", Uint8Array.from(archiveCases[2]!.right), null)).toThrow();
    expect(() => archiveWitnessV1("wrong.bin", Uint8Array.from(archiveCases[0]!.left), null)).toThrow();
  } finally { db.close(); }
});

test("corrupt import requires refusal and unchanged shared document", () => {
  expect(new Ajv({ strict: true }).validate(corruptSchema, corruptCases)).toBe(true);
  const db = new DatabaseSync(":memory:");
  try {
    const query = db.prepare("SELECT lower(?1) LIKE '%image%' AND lower(?1) LIKE '%invalid%' AND ?2=1 AND json_array_length(?3)=2 AND ?3=?4 AND ?4=?5 AS ok");
    for (const row of corruptCases) {
      const oracle = query.get(row.alert, Number(row.sameArchive), JSON.stringify(row.before), JSON.stringify(row.after), JSON.stringify(row.peer)) as { ok: number };
      expect(Boolean(oracle.ok)).toBe(row.pass);
      expect(corruptImportObservedV1(row.alert, row.sameArchive, row.before, row.after, row.peer)).toBe(row.pass);
    }
  } finally { db.close(); }
});
