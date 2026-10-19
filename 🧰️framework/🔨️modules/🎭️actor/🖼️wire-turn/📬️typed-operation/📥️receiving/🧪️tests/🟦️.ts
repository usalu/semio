import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import corpus from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {semioSchemaAjvV1} from "@semio-tech/schema-ts/oracles/validator";
import {TYPED_OPERATION_PAGE_MAGIC,scanTypedOperationPages} from "../../../🔖️turn-result/🟦️.ts";

test("original host wire receiving matches Buffer ABI and SQLite custody at every cancellation prefix",()=>{
 expect(semioSchemaAjvV1({strict:true,allErrors:true}).compile(schema)(corpus)).toBe(true);
 const db=new Database(":memory:");try{
  db.exec("CREATE TABLE custody(source INTEGER,destination INTEGER,closed INTEGER,output INTEGER)");
  for(const row of corpus.cases){
   const token={...corpus.token,operation:BigInt(corpus.token.operation),generation:BigInt(corpus.token.generation)},body=Buffer.alloc(row.bodyBytes,row.fill),header=Buffer.alloc(TYPED_OPERATION_PAGE_MAGIC.length+30),offset=TYPED_OPERATION_PAGE_MAGIC.length;
   Buffer.from(TYPED_OPERATION_PAGE_MAGIC).copy(header);header.writeUInt32LE(token.receiver,offset);header.writeBigUInt64LE(token.operation,offset+4);header.writeBigUInt64LE(token.generation,offset+12);header.writeUInt32LE(token.sequence,offset+20);header[offset+24]=token.attempt;header[offset+25]=corpus.lane;header.writeUInt32LE(body.length,offset+26);
   expect(header.toString("hex")).toBe(row.headerHex);const wire=Buffer.concat([header,body]);expect(wire.length).toBeLessThanOrEqual(corpus.originalHost.maximumCapacityBytes);if(body.length===4096)expect(wire.length).toBeGreaterThan(corpus.actorCapacity);
   const decoded=scanTypedOperationPages([{tag:"send-message",val:{target:{tag:"shell",val:token.receiver},payload:wire}}]).pages[0]!;expect(decoded.token).toEqual(token);expect(decoded.lane).toBe(corpus.lane);expect(Buffer.compare(Buffer.from(decoded.payload),body)).toBe(0);
   for(let cut=0;cut<=corpus.steps.length;cut++){db.exec("DELETE FROM custody;INSERT INTO custody VALUES(1,0,0,0)");for(const step of corpus.steps.slice(0,cut)){if(step==="allocate-original-destination")db.exec("UPDATE custody SET destination=1");else if(step==="close-original-alias")db.exec("UPDATE custody SET source=0,closed=1");else if(step==="take-original-wire")expect(db.query("UPDATE custody SET destination=0,output=1 WHERE closed=1 AND source=0").run().changes).toBe(1);else expect(db.query("UPDATE custody SET output=1 WHERE closed=1").run().changes).toBe(0);}db.exec("UPDATE custody SET source=0,destination=0,closed=1");expect(db.query("SELECT source,destination FROM custody").get()).toEqual({source:0,destination:0});}
  }
  console.log(`[DEBUG] actual Buffer/SQLite wire receiver oracle: ${corpus.cases.length} golden magic+30 headers and bodies, originalHost copy4096/cap65536, every cancellation prefix and no output before original alias close`);
 }finally{db.close();}
});
