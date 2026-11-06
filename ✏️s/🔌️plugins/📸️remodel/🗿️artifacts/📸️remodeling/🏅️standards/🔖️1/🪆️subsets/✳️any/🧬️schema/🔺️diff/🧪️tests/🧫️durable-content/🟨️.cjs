const assert = require("node:assert/strict");
const { applyPatch } = require("fast-json-patch");
const fixture = require("./🔣️.json");

for (const test of fixture.cases) {
  let document = test.initial.length || test.present ? { asset: [...test.initial] } : {};
  let refused = false;
  for (const row of test.rows) {
    const present = Object.hasOwn(document, "asset");
    if ((row.operation === "create") === present || (row.operation === "truncate" && row.from >= document.asset.length)) {
      refused = true;
      break;
    }
    const operations = row.operation === "create"
      ? [{ op: "add", path: "/asset", value: row.chunks }]
      : row.operation === "append"
        ? row.chunks.map(value => ({ op: "add", path: "/asset/-", value }))
        : row.from === 0
          ? [{ op: "remove", path: "/asset" }]
          : Array.from({ length: document.asset.length - row.from }, () => ({ op: "remove", path: `/asset/${row.from}` }));
    document = applyPatch(document, operations, true, false).newDocument;
  }
  assert.equal(refused, !!test.refused, test.id);
  if (!refused) assert.deepEqual(document.asset ?? [], test.expected, test.id);
}
