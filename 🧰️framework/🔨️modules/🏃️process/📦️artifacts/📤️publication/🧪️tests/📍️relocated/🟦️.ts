import{expect,test}from"bun:test";
import corpus from"./🔣️.json";
import{artifactOwnerRelocatedV1}from"../../🟦️.ts";

test("an artifact owner marker of a relocated worktree is recognised by its shared relative tail, not by its absolute root",()=>{
 for(const row of corpus.cases)expect(artifactOwnerRelocatedV1(row.marker,row.owner),row.id).toBe(row.relocated);
});
