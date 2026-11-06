import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import Ajv from "ajv";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { windowDocumentScopeV1, windowIdentityAttributesV1 } from "../🟦️.ts";

test("window identity follows each concrete session and document", () => {
  const rows = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  assert.equal(new Ajv().validate(schema, rows), true);
  for (const row of rows) {
    const html = renderToStaticMarkup(createElement("div", windowIdentityAttributesV1(row)));
    for (const [key, value] of Object.entries({ "app-id": row.app, role: row.role, "session-instance-id": row.instance, "window-id": row.window, "window-kind-id": row.kind }))
      assert.ok(html.includes(`data-surface-${key}="${value}"`));
    if (row.document) {
      assert.ok(html.includes('data-surface-space-id="space-a"'));
      assert.ok(html.includes('data-surface-document-id="artifact-a"'));
    } else assert.doesNotMatch(html, /data-surface-(space|document)-id=/);
  }
});

test("scope resolves only for one exact session owner", () => {
  const a = { session: { pluginId: "plugin", app: { id: "editor" }, instanceId: 1 }, scope: { spaceId: "space-a", documentId: "artifact-a" } };
  const b = { session: { pluginId: "plugin", app: { id: "viewer" }, instanceId: 2 }, scope: { spaceId: "space-b", documentId: "artifact-b" } };
  const owner = { pluginId: "plugin", appId: "editor", instanceId: 1 };
  assert.deepEqual(windowDocumentScopeV1([b, a], owner), a.scope);
  assert.equal(windowDocumentScopeV1([b], owner), null);
  assert.equal(windowDocumentScopeV1([a, a], owner), null);
  assert.equal(windowDocumentScopeV1([{ session: a.session }], owner), null);
});
