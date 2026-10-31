import {test,expect} from "bun:test";
import {observe} from "./🧰️owner.ts";
import policy from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json";
import corpus from "./🔣️.json";

for(const manifest of corpus.unitComponents)test("every original private geometry unit suite fits its real compilation owner: "+manifest,async()=>{const inputs=await observe(manifest,[...corpus.cfg,"test"]),limits=policy.storage.sourceBoundary;expect(inputs.length).toBeLessThanOrEqual(limits.maxSourceFiles);expect(inputs.reduce((sum,input)=>sum+input.bytes,0)).toBeLessThanOrEqual(limits.maxSourceBytes);expect(inputs.reduce((sum,input)=>sum+input.activeLines,0)).toBeLessThanOrEqual(limits.maxActiveLines);});
