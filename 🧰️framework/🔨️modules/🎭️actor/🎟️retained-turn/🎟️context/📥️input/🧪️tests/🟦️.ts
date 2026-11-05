import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import corpus from"../🔣️.json";
import schema from"../🧬️.json";
test("actor input disposition preserves the actual returned wire independently of readiness",()=>{const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");for(const row of corpus.cases){const shape={disposition:row.disposition,returnedWire:row.returnedWire};expect(Boolean(validate(shape))).toBe(row.valid);const sql=db.query("select(json_extract(?,'$.disposition')='returned')=(json_type(?,'$.returnedWire')='array') valid").get(JSON.stringify(shape),JSON.stringify(shape))as{valid:number};expect(Boolean(sql.valid)).toBe(row.valid)}db.close()});
