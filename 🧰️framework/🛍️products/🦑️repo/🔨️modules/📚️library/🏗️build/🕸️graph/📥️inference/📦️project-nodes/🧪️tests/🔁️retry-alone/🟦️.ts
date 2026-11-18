/** 🔁️ Contract: a selected read that waited in vain for the fleet runs the owners it waited for here, once, alone, before it refuses; a second end of the wait refuses with no further try, and an owner the retry stops again is named in the refusal.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{heavyRetryNamesV1}from"../../🟨️.mjs";

const source=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),wait=source.slice(source.indexOf("const waitHeavy=async()=>{"),source.indexOf("const largestOf="));

test("the first end of the wait retries every owner it waited for, the second retries none",()=>{
 expect(heavyRetryNamesV1(["a","b"],false)).toEqual(["a","b"]);
 expect(heavyRetryNamesV1(["a","b"],true)).toEqual([]);
 expect(heavyRetryNamesV1([],false)).toEqual([]);
});

test("the wait of the owner pass ends in the retry before the refusal: owners leave `never`, are queued again, run through the loop, and what is stopped again is refused by name",()=>{
 expect(wait).toContain("heavyRetryNamesV1(left,retriedAlone)");
 const retry=wait.slice(wait.indexOf("if(again.length>0)"),wait.indexOf("if(left.length>0){"));
 expect(retry).toContain("retriedAlone=true");
 expect(retry).toContain("never.delete(name)");
 expect(retry).toContain("jobs.push(job)");
 expect(retry).toContain("await loop(Infinity)");
 expect(retry).toContain("HEAVY_RETRY_V1");
 expect(retry).toContain("throw refuseHeavy(rest)");
 expect(wait.indexOf("if(again.length>0)")).toBeLessThan(wait.indexOf("if(left.length>0){"));
 expect(source).toContain("let retriedAlone=false;");
});
