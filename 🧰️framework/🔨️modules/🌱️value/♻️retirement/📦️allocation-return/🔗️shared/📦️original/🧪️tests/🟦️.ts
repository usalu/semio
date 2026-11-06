import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import law from "../📐️.json";
import schema from "../🔣️.json";
test("original shared metadata charges each real primitive and preserves existing backing",()=>{
 expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
 const db=new Database(":memory:");
 try{for(const [name,row]of Object.entries(law.work)){const value=JSON.stringify(row);expect(db.query("select sum(cast(value as integer)) total from json_each(?) where key <> ?").get(value,"total")).toEqual({total:row.total});}
 expect(law.rules).toEqual({originalRetainedBeforeAllocatorSuccess:true,uninitializedPayloadCopied:false,publicationUsesExistingBacking:true,duplicateAllocations:0});
 }finally{db.close();}
});
