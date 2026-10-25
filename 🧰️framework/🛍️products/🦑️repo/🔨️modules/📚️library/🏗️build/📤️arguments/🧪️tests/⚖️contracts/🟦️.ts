/** ⚖️ Contract: the owner contract validator of the build equals the authored verdicts in `./🔣️.json`, an independent Python derivation of the same JSON Schema subset and, except on rows marked as deliberately stricter, Ajv; it compares objects by content, counts string length in code points and admits nothing it does not understand. */
import {expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {conformsOwnerContractV1} from "../../🟨️.mjs";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ajv=new Ajv({strict:false,ownProperties:true}),cases=corpus.contracts.flatMap(row=>row.values.map(entry=>({row,entry})));

test("the corpus is closed by its schema",()=>{
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,contracts:[{...corpus.contracts[0],values:[{value:1}]}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,contracts:[{...corpus.contracts[0],values:[{value:1,accepted:true,ajv:"same"}]}]})).toBe(false);
});

test("every authored verdict is what the validator answers",()=>{
 for(const{row,entry}of cases)expect(conformsOwnerContractV1(row.contract,entry.value),row.name+" "+JSON.stringify(entry.value)).toBe(entry.accepted);
});

test("an independent Python derivation answers the same verdicts",()=>{
 const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({cases:cases.map(({row,entry})=>({contract:row.contract,value:entry.value}))}),encoding:"utf8",maxBuffer:1<<26});
 expect(run.status,run.stderr).toBe(0);
 const answers=JSON.parse(run.stdout);expect(answers.length).toBe(cases.length);
 for(const[index,{row,entry}]of cases.entries())expect(answers[index],row.name+" "+JSON.stringify(entry.value)).toBe(entry.accepted);
});

test("Ajv agrees on every row that is not marked as deliberately stricter and disagrees on every marked row",()=>{
 let marked=0;
 for(const row of corpus.contracts){
  const validate=ajv.compile(row.contract);
  for(const entry of row.values){
   const verdict=validate(entry.value);
   if(Reflect.get(entry,"ajv")==="differs"){marked++;expect(verdict,row.name+" "+JSON.stringify(entry.value)).toBe(!entry.accepted);}
   else expect(verdict,row.name+" "+JSON.stringify(entry.value)).toBe(entry.accepted);
  }
 }
 expect(marked).toBeGreaterThan(0);
});
