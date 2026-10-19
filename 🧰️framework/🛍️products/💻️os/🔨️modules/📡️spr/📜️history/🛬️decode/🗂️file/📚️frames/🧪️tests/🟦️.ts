import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{existsSync,readFileSync}from"node:fs";
import crc from"crc-32/crc32c.js";
import {createRequire} from "node:module";
/** 🧪️ Only this first-party test port observes the independent LEB128 reference implementation. */
interface Leb128Oracle {encodeU32(value:number):number[];decodeUInt32(bytes:Uint8Array):{value:number;nextIndex:number}}
const leb=createRequire(import.meta.url)("@webassemblyjs/leb128") as Leb128Oracle;
import schema from"../../🧬️schema/🔣️.json";
import law from"../../🧪️tests/🧫️fixtures/🔣️.json";
test("original verified source frame ranges preserve exact retained source coordinates",()=>{
 expect(new Ajv({strict:false}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");let position=32;
 try{db.exec("CREATE TABLE original_frame(start INTEGER,end INTEGER,payload_start INTEGER,payload_end INTEGER,kind INTEGER,flags INTEGER)");
 for(const row of law.records){const body=Buffer.concat([Buffer.from([row.kind,row.flags]),Buffer.from(row.payloadHex,"hex")]);const prefix=Buffer.from(leb.encodeU32(body.length));const trailer=Buffer.alloc(8);trailer.writeUInt32LE(crc.buf(body)>>>0);trailer.writeUInt32LE(prefix.length+body.length+8,4);const frame=Buffer.concat([prefix,body,trailer]);const length=leb.decodeUInt32(frame,0).value;const payloadStart=position+prefix.length+2,payloadEnd=position+prefix.length+length,end=position+frame.length;
 expect(payloadEnd-payloadStart).toBe(Buffer.from(row.payloadHex,"hex").length);expect(end-payloadEnd).toBe(8);expect(frame.readUInt32LE(frame.length-8)).toBe(crc.buf(body)>>>0);db.query("INSERT INTO original_frame VALUES(?,?,?,?,?,?)").run(position,end,payloadStart,payloadEnd,row.kind,row.flags);position=end;}
 expect(db.query("SELECT count(*) as count,max(end) as end FROM original_frame").get()).toEqual({count:law.records.length,end:position});
 }finally{db.close()}
 const path=new URL("../🦀️.rs",import.meta.url);expect(existsSync(path)).toBe(true);if(!existsSync(path))return;const source=readFileSync(path,"utf8");
 for(const proof of["pub struct RetainedHistorySourceFrames","RetainedSprVerification","HistoryReadLimits","OriginalWireSourceIdentity","ledger_identity","retirement_recipient_identity","pub fn take_frame_into","pub fn close_step"]){expect(source.includes(proof)).toBe(true)}
 for(const forbidden of["RetainedSprLimits::default","source.to_vec()","FrameCursor","DictReader"]){expect(source.includes(forbidden)).toBe(false)}
});
