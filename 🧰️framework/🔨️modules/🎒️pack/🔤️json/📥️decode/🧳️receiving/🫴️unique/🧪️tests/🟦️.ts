import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import corpus from "../🔣️.json";
import schema from "../🧬️.json";
test("unique original JSON output retains a complete typed projection and rejects duplicate members",()=>{
 const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");
 for(const row of corpus.cases){const syntax=db.query("select json_valid(?) valid").get(row.wire)as{valid:number};let accepted=false;if(syntax.valid){const duplicate=db.query("select count(*) duplicates from(select key from json_each(?) group by key having count(*)>1)").get(row.wire)as{duplicates:number};accepted=duplicate.duplicates===0&&Boolean(validate(JSON.parse(row.wire)));}expect(accepted,row.name).toBe(row.accepted);}
 expect(corpus.phases.at(-1)).toBe("capture");expect(new Set(corpus.phases).size).toBe(corpus.phases.length);db.close();
});
