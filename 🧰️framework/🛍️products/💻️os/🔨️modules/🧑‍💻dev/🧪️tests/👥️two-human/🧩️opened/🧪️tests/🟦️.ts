import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import { test } from "vitest";
import Ajv from "ajv";
import { DatabaseSync } from "node:sqlite";
import { OPENED_CHECKS_COMPLETE_V1, openedCheckNameV1, openedChecksPassV1, runOpenedChecksV1 } from "../🟦️.ts";

test("opened artifact checks refuse missing, duplicate, unsupported and empty evidence", () => {
  const rows = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  assert.equal(new Ajv().validate(schema, rows), true);
  const db = new DatabaseSync(":memory:");
  try {
    const query = db.prepare(`WITH r AS (SELECT value id FROM json_each(?1)), c AS (SELECT json_extract(value,'$.id') id, json_extract(value,'$.status') status, json_extract(value,'$.evidence') evidence FROM json_each(?2))
      SELECT (SELECT count(*) FROM r)>0 AND (SELECT count(*) FROM r)=(SELECT count(DISTINCT id) FROM r)
      AND (SELECT count(*) FROM r)=(SELECT count(*) FROM c) AND (SELECT count(*) FROM c)=(SELECT count(DISTINCT id) FROM c)
      AND NOT EXISTS(SELECT 1 FROM r WHERE length(trim(id))=0 OR NOT EXISTS(SELECT 1 FROM c WHERE c.id=r.id))
      AND NOT EXISTS(SELECT 1 FROM c WHERE status<>'passed' OR NOT EXISTS(SELECT 1 FROM json_each(c.evidence) WHERE length(trim(value))>0)) AS ok`);
    for (const row of rows) {
      const oracle = query.get(JSON.stringify(row.required), JSON.stringify(row.checks)) as { ok: number };
      assert.equal(Boolean(oracle.ok), row.pass);
      assert.equal(openedChecksPassV1(row.required, row.checks), Boolean(oracle.ok));
    }
  } finally { db.close(); }
});

test("opened checks preserve the original signal and refuse missing scope, failure and cancellation", async () => {
  const controller = new AbortController();
  const ui = { observe: async () => [], canvas: async () => null, click: async () => {}, press: async () => {}, chooseFiles: async () => {}, download: async () => ({ name: "", mime: null, bytes: new Uint8Array() }) };
  const context = { spaceId: "space", documentId: "document", locale: "en" as const, signal: controller.signal, a: ui, b: ui };
  let seen = false;
  const success = { required: ["a"], run: async (actual: typeof context) => { seen = actual === context && actual.signal === controller.signal; return [{ id: "a", status: "passed" as const, evidence: ["seen"] }]; } };
  assert.equal(openedChecksPassV1(success.required, await runOpenedChecksV1(success, context, controller.signal)), true);
  assert.equal(seen, true);
  assert.equal(openedChecksPassV1(success.required, await runOpenedChecksV1(success, null, controller.signal)), false);
  assert.equal(openedChecksPassV1(success.required, await runOpenedChecksV1(success, { ...context, documentId: "" }, controller.signal)), false);
  const failed = { required: ["a"], run: async () => { throw Error("failure"); } };
  assert.equal(openedChecksPassV1(failed.required, await runOpenedChecksV1(failed, context, controller.signal)), false);
  controller.abort(Error("cancel"));
  assert.equal(openedChecksPassV1(success.required, await runOpenedChecksV1(success, context, controller.signal)), false);
});

test("product check names cannot collide with the completion receipt", () => {
  assert.notEqual(openedCheckNameV1("complete"), OPENED_CHECKS_COMPLETE_V1);
  assert.notEqual(openedCheckNameV1("opened checks complete"), OPENED_CHECKS_COMPLETE_V1);
});

test("blank declared check identifiers fail", () => {
  for (const id of ["", " "]) assert.equal(openedChecksPassV1([id], [{ id, status: "passed", evidence: ["seen"] }]), false);
});

test("unknown phase identifiers cannot pass at runtime", async () => {
  const { openedPhasesPassV1 } = await import("../🟦️.ts");
  const required = [{ id: "a", phase: "unknown" }];
  const receipts = [{ id: "a", phase: "unknown", spaceId: "s", documentId: "d", expectedActors: { a: "editor", b: "editor" }, status: "passed", evidence: ["seen"] }];
  assert.equal(openedPhasesPassV1(required as never, receipts as never, "s", "d"), false);
});

