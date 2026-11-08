import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import law from "./failure.json";
import schema from "./failure-schema.json";
test("native receiving retains all independent phase causes and original enclosing progress",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{db.run("create table causes(position integer primary key,cause text)");for(const[position,cause]of law.causes.entries())db.run("insert into causes values(?,?)",position,cause);expect(db.query("select count(*) causes,count(distinct cause) distinctCauses from causes").get()).toEqual({causes:6,distinctCauses:6});expect(JSON.parse((db.query("select json_group_array(cause) retained from (select cause from causes order by position)").get()as{retained:string}).retained)).toEqual(law.causes);expect(law.variants).toHaveLength(4);expect(law.wrappingAllocations).toBe(0)}finally{db.close()}});
