import { expect, spyOn, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { compileJsonSchemaSubset } from "../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type { ScriptProgress } from "../../../🟦️.ts";
import { createScriptProcessEnvelope, formatScriptProcessActivityV1, requireIssuedScriptInvocationV1, withScriptProcessEnvelope } from "../../🟦️.ts";
import contract from "../../🧬️schema/📈️progress.json";
import fixture from "./🔣️.json";

test("process progress uses neutral long-activity words and preserves the actual structured publisher", async () => {
  const database = new Database(":memory:");
  expect(new Ajv({strict:true}).compile(contract)(contract.const)).toBe(true);
  expect(compileJsonSchemaSubset(contract).validate(contract.const)).toEqual([]);
  try {
    for (const row of fixture.cases) {
      const progress = row.progress as ScriptProgress, activity = progress.activity;
      const oracle = database.query("select case when ?1 is null or ?1<?2 then null else ?3||' remains active on '||?4||' ('||?5||', '||cast(round(?1/1000.0) as integer)||' s)' end message").get(activity?.elapsedMs ?? null, contract.const.minimumElapsedMilliseconds, progress.command, activity?.resource ?? null, activity?.mode ?? null) as {message:string|null};
      expect(oracle.message).toBe(row.expected);
      expect(formatScriptProcessActivityV1(progress)).toBe(row.expected);
      const writes:string[]=[],spy=spyOn(process.stderr,"write").mockImplementation(((text:unknown)=>{writes.push(String(text));return true;}) as typeof process.stderr.write);
      try {
        await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"process-progress-fixture",maximumElapsedMilliseconds:0},{},Date.now()),async invocation=>{
          requireIssuedScriptInvocationV1(invocation);
          await invocation.control.publish(progress);
          await invocation.control.yieldContinuation();
          requireIssuedScriptInvocationV1(invocation);
        });
      } finally { spy.mockRestore(); }
      expect(writes).toEqual([`[script] ${JSON.stringify(progress)}\n`,...(row.expected===null?[]:[`[script] ${row.expected}\n`])]);
    }
  } finally { database.close(); }
  console.log("[DEBUG] Actual Original publisher retains structured events and describes active work and contention with the same neutral words");
});


/** 🌐️ Renders bounded source work in the selected language without replacing its structured event. */
test("source admission work has localized bounded counters and keeps actual progress authority",async()=>{
 const declaration=contract.const.sourceAdmission,database=new Database(":memory:"),{const:_declaration,...caseSchema}=contract,validate=new Ajv({strict:true}).compile({...caseSchema,$ref:"#/definitions/SourceCases"});expect(validate(fixture.sourceCases)).toBe(true);
 try{for(const row of fixture.sourceCases){const progress:ScriptProgress={owner:"process-progress-fixture",command:"compiler",stage:"running",activity:{resource:row.resource,mode:"shared",elapsedMs:0}},payload=row.resource.slice(declaration.resourcePrefix.length);let independent:string|null=null;
 if(row.resource.length<=declaration.maximumResourceCharacters&&(database.query("select json_valid(?1) valid").get(payload)as{valid:number}).valid){const fields=["unit","units","readOperations","files","bytes"],valid=fields.map(name=>`json_type(?1,'$.${name}')='integer' and json_extract(?1,'$.${name}') between ${name==='unit'||name==='units'?1:0} and 9007199254740991`).join(" and "),admitted=database.query("select "+valid+" and json_type(?1,'$.version')='integer' and json_extract(?1,'$.version')=1 and (select count(*) from json_each(?1))=8 and json_extract(?1,'$.phase') in ('cfg','read','complete') and json_type(?1,'$.packageName')='text' and length(json_extract(?1,'$.packageName')) between 1 and 256 and json_extract(?1,'$.packageName') not glob '*[^A-Za-z0-9_-]*' and json_extract(?1,'$.unit')<=json_extract(?1,'$.units') and json_extract(?1,'$.files')<=json_extract(?1,'$.readOperations') valid").get(payload)as{valid:number};
  if(admitted.valid){const language=(database.query("select coalesce((select lower(substr(json_extract(?1,'$.'||value),1,2)) from json_each(?2) where lower(substr(json_extract(?1,'$.'||value),1,2)) in ('en','de') and (length(json_extract(?1,'$.'||value))=2 or substr(json_extract(?1,'$.'||value),3,1) in ('_','-','.','@')) order by cast(key as integer) limit 1),json_extract(?3,'$[0]')) value").get(JSON.stringify(row.environment),JSON.stringify(declaration.systemVariables),JSON.stringify(declaration.fallback))as{value:string}).value,labels=declaration.labels[language as "en"|"de"];
   independent=(database.query("select 'compiler · '||json_extract(?2,'$.phases.'||json_extract(?1,'$.phase'))||': '||json_extract(?1,'$.unit')||'/'||json_extract(?1,'$.units')||' '||json_extract(?1,'$.packageName')||' · '||json_extract(?1,'$.readOperations')||case when ?3='de' then ' Lesevorgänge · ' else ' reads · ' end||json_extract(?1,'$.files')||case when ?3='de' then ' Dateien · ' else ' files · ' end||json_extract(?1,'$.bytes')||case when ?3='de' then ' Bytes' else ' bytes' end value").get(payload,JSON.stringify(labels),language)as{value:string}).value;
  }
 }
 expect(independent,row.name).toBe(row.expected);expect(formatScriptProcessActivityV1(progress,row.environment),row.name).toBe(row.expected);
 const writes:string[]=[],before=Object.fromEntries(declaration.systemVariables.map(name=>[name,process.env[name]])),spy=spyOn(process.stderr,"write").mockImplementation(((text:unknown)=>{writes.push(String(text));return true;})as typeof process.stderr.write);
 try{for(const key of declaration.systemVariables)delete process.env[key];Object.assign(process.env,row.environment);await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"source-progress-fixture",maximumElapsedMilliseconds:0},{fixture:"source-work"},Date.now()),async invocation=>{const signal=invocation.control.signal;await invocation.control.publish(progress);await invocation.control.yieldContinuation();requireIssuedScriptInvocationV1(invocation);expect(invocation.control.signal).toBe(signal);expect(signal.aborted).toBe(false);});}finally{spy.mockRestore();for(const[name,value]of Object.entries(before))if(value===undefined)delete process.env[name];else process.env[name]=value;}
 expect(writes,row.name).toEqual([`[script] ${JSON.stringify(progress)}\n`,...(row.expected===null?[]:[`[script] ${row.expected}\n`])]);
 }}finally{database.close();}
});


