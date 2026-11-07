import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {installedChildrenDigestV1,verifyInstalledLedgerV1} from "../../🟦️.ts";
import {installedPortsV1} from "../../💾️storage/🟦️.ts";
import {link,lstat,mkdtemp,mkdir,rm,symlink,writeFile} from "node:fs/promises";
import {join} from "node:path";
import fixture from "./🔣️.json";

const stamp=["1","2","3","1","0","1","1"],row=(path:string)=>({path,stamp,settled:true,children:"",directories:0,files:0,links:0}),receipt=(count:number)=>({device:"1",shape:{entries:count,directories:count,files:0,links:0}}),facts={kind:"directory",stamp};

test("ledger progress counts completed metadata rows against SQLite",async()=>{
 const db=new Database(":memory:");try{for(const sample of fixture.cases){const rows=Array.from({length:sample.rows},(_,i)=>({...row(String(i)),directories:i===0?sample.rows-1:0})),ticks:number[]=[],result=await verifyInstalledLedgerV1(receipt(rows.length) as never,rows as never,{stat:async()=>facts as never,children:async()=>[],control:{check:()=>{},tick:async n=>{ticks.push(n);}}});const expected=db.query("with recursive n(x) as (select 32 union all select x+32 from n where x+32<=?1) select x from n where x<=?1").all(sample.rows).map((r:any)=>r.x);expect(result.status).toBe("match");expect(ticks).toEqual(sample.ticks);expect(ticks).toEqual(expected);}}finally{db.close();}
});

test("cancellation during metadata waits for settlement and retains its reason",async()=>{
 const controller=new AbortController(),reason=Error("cancelled"),rows=[row("a")];let release!:()=>void,entered!:()=>void,settled=false;const started=new Promise<void>(r=>entered=r),gate=new Promise<void>(r=>release=r),result=verifyInstalledLedgerV1(receipt(1) as never,rows as never,{stat:async()=>{entered();await gate;settled=true;return facts as never;},children:async()=>[],control:{check:()=>controller.signal.throwIfAborted(),tick:async()=>{}}});await started;controller.abort(reason);expect(settled).toBe(false);release();await expect(result).rejects.toBe(reason);expect(settled).toBe(true);
});

test("relist settlement, failures and progress cancellation retain their meanings",async()=>{
 const reason=Error("cancelled"),controller=new AbortController(),children=[{name:"f",kind:"file" as const}],rows=[{...row("a"),settled:false,children:installedChildrenDigestV1(children)}];let release!:()=>void,entered!:()=>void;const started=new Promise<void>(r=>entered=r),gate=new Promise<void>(r=>release=r),ports={stat:async()=>facts as never,children:async()=>{entered();await gate;return children;},control:{check:()=>controller.signal.throwIfAborted(),tick:async()=>{}}},pending=verifyInstalledLedgerV1(receipt(1) as never,rows as never,ports);await started;controller.abort(reason);release();await expect(pending).rejects.toBe(reason);
 const idle={check:()=>{},tick:async()=>{}},base={stat:async()=>facts as never,children:async()=>children,control:idle};expect((await verifyInstalledLedgerV1(receipt(1) as never,rows as never,base)).status).toBe("match");expect((await verifyInstalledLedgerV1(receipt(1) as never,rows as never,{...base,stat:async()=>{throw Error("io");}})).status).toBe("mismatch");expect((await verifyInstalledLedgerV1(receipt(1) as never,rows as never,{...base,children:async()=>{throw Error("io");}})).status).toBe("mismatch");
 const many=Array.from({length:33},(_,i)=>({...row(String(i)),directories:i===0?32:0})),cancel=new AbortController();let calls=0;await expect(verifyInstalledLedgerV1(receipt(33) as never,many as never,{stat:async()=>{calls++;return facts as never;},children:async()=>[],control:{check:()=>cancel.signal.throwIfAborted(),tick:async()=>cancel.abort(reason)}})).rejects.toBe(reason);expect(calls).toBe(32);
});

test("physical metadata adapter preserves directory kinds and missing paths",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");const root=await mkdtemp(join(artifacts,"ledger-"));try{await mkdir(join(root,"d"));await writeFile(join(root,"f"),"x");const ports=installedPortsV1(root,{check:()=>{},tick:async()=>{}}),facts=await ports.stat("."),children=await ports.children(".");expect(facts?.kind).toBe("directory");expect(await ports.stat("missing")).toBeUndefined();expect(children.map(({name,kind})=>[name,kind]).sort()).toEqual([["d","directory"],["f","file"]]);}finally{await rm(root,{recursive:true,force:true});}
});

test("link kind mutations match the independent SQLite decision",async()=>{
 const db=new Database(":memory:");try{for(const item of fixture.links){const before=[{name:"entry",kind:item.before as "file"|"link"}],after=[{name:"entry",kind:item.after as "file"|"link"}],rows=[{...row("."),settled:false,children:installedChildrenDigestV1(before)}],result=await verifyInstalledLedgerV1(receipt(1) as never,rows as never,{stat:async()=>facts as never,children:async()=>after,control:{check:()=>{},tick:async()=>{}}}),oracle=db.query("select case when ?1=?2 then 'match' else 'mismatch' end status,case when ?1=?2 then null else 'listing-changed' end class").get(item.before,item.after);expect({status:result.status,class:result.status==="mismatch"?result.class:null},item.id).toEqual(oracle);expect({status:item.status,class:item.class},item.id).toEqual(oracle);}}finally{db.close();}
});

test("directory mode identity matches the independent SQLite decision",async()=>{
 const db=new Database(":memory:");try{for(const item of fixture.modes){const changed={...facts,stamp:[...stamp.slice(0,2),item.after,...stamp.slice(3)]},result=await verifyInstalledLedgerV1(receipt(1) as never,[row(".")] as never,{stat:async()=>changed as never,children:async()=>[],control:{check:()=>{},tick:async()=>{}}}),oracle=db.query("select case when ?1=?2 then 'match' else 'mismatch' end status,case when ?1=?2 then null else 'directory-changed' end class").get(item.before,item.after);expect({status:result.status,class:result.status==="mismatch"?result.class:null}).toEqual(oracle);expect({status:item.status,class:item.class}).toEqual(oracle);}}finally{db.close();}
});

test("Windows directory junctions and file hardlinks retain distinct physical kinds",async()=>{
 if(process.platform!=="win32")return;const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");const root=await mkdtemp(join(artifacts,"junction-"));try{await mkdir(join(root,"target"));await writeFile(join(root,"file"),"x");await symlink(join(root,"target"),join(root,"junction"),"junction");await link(join(root,"file"),join(root,"hardlink"));const ports=installedPortsV1(root,{check:()=>{},tick:async()=>{}}),children=await ports.children(".");expect(children.find(entry=>entry.name==="junction")?.kind).toBe("link");expect(children.find(entry=>entry.name==="hardlink")?.kind).toBe("file");expect((await lstat(join(root,"hardlink"))).nlink).toBe(2);}finally{await rm(root,{recursive:true,force:true});}
});
