import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "../../../../../🧩️composition/📨️emission/🧪️tests/🧫️fixtures/🔣️.json";
import schema from "./refusal.schema.json";
import factorySchema from "./factory.schema.json";
import handoffSchema from "./handoff.schema.json";

test("child retirement keeps independent causes and every owned prose byte",()=>{
 const law=fixture.ownedPreparation.retirementRefusal;
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try{
  const row=db.query("SELECT length(CAST(? AS BLOB))+length(CAST(? AS BLOB)) AS ownedCauseBytes, ? != ? AS distinctKinds").get(law.message,law.workMessage,law.kind,law.workKind);
  expect(row).toEqual({ownedCauseBytes:law.ownedCauseBytes,distinctKinds:1});
  expect(Buffer.byteLength(law.message)+Buffer.byteLength(law.workMessage)).toBe(law.ownedCauseBytes);
 }finally{db.close();}
});

test("factory refusal keeps the source until retry transfers its one original operation",()=>{
 const law=fixture.factoryAdmissionRefusal;
 expect(new Ajv({strict:true}).compile(factorySchema)(law)).toBe(true);
 const db=new Database(":memory:");
 try{
  const row=db.query("WITH attempts(number,source,provider,status) AS (VALUES(1,17,23,'refused'),(2,17,23,'accepted')) SELECT count(*) AS attemptsAfterRetry, min(number) AS attemptsBeforeRefusal, count(DISTINCT source)=1 AS sameSource, count(DISTINCT provider)=1 AS sameProvider, sum(status='accepted') AS retiredOperations FROM attempts").get();
  expect(row).toEqual({attemptsBeforeRefusal:law.attemptsBeforeRefusal,attemptsAfterRetry:law.attemptsAfterRetry,sameSource:Number(law.sameSource),sameProvider:Number(law.sameProvider),retiredOperations:law.retiredOperations});
  expect(law.ownedCauseBytes).toBe(fixture.ownedPreparation.retirementRefusal.ownedCauseBytes);
  expect(db.query("SELECT length(CAST(? AS BLOB))+length(CAST(? AS BLOB))+2+1 AS work").get(fixture.ownedPreparation.retirementRefusal.message,fixture.ownedPreparation.retirementRefusal.workMessage)).toEqual({work:law.attemptWork});
 }finally{db.close();}
});

test("provider handback quotes both original and receiving native headers",()=>{
 const law=fixture.providerHandback;expect(new Ajv({strict:true}).compile(handoffSchema)(law)).toBe(true);const db=new Database(":memory:");
 try{expect(db.query("WITH words(bytes) AS (VALUES(4),(8)) SELECT bytes AS nativeWordBytes, bytes*?*? AS copyBytes FROM words ORDER BY bytes").all(law.headers,law.pointerWords)).toEqual([{nativeWordBytes:4,copyBytes:16},{nativeWordBytes:8,copyBytes:32}]);expect(law.deniedAxes).toEqual(["items","copy","depth"]);expect(law.work).toBe(1);}finally{db.close();}
});
