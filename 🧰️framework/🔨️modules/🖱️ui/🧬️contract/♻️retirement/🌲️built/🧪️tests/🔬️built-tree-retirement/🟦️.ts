/** 🔬️ Canonical testBuiltTreeRetirementFixture fixture and oracle checks. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { inspectRustModuleGraphFacts } from "../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

export function testBuiltTreeRetirementFixture(): void {
  const read = (path: string) => readFileSync(new URL(path, new URL("../../", import.meta.url)), "utf8");
  const fixture = JSON.parse(read("./🧫️fixture/🔣️.json"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(read("./🧬️schema/🔣️.json")));
  assert(validate(fixture), JSON.stringify(validate.errors));
  const valueBytes = (value: unknown): number => typeof value === "string" ? Buffer.byteLength(value) : Array.isArray(value) ? value.reduce((sum, item) => sum + valueBytes(item), 0) : value && typeof value === "object" ? Object.entries(value).reduce((sum, [key, item]) => sum + Buffer.byteLength(key) + valueBytes(item), 0) : 0;
  const binding = fixture.binding;
  const extras = Buffer.byteLength("K") + Buffer.byteLength(binding.action.scope + binding.action.name + binding.capability) + valueBytes(binding.args) + Buffer.byteLength(fixture.menu.id) + valueBytes(fixture.menu.args) + Buffer.byteLength("CchildRrejected");
  assert.equal(extras, fixture.extraPayloadBytes);
  assert.equal(fixture.chain.nodes, fixture.maximumPages + 1);
  assert.equal(fixture.chain.ordinaryPages + fixture.chain.rejectedPages, fixture.maximumPages);
  assert(fixture.maximumPages > fixture.chain.observerDepth);
  assert.equal(fixture.foreignPage.ownedPages + fixture.foreignPage.queuedPages, fixture.maximumPages);
  assert.equal(fixture.foreignPage.cursor, 0);
  const frames: number[] = [];
  for (let index = 0; index < fixture.chain.pages; index++) frames.push(index);
  assert.deepEqual(frames.toReversed(), Array.from({ length: fixture.maximumPages }, (_, index) => fixture.maximumPages - index - 1));
  let hostile = 0;
  for (const [key, value] of Object.entries(fixture.ownership)) { assert(!validate({ ...fixture, ownership: { ...fixture.ownership, [key]: !value } })); hostile++; }
  assert(!validate({ ...fixture, maximumPages: fixture.chain.observerDepth })); hostile++;
  const native = read("./🦀️.rs");
  assert(native.includes("pub struct BuiltTreeRetirement") && native.includes("UiTypedRetirementCursor") && native.includes("try_next_or_release"));
  assert(!native.includes("close_ui_value_page_one") && !native.includes("close_built_node_page_one"), "exact tree closure cannot advance a global retirement queue");
  const modules = inspectRustModuleGraphFacts(native).modules.filter(module => module.name === "tests" && module.conditional && !module.inline && module.pathTarget !== null);
  assert.equal(modules.length, 1);
  const testUrl = new URL(modules[0]!.pathTarget!, new URL("../../", import.meta.url));
  const fixtureUrl = new URL("../../🧫️fixture/🔣️.json", import.meta.url);
  const includes = [...readFileSync(testUrl, "utf8").matchAll(/include_str!\(\s*"([^"]+)"\s*\)/gu)];
  assert(includes.some(match => new URL(match[1]!, testUrl).href === fixtureUrl.href));
  const typedDepths = ["UiText", "crate::Component", "crate::LayoutSpec", "crate::StyleSpec", "crate::Activity", "bool", "crate::AccessibilitySpec", "crate::UiNodeBindings", "Option<MenuRef>"];
  assert.equal(typedDepths.length, fixture.payloadFields.length);
  for (const type of typedDepths) assert(native.includes(`<${type} as UiTypedRetire>::DEPTH`), `missing typed depth guard for ${type}`);
  assert(/while index < depths\.len\(\)\s*\{\s*assert!\(depths\[index\] <= typed::UI_TYPED_RETIREMENT_DEPTH\);\s*index \+= 1;\s*\}/u.test(native), "typed depth guards must exhaust the guarded depth array");
  console.log(`[DEBUG] built-tree ownership source: 384-page chain, 9 typed fields, 30 exact extra bytes, ${hostile} denials; native closure and safe abandonment unverified`);
}
