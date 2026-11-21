const assert = require("node:assert/strict");
const Ajv = require("ajv/dist/2020");
const { applyPatch } = require("fast-json-patch");
const fixture = require("./🔣️.json");
const validate = new Ajv({ strict: true }).compile(require("./🧬️schema/🔣️.json"));
assert.ok(validate(fixture), JSON.stringify(validate.errors));

for (const test of fixture.cases) {
  const base = { id: fixture.assignedId.before, streamOffset: fixture.streamOffset.before };
  if (test.initial.length || test.present) base.asset = test.initial;
  let document = applyPatch(base, [
    { op: "replace", path: "/id", value: fixture.assignedId.after },
    { op: "replace", path: "/streamOffset", value: fixture.streamOffset.after },
  ], true, false).newDocument;
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
  const post = refused ? base : document;
  assert.equal(post.id, refused ? fixture.assignedId.before : fixture.assignedId.after, test.id);
  assert.equal(post.streamOffset, refused ? fixture.streamOffset.before : fixture.streamOffset.after, test.id);
  assert.deepEqual(post.asset ?? [], refused ? test.initial : test.expected, test.id);
}
