/** 🧾️ Contract: every refusal a build or run raises when something it reads changed under it belongs to exactly one declared class (`reobserve`, `reexec`, `fatal`); the throw sites of a retried class use the constructor of that class, the sites of a fatal class never do, and the table names real sites; an independent Python scan of the sources agrees. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import schema from "../../🧬️schema/🧾️refusals.json";
import churn from "../../🧬️schema/🔣️.json";
import {testPythonV1} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {churnProtocolV1,concurrentEditOfV1,concurrentEditV1,concurrentEditWireV1,staleImplementationV1} from "../../🟦️.ts";
import {failureChurnOfTailsV1,refusalClassOfTextV1} from "../../🧾️refusals/🟦️.ts";
import vectors from "./🔣️.json";

type Row={id:string;disposition:"reobserve"|"reexec"|"fatal";layer?:string;markers:string[];messages:{text:string;sites:string[]}[];describe:Record<string,string>};
const table=schema.definitions.Table.const as {version:1;constructors:string[];lookbehind:number;classes:Row[]};
const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),root=fileURLToPath(new URL("../../../../../../../",import.meta.url)),protocol=churnProtocolV1();
const answer=JSON.parse(Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),root,join(here,"..","..","🧬️schema","🧾️refusals.json"),join(here,"..","..","🧬️schema","🔣️.json"),join(here,"🔣️.json")]).stdout.toString()) as {scan:Record<string,{occurrences:number;typed:number;marked:number}>;texts:Record<string,{class:string;disposition:string}|null>};
const oracle=answer.scan;

function scan(){
 const report:Record<string,{occurrences:number;typed:number;marked:number}>={};
 for(const row of table.classes)for(const message of row.messages)for(const site of message.sites){
  const text=readFileSync(join(root,site),"utf8");let occurrences=0,typed=0,marked=0,from=0;
  for(;;){const index=text.indexOf(message.text,from);if(index<0)break;occurrences++;const window=text.slice(Math.max(0,index-table.lookbehind),index),before=window.slice(Math.max(window.lastIndexOf("throw "),window.lastIndexOf("return "),0));if(table.constructors.some(item=>before.includes(item)))typed++;if(row.markers.some(item=>before.includes(item)))marked++;from=index+1;}
  report[row.id+"|"+message.text+"|"+site]={occurrences,typed,marked};
 }
 return report;
}

test("the class table is closed under its schema: one disposition per class, a declared layer for every retried read, EN and DE descriptions that differ",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Disposition"})("reobserve")).toBe(true);
 const ids=new Set<string>();
 for(const row of table.classes){
  expect(ids.has(row.id),row.id).toBe(false);ids.add(row.id);
  expect(schema.definitions.Disposition.enum).toContain(row.disposition);
  if(row.disposition==="reobserve")expect(Object.keys(protocol.profiles),row.id).toContain(row.layer!);else expect(row.layer,row.id).toBeUndefined();
  expect(Object.keys(row.describe).sort(),row.id).toEqual([...protocol.languages].sort());
  expect(row.describe.en,row.id).not.toBe(row.describe.de);
  expect(row.describe.en!.length,row.id).toBeGreaterThan(20);
 }
 for(const required of ["source-read","source-owner-generation","bun-ownership","cargo-workspace-topology","stale-loaded-implementation","moved-format-stamp","half-edited-module","escaped-concurrent-edit","retries-exhausted","lockfile-frozen","own-output-or-store-custody"])expect(ids.has(required),required).toBe(true);
 for(const constructor of table.constructors)expect(constructor.length).toBeGreaterThan(3);
});

test("every declared message exists at its declared site, and an independent Python scan counts the same occurrences",()=>{
 const own=scan();
 expect(Object.keys(own).length).toBeGreaterThan(30);
 expect(own).toEqual(oracle);
 for(const [key,row] of Object.entries(own))expect(row.occurrences,key).toBeGreaterThan(0);
});

test("a class that is retried raises its messages through its own constructor and never through a plain Error; a fatal class never does",()=>{
 const report=scan();
 for(const row of table.classes)for(const message of row.messages)for(const site of message.sites){
  const found=report[row.id+"|"+message.text+"|"+site]!;
  if(row.disposition==="fatal"){expect(found.typed,row.id+" | "+message.text).toBe(0);continue;}
  if(row.markers.length===0)continue;
  expect(found.marked,row.id+" | "+message.text+" | "+site).toBeGreaterThan(0);
 }
});

test("a reader in any language raises a retried class through the wire form, and the layer that owns the snapshot reads it like the typed one",()=>{
 const wire=concurrentEditWireV1("generation","/p/package.json","Current shared Source-owner generation changed during consumption");
 expect(wire.message).toContain("generation changed");
 expect(Object.keys(wire as object).sort()).toEqual(["code","path","rank","reason"].sort());
 expect((wire as unknown as {code:string}).code).toBe(churn.definitions.Protocol.const.concurrentEditCode);
 expect(concurrentEditOfV1(wire)).toEqual({reason:"generation",path:"/p/package.json",rank:protocol.profiles.snapshot.rank-1});
 expect(concurrentEditOfV1(wire,protocol.profiles.snapshot.rank)?.path).toBe("/p/package.json");
 expect(concurrentEditOfV1(wire,protocol.profiles.snapshot.rank-1)).toBeUndefined();
 expect(concurrentEditOfV1(new Error("wrapped",{cause:wire}))?.reason).toBe("generation");
 expect(concurrentEditOfV1(Object.assign(new Error("x"),{code:"SEMIO_CONCURRENT_EDIT",reason:"nonsense",path:"/p"}))).toBeUndefined();
 expect(concurrentEditOfV1(Object.assign(new Error("x"),{code:"OTHER",reason:"bytes",path:"/p"}))).toBeUndefined();
 expect(concurrentEditOfV1(concurrentEditV1("generation","/q"))?.reason).toBe("generation");
 expect(staleImplementationV1("x")).toBeInstanceOf(Error);
});

test("a text captured from a failed child is classified exactly like the independent oracle and like the vector says, in both languages",()=>{
 expect(Object.keys(answer.texts).sort()).toEqual(vectors.texts.map(row=>row.id).sort());
 for(const row of vectors.texts){
  const found=refusalClassOfTextV1(row.text);
  expect(found?{class:found.id,disposition:found.disposition as string}:null,row.id).toEqual(row.class===null?null:{class:row.class,disposition:row.disposition!});
  expect(answer.texts[row.id]??null,row.id).toEqual(row.class===null?null:{class:row.class,disposition:row.disposition!});
 }
});

test("a failed run is classified by the output of its failed tasks, whichever tool ran them: an outdated implementation restarts the command, edited inputs run the build again, anything else stands",()=>{
 const text=(id:string)=>vectors.texts.find(row=>row.id===id)!.text;
 expect(failureChurnOfTailsV1([])).toBeUndefined();
 expect(failureChurnOfTailsV1([{task:"@a/x:test",text:"error TS2322: nope"}])).toBeUndefined();
 const edit=failureChurnOfTailsV1([{task:"@a/x:test",text:"error TS2322: nope"},{task:"@a/y:build",text:text("exhausted-en")}]);
 expect(concurrentEditOfV1(edit,protocol.profiles.build.rank)).toEqual({reason:"input",path:"@a/y:build",rank:protocol.profiles.build.rank-1});
 expect(concurrentEditOfV1(edit,protocol.profiles.snapshot.rank)).toBeUndefined();
 const stale=failureChurnOfTailsV1([{task:"@a/y:build",text:text("exhausted-en")},{task:"@a/z:test",text:"Stale loaded Source revision reader; restart required"}]) as Error&{code?:string};
 expect(stale.code).toBe(churn.definitions.Protocol.const.staleImplementationCode);expect(stale.message).toContain("@a/z:test");
 expect(concurrentEditOfV1(stale)).toBeUndefined();
 expect(failureChurnOfTailsV1([{task:"@a/f:build",text:text("own-custody")}])).toBeUndefined();
});