test("parser source work has localized bounded operation counters at the actual writer",async()=>{
 const declaration=contract.const.parserSource,locale=contract.const.sourceAdmission,database=new Database(":memory:"),{const:_declaration,...caseSchema}=contract,validate=new Ajv({strict:true}).compile({...caseSchema,$ref:"#/definitions/ParserCases"});expect(validate(fixture.parserCases)).toBe(true);expect(compileJsonSchemaSubset({...caseSchema,$ref:"#/definitions/ParserCases"}).validate(fixture.parserCases)).toEqual([]);
 try{for(const row of fixture.parserCases){
  const progress:ScriptProgress={owner:"process-progress-fixture",command:row.command,stage:"running",activity:{resource:row.resource,mode:row.mode as "shared"|"exclusive",elapsedMs:row.elapsedMs}},language=(database.query("select coalesce((select lower(substr(json_extract(?1,'$.'||value),1,2)) from json_each(?2) where lower(substr(json_extract(?1,'$.'||value),1,2)) in ('en','de') and (length(json_extract(?1,'$.'||value))=2 or substr(json_extract(?1,'$.'||value),3,1) in ('_','-','.','@')) order by cast(key as integer) limit 1),json_extract(?3,'$[0]')) value").get(JSON.stringify(row.environment),JSON.stringify(locale.systemVariables),JSON.stringify(locale.fallback))as{value:"en"|"de"}).value;
  const oracle=database.query("WITH raw AS (SELECT substr(?1,length(?2)+1) tail),selected AS (SELECT substr(tail,1,instr(tail,':')-1) operation,substr(tail,instr(tail,':')+1) completed FROM raw) SELECT CASE WHEN substr(?1,1,length(?2))=?2 AND length(?1)<=?3 AND length(?4) BETWEEN 1 AND ?5 AND ?4 NOT GLOB '*[^A-Za-z0-9:_/.-]*' AND ?6='shared' AND ?7=0 AND length(completed) BETWEEN 1 AND 16 AND substr(completed,1,1) BETWEEN '1' AND '9' AND completed NOT GLOB '*[^0-9]*' AND CAST(completed AS INTEGER) BETWEEN 1 AND ?8 AND (SELECT value FROM json_each(?9,'$.operations') WHERE key=selected.operation) IS NOT NULL THEN ?4||' · '||(SELECT value FROM json_each(?9,'$.operations') WHERE key=selected.operation)||': '||CAST(completed AS INTEGER)||' '||json_extract(?9,'$.units') ELSE NULL END message FROM selected").get(row.resource,declaration.resourcePrefix,declaration.maximumResourceCharacters,row.command,declaration.maximumCommandCharacters,row.mode,row.elapsedMs,declaration.maximumCompleted,JSON.stringify(declaration.labels[language]))as{message:string|null};
  expect(oracle.message,row.name).toBe(row.expected);expect(formatScriptProcessActivityV1(progress,row.environment),row.name).toBe(row.expected);
  const writes:string[]=[],before=Object.fromEntries(locale.systemVariables.map(name=>[name,process.env[name]])),spy=spyOn(process.stderr,"write").mockImplementation(((text:unknown)=>{writes.push(String(text));return true;})as typeof process.stderr.write);
  try{for(const name of locale.systemVariables)delete process.env[name];Object.assign(process.env,row.environment);await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"parser-progress-fixture",maximumElapsedMilliseconds:0},{fixture:"parser-work"},Date.now()),async invocation=>{const signal=invocation.control.signal;await invocation.control.publish(progress);await invocation.control.yieldContinuation();requireIssuedScriptInvocationV1(invocation);expect(invocation.control.signal).toBe(signal);expect(signal.aborted).toBe(false);});}finally{spy.mockRestore();for(const[name,value]of Object.entries(before))if(value===undefined)delete process.env[name];else process.env[name]=value;}
  expect(writes,row.name).toEqual([`[script] ${JSON.stringify(progress)}\n`,...(row.expected===null?[]:[`[script] ${row.expected}\n`])]);
 }}finally{database.close();}
 console.log("[DEBUG] Actual Original writer localizes the six bounded parser-source counters and preserves every structured event");
});
