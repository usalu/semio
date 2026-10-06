/** 🛬️ Neutral concrete Semio owner inventory and independent coordinate cardinality. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import fixture from "./🧫️fixtures/🔣️.json";


test("Semio concrete controlled native owners have independent unique coordinates and both encodings",()=>{const d=new Database(":memory:");try{const result=d.query("SELECT count(*) AS count,count(DISTINCT value) AS unique_count FROM json_each(?)").get(JSON.stringify(fixture.owners));expect(result).toEqual({count:19,unique_count:19});expect(d.query("SELECT count(*) AS count FROM json_each(?) owners CROSS JOIN json_each(?) encodings").get(JSON.stringify(fixture.owners),JSON.stringify(fixture.encodings))).toEqual({count:38});const names=d.query("SELECT value FROM json_each(?) ORDER BY key").all(JSON.stringify(fixture.owners)) as {value:string}[];expect(names.map(row=>row.value)).toEqual(fixture.owners);}finally{d.close();}});
