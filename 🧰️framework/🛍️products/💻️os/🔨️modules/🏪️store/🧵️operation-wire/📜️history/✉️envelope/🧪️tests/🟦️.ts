import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import {readFileSync} from "node:fs";
const law=JSON.parse(readFileSync(new URL("🔣️.json",import.meta.url),"utf8"));
test("canonical history envelope header agrees with independent SQLite JSON construction",()=>{expect(new Ajv2020({strict:true}).compile(JSON.parse(readFileSync(new URL("../🧬️.json",import.meta.url),"utf8")))(law)).toBe(true);const db=new Database(":memory:");for(const row of law.cases){const result=db.query("SELECT json_object('document',?,'actor',?,'observed',?,'dependencies',json(?),'target',json(?),'schema','semio.history.transition','inverseBytes',0,'transaction',NULL,'verb',NULL,'line',NULL) AS value").get(row.document,row.actor,row.observed,JSON.stringify(row.dependencies),JSON.stringify(row.target))as{value:string};expect(JSON.parse(result.value)).toEqual(row.expected);}db.close();});
