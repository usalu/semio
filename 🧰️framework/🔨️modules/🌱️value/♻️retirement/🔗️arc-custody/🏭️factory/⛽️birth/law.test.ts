import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./schema.json";import law from "./fixture.json";
test("original shared birth preserves explicit action work",()=>{expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{const row=db.query("select ?+?+? success, ?+?+?+?+? allocationRefusal, 0 denied").get(law.slotTransfer,law.cloneAlias,law.allocation,law.slotTransfer,law.cloneAlias,law.allocation,law.restoreSlot,law.releaseAlias);expect(row).toEqual({success:law.success,allocationRefusal:law.allocationRefusal,denied:law.denied})}finally{db.close()}});
