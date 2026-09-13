/** 🛍️ TypeScript twin of the packed carrier map laws — the JavaScript engine itself is the oracle.
 *
 * A `UiFixedMap`'s wire form is a JSON object. The wgpu bridge's `renderDocument` hands the guest's
 * document to `JSON.stringify`, so whatever ECMAScript says about own-property order is what the Rust
 * decoder receives: array-index-like keys (`"10".."32"`) first in ascending numeric order, then the
 * remaining string keys (`"01".."09"`) in insertion order. These checks pin that this really happens
 * and that sorting the keys — which is exactly what the React renderer's `packedTextLeaf` does, and
 * what `UiFixedMap`'s decoder now does — recovers the catalogue payload byte-for-byte.
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

type CarrierMapFixture = {
  readonly uiTextMaxBytes: number;
  readonly uiFixedListItems: number;
  readonly pack: number;
  readonly value: string;
  readonly ascendingKeys: readonly string[];
  readonly javascriptKeys: readonly string[];
  readonly dataAttributesJson: string;
  readonly payload: string;
};

/** 🧩️ `value` then the attribute values in ASCENDING key order — the reader both renderers share. */
function packedLeaf(value: string, dataAttributes: { readonly [key: string]: string }): string {
  let payload = value;
  for (const key of Object.keys(dataAttributes).sort()) payload += dataAttributes[key];
  return payload;
}

export function catalogueCarrierMapSelfTests(): number {
  const fixture: CarrierMapFixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🛍️catalogue-carrier-map.json", import.meta.url), "utf8"));
  let checks = 0;

  assert.equal(fixture.ascendingKeys.length, fixture.uiFixedListItems, "the carrier packs exactly one full map");
  assert.equal(fixture.pack, 1 + fixture.uiFixedListItems, "one leaf carries the label slice plus a full map");
  checks += 2;

  const ascending: Record<string, string> = {};
  const decoded: Record<string, string> = JSON.parse(fixture.dataAttributesJson);
  for (const key of fixture.ascendingKeys) ascending[key] = decoded[key]!;
  const emitted = Object.keys(JSON.parse(JSON.stringify(ascending)));
  assert.deepEqual(emitted, [...fixture.javascriptKeys], "this engine emits the key order the fixture pins");
  assert.notDeepEqual(emitted, [...fixture.ascendingKeys], "a wire that were already ascending would make the law vacuous");
  checks += 2;

  assert.deepEqual(Object.keys(decoded), [...fixture.javascriptKeys], "the pinned wire bytes carry the engine's order");
  assert.deepEqual(Object.keys(decoded).slice().sort(), [...fixture.ascendingKeys], "the same keys, in a different order");
  checks += 2;

  assert.equal(packedLeaf(fixture.value, decoded), fixture.payload, "sorting the keys recovers the catalogue payload byte-for-byte");
  assert.notEqual(fixture.value + Object.keys(decoded).map((key) => decoded[key]).join(""), fixture.payload, "reading the wire in arrival order would corrupt the payload");
  checks += 2;

  const bytes = new TextEncoder().encode(fixture.payload).length;
  assert.equal(bytes, fixture.pack * fixture.uiTextMaxBytes, "every slice of the pinned pack is a full text slice");
  checks += 1;

  return checks;
}
