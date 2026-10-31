import AjvDraft07 from "ajv";
/** 📏️ Checks canonical UTF8 and packed generation ceilings against portable vectors and independent SQLite JSON. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {brotliDecompressSync} from "node:zlib";
import Ajv2020 from "ajv/dist/2020";
import {GENERATION_POLICY_V1,canonicalJsonV1,packGenerationDocumentV1} from "../../🟨️.mjs";
import fixture from "../🔣️.json";
import fixtureSchema from "../🧬️schema/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";

for(const row of fixture.documentSizes)test("stored generation document byte boundary: "+row.name,()=>{
 const database=new Database(":memory:");
 try{const oracle=database.query("SELECT length(CAST(json(?) AS BLOB)) AS bytes").get(JSON.stringify(row.value)) as {bytes:number};expect(oracle.bytes).toBe(row.expandedBytes);}finally{database.close();}
 const canonical=canonicalJsonV1(row.value);expect(Buffer.byteLength(canonical)).toBe(row.expandedBytes);
 if(row.refusal){expect(()=>packGenerationDocumentV1(row.value,row.maximumBytes)).toThrow("Stored document exceeds its finite limit");return;}
 const packed=packGenerationDocumentV1(row.value,row.maximumBytes);expect(packed.byteLength).toBeLessThanOrEqual(row.maximumBytes);
 expect(brotliDecompressSync(packed,{maxOutputLength:row.maximumBytes}).toString("utf8")).toBe(canonical);
});

test("generation document limits cannot exceed the declared reader ceiling",()=>{
 expect(new Ajv2020({strict:true}).validate(fixtureSchema,fixture)).toBe(true);
 expect(schema.definitions.DocumentByteLimit.maximum).toBe(GENERATION_POLICY_V1.maximumIndexBytes);
 const validate=new AjvDraft07({strict:true}).compile(schema.definitions.DocumentByteLimit);
 for(const maximum of[1,GENERATION_POLICY_V1.maximumIndexBytes])expect(validate(maximum)).toBe(true);
 for(const maximum of[0,-1,0.5,NaN,Infinity,Number.MAX_SAFE_INTEGER,GENERATION_POLICY_V1.maximumIndexBytes+1]){expect(validate(maximum)).toBe(false);expect(()=>packGenerationDocumentV1(null,maximum)).toThrow("Finite generation document byte limit required");}
});
