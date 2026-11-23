/** 🧭️ Contract: a capture that shares one ancestry ledger reads exactly the bytes a ledger-free read returns, looks at each directory once, and still refuses when a directory it relied on is replaced before the capture ends. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {chmodSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,renameSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {captureNxAuthorizerSourcesV1,createNxAuthorizerAncestryV1,readNxAuthorizerSourceV1,verifyNxAuthorizerAncestryV1} from "../../🟦️.ts";

const signal=new AbortController().signal;
const tree=()=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"ancestry-")));mkdirSync(join(root,"a/b/c"),{recursive:true});mkdirSync(join(root,"elsewhere"));for(const[path,text]of Object.entries({"a/one.ts":"export const one=1;\n","a/b/two.ts":"export const two=2;\n","a/b/c/three.ts":"export const three=3;\n"}))writeFileSync(join(root,path),text);return root;};

test("a ledger changes how often directories are looked at, never what is read",async()=>{
 const root=tree();
 try{
  const ledger=createNxAuthorizerAncestryV1();
  for(const path of["a/one.ts","a/b/two.ts","a/b/c/three.ts"]){
   const plain=await readNxAuthorizerSourceV1(root,path,signal),shared=await readNxAuthorizerSourceV1(root,path,signal,ledger);
   expect(Buffer.from(shared).equals(Buffer.from(plain)),path).toBe(true);
  }
  expect([...ledger.known.keys()].map(path=>path.slice(root.length).replaceAll("\\","/")).sort()).toEqual(["","/a","/a/b","/a/b/c"]);
  await verifyNxAuthorizerAncestryV1(ledger,signal);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a directory that is replaced while the capture runs is refused when the capture ends",async()=>{
 const root=tree();
 try{
  const ledger=createNxAuthorizerAncestryV1();
  await readNxAuthorizerSourceV1(root,"a/b/two.ts",signal,ledger);
  renameSync(join(root,"a/b"),join(root,"a/b.moved"));symlinkSync(join(root,"elsewhere"),join(root,"a/b"));
  await expect(verifyNxAuthorizerAncestryV1(ledger,signal)).rejects.toThrow("tools.authorizer-ancestry-changed");
  rmSync(join(root,"a/b"),{force:true});renameSync(join(root,"a/b.moved"),join(root,"a/b"));
  await verifyNxAuthorizerAncestryV1(ledger,signal);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a symlinked ancestor is refused at once and a missing one keeps its error code for the caller",async()=>{
 const root=tree();
 try{
  symlinkSync(join(root,"a"),join(root,"linked"));
  const ledger=createNxAuthorizerAncestryV1();
  await expect(readNxAuthorizerSourceV1(root,"linked/one.ts",signal,ledger)).rejects.toThrow("tools.authorizer-directory-linked");
  await expect(readNxAuthorizerSourceV1(root,"absent/one.ts",signal,ledger)).rejects.toMatchObject({code:"ENOENT"});
  await expect(readNxAuthorizerSourceV1(root,"absent/two.ts",signal,ledger)).rejects.toMatchObject({code:"ENOENT"});
  await verifyNxAuthorizerAncestryV1(ledger,signal);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a cancelled capture does not finish the final look",async()=>{
 const root=tree(),abort=new AbortController();
 try{
  const ledger=createNxAuthorizerAncestryV1();
  await readNxAuthorizerSourceV1(root,"a/one.ts",signal,ledger);abort.abort(Error("cancelled"));
  await expect(verifyNxAuthorizerAncestryV1(ledger,abort.signal)).rejects.toThrow("cancelled");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a failed required source reports its actual request lineage without replacing the filesystem refusal",async()=>{
 const root=tree();
 try{
  writeFileSync(join(root,"entry.ts"),'import { value } from "./absent/leaf.ts"; export { value };\n');
  let primary:unknown;
  try{await captureNxAuthorizerSourcesV1(root,"entry.ts",signal);}catch(error){primary=error;}
  let oracle:unknown;try{lstatSync(join(root,"absent"));}catch(error){oracle=error;}
  expect(primary).toMatchObject({code:"ENOENT",syscall:"lstat",path:join(root,"absent")});
  expect(primary).toMatchObject({code:(oracle as NodeJS.ErrnoException).code,syscall:(oracle as NodeJS.ErrnoException).syscall,path:(oracle as NodeJS.ErrnoException).path});
  expect((primary as {lineage:unknown}).lineage).toEqual({root,entry:"entry.ts",installed:false,operation:"required-source-read",request:"absent/leaf.ts",origin:{kind:"import",importer:"entry.ts",specifier:"./absent/leaf.ts"},code:"ENOENT",failedPath:join(root,"absent")});
  writeFileSync(join(root,"entry.ts"),'export const optional="./optional/not-there.json";\n');
  const captured=await captureNxAuthorizerSourcesV1(root,"entry.ts",signal);
  expect(captured.manifest.sources.map(row=>row.path)).toEqual(["entry.ts"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a capture of a small closure touches only that closure, finishes in bounded time beside an unrelated held directory and reports its walk",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"closure-")));
 const unrelated=join(root,"unrelated");
 try{
  mkdirSync(join(root,"leaf"));mkdirSync(unrelated);
  for(let index=0;index<60;index++){writeFileSync(join(root,"leaf",`m${index}.ts`),`export const v${index}=${index};\n`);writeFileSync(join(unrelated,`u${index}.ts`),`export const u${index}=${index};\n`);}
  writeFileSync(join(root,"entry.ts"),Array.from({length:60},(_,index)=>`import {v${index}} from "./leaf/m${index}.ts";`).join("\n")+"\nexport const all=[];\n");
  chmodSync(unrelated,0o000);
  const reports:string[]=[],started=performance.now(),captured=await captureNxAuthorizerSourcesV1(root,"entry.ts",signal,{report:text=>reports.push(text)});
  expect(performance.now()-started).toBeLessThan(10_000);
  expect(captured.manifest.sources.map(row=>row.path)).toEqual(["entry.ts",...Array.from({length:60},(_,index)=>`leaf/m${index}.ts`).sort()]);
  expect(captured.manifest.sources.some(row=>row.path.startsWith("unrelated/"))).toBe(false);
  expect(reports.length).toBeGreaterThanOrEqual(3);
  expect(reports.every(text=>/^walked \d+ files, now /u.test(text))).toBe(true);
  for(const row of captured.manifest.sources)expect(row.sha256).toBe(createHash("sha256").update(readFileSync(join(root,row.path))).digest("hex"));
 }finally{chmodSync(unrelated,0o755);rmSync(root,{recursive:true,force:true});}
});

test("a directory that vanishes between resolving and reading a source is a membership change: the walk starts again and the path counts as removed",async()=>{
 const root=tree(),reports:string[]=[];
 try{
  mkdirSync(join(root,"gone"));writeFileSync(join(root,"gone/leaf.ts"),"export const leaf=1;\n");
  writeFileSync(join(root,"entry.ts"),'import {leaf} from "./gone/leaf.ts"; export {leaf};\n');
  const probed:string[]=[],captured=await captureNxAuthorizerSourcesV1(root,"entry.ts",signal,{report:text=>reports.push(text),probe:(kind,path,attempt)=>{probed.push(attempt+":"+kind+":"+path);if(attempt===1&&kind==="read"&&path==="gone/leaf.ts"){rmSync(join(root,"gone"),{recursive:true});writeFileSync(join(root,"entry.ts"),"export const entry=1;\n");}}});
  expect(captured.manifest.sources.map(row=>row.path)).toEqual(["entry.ts"]);
  expect(probed).toContain("1:read:gone/leaf.ts");
  expect(reports.filter(text=>text.startsWith("sources moved while they were read"))).toHaveLength(1);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a data entry that vanishes between listing and probing is a membership change and a path that stays gone still refuses with its filesystem error",async()=>{
 const root=tree();
 try{
  mkdirSync(join(root,"🧬️schema"));writeFileSync(join(root,"🧬️schema/a.json"),"{}\n");writeFileSync(join(root,"🧬️schema/b.json"),"{}\n");writeFileSync(join(root,"entry.ts"),"export const entry=1;\n");
  const captured=await captureNxAuthorizerSourcesV1(root,"entry.ts",signal,{probe:(kind,path,attempt)=>{if(attempt===1&&kind==="entry"&&path.endsWith("b.json"))rmSync(join(root,"🧬️schema/b.json"));}});
  expect(captured.manifest.sources.map(row=>row.path)).toEqual(["entry.ts","🧬️schema/a.json"]);
  writeFileSync(join(root,"entry.ts"),'import {x} from "./absent/leaf.ts"; export {x};\n');
  let attempts=0;
  await expect(captureNxAuthorizerSourcesV1(root,"entry.ts",signal,{probe:(kind,_path,attempt)=>{if(kind==="read")attempts=Math.max(attempts,attempt);}})).rejects.toMatchObject({code:"ENOENT"});
  expect(attempts).toBe(4);
 }finally{rmSync(root,{recursive:true,force:true});}
});
