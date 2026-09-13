/** 🧪️ Validates explicit host preferences against the shared neutral schema and Ajv. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import { parseResolvedPluginViewState, VIEW_CONTEXT_LONG_STRING_CHARS, VIEW_CONTEXT_LONG_STRING_FIELDS } from "../../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "../../🧫️fixtures/🪟️resolved-host-context/🔣️.json";

export function testResolvedHostContext(): void {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture.valid), JSON.stringify(validate.errors));
  const actual = parseResolvedPluginViewState(fixture.valid);
  assert.deepEqual(actual, fixture.valid);
  assert.notEqual(actual, fixture.valid);
  assert.equal(schema.$defs.SessionIdentity.properties.userId.$ref, "#/$defs/Identifier");
  assert.equal(schema.$defs.SessionIdentity.properties.displayName.$ref, "#/$defs/Identifier");
  assert.equal(schema.$defs.Identifier.maxLength, fixture.identityCapacityChars);
  for (const row of fixture.invalid) {
    const value: Record<string, unknown> = structuredClone(fixture.valid);
    if ("remove" in row) for (const key of row.remove) delete value[key];
    if ("set" in row) Object.assign(value, row.set);
    const repeat = "repeat" in row ? row.repeat : undefined;
    if (repeat) value[repeat.field] = repeat.character.repeat(repeat.count);
    assert.equal(validate(value), false, row.name);
    assert.throws(() => parseResolvedPluginViewState(value), undefined, row.name);
  }
  // 🧩️ LAW: contributions are not a view-state field. The host publisher installs them into the
  // guest through the paged `setContributions` run (`🛠️ShellHelpers/🧩️contributions/🟦️.ts`), which
  // the guest folds into its own registry; a producer that puts the aggregated closure back into a
  // refresh's view state re-creates the 248 635-character payload this schema bounds at 65 536 and
  // breaks every window body of the app (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
  assert(!Object.hasOwn(schema.properties, "contributionsJson"), "contributionsJson must not be a view-context property");
  const refusedFieldRows = fixture.invalid.map((row) => ("set" in row && row.set ? Object.keys(row.set).join(",") : ""));
  assert(refusedFieldRows.includes("contributionsJson"), "the fixture must pin contributionsJson as a refused field");
  assert.throws(() => parseResolvedPluginViewState({ ...fixture.valid, contributionsJson: "[]" }), /explicit supported preferences required/, "contributionsJson");
  // 📏️ The one long field the contract keeps is bounded at the schema's own capacity, in characters.
  const capacity = fixture.panelCapacityChars;
  assert.equal(schema.properties.panelJson.maxLength, capacity);
  assert.equal(VIEW_CONTEXT_LONG_STRING_CHARS, capacity);
  assert.deepEqual([...VIEW_CONTEXT_LONG_STRING_FIELDS], ["panelJson"]);
  assert.doesNotThrow(() => parseResolvedPluginViewState({ ...fixture.valid, panelJson: "p".repeat(capacity) }), "panelJson at capacity");
  assert.throws(() => parseResolvedPluginViewState({ ...fixture.valid, panelJson: "p".repeat(capacity + 1) }), /invalid panel data at panelJson/, "panelJson over capacity");
  // 🗣️ The guest half of the SAME fixture: every row the plugin's own `ViewModel` decoder rejects
  // (`🧪️tests/🪟️resolved-host-context/🦀️.rs` pins the exact fault text) must be a row this
  // admission already refuses, and must name a field this admission itself requires — otherwise a
  // host path could hand a plugin a context that only the guest notices, which is how the anonymous
  // `missing field `locale`` reached the page (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
  for (const row of fixture.guestDecode) {
    const invalid = fixture.invalid.find((candidate) => candidate.name === row.name);
    assert(invalid !== undefined, `guestDecode row ${row.name} must also be an invalid row`);
    assert("remove" in invalid && invalid.remove.length === 1, `guestDecode row ${row.name} must remove exactly the field it names`);
    assert.equal(row.fault, `missing field \`${invalid.remove[0]}\``, row.name);
    assert.throws(() => parseResolvedPluginViewState({ ...fixture.valid, [invalid.remove[0]!]: undefined }), undefined, row.name);
  }
  console.log(`resolved-host-context cases=${fixture.invalid.length + 1} guest-rejections=${fixture.guestDecode.length} schema=valid explicit-preferences=required long-fields=${Object.values(schema.properties).filter((field) => "maxLength" in field && field.maxLength === capacity).length} contributions-in-view-state=refused`);
}