test("viewer scope requires the same visible artifact and excludes a conflicting editor", async () => {
  const { openedViewerScopeV1, openedEditorsScopeV1 } = await import("../🟦️.ts");
  const rows = JSON.parse(readFileSync(new URL("../🧫️fixtures/🪟️viewer.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🪟️viewer.json", import.meta.url), "utf8"));
  assert.equal(new Ajv().validate(schema, rows), true);
  const db = new DatabaseSync(":memory:");
  try {
    const query = db.prepare(`WITH a AS (SELECT json_extract(value,'$.role') role, json_extract(value,'$.space') space, json_extract(value,'$.document') document, json_extract(value,'$.visible') visible, json_extract(value,'$.instance') instance, json_extract(value,'$.window') window FROM json_each(?1)), b AS (SELECT json_extract(value,'$.role') role, json_extract(value,'$.space') space, json_extract(value,'$.document') document, json_extract(value,'$.visible') visible, json_extract(value,'$.instance') instance, json_extract(value,'$.window') window FROM json_each(?2)) SELECT EXISTS(SELECT 1 FROM a WHERE visible=1 AND role='editor' AND space='s' AND document='d' AND length(trim(instance))>0 AND length(trim(window))>0) AND EXISTS(SELECT 1 FROM b WHERE visible=1 AND space='s' AND document='d') AND NOT EXISTS(SELECT 1 FROM b WHERE visible=1 AND space='s' AND document='d' AND (role<>?3 OR length(trim(instance))=0 OR length(trim(window))=0)) AS ok`);
    const editors = JSON.parse(readFileSync(new URL("../🧫️fixtures/🪟️editors.json", import.meta.url), "utf8"));
    assert.equal(new Ajv().validate(schema, editors), true);
    for (const [items, role, check] of [[rows, "viewer", openedViewerScopeV1], [editors, "editor", openedEditorsScopeV1]] as const) for (const row of items) { const oracle = query.get(JSON.stringify(row.a), JSON.stringify(row.b), role) as { ok: number }; assert.equal(Boolean(oracle.ok), row.pass); assert.equal(check(row.a, row.b, "s", "d"), Boolean(oracle.ok)); }
  } finally { db.close(); }
});

test("phase receipts require exact phase and shared identity", async () => {
  const { openedPhasesPassV1 } = await import("../🟦️.ts");
  const rows = JSON.parse(readFileSync(new URL("../🧫️fixtures/🪜️phases.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🪜️phases.json", import.meta.url), "utf8"));
  assert.equal(new Ajv().validate(schema, rows), true);
  const db = new DatabaseSync(":memory:");
  try {
    const query = db.prepare(`WITH r AS (SELECT json_extract(value,'$.id') id, json_extract(value,'$.phase') phase FROM json_each(?1)), c AS (SELECT json_extract(value,'$.id') id, json_extract(value,'$.phase') phase, json_extract(value,'$.spaceId') space, json_extract(value,'$.documentId') document, json_extract(value,'$.status') status, json_extract(value,'$.expectedActors.a') actor_a, json_extract(value,'$.expectedActors.b') actor_b, json_extract(value,'$.evidence') evidence FROM json_each(?2)) SELECT (SELECT count(*) FROM r)>0 AND (SELECT count(*) FROM r)=(SELECT count(DISTINCT id) FROM r) AND (SELECT count(*) FROM r)=(SELECT count(*) FROM c) AND (SELECT count(*) FROM c)=(SELECT count(DISTINCT id) FROM c) AND NOT EXISTS(SELECT 1 FROM r WHERE length(trim(id))=0 OR NOT EXISTS(SELECT 1 FROM c WHERE c.id=r.id AND c.phase=r.phase)) AND NOT EXISTS(SELECT 1 FROM c WHERE space<>?3 OR document<>?4 OR actor_a<>'editor' OR actor_b<>(CASE WHEN phase='viewer' THEN 'viewer' ELSE 'editor' END) OR status<>'passed' OR NOT EXISTS(SELECT 1 FROM json_each(c.evidence) WHERE length(trim(value))>0)) AS ok`);
    for (const row of rows) { const oracle = query.get(JSON.stringify(row.required), JSON.stringify(row.receipts), "s", "d") as { ok: number }; assert.equal(Boolean(oracle.ok), row.pass); assert.equal(openedPhasesPassV1(row.required, row.receipts, "s", "d"), Boolean(oracle.ok)); }
  } finally { db.close(); }
});


test("in-flight cancellation produces incomplete receipts with the original reason", async () => {
  const controller = new AbortController(), reason = Error("stop");
  const ui = { observe: async () => [], canvas: async () => null, click: async () => {}, press: async () => {}, chooseFiles: async () => {}, download: async () => ({ name: "", mime: null, bytes: new Uint8Array() }) };
  const context = { spaceId: "space", documentId: "doc", locale: "en" as const, signal: controller.signal, a: ui, b: ui };
  const opened = { required: ["a"], run: async () => new Promise<never>(() => {}) };
  const pending = runOpenedChecksV1(opened, context, controller.signal);
  controller.abort(reason);
  const checks = await pending;
  assert.equal(checks[0]?.status, "unexecuted");
  assert.match(checks[0]?.evidence[0] ?? "", /stop/);
  assert.equal(openedChecksPassV1(opened.required, checks), false);
  assert.equal(controller.signal.reason, reason);
});
