import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import Ajv from "ajv/dist/2020";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};

test("retained content fingerprint corpus matches independent system SHA256",()=>{
 const validate=new Ajv({strict:false}).compile(schema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 for(const row of fixture.cases){const bytes=Buffer.from(row.value,row.encoding==="hex"?"hex":"utf8"),hash=createHash("sha256");for(let i=0;i<row.repeat;i++)hash.update(bytes);expect(bytes.length*row.repeat,row.name).toBe(row.bytes);expect(hash.digest("hex"),row.name).toBe(row.sha256);}
});
