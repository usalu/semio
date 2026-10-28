/** 🪪️ Contract: the key of an observation row is its canonical text while that is short and `#` plus the SHA-256 of the text when it is long, so the key of a row with kilobytes of nested facts stays below 70 bytes; two rows have one key exactly when an independent Python model says they are the same row (kind, path and fact equal as JSON values), for short and long facts, and for rows that differ only in the last byte of a long fact. */
import{expect,test}from"bun:test";
import{join}from"node:path";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{ROW_KEY_TEXT_LIMIT_V1,rowKeyV1}from"../../🟨️.mjs";

const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️.py");
type Row={kind:string;path:string;fact:unknown};
const nested=(depth:number,seed:string):unknown=>depth===0?{sha256:seed.repeat(8).slice(0,64),size:seed.length,mtimeMs:1.5}:{nestedObservations:[nested(depth-1,seed),nested(depth-1,seed+"x")],provider:"build:command-imports:v1:"+seed};
const reorder=(value:unknown):unknown=>Array.isArray(value)?value.map(reorder):value!==null&&typeof value==="object"?Object.fromEntries(Object.entries(value as Record<string,unknown>).reverse().map(([key,item])=>[key,reorder(item)])):value;

const rows:Row[]=[
 {kind:"file",path:"a/b.ts",fact:{sha256:"0".repeat(64),size:3}},
 {kind:"lstat",path:"🧰️x/🔨️y/z.ts",fact:{type:"file"}},
 {kind:"analysis",path:"a/long.ts",fact:nested(5,"ab")},
 {kind:"analysis",path:"a/long.ts",fact:nested(5,"ac")},
 {kind:"analysis",path:"a/long2.ts",fact:nested(5,"ab")},
 {kind:"analysis",path:"a/"+"d".repeat(200),fact:null},
 {kind:"analysis",path:"a/"+"d".repeat(200),fact:0},
 {kind:"exists",path:"e",fact:true},{kind:"exists",path:"e",fact:false},
];

test("short keys are the canonical text, long keys are 65 bytes whatever the fact",()=>{
 expect(rowKeyV1(rows[0]!)).toBe('["file","a/b.ts",{"sha256":"'+"0".repeat(64)+'","size":3}]');
 for(const row of rows.filter(row=>JSON.stringify(row).length>400))expect(rowKeyV1(row)).toMatch(/^#[a-f0-9]{64}$/u);
 expect(rowKeyV1(rows[5]!).length).toBe(65);expect(ROW_KEY_TEXT_LIMIT_V1).toBe(128);
});

test("every pair of rows, reordered or not, is the same row exactly when the independent model says so",()=>{
 const pairs:[Row,Row][]=[];
 for(const a of rows)for(const b of rows){pairs.push([a,b]);pairs.push([a,{...b,fact:reorder(b.fact)}]);}
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({pairs}))});expect(run.exitCode).toBe(0);
 const same=JSON.parse(run.stdout.toString()).same as boolean[];
 pairs.forEach(([a,b],at)=>expect(rowKeyV1(a)===rowKeyV1({...b}),JSON.stringify([a.path,b.path])).toBe(same[at]));
 expect(same.filter(Boolean).length).toBeGreaterThan(rows.length);expect(same.filter(value=>!value).length).toBeGreaterThan(rows.length);
});
