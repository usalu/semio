import {expect,test} from "bun:test";
import {ValueError} from "../../../../🌱️value/⚠️refusal/🟦️.ts";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./close-failure.schema.json";
import cases from "./close-failure.json";
/** ⚠️ A Source close keeps cancellation and retirement causes distinct. */
test("Source close preserves each original cause class",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE cause(kind TEXT PRIMARY KEY, detail TEXT NOT NULL)");for(const row of cases)db.run("INSERT INTO cause VALUES (?,?)",[row.kind,row.detail]);const rows=db.query("SELECT kind,detail FROM cause ORDER BY kind").all();expect(rows).toEqual([...cases].sort((a,b)=>a.kind.localeCompare(b.kind)))}finally{db.close()}});

import {faultFromOriginalTurnFailure} from "../../../⚠️failure/🟦️.ts";
test("Source close wire retains the original nested cause",()=>{const progress={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};const original={kind:"cancellationBlocked" as const,original:"weakAlias" as const};const fault=faultFromOriginalTurnFailure({effects:progress,primary:{kind:"sourceClose",sourceClose:original}});expect(fault.code).toBe("original.sourceClose");expect(fault.domain).toEqual({kind:"object",members:[{name:"kind",value:{kind:"text",value:"cancellationBlocked"}},{name:"original",value:{kind:"text",value:"weakAlias"}}]});});

test("receiving Work overrun remains a separate original field",()=>{const progress={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};const overrun=new ValueError("workLimit","original overrun").withRetainedProgress(progress);const fault=faultFromOriginalTurnFailure({effects:progress,primary:{kind:"receiving",receiving:{workOverrun:overrun,retainedProgress:progress}}});expect(fault.message).toBe("original overrun");expect(fault.domain?.kind).toBe("object");if(fault.domain?.kind!=="object")throw new Error("missing original domain");expect(fault.domain.members.find(member=>member.name==="workOverrun")?.value).toMatchObject({kind:"object"});});
