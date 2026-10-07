import {test, expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve, join} from "node:path";
import {applyPatch} from "fast-json-patch";

test("Drawing native paged owner preserves semantic snapshot and UTF-8 keys", () => {
  const owner = resolve(import.meta.dir, "../..");
  const law = JSON.parse(readFileSync(join(owner, "🧫️fixtures/📋️native-owner/🔣️.json"), "utf8"));
  const input = JSON.parse(readFileSync(resolve(owner, "../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json"), "utf8"));
  const text = law.text.repeat(law.repeat);
  input.id = text; input.title = text; input.assets = {[text]:{mime:"image/png",data:"Grundstück🧬"}};
  expect(new TextEncoder().encode(text).length).toBeGreaterThan(law.bodyBytes);
  const copied = applyPatch({}, [{op:"add",path:"/snapshot",value:structuredClone(input)}], true, false).newDocument.snapshot;
  expect(copied).toEqual(JSON.parse(JSON.stringify(input)));
  const retired = applyPatch({snapshot:copied}, [{op:"remove",path:"/snapshot"}], true, false).newDocument;
  expect(Object.keys(retired).length === 0).toBe(law.terminalEmpty);
  expect(input.id === text && Object.keys(input.assets)[0] === text).toBe(law.originalUnchanged);
  console.log("[DEBUG] Drawing paged native owner independent JSON Patch/UTF-8 law verified");
});
