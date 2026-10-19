import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import {NativeDecodeControl} from "@semio-tech/value-ts/decode";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

test("original IO retirement neutral receipts preserve the cancelled original ledger and SQLite wallet",async()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,initialOwnedBytes:0})).toBe(false);
 let live=true;const control=new NativeDecodeControl(64,()=>live);await control.charge(fixture.initialOwnedBytes);live=false;await expect(control.checkpoint()).rejects.toMatchObject({kind:"canceled"});
 const database=new Database(":memory:");try{database.exec("create table receipt(items integer,copy integer,capacity integer,release integer)");let owned=fixture.initialOwnedBytes;for(let turn=0;turn<fixture.turns;turn++){const receipt=fixture.receipt;const grant={maximumItems:receipt.copiedItems,maximumCopyBytes:receipt.copiedBytes,maximumCapacityBytes:receipt.retainedCapacityBytes,maximumReleaseBytes:receipt.releasedBytes,maximumDepth:fixture.grant.maximumDepth};await control.withRetirement(grant,async original=>{expect(original.ledgerIdentity).toBe(control);await original.charge(receipt.retainedCapacityBytes);owned+=receipt.retainedCapacityBytes;expect(original.ownedBytes).toBe(owned);});database.query("insert into receipt values(?,?,?,?)").run(receipt.copiedItems,receipt.copiedBytes,receipt.retainedCapacityBytes,receipt.releasedBytes);const remaining=database.query("select ?-sum(items) as items,?-sum(copy) as copy,?-sum(capacity) as capacity,?-sum(release) as release from receipt").get(fixture.grant.maximumItems,fixture.grant.maximumCopyBytes,fixture.grant.maximumCapacityBytes,fixture.grant.maximumReleaseBytes)as Record<string,number>;for(const [axis,value]of Object.entries(remaining))expect(value).toBe((fixture.turns-turn-1)*({items:receipt.copiedItems,copy:receipt.copiedBytes,capacity:receipt.retainedCapacityBytes,release:receipt.releasedBytes}[axis]??NaN));}expect(control.ownedBytes).toBe(fixture.expectedOwnedBytes);expect(live).toBe(false);await expect(control.checkpoint()).rejects.toMatchObject({kind:"canceled"});}finally{database.close();}
 console.log("[DEBUG] original IO neutral receipt contract: strict schema, actual portable retirement decoder, independent SQLite wallet, cumulative11, semantic cancellation retained; native IO admission tested separately");
});
