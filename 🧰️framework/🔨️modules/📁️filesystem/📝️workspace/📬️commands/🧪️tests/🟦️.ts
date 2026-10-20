import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};

test("filesystem command worker retains bounded progress and exact domain dispatch",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8"),db=new Database(":memory:");
 try{for(const row of fixture.cases){const result=db.query("select case when ?1 then 'Cancelled' when ?2='list' then 'List' when ?2='load' then 'Load' else 'RecoveryList' end outcome").get(row.cancelled?1:0,row.kind)as{outcome:string};expect(result.outcome,row.id).toBe(row.outcome);}
 const saturation=db.query("select min(?1,?2) queued,max(0,?1-?2) dropped").get(fixture.saturation.publishes,fixture.messageCapacity)as{queued:number;dropped:number};expect(saturation.queued).toBe(fixture.saturation.queued);expect(saturation.dropped).toBe(fixture.saturation.dropped);expect(fixture.saturation.wakes).toBe(saturation.queued);
 const policy=schema.$defs.Policy.const;expect(source).toContain("pub const MESSAGE_CAPACITY: usize = "+policy.messageCapacity+";");expect(source).toContain("mpsc::sync_channel(MESSAGE_CAPACITY)");expect(source).toContain("send.try_send(Message::Progress(progress))");expect(source).toContain("send.send(Message::Finished(result))");
 for(const method of ["list","load","list_recovery","load_recovery","save","inspect_entry","create_file","create_directory","rename_entry","retain_entry"])expect(source).toContain("workspace."+method+"(");
 }finally{db.close();}
});

import "../📝️buffer/🧪️tests/🟦️.ts";

import "../🗂️browser/🧪️tests/🟦️.ts";
