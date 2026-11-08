import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "./resident-birth.schema.json";
import fixture from "./resident-birth.json";
test("resident record field owns each seeded native stage",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{const check=db.query<{owners:number},[number,number]>("SELECT ?1+?2 AS owners");for(const stage of fixture.stages){const owners=check.get(Number(stage.initialized),Number(stage.typed))!.owners;expect(owners).toBe(["seeded","typed"].includes(stage.name)?1:0);if(stage.initialized)expect(stage.backing).toBe(true)}}finally{db.close()}});
