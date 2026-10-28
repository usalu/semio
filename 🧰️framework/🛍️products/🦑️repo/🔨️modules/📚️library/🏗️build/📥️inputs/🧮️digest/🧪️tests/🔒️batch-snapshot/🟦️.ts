/** 🔒️ Contract: one exact Source batch hashes its inputs from ONE tree state: an input rewritten, replaced or created after it was read ends the batch as a concurrent edit of the `snapshot` layer and the whole batch runs again, so the digest never mixes two states; an entry edited a moment before the batch ends is waited out (announced in both languages) and looked at again; It needs the repository envelope: run it through `repo exec`. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {withExactSourceReadBatchV1,readExactSourceFileV1} from "../../📥️read/🟨️.mjs";
import {receiveScriptProcessInvocation,createIssuedResourceControlV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const hash=(text:string)=>createHash("sha256").update(text).digest("hex");
const quiet=(): {restore:()=>void;lines:string[]}=>{const lines:string[]=[],write=process.stderr.write.bind(process.stderr);process.stderr.write=((chunk:string|Uint8Array)=>{lines.push(String(chunk));return true;}) as typeof process.stderr.write;return{lines,restore:()=>{process.stderr.write=write;}};};

test("a batch hashes its inputs from one tree state: edits, replacements and creations during the batch run it again",()=>receiveScriptProcessInvocation(process.env,async original=>{
 const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Ticket Source fixture sink required");
 const root=mkdtempSync(join(sink,"bs-")),control=createIssuedResourceControlV1(original,"source:batch-snapshot",120000);
 const saved={root:process.env.SEMIO_WORKSPACE_ROOT,language:process.env.SEMIO_LANGUAGE},capture=quiet();
 try{
  writeFileSync(join(root,"AGENTS.md"),"x");writeFileSync(join(root,"package.json"),"{}");process.env.SEMIO_WORKSPACE_ROOT=root;delete process.env.SEMIO_LANGUAGE;
  const body=(name:string,text:string)=>writeFileSync(join(root,name),text);
  const digestOf=async(change?:(round:number)=>void)=>{let rounds=0;const digest=await withExactSourceReadBatchV1(root,control,async batch=>{rounds++;const rows=[];for(const name of["a.txt","b.txt","gone.txt"])rows.push(name+"="+((await readExactSourceFileV1(batch,name))?.sha256??"absent"));change?.(rounds);return rows.join(";");});return{digest,rounds};};
  body("a.txt","A0");body("b.txt","B0");
  const quietRun=await digestOf();
  expect(quietRun.rounds).toBe(1);expect(quietRun.digest).toBe(["a.txt="+hash("A0"),"b.txt="+hash("B0"),"gone.txt=absent"].join(";"));
  const edited=await digestOf(round=>{if(round===1)body("a.txt","A1");});
  expect(edited.rounds).toBe(2);expect(edited.digest).toBe(["a.txt="+hash("A1"),"b.txt="+hash("B0"),"gone.txt=absent"].join(";"));
  const created=await digestOf(round=>{if(round===1)body("gone.txt","G");});
  expect(created.rounds).toBe(2);expect(created.digest).toBe(["a.txt="+hash("A1"),"b.txt="+hash("B0"),"gone.txt="+hash("G")].join(";"));
  const swapped=await digestOf(round=>{if(round===1){renameSync(join(root,"a.txt"),join(root,"t.txt"));renameSync(join(root,"b.txt"),join(root,"a.txt"));renameSync(join(root,"t.txt"),join(root,"b.txt"));}});
  expect(swapped.rounds).toBe(2);expect(swapped.digest).toBe(["a.txt="+hash("B0"),"b.txt="+hash("A1"),"gone.txt="+hash("G")].join(";"));
  const removed=await digestOf(round=>{if(round===1)rmSync(join(root,"gone.txt"));});
  expect(removed.rounds).toBe(2);expect(removed.digest).toBe(["a.txt="+hash("B0"),"b.txt="+hash("A1"),"gone.txt=absent"].join(";"));
 }finally{capture.restore();if(saved.root===undefined)delete process.env.SEMIO_WORKSPACE_ROOT;else process.env.SEMIO_WORKSPACE_ROOT=saved.root;if(saved.language!==undefined)process.env.SEMIO_LANGUAGE=saved.language;rmSync(root,{recursive:true,force:true});}
}),120000);

test("an entry edited a moment before the batch ends is waited out and announced in both languages, then the batch is accepted",()=>receiveScriptProcessInvocation(process.env,async original=>{
 const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Ticket Source fixture sink required");
 const root=mkdtempSync(join(sink,"bs-")),control=createIssuedResourceControlV1(original,"source:batch-settle",120000);
 const saved=process.env.SEMIO_LANGUAGE,capture=quiet();
 try{
  delete process.env.SEMIO_LANGUAGE;writeFileSync(join(root,"a.txt"),"fresh");
  const started=Date.now();
  const result=await withExactSourceReadBatchV1(root,control,async batch=>(await readExactSourceFileV1(batch,"a.txt"))?.sha256);
  expect(result).toBe(hash("fresh"));
  const text=capture.lines.join("");
  expect(text).toContain("ms ago");expect(text).toContain("vor ");expect(Date.now()-started).toBeGreaterThanOrEqual(100);
 }finally{capture.restore();if(saved===undefined)delete process.env.SEMIO_LANGUAGE;else process.env.SEMIO_LANGUAGE=saved;rmSync(root,{recursive:true,force:true});}
}),120000);
