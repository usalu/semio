import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {NativeDecodeControl} from "../../🟦️.ts";
import type {NativeDecodeCloseControl,NativeRetirementDecodeControl} from "../../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("original cancelled decoder lends only paid retirement with the same cumulative ledger",async()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,policy:"cancel-original-retirement"})).toBe(false);
 let cancelled=false,calls=0;const original=new NativeDecodeControl(128,()=>{calls++;return!cancelled;});await original.charge(fixture.originalBytes);const receivingClose:NativeDecodeCloseControl=original;expect(receivingClose.ledgerIdentity).toBe(original);expect(receivingClose.ownedBytes).toBe(fixture.originalBytes);cancelled=true;await expect(original.checkpoint()).rejects.toMatchObject({kind:"canceled"});
 const capacity=fixture.charges.reduce((a,b)=>a+b,0),grant={maximumItems:1,maximumCopyBytes:fixture.copyQuantum,maximumCapacityBytes:capacity,maximumReleaseBytes:65538,maximumDepth:64};let retained:NativeRetirementDecodeControl|undefined;
 const db=new Database(":memory:");try{db.exec("create table charges(amount integer)");for(const amount of fixture.charges)db.query("insert into charges values(?)").run(amount);const expected=db.query("select ?+sum(amount) as owned from charges").get(fixture.originalBytes)as{owned:number};
  await original.withRetirement(grant,async loan=>{retained=loan;const close:NativeDecodeCloseControl=loan;expect(close.ledgerIdentity).toBe(original.ledgerIdentity);expect("checkpoint" in close).toBe(false);expect("withRetirement" in close).toBe(false);await expect(original.charge(0)).rejects.toMatchObject({kind:"invariantViolated"});for(const amount of fixture.charges)await loan.charge(amount);expect(loan.ownedBytes).toBe(expected.owned);const before=calls;grant.maximumCapacityBytes++;await expect(loan.charge(1)).rejects.toMatchObject({kind:"ownershipLimit"});expect(calls).toBe(before);expect(loan.ownedBytes).toBe(expected.owned);});expect(original.ownedBytes).toBe(fixture.expectedBytes);await expect(retained!.charge(0)).rejects.toMatchObject({kind:"invariantViolated"});await expect(original.checkpoint()).rejects.toMatchObject({kind:"canceled"});
 }finally{db.close();}
});
