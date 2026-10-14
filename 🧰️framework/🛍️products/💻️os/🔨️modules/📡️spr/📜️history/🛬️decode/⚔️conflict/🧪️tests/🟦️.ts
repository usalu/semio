import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {existsSync,readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
test("original conflict receives actors original edit references envelopes and messages without copies of prior identity",()=>{
 expect(new Ajv({strict:false}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");
 try{for(const row of law.conflicts){const source=Buffer.from(row.wireHex,"hex");let at=0;
 const scalar=()=>{let value=0;for(let shift=0;shift<70;shift+=7){const byte=source[at++];value+=(byte&127)*2**shift;if(!(byte&128))return value}throw Error("scalar")};
 const raw=()=>{const size=scalar(),bytes=source.subarray(at,at+size);at+=size;return bytes};
 const id=()=>{const tag=source[at++];if(tag===3)return law.priorEditIds[scalar()];expect(tag).toBe(0);return raw().toString("utf8")};
 const list=<T>(read:()=>T)=>Array.from({length:scalar()},read);
 const actual={id:id(),kind:source[at++],status:source[at++],actors:list(id),hlt:[scalar(),scalar(),scalar()],editIds:list(id),envelopesHex:list(()=>raw().toString("hex")),messages:list(()=>({level:source[at++],code:id(),message:raw().toString("utf8"),target:list(()=>raw().toString("utf8")),opIndex:source[at++]?scalar():null}))};
 expect(actual).toEqual(row.expected);expect(at).toBe(source.length);expect(db.query("select json(?) as value").get(JSON.stringify(actual))).toEqual({value:JSON.stringify(row.expected)});
 }}finally{db.close()}
 console.log("[DEBUG] Original Conflict Buffer/SQLite independent controls agree on all scalar, actor, prior-edit, envelope and typed message fields");
 const path=new URL("../🦀️.rs",import.meta.url);expect(existsSync(path)).toBe(true);if(!existsSync(path))return;const source=readFileSync(path,"utf8");
 for(const proof of["pub struct RetainedHistoryConflictDecode","RetainedHistoryMessageDecode","RetainedHistoryIdDecode","RetainedHistoryRecords","HistoryReadLimits","control.charge","pub fn take_ready_into","pub fn take_partial_into"]){expect(source.includes(proof)).toBe(true)}
 for(const cold of["read_conflict(","source.to_vec()","HistoryReadLimits::default","prior.to_vec()"]){expect(source.includes(cold)).toBe(false)}
});
