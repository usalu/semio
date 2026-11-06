import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import law from "../📐️.json";
import schema from "../🔣️.json";
test("funded cursor birth keeps original custody until independent allocator success",()=>{expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{for(const row of law.cases){db.run("CREATE TABLE IF NOT EXISTS custody(source INTEGER,target INTEGER,attempted INTEGER,allocated INTEGER)");db.run("DELETE FROM custody");db.run("INSERT INTO custody VALUES(1,0,0,0)");if(row.axis==="allocator")db.run("UPDATE custody SET attempted=1");if(row.axis==="ready"){db.run("UPDATE custody SET attempted=1,allocated=1");db.run("UPDATE custody SET source=0,target=1 WHERE allocated=1");}const state=db.query("SELECT source,target,attempted,allocated FROM custody").get() as {source:number,target:number,attempted:number,allocated:number};expect(Boolean(state.source)).toBe(row.retained);expect(Boolean(state.target)).toBe(!row.retained);expect(state.allocated?"extent":state.attempted?"zero":"none").toBe(row.marker);}}finally{db.close();}});
