/** 🏚️ Relocation: 10 tests covering the root boundary of a rebase, facts of a moved tree held by content (and a change never missed), the journal's relative path, the directory stamps and the roots a fact index names. */
import{afterEach,expect,test}from"bun:test";
import{cpSync,mkdtempSync,rmSync,writeFileSync,mkdirSync,realpathSync}from"node:fs";
import{tmpdir}from"node:os";
import{join,relative}from"node:path";
import{createHash}from"node:crypto";
import{forgetFormerRootsV1,noteFormerRootV1,rebaseRowV1,rebaseTextV1,recordedRootsOfIndexV1,recordedRootsV1,adoptRecordedRootsV1,formerRootsV1,noteLiveRootV1}from"../🟨️.mjs";
import{observationsHold,observationRowsHoldV1}from"../../🟨️.mjs";
import{relativeFactPathV1}from"../../../🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs";
import{createEpochDoubtV1}from"../../../🕸️graph/🕰️generations/🔎️doubt/🟨️.mjs";

afterEach(()=>forgetFormerRootsV1());
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");

test("a rebase replaces a former root only at a path boundary and never twice",()=>{
 expect(noteFormerRootV1("/Users/Shared/semio","/Users/me/semio")).toBe(true);
 expect(rebaseTextV1("/Users/Shared/semio/a/b")).toBe("/Users/me/semio/a/b");
 expect(rebaseTextV1("/Users/Shared/semio-store/a")).toBe("/Users/Shared/semio-store/a");
 expect(rebaseTextV1("/x/Users/Shared/semio/a")).toBe("/x/Users/Shared/semio/a");
 expect(rebaseTextV1('["/Users/Shared/semio","n"]')).toBe('["/Users/me/semio","n"]');
 expect(rebaseTextV1(rebaseTextV1("/Users/Shared/semio/a"))).toBe("/Users/me/semio/a");
 const row={kind:"resolve",path:'["/Users/Shared/semio/a.ts","./b"]',fact:"/Users/Shared/semio/b.ts"};
 expect(rebaseRowV1(row)).toEqual({kind:"resolve",path:'["/Users/me/semio/a.ts","./b"]',fact:"/Users/me/semio/b.ts"});
 expect(rebaseRowV1({kind:"stamp",path:"x",fact:"y"})).toEqual({kind:"stamp",path:"x",fact:"y"});
 expect(noteFormerRootV1("/Users/me","/Users/me/semio")).toBe(false);
 expect(noteFormerRootV1("/Users/me/semio/inner","/Users/me/semio")).toBe(false);
 expect(formerRootsV1()).toEqual([["/Users/Shared/semio","/Users/me/semio"]]);
});

test("facts recorded under a moved tree hold by content in the new tree and a change is never missed",async()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"reloc-"))),from=join(base,"old"),to=join(base,"new");
 try{
  mkdirSync(join(from,"d"),{recursive:true});writeFileSync(join(from,"d","a.txt"),"alpha");
  const rows=[
   {kind:"exists",path:join(from,"d","a.txt"),fact:true},
   {kind:"exists",path:join(from,"d","gone.txt"),fact:false},
   {kind:"file",path:join(from,"d","a.txt"),fact:{size:5,mtimeMs:1,ctimeMs:1,ino:1,sha256:sha("alpha")}},
  ];
  expect(await observationsHold(rows as never,8,new Set(),true)).toBe(true);
  cpSync(from,to,{recursive:true});rmSync(from,{recursive:true});
  expect(await observationsHold(rows as never,8,new Set(),true)).toBe(false);
  expect(noteFormerRootV1(from,to)).toBe(true);
  expect(await observationsHold(rows as never,8,new Set(),true)).toBe(true);
  writeFileSync(join(to,"d","a.txt"),"alpHa");
  expect(await observationsHold(rows as never,8,new Set(),true)).toBe(false);
  writeFileSync(join(to,"d","a.txt"),"alpha");writeFileSync(join(to,"d","gone.txt"),"x");
  expect(await observationsHold(rows as never,8,new Set(),true)).toBe(false);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the journal reads a fact path of a former root as the same workspace-relative path",()=>{
 expect(relativeFactPathV1("/Users/me/semio","/Users/Shared/semio/a/b.ts")).toBeUndefined();
 noteFormerRootV1("/Users/Shared/semio","/Users/me/semio");
 expect(relativeFactPathV1("/Users/me/semio","/Users/Shared/semio/a/b.ts")).toBe("a/b.ts");
 expect(relativeFactPathV1("/Users/me/semio","/Users/Shared/semio")).toBe("");
 expect(relativeFactPathV1("/Users/me/semio","/Users/Shared/semio/a/b.ts")).toBe(relative("/Users/me/semio","/Users/me/semio/a/b.ts"));
 expect(relativeFactPathV1("/Users/me/semio","/Users/Shared/semio-store/a")).toBeUndefined();
});

test("the directory stamps vouch for the rows of a former root through the live directory",async()=>{
 noteFormerRootV1("/old/ws","/new/ws");
 const asked:string[]=[];
 const stamps={classify:async(paths:string[])=>{asked.push(...paths);return{unchanged:new Set(paths),measured:new Map(),links:new Map(),missing:[]};},save:async()=>undefined};
 const doubt=createEpochDoubtV1({stamps:stamps as never,statFile:async()=>undefined});
 const flags=await doubt.classify([{kind:"readdir",path:"/old/ws/a",fact:"x"},{kind:"lstat",path:"/old/ws/a/f",fact:"file"},{kind:"registry",path:"r",fact:"x"}] as never);
 expect([...flags]).toEqual([0,0,1]);
 expect(asked).toEqual(["/new/ws/a"]);
});

