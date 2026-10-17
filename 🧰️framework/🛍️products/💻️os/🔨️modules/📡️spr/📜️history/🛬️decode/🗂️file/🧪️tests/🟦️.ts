import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {existsSync,readFileSync} from "node:fs";
import crc from "crc-32/crc32c.js";
import * as leb from "@webassemblyjs/leb128";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
test("original History source file frames receive actual typed indexes without copied edit identities",()=>{
 expect(new Ajv({strict:false}).compile(schema)(law)).toBe(true);expect(crc.buf(Buffer.from("123456789"))>>>0).toBe(0xe3069283);
 const frames=law.records.map(row=>{const body=Buffer.concat([Buffer.from([row.kind,row.flags]),Buffer.from(row.payloadHex,"hex")]);const prefix=Buffer.from(leb.encodeU32(body.length));const tail=Buffer.alloc(8);tail.writeUInt32LE(crc.buf(body)>>>0);tail.writeUInt32LE(prefix.length+body.length+8,4);return Buffer.concat([prefix,body,tail]);});
 const bytes=Buffer.concat([Buffer.from(law.headerHex,"hex"),...frames]);const db=new Database(":memory:");
 try{db.exec("CREATE TABLE frame(position INTEGER PRIMARY KEY,kind INTEGER,body TEXT)");let position=32;for(let i=0;i<frames.length;i++){const original=frames[i];let at=position,size=0,shift=0;for(;;){const byte=bytes[at++];size+=(byte&127)*2**shift;if(!(byte&128))break;shift+=7;}expect(bytes[at]).toBe(law.records[i].kind);expect(bytes[at+1]).toBe(law.records[i].flags);expect(bytes.subarray(at+2,at+size).toString("hex")).toBe(law.records[i].payloadHex);expect(bytes.readUInt32LE(at+size)).toBe(crc.buf(bytes.subarray(at,at+size))>>>0);expect(bytes.readUInt32LE(at+size+4)).toBe(original.length);db.query("INSERT INTO frame VALUES(?,?,?)").run(position,bytes[at],bytes.subarray(at+2,at+size).toString("hex"));position=at+size+8;}expect(position).toBe(bytes.length);expect(db.query("SELECT count(*) as count FROM frame").get()).toEqual({count:law.records.length});
  const prior:string[]=[];for(const row of law.records.filter(row=>row.kind===4)){const input=Buffer.from(row.payloadHex,"hex");let at=0;const scalar=()=>input[at++];const raw=()=>{const size=scalar(),text=input.subarray(at,at+size).toString("utf8");at+=size;return text};const id=()=>{const tag=input[at++];return tag===3?prior[scalar()]:(expect(tag).toBe(0),raw())};expect(input[at++]).toBe(2);const presence=input[at++],value=id();expect(input[at++]).toBe(0);raw();const actor=presence&1?id():null;expect(actor).toBe(law.expected.actors[prior.length]);expect(value).toBe(law.expected.editIds[prior.length]);prior.push(value);expect(input[at++]).toBe(0);expect(input[at++]).toBe(1);expect(input[at++]).toBe(2);const count=scalar();expect(input.subarray(at,at+count).toString("hex")).toBe(law.expected.binaryOpsHex[prior.length-1]);at+=count;expect(input[at++]).toBe(0);expect(input[at++]).toBe(0);expect(input[at++]).toBe(law.expected.baseVersions[prior.length-1]);expect(input[at++]).toBe(0);expect(at).toBe(input.length);}
 }finally{db.close()}
 const path=new URL("../🦀️.rs",import.meta.url);expect(existsSync(path)).toBe(true);if(!existsSync(path))return;const source=readFileSync(path,"utf8");
 for(const proof of["RetainedHistorySourceFrames","RetainedHistoryTransitionPayloadValidation","RetainedHistoryEditDecode","RetainedHistoryTransitionDecode","RetainedHistoryIdentityRecordDecode","RetainedHistoryConflictDecode","RetainedSourceDictionary","RetainedHistoryRecords","HistoryReadLimits","OriginalWireSourceIdentity","pub fn take_ready_into","pub fn take_partial_into","control.charge"]){expect(source.includes(proof)).toBe(true)}
 for(const cold of["decode_frame(","decode_edit(","decode_conflicts(","edit_ids:Vec<String>","edit.id.clone()","source.to_vec()","RetainedSprLimits::default"]){expect(source.includes(cold)).toBe(false)}
});
