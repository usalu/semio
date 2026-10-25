import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("MCP resource templates bind one nonempty identity",()=>{
 const validate=new Ajv({strict:false}).compile(schema);expect(validate(fixture)).toBe(true);
 const db=new Database(":memory:");try{for(const row of fixture.cases){const open=row.template.indexOf("{");const close=row.template.indexOf("}",open);const prefix=row.template.slice(0,open);const suffix=row.template.slice(close+1);const actual=row.uri.startsWith(prefix)&&row.uri.endsWith(suffix)&&row.uri.length>prefix.length+suffix.length;const result=db.query("SELECT substr(?1,1,length(?2))=?2 AND (?3='' OR substr(?1,-length(?3))=?3) AND length(?1)>length(?2)+length(?3) AS matched").get(row.uri,prefix,suffix) as {matched:number};expect(actual).toBe(row.expected);expect(Boolean(result.matched)).toBe(row.expected);}}finally{db.close();}
});
