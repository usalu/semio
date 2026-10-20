import { expect, spyOn, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { compileJsonSchemaSubset } from "../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type { ScriptProgress } from "../../../🟦️.ts";
import { createScriptProcessEnvelope, formatScriptProcessActivityV1, requireIssuedScriptInvocationV1, withScriptProcessEnvelope, createIssuedResourceControlV1 } from "../../🟦️.ts";
import {readFileSync} from "node:fs";
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

test("provider presentation follows neutral monotonic clock rows and independent SQLite scheduling",async()=>{
 const ts=await import("typescript"),source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("process.ts",source,ts.ScriptTarget.Latest,true),names=["sourceProviderCounterV1","createScriptProcessProgressWriterV1"],definitions=tree.statements.filter((node):node is import("typescript").FunctionDeclaration=>ts.isFunctionDeclaration(node)&&names.includes(node.name?.text??""));expect(definitions).toHaveLength(2);
 const make=new Function("progressSchema","formatScriptProcessActivityV1",ts.transpileModule(definitions.map(node=>node.getText(tree)).join("\n")+";return createScriptProcessProgressWriterV1;",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText)(contract,formatScriptProcessActivityV1),{const:_policy,...schema}=contract,validate=new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/ProviderClockCases"}),db=new Database(":memory:");expect(validate(fixture.providerClockCases)).toBe(true);
 try{for(const row of fixture.providerClockCases){
  let now=0;const writes:string[]=[],writer=make((line:string)=>writes.push(line),{LANG:"en"},()=>now);
  const reset="(r.kind<>'provider' OR s.last_at IS NULL OR r.c<s.last_count OR r.at<s.last_clock OR r.elapsed<s.last_elapsed)",due="("+reset+" OR r.at-s.last_at>=?2)";
  const oracle=db.query(`WITH RECURSIVE r AS(SELECT CAST(key AS INTEGER)n,json_extract(value,'$.at')at,json_extract(value,'$.count')c,json_extract(value,'$.kind')kind,coalesce(json_extract(value,'$.elapsed'),json_extract(value,'$.at'))elapsed FROM json_each(?1)),s(n,last_at,last_count,last_clock,last_elapsed,pending,wire)AS(SELECT -1,NULL,NULL,NULL,NULL,NULL,'[]' UNION ALL SELECT r.n,CASE WHEN r.kind<>'provider' THEN NULL WHEN ${due} THEN r.at ELSE s.last_at END,r.c,r.at,r.elapsed,CASE WHEN ${due} THEN NULL ELSE r.n END,CASE WHEN ${due} THEN json_insert(CASE WHEN ${reset} AND s.pending IS NOT NULL THEN json_insert(s.wire,'$[#]',s.pending) ELSE s.wire END,'$[#]',r.n) ELSE s.wire END FROM s JOIN r ON r.n=s.n+1),last AS(SELECT * FROM s ORDER BY n DESC LIMIT 1),indices AS(SELECT CAST(key AS INTEGER)ord,value n FROM last,json_each(CASE WHEN pending IS NULL THEN wire ELSE json_insert(wire,'$[#]',pending) END))SELECT CASE WHEN r.kind='provider' THEN 'provider:'||r.c ELSE r.kind END event FROM indices JOIN r USING(n) ORDER BY indices.ord`).all(JSON.stringify(row.steps),contract.const.sourceProvider.minimumPublicationMilliseconds)as{event:string}[];
  expect(oracle.map(value=>value.event),row.name).toEqual(row.expected);
  for(const step of row.steps){now=step.at;writer.publish({owner:"provider-fixture",command:"repository:command",stage:step.kind==="complete"?"complete":"running",...(step.kind==="complete"?{}:{activity:{resource:step.kind==="provider"?"javascript-source-provider:"+step.count:"ordinary-progress",mode:"shared",elapsedMs:"elapsed" in step?step.elapsed:step.at}})});}writer.flush();
  const actual=writes.filter(line=>line.startsWith("[script] {")).map(line=>{const progress=JSON.parse(line.slice(9));return progress.stage==="complete"?"complete":progress.activity.resource.startsWith("javascript-source-provider:")?"provider:"+progress.activity.resource.split(":").at(-1):"other";});expect(actual,row.name).toEqual(row.expected);
 }}finally{db.close();}
});

test("provider actual Original publisher preserves every checkpoint and flushes before failure settlement",async()=>{
 const writes:string[]=[],primary=Object.freeze({providerPrimary:true}),spy=spyOn(process.stderr,"write").mockImplementation(((text:unknown)=>{writes.push(String(text));return true;})as typeof process.stderr.write);let callbacks=0,yields=0,checks=0,signal:AbortSignal|undefined;
 try{await expect(withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"provider-original",maximumElapsedMilliseconds:0},{fixture:"provider"},Date.now()),async invocation=>{
  signal=invocation.control.signal;const stop=new AbortController(),phase=createIssuedResourceControlV1(invocation,"provider-work",0,stop.signal);
  for(let count=0;count<5;count++){requireIssuedScriptInvocationV1(invocation);checks++;await phase.publish({resource:"javascript-source-provider:"+count,mode:"shared",elapsedMs:count});callbacks++;await phase.yieldContinuation();yields++;expect(invocation.control.signal).toBe(signal);}
  stop.abort(primary);await expect(phase.publish({resource:"javascript-source-provider:6",mode:"shared",elapsedMs:6})).rejects.toBe(primary);requireIssuedScriptInvocationV1(invocation);checks++;throw primary;
 })).rejects.toBe(primary);}finally{spy.mockRestore();}
 expect(callbacks).toBe(5);expect(yields).toBe(5);expect(checks).toBe(6);expect(signal?.aborted).toBe(false);const rows=writes.filter(line=>line.startsWith("[script] {")).map(line=>JSON.parse(line.slice(9)));expect(rows[0].activity.resource).toBe("javascript-source-provider:0");expect(rows.at(-1).activity.resource).toBe("javascript-source-provider:4");expect(rows.every(row=>row.owner==="provider-original")).toBe(true);
});

