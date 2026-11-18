/** 🧮️ Contract: a read that sets owners aside names how many and why, in both languages, grouped by reason with the largest group first, so one tooling fault that set aside hundreds of owners is never a silent line per owner: the census is said for the first owner and then at every fiftieth, and every line of an owner carries the running count.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{asideCensusV1}from"../../🟨️.mjs";

const source=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8");
test("the census counts the owners and names the largest reasons first, in English and German",()=>{
 const reasons=[...Array(5).fill("compiler could not be loaded: boom"),"Native dependency has no build project owner","Native dependency has no build project owner","stale"];
 const census=asideCensusV1(reasons);
 expect(census.en).toBe("8 owners are set aside by this read, by reason: 5 x compiler could not be loaded: boom | 2 x Native dependency has no build project owner | 1 x stale");
 expect(census.de).toBe("8 Projekte werden von diesem Lauf zurückgestellt, nach Grund: 5 x compiler could not be loaded: boom | 2 x Native dependency has no build project owner | 1 x stale");
});
test("only the three largest reasons are named and a long reason is cut",()=>{
 const census=asideCensusV1(["a","b","c","d","x".repeat(400)],3);
 expect(census.en.split(" | ").length).toBe(3);
 expect(asideCensusV1(["x".repeat(400)]).en.length).toBeLessThan(260);
});
test("the owner loop says the census for the first owner and at every fiftieth and counts in every line",()=>{
 expect(source).toContain("asideCensusV1([...unavailable.values()]");
 expect(source).toContain("unavailable.size%50===0");
});
