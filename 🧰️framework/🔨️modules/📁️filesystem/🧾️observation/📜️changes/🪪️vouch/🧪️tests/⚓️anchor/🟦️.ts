import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {mergeChangeRowsV1,type ChangeCursorV1,type ChangeRowV1} from "../../../🟦️.ts";
import type {ChangeAnswerV1,ChangeJournalClientV1} from "../../../🧭️client/🟦️.ts";
import {openVouchV1} from "../../🟦️.ts";

const EPOCH="0123456789abcdef";
let sandbox="";
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"rev4-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));
function fakeJournal(){
 const log:ChangeRowV1[][]=[];
 const cursor=():ChangeCursorV1=>({epoch:EPOCH,seq:log.length});
 const client:ChangeJournalClientV1={
  excluder:()=>()=>false,
  async cursor(){return cursor();},
  async since(from):Promise<ChangeAnswerV1>{return{exact:true,cursor:cursor(),rows:mergeChangeRowsV1(log.slice(from.seq).flat())};},
 };
 return{client,change:(...rows:ChangeRowV1[])=>{log.push(rows);}};
}
test("[rev4] a change between the two fences of ONE recheck must not be sealed as verified",async()=>{
 const directory=join(sandbox,"d");mkdirSync(directory,{recursive:true});
 const journal=fakeJournal(),inputs=["pkg/package.json"],frontier=["pkg","."].filter(p=>p!==".");
 const first=await openVouchV1({client:journal.client,directory,consumer:"c",key:"k",insensitive:false});
 expect(first.mode).toBe("cursor");
 const known=await first.settled(inputs);
 expect([...known]).toEqual(["pkg/package.json"]);
 journal.change(["m",0,"pkg/package.json"]);
 await first.settled(frontier);
 await first.seal();
 const next=await openVouchV1({client:journal.client,directory,consumer:"c",key:"k",insensitive:false});
 expect(next.mode).toBe("journal");
 expect([...next.skippable(inputs)]).toEqual([]);
});