test("provider counters use bounded localized actual writer messages",async()=>{
 const policy=contract.const.sourceProvider,db=new Database(":memory:"),{const:_policy,...schema}=contract;expect(new Ajv({strict:true}).compile({...schema,$ref:"#/definitions/ParserCases"})(fixture.providerCases)).toBe(true);
 try{for(const row of fixture.providerCases){const text=row.resource.slice(policy.resourcePrefix.length),language=row.environment.LANG.startsWith("de")?"de":"en",oracle=db.query("select case when ?1=?2 and length(?3)>0 and ?3 not glob '*[^0-9]*' and (?3='0' or substr(?3,1,1) between '1' and '9') and cast(?3 as integer)<=?4 then replace(replace(?5,'{command}',?6),'{completed}',cast(cast(?3 as integer)as text)) else null end message").get(row.resource.slice(0,policy.resourcePrefix.length),policy.resourcePrefix,text,policy.maximumCompleted,policy.labels[language],row.command)as{message:string|null};expect(oracle.message,row.name).toBe(row.expected);const progress:ScriptProgress={owner:"provider-fixture",command:row.command,stage:"running",activity:{resource:row.resource,mode:row.mode as"shared",elapsedMs:row.elapsedMs}};expect(formatScriptProcessActivityV1(progress,row.environment),row.name).toBe(row.expected);}}finally{db.close();}
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
 console.log("[DEBUG] Actual Original writer localizes seven bounded parser-source counters and preserves every structured event");
});


