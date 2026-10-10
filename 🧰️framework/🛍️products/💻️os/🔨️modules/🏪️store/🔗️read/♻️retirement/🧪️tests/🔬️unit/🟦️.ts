/** 🧪️ SQL independently checks neutral read-return custody through interrupted closure. */
import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import {OriginalReadRetirement} from "../../🟦️.ts";
import fixture from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
test("original read closure retains refused capabilities and original issuer through interruption",()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");db.run("CREATE TABLE custody(root TEXT,captured INTEGER,returned INTEGER,closed INTEGER)");
 for(const row of fixture.cases){for(const copy of fixture.copyGrants){db.run("DELETE FROM custody");db.run("INSERT INTO custody VALUES(?,1,0,0)",[fixture.owner]);const original={root:fixture.owner};let busy=true,returnedEmpty=false;const retired=new OriginalReadRetirement({value:()=>original,tryReturn(){if(busy)return undefined;db.run("UPDATE custody SET captured=0,returned=1");return{closeStep(items){if(items<1)return false;returnedEmpty=true;db.run("UPDATE custody SET returned=0,closed=1");return true;},terminalIsEmpty:()=>returnedEmpty};}});expect(retired.closeStep(0)).toBe(false);expect(retired.closeStep(1)).toBe(false);expect(retired.originalValue()).toBe(original);expect(db.query("SELECT captured,returned,closed FROM custody").get()).toEqual({captured:1,returned:0,closed:0});busy=false;for(let turn=0;turn<row.interruptedAfter;turn++)retired.closeStep(copy);for(let turn=0;turn<fixture.maximumTurns&&!retired.terminalIsEmpty();turn++){expect(retired.closeStep(0)).toBe(false);retired.closeStep(copy);}expect(retired.terminalIsEmpty()).toBe(true);expect(retired.originalValue()).toBeUndefined();expect(db.query("SELECT root,captured,returned,closed FROM custody").get()).toEqual({root:fixture.owner,captured:0,returned:0,closed:1});console.log(`[DEBUG] Original read kind=${row.kind} interrupted=${row.interruptedAfter} grant=${copy} originalIssuer=true terminal=true`);}}
 db.close();
});
