import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import law from "./native.json";
import schema from "./native.schema.json";
test("native map clone retains original page rows and cancellation cuts",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{db.run("create table entries(case_size integer, ordinal integer primary key, value integer)");for(const count of law.entryCounts){db.run("delete from entries");for(let ordinal=0;ordinal<count;ordinal++)db.run("insert into entries values(?,?,?)",count,ordinal,ordinal);expect(db.query("select ordinal from entries order by ordinal").all()).toEqual(Array.from({length:count},(_,ordinal)=>({ordinal})));}expect(law.interruptAfter).toEqual(Array.from({length:129},(_,cut)=>cut));expect(law.birthPrefixActions).toBe(11);expect(law.innerTupleSeedParts).toBe(13);expect(law.maximumCopyBytes).toBe(4096)}finally{db.close()}});
