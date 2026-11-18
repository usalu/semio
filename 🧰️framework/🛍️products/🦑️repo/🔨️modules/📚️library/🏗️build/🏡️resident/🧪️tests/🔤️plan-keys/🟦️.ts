import {describe,expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

/** 🔤️ Every message key the host service names exists in both languages: a missing key fails the whole graph read of a roster plan (`Unknown message key host-plan-roster`). */
const host=resolve(import.meta.dir,"..","..","🏛️service","🌐️host","🟨️.mjs"),messages=schema.definitions.Policy.const.messages as Record<"en"|"de",Record<string,string>>;
describe("host message keys",()=>{
 test("each host-* key a host progress line names is declared in English and German",()=>{
  const used=[...new Set([...readFileSync(host,"utf8").matchAll(/progress\("[a-z-]+",[^,]+,[^,]+,((?:[^,{]|\([^)]*\))+)/gu)].flatMap(call=>[...call[1]!.matchAll(/"(host-[a-z0-9-]*[a-z0-9])"/gu)].map(found=>found[1]!)))];
  expect(used).toContain("host-plan-roster");
  expect(used.filter(key=>messages.en[key]===undefined)).toEqual([]);
  expect(used.filter(key=>messages.de[key]===undefined)).toEqual([]);
 });
});
