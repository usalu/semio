/** 🧂️ Contract: the chunk cuts of a source roster are a pure function of the paths, so remembering the cut of each path (until the input memory is released, and never beyond 131072 paths) changes no chunk: warm, released and over-full memos cut exactly where an independent Python derivation (SHA-256 first byte, multiple of 128, at least 16 paths per chunk) cuts, and the portable input patterns keep matching exactly the paths they matched with the escape and glob-character expressions compiled once. */
import{expect,test}from"bun:test";
import{testPythonV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

const inputs=await import(new URL("../../🟨️.mjs",import.meta.url).href) as typeof import("../../🟨️.mjs")&{releaseInputMemoryV1():void;createBuildInputPathMatcherV1(pattern:string):(path:string)=>boolean;buildInputPathMatchesV1(path:string,pattern:string):boolean};
const python=testPythonV1();
const names=["🧰️framework","🛍️products","src","lib","é東","a b"];
const roster=(count:number,seed:number)=>Array.from({length:count},(_,index)=>`${names[(index*7+seed)%names.length]}${(index*31+seed)%97}/${names[index%names.length]}/f${index}-${seed}.ts`).sort();
const oracle=(files:string[])=>{
 const run=Bun.spawnSync([python!,"-I","-c",`import sys,json,hashlib
files=json.load(sys.stdin);chunks=[];cur=[]
for f in files:
    cur.append(f)
    if len(cur)>=16 and hashlib.sha256(f.encode("utf-16","surrogatepass").decode("utf-16","replace").encode()).digest()[0]%128==0:
        chunks.append(cur);cur=[]
if cur: chunks.append(cur)
print(json.dumps(chunks,ensure_ascii=True))`],{stdin:Buffer.from(JSON.stringify(files))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return JSON.parse(run.stdout.toString()) as string[][];
};

test("cold, warm and released memos cut a roster where the Python derivation cuts",()=>{
 inputs.releaseInputMemoryV1();
 const files=roster(4000,5),expected=oracle(files);
 expect(expected.length).toBeGreaterThan(5);
 expect(inputs.splitInputClosureV1(files)).toEqual(expected);
 expect(inputs.splitInputClosureV1(files)).toEqual(expected);
 inputs.releaseInputMemoryV1();
 expect(inputs.splitInputClosureV1(files)).toEqual(expected);
});

test("a roster with an added path still equals the derivation when the memo already knows the others",()=>{
 inputs.releaseInputMemoryV1();
 const files=roster(2500,9);inputs.splitInputClosureV1(files);
 const grown=[...files,"zz/added.ts"].sort();
 expect(inputs.splitInputClosureV1(grown)).toEqual(oracle(grown));
});

test("a memo that outgrew its limit is emptied and the cuts stay exact",()=>{
 inputs.releaseInputMemoryV1();
 const many=roster(140000,3);
 expect(many.length).toBeGreaterThan(131072);
 const chunks=inputs.splitInputClosureV1(many);
 expect(chunks.flat()).toEqual(many);
 expect(chunks).toEqual(oracle(many));
 expect(inputs.splitInputClosureV1(many.slice(0,3000))).toEqual(oracle(many.slice(0,3000)));
});

test("portable input patterns match exactly, however often and however many patterns are asked",()=>{
 const match=inputs.createBuildInputPathMatcherV1("src/**/*.{ts,mjs}");
 expect(match("src/a/b/c.ts")).toBe(true);
 expect(match("src/c.mjs")).toBe(true);
 expect(match("src/c.json")).toBe(false);
 expect(match("lib/c.ts")).toBe(false);
 for(let round=0;round<3;round++){
  expect(inputs.buildInputPathMatchesV1("a.b/c+d.ts","a.b/c+d.ts")).toBe(true);
  expect(inputs.buildInputPathMatchesV1("axb/c+d.ts","a.b/c+d.ts")).toBe(false);
  expect(inputs.buildInputPathMatchesV1("x/y.ts","!(a|b)/*.ts")).toBe(true);
  expect(inputs.buildInputPathMatchesV1("a/y.ts","!(a|b)/*.ts")).toBe(false);
 }
 for(let index=0;index<5000;index++)expect(inputs.buildInputPathMatchesV1("p"+index+"/x",`p${index}/**`)).toBe(true);
 expect(inputs.buildInputPathMatchesV1("src/x.ts","src/*.ts")).toBe(true);
});
