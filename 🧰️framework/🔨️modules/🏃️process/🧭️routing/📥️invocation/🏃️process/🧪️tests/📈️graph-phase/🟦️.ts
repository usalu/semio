/** 📈️ Contract: graph phase counters are rendered in English and German exactly as the independent Python label model renders them, and every malformed counter stays silent. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import progressSchema from "../../🧬️schema/📈️progress.json";
import {formatScriptProcessActivityV1} from "../../🟦️.ts";
import type {ScriptProgress} from "../../../🟦️.ts";

const progress=(row:{command:string;resource:string;mode:string;elapsedMs:number}):ScriptProgress=>({owner:"graph-phase-fixture",command:row.command,stage:"running",activity:{resource:row.resource,mode:row.mode as "shared"|"exclusive",elapsedMs:row.elapsedMs}});

test("the fixture is closed under its schema and the declared labels equal the model labels",()=>{
 expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);
 expect(progressSchema.const.graphPhase.labels).toEqual(fixture.labels);
 expect(progressSchema.const.graphPhase.resourcePrefix).toBe("build:graph-phase:");
});
for(const row of fixture.cases)test("rendered: "+row.name,()=>{expect(formatScriptProcessActivityV1(progress(row),row.environment)).toBe(row.expected);});
for(const row of fixture.rejected)test("silent: "+row.name,()=>{expect(formatScriptProcessActivityV1(progress(row),row.environment)).toBeNull();});
test("the python model renders the same text",()=>{
 const python=testPythonV1();if(!python)return;
 const script="import json,sys\nfixture=json.load(open(sys.argv[1],encoding='utf8'))\nfor row in fixture['cases']:\n    language='de' if row['environment']['LANG'].startswith('de') else 'en'\n    labels=fixture['labels'][language]\n    _,_,phase,done,total=row['resource'].split(':')\n    print(labels['template'].replace('{command}',row['command']).replace('{phase}',labels['phases'][phase]['label']).replace('{done}',done).replace('{total}',total).replace('{units}',labels['phases'][phase]['units']))";
 const result=Bun.spawnSync([python,"-I","-c",script,fileURLToPath(new URL("./🔣️.json",import.meta.url))]);
 expect(result.stdout.toString().trim().split("\n")).toEqual(fixture.cases.map(row=>row.expected));
});
