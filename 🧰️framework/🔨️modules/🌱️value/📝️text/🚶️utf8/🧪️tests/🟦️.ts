import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "../../../../🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {RetainedUtf8Cursor,RetainedUtf8Buffer} from "../🟦️.ts";

test("original UTF8 pages and scalar offsets agree with SQLite",()=>{
 expect(semioSchemaAjvV1({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:"),encoder=new TextEncoder(),grant={maximumItems:1,maximumCopyBytes:fixture.bodyBytes,maximumCapacityBytes:65536,maximumReleaseBytes:65536,maximumDepth:8};
 for(const row of fixture.cases){const text=row.text.repeat(row.repeat),bytes=encoder.encode(text);const oracle=db.query<{bytes:number,scalars:number},[string,string]>("SELECT length(CAST(?1 AS BLOB)) bytes,length(?2) scalars").get(text,text)!;expect(oracle).toEqual({bytes:row.bytes,scalars:row.scalars});for(const word of [4,8]as const){const cursor=new RetainedUtf8Cursor(word);while(!cursor.ready){const previous=cursor.byteOffset;expect(cursor.advance(bytes,{...grant,maximumCopyBytes:0}).progress.copiedBytes).toBe(0);expect(cursor.byteOffset).toBe(previous);const step=cursor.advance(bytes,grant);expect(step.progress.copiedBytes).toBeLessThanOrEqual(grant.maximumCopyBytes);expect(step.endByte).toBeGreaterThanOrEqual(step.startByte);}expect(cursor.byteOffset).toBe(oracle.bytes);expect(cursor.scalarOffset).toBe(oracle.scalars);}}
 db.close();console.log("[DEBUG] funded UTF8 byte/scalar pages with explicit4/8-byte metadata match independent SQLite across astral and CRLF sources");
});

test("original UTF8 buffer transfer and cancellation preserve independent work currencies",()=>{
 const db=new Database(":memory:"),encoder=new TextEncoder(),grant={maximumItems:1,maximumCopyBytes:fixture.bodyBytes,maximumCapacityBytes:65536,maximumReleaseBytes:65536,maximumDepth:8};
 for(const row of fixture.cases){const bytes=encoder.encode(row.text.repeat(row.repeat));for(const cut of [...fixture.cancellationCuts,null]){const writer=new RetainedUtf8Buffer(bytes.length);expect(writer.reserve({...grant,maximumItems:0}).copiedItems).toBe(0);const born=writer.reserve(grant);expect(born.retainedCapacityBytes).toBe(bytes.length);let offset=0,turns=0;while(offset<bytes.length&&(cut===null||turns<cut)){const step=writer.append(bytes,offset,grant);expect(step.progress.copiedBytes).toBeLessThanOrEqual(grant.maximumCopyBytes);expect(step.endByte).toBeGreaterThan(offset);offset=step.endByte;turns++;}if(cut===null){const result=writer.take(grant)!;expect(Boolean(db.query<{equal:number},[Uint8Array,Uint8Array]>("SELECT ?1=?2 equal").get(result.bytes,bytes)!.equal)).toBe(true);expect(result.progress.retainedCapacityBytes).toBe(0);expect(writer.close(grant).releasedBytes).toBe(0);}else{expect(writer.close({...grant,maximumReleaseBytes:0,maximumCopyBytes:0}).copiedItems).toBe(0);const closed=writer.close(grant);expect(closed.releasedBytes).toBe(bytes.length);expect(writer.close(grant).releasedBytes).toBe(0);}}}
 db.close();console.log("[DEBUG] bounded original UTF8 allocation/copy/transfer and seven cancellation cuts preserve exact bytes matching SQLite blobs");
});