test("queue wait names the head request and position in the selected language at the actual writer",async()=>{
 const declaration=contract.const.queueWait,locale=contract.const.sourceAdmission,database=new Database(":memory:"),{const:_declaration,...caseSchema}=contract,validate=new Ajv({strict:true}).compile({...caseSchema,$ref:"#/definitions/QueueCases"});expect(validate(fixture.queueCases)).toBe(true);expect(compileJsonSchemaSubset({...caseSchema,$ref:"#/definitions/QueueCases"}).validate(fixture.queueCases)).toEqual([]);
 try{for(const row of fixture.queueCases){
  const progress:ScriptProgress={owner:"process-progress-fixture",command:row.command,stage:"running",activity:{resource:row.resource,mode:row.mode as "shared"|"exclusive",elapsedMs:row.elapsedMs}},language=(database.query("select coalesce((select lower(substr(json_extract(?1,'$.'||value),1,2)) from json_each(?2) where lower(substr(json_extract(?1,'$.'||value),1,2)) in ('en','de') and (length(json_extract(?1,'$.'||value))=2 or substr(json_extract(?1,'$.'||value),3,1) in ('_','-','.','@')) order by cast(key as integer) limit 1),json_extract(?3,'$[0]')) value").get(JSON.stringify(row.environment),JSON.stringify(locale.systemVariables),JSON.stringify(locale.fallback))as{value:"en"|"de"}).value,payload=row.resource.slice(declaration.resourcePrefix.length),labels=declaration.labels[language];
  const oracle=database.query("SELECT CASE WHEN substr(?1,1,length(?2))=?2 AND length(?1)<=?3 AND length(?4) BETWEEN 1 AND ?5 AND ?4 NOT GLOB '*[^A-Za-z0-9:_/.-]*' AND ?6=0 AND json_valid(?7) AND json_type(?7)='object' AND (SELECT count(*) FROM json_each(?7))=7 AND json_type(?7,'$.version')='integer' AND json_extract(?7,'$.version')=1 AND json_type(?7,'$.position')='integer' AND json_extract(?7,'$.position') BETWEEN 2 AND 999999 AND json_type(?7,'$.queued')='integer' AND json_extract(?7,'$.queued') BETWEEN 2 AND 999999 AND json_extract(?7,'$.position')<=json_extract(?7,'$.queued') AND json_extract(?7,'$.holderMode') IN ('shared','exclusive') AND json_type(?7,'$.holderPid')='integer' AND json_extract(?7,'$.holderPid') BETWEEN 1 AND 9999999999 AND json_type(?7,'$.holderSeconds')='integer' AND json_extract(?7,'$.holderSeconds') BETWEEN 0 AND 31536000 AND json_type(?7,'$.resource')='text' AND length(json_extract(?7,'$.resource')) BETWEEN 1 AND 401 AND ?8 IN ('shared','exclusive') THEN replace(replace(replace(replace(replace(replace(replace(replace(json_extract(?9,'$.template'),'{command}',?4),'{resource}',json_extract(?7,'$.resource')),'{mode}',json_extract(?9,'$.modes.'||?8)),'{position}',json_extract(?7,'$.position')),'{queued}',json_extract(?7,'$.queued')),'{holderPid}',json_extract(?7,'$.holderPid')),'{holderMode}',json_extract(?9,'$.modes.'||json_extract(?7,'$.holderMode'))),'{holderSeconds}',json_extract(?7,'$.holderSeconds')) ELSE NULL END message").get(row.resource,declaration.resourcePrefix,declaration.maximumResourceCharacters,row.command,declaration.maximumCommandCharacters,row.elapsedMs,payload,row.mode,JSON.stringify(labels))as{message:string|null};
  expect(oracle.message,row.name).toBe(row.expected);expect(formatScriptProcessActivityV1(progress,row.environment),row.name).toBe(row.expected);
  const writes:string[]=[],before=Object.fromEntries(locale.systemVariables.map(name=>[name,process.env[name]])),spy=spyOn(process.stderr,"write").mockImplementation(((text:unknown)=>{writes.push(String(text));return true;})as typeof process.stderr.write);
  try{for(const name of locale.systemVariables)delete process.env[name];Object.assign(process.env,row.environment);await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"queue-progress-fixture",maximumElapsedMilliseconds:0},{fixture:"queue-wait"},Date.now()),async invocation=>{const signal=invocation.control.signal;await invocation.control.publish(progress);await invocation.control.yieldContinuation();requireIssuedScriptInvocationV1(invocation);expect(invocation.control.signal).toBe(signal);expect(signal.aborted).toBe(false);});}finally{spy.mockRestore();for(const[name,value]of Object.entries(before))if(value===undefined)delete process.env[name];else process.env[name]=value;}
  expect(writes,row.name).toEqual([`[script] ${JSON.stringify(progress)}\n`,...(row.expected===null?[]:[`[script] ${row.expected}\n`])]);
 }}finally{database.close();}
});
