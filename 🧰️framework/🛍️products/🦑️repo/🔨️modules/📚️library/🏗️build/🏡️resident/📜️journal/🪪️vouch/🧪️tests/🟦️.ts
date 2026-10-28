/** 🔗️ Contract: the journal glue of the vouch never vouches a path below a symbolic-link directory. A change made through a link is reported by the file watcher at the link's target, so `alias/x.json` of `alias -> real` is touched when `real/x.json` is; the directories above the verified paths that are links are found by the first complete look, sealed with the cursor and used by every later look of the same receipt. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {formatChangeCursorV1,mergeChangeRowsV1,type ChangeCursorV1,type ChangeRowV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import type {ChangeAnswerV1,ChangeJournalClientV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {readVouchSealV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🟦️.ts";
import {openJournalVouchV1} from "../🟦️.ts";

const EPOCH="0123456789abcdef";
let sandbox="";
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"vouch-linked-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));

function journal(){
 const log:{seq:number;row:ChangeRowV1}[]=[];let seq=0;
 const client:ChangeJournalClientV1={
  excluder:()=>()=>false,
  cursor:async()=>({epoch:EPOCH,seq}),
  since:async(from:ChangeCursorV1):Promise<ChangeAnswerV1>=>({exact:true,cursor:{epoch:EPOCH,seq},rows:mergeChangeRowsV1(log.filter(entry=>entry.seq>from.seq).map(entry=>entry.row))}),
 };
 return{client,report:(row:ChangeRowV1)=>{log.push({seq:++seq,row});}};
}

test("a path below a symbolic-link directory is never vouched, in the first look, in the sealed record and in every later look",async()=>{
 const root=join(sandbox,"ws");mkdirSync(join(root,"real"),{recursive:true});mkdirSync(join(root,"other"));
 writeFileSync(join(root,"real/x.json"),"1");writeFileSync(join(root,"other/y.json"),"1");symlinkSync("real",join(root,"alias"));
 const paths=["alias/x.json","real/x.json","other/y.json"],directory=join(sandbox,"records"),{client,report}=journal(),environment={};
 const cold=await openJournalVouchV1(root,"c","k",environment,undefined,{client,directory});
 expect(cold.mode).toBe("cursor");
 expect([...await cold.settled(paths)].sort(),"first look").toEqual(["other/y.json","real/x.json"]);
 await cold.seal();
 expect(readVouchSealV1(directory,"c","k")?.linked).toEqual(["alias"]);
 report(["m",0,"real/x.json"]);
 const warm=await openJournalVouchV1(root,"c","k",environment,undefined,{client,directory});
 expect(warm.mode).toBe("journal");
 expect([...warm.skippable(paths)].sort(),"sealed record").toEqual(["other/y.json"]);
 expect([...await warm.settled(paths)].sort(),"second look").toEqual(["other/y.json"]);
 await warm.seal();
 const again=await openJournalVouchV1(root,"c","k",environment,undefined,{client,directory});
 expect([...again.skippable(paths)].sort(),"later look: the file verified at the new seal is vouched again, its alias never").toEqual(["other/y.json","real/x.json"]);
 expect(formatChangeCursorV1((await client.cursor()) as ChangeCursorV1).length).toBeGreaterThan(0);
});

test("a record sealed without a link list is no record and the verification starts from a fresh cursor",async()=>{
 const root=join(sandbox,"ws2");mkdirSync(root,{recursive:true});
 const directory=join(sandbox,"records2"),{client}=journal();
 mkdirSync(directory,{recursive:true});
 const {vouchFileV1}=await import("../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🟦️.ts");
 writeFileSync(join(directory,vouchFileV1("c")),JSON.stringify({version:1,consumer:"c",key:"k",cursor:formatChangeCursorV1({epoch:EPOCH,seq:0})}));
 const vouch=await openJournalVouchV1(root,"c","k",{},undefined,{client,directory});
 expect(vouch.mode).toBe("cursor");
});
