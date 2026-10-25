/** ✂️ Contract: a source roster is stored as content-defined chunks, so rosters that differ in a few paths of a shared base share every other chunk file and an unchanged roster writes nothing new. An independent Python derivation decides the cut points. */
import{expect,test}from"bun:test";
import{fileURLToPath}from"node:url";
import fixture from"./🔣️.json";

const inputs=await import(new URL("../../🟨️.mjs",import.meta.url).href) as typeof import("../../🟨️.mjs");
const roster=[...fixture.roster].sort(),cut=(files:readonly string[])=>inputs.splitInputClosureV1(files);
const python=JSON.parse(Bun.spawnSync(["python3","-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))]).stdout.toString());
const keys=(chunks:string[][])=>new Set(chunks.map(chunk=>JSON.stringify(chunk)));

test("the cut points equal the independent Python derivation for the roster, an added path and a removed path",()=>{
 expect(cut(roster)).toEqual(python.base);
 expect(cut([...roster,...fixture.added].sort())).toEqual(python.added);
 expect(cut(roster.filter(path=>path!==fixture.removed))).toEqual(python.removed);
});

test("the chunks concatenate to the roster, hold at least 16 paths except the last and an unchanged roster cuts identically",()=>{
 const chunks=cut(roster);
 expect(chunks.flat()).toEqual(roster);
 for(const chunk of chunks.slice(0,-1))expect(chunk.length).toBeGreaterThanOrEqual(16);
 expect(chunks.length).toBeGreaterThan(3);
 expect(cut(roster)).toEqual(chunks);
 expect(cut([])).toEqual([]);
});

test("one added or removed path changes at most two chunk files; every other chunk file is the identical content-addressed file",()=>{
 const base=keys(cut(roster));
 for(const changed of[[...roster,fixture.added[0]!].sort(),roster.filter(path=>path!==fixture.removed)]){
  const next=keys(cut(changed)),fresh=[...next].filter(key=>!base.has(key));
  expect(fresh.length).toBeGreaterThan(0);expect(fresh.length).toBeLessThanOrEqual(2);
 }
});
