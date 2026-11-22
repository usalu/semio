/** ♻️ Contract: a moved owner registry restarts the owner stage, and an owner that the earlier pass settled stays retained (bounded by the image budget of the observation module) unless the memory governor reports pressure, so the next pass answers it from the retained image after a row check instead of a full look. @see ../../🟨️.mjs */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import ts from "typescript";
import policy from "../../../../../🧠️memory/🧬️schema/🔣️.json";

test("the owner loop releases the retained image of an owner only under memory pressure",()=>{
 const text=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),file=ts.createSourceFile("nodes.mjs",text,ts.ScriptTarget.Latest,true);
 expect(file.statements.length).toBeGreaterThan(0);
 expect(text).toContain('if(governor===undefined||governor.pressure()===true)releaseProjectInferenceRetentionV1(cachePath);');
 expect(text.match(/releaseProjectInferenceRetentionV1\(cachePath\)/gu)?.length).toBe(1);
 const bound=(policy as unknown as {definitions:{Policy:{const:{retainedImageBytes:number}}}}).definitions.Policy.const.retainedImageBytes;
 expect(bound).toBeGreaterThan(0);expect(bound).toBeLessThanOrEqual(64*1048576);
});