test("the roots a fact index names are found from closure, registry and derived rows only",()=>{
 const rows=[
  {kind:"closure",path:'["/Users/Shared/semio","abc",["some"]]',fact:"true"},
  {kind:"registry",path:'["/Users/Shared/semio","@a/b"]',fact:"x"},
  {kind:"derived",path:'{"walk":1,"workspaceRoot":"/Users/other/ws","roots":[]}',fact:"x"},
  {kind:"exists",path:"/Users/ignored/x",fact:true},
 ];
 expect(recordedRootsV1(rows)).toEqual(["/Users/Shared/semio","/Users/other/ws"]);
 const texts=rows.map(row=>row.path);
 const view={rows,rowKindOf:(id:number)=>rows[id]!.kind,rowPathIdOf:(id:number)=>id,textOf:(at:number)=>texts[at]!};
 expect(recordedRootsOfIndexV1(view as never)).toEqual(["/Users/Shared/semio","/Users/other/ws"]);
 expect(adoptRecordedRootsV1(view as never,"/Users/me/semio")).toBe(true);
 expect(formerRootsV1().map(pair=>pair[0])).toEqual(["/Users/Shared/semio","/Users/other/ws"]);
 expect(adoptRecordedRootsV1(view as never,"/Users/me/semio")).toBe(false);
 expect(adoptRecordedRootsV1(view as never,undefined)).toBe(false);
});

test("a rebase agrees with plain string replacement on random paths",()=>{
 noteFormerRootV1("/Users/Shared/semio","/Users/me/semio");
 const parts=["a","🧰️x","b c","semio-store","node_modules"];
 let state=7;const next=()=>(state=(state*1103515245+12345)&0x7fffffff);
 for(let n=0;n<300;n++){
  const tail=Array.from({length:1+next()%4},()=>parts[next()%parts.length]).join("/");
  expect(rebaseTextV1("/Users/Shared/semio/"+tail)).toBe(("/Users/Shared/semio/"+tail).replace("/Users/Shared/semio","/Users/me/semio"));
 }
});

test("the synchronous record checks read a recorded old-root path as the live path, also while the old tree still exists",()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"reloc-sync-"))),from=join(base,"old"),to=join(base,"new");
 try{
  mkdirSync(join(from,"d"),{recursive:true});writeFileSync(join(from,"d","a.txt"),"alpha");
  cpSync(from,to,{recursive:true});
  const row={kind:"file",path:join(from,"d","a.txt"),fact:{size:5,mtimeMs:1,ctimeMs:1,ino:1,sha256:sha("alpha")}};
  noteFormerRootV1(from,to);
  expect(observationRowsHoldV1([row] as never),"live equals the record").toBe(true);
  writeFileSync(join(to,"d","a.txt"),"alpHa");
  expect(observationRowsHoldV1([row] as never),"the old tree still holds the recorded bytes, the live tree was edited").toBe(false);
  rmSync(from,{recursive:true});writeFileSync(join(to,"d","a.txt"),"alpha");
  expect(observationRowsHoldV1([row] as never),"the old tree is gone").toBe(true);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a root of one workspace never rewrites the rows of another workspace served by the same process",()=>{
 noteLiveRootV1("/Users/Shared/semio");
 expect(noteFormerRootV1("/Users/Shared/semio","/Users/me/semio")).toBe(false);
 expect(rebaseTextV1("/Users/Shared/semio/a")).toBe("/Users/Shared/semio/a");
 forgetFormerRootsV1();
 expect(noteFormerRootV1("/Users/Shared/semio","/Users/me/semio")).toBe(true);
 noteLiveRootV1("/Users/Shared/semio");
 expect(rebaseTextV1("/Users/Shared/semio/a"),"served later: the mapping is dropped").toBe("/Users/Shared/semio/a");
 expect(noteFormerRootV1("/Users/Shared/semio","/Users/me/semio"),"and never comes back").toBe(false);
 forgetFormerRootsV1();
 expect(noteFormerRootV1("/old","/one")).toBe(true);
 expect(noteFormerRootV1("/old","/two"),"one root, two live roots: ambiguous").toBe(false);
 expect(rebaseTextV1("/old/x")).toBe("/old/x");
});

test("chained and nested former roots give the same answer in every insertion order",()=>{
 const orders=[[["/a","/b"],["/b","/c"]],[["/b","/c"],["/a","/b"]],[["/a","/x"],["/a/sub","/y"]],[["/a/sub","/y"],["/a","/x"]]];
 const answers=orders.map(order=>{forgetFormerRootsV1();for(const[former,live]of order)noteFormerRootV1(former,live);return["/a/x","/b/x","/a/sub/x"].map(text=>rebaseTextV1(text));});
 expect(answers[0]).toEqual(answers[1]);expect(answers[2]).toEqual(answers[3]);
});

test("a Windows root is rebased in the JSON text of a question and in a plain path",()=>{
 noteFormerRootV1("C:\\Users\\x\\ws","D:\\w");
 expect(rebaseTextV1('["C:\\\\Users\\\\x\\\\ws","n"]')).toBe('["D:\\\\w","n"]');
 expect(rebaseTextV1("C:\\Users\\x\\ws\\a.ts")).toBe("D:\\w\\a.ts");
 expect(rebaseTextV1("C:\\Users\\x\\ws2\\a.ts")).toBe("C:\\Users\\x\\ws2\\a.ts");
 expect(recordedRootsV1([{kind:"closure",path:'["C:\\\\Users\\\\x\\\\ws","h",["some"]]'}])).toEqual(["C:\\Users\\x\\ws"]);
});
