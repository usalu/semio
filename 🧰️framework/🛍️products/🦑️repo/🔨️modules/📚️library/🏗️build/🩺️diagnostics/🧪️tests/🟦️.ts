/** 🩺️ Contract: `SEMIO_DIAGNOSTICS=<area,...>` enables typed areas (unknown ones are refused in both languages, `all` enables every area), a disabled channel builds no record, an enabled one prints one `[diagnostic:<area>] {json}` line; and no temporary debug line survives in landed code under the build system roots (only the pinned recorded producer log is allowed). An independent Python model reads the same schema and corpus.
 * @see ../🧬️schema/🔣️.json */
import{expect,test}from"bun:test";
import{readFileSync,readdirSync,realpathSync,lstatSync,statSync}from"node:fs";
import{dirname,join,relative,sep}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{DIAGNOSTICS_AREAS_V1,DIAGNOSTICS_PROTOCOL_V1,createDiagnosticsV1,readDiagnosticAreasV1,renderDiagnosticLineV1}from"../🟨️.mjs";
import schema from"../🧬️schema/🔣️.json";
import corpus from"./🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),schemaPath=join(here,"..","🧬️schema","🔣️.json"),framework=join(here,"..","..","..","..","..","..","..");
const law=schema.definitions.Law.const,marker=law.forbiddenMarkerParts.join("");

test("every environment of the corpus reads as the independent model says, and every area has an English and a German text",()=>{
 expect(python,"python3 is required for the independent model").toBeTruthy();
 const model=JSON.parse(Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),"areas",schemaPath,join(here,"🔣️.json")]).stdout.toString()) as {areas?:string[];refused?:string[]}[];
 expect(model.length).toBe(corpus.environments.length);
 corpus.environments.forEach((item,at)=>{
  const environment=item.value===null?{}:{SEMIO_DIAGNOSTICS:item.value};
  if(model[at]!.refused){
   expect(()=>readDiagnosticAreasV1(environment),item.name).toThrow(/Unknown diagnostics area .* \/ Unbekannter Diagnosebereich/u);
   for(const name of model[at]!.refused!)expect(()=>readDiagnosticAreasV1(environment),item.name).toThrow(name);
  }else expect([...readDiagnosticAreasV1(environment)],item.name).toEqual(model[at]!.areas! as never);
 });
 for(const [area,text] of Object.entries(DIAGNOSTICS_AREAS_V1))expect(text.en.length>0&&text.de.length>0,area).toBe(true);
 expect(Object.keys(DIAGNOSTICS_AREAS_V1).sort()).toEqual([...schema.definitions.Area.enum].sort());
});

test("a disabled area builds nothing, an enabled one prints one tagged JSON line, an unknown area is a programming error",()=>{
 const written:string[]=[];let built=0;
 const channel=createDiagnosticsV1({environment:{SEMIO_DIAGNOSTICS:"graph"},now:()=>12.6,write:text=>written.push(text)});
 expect(channel.diagnose("cache",()=>{built++;return {event:"x"};})).toBe(false);
 expect(built).toBe(0);expect(written).toEqual([]);
 expect(channel.diagnose("graph",()=>{built++;return {event:"opened",count:3};})).toBe(true);
 expect(written).toEqual(['[diagnostic:graph] {"at":13,"event":"opened","count":3}\n']);
 expect(()=>channel.diagnose("nowhere" as never,()=>({}))).toThrow(/Unknown diagnostics area/u);
 const cyclic:Record<string,unknown>={};cyclic.self=cyclic;
 expect(renderDiagnosticLineV1("graph",cyclic,1)).toMatch(/^\[diagnostic:graph\] \{"at":1,"unserializable":".*"\}\n$/u);
 expect(renderDiagnosticLineV1("graph",{text:"x".repeat(DIAGNOSTICS_PROTOCOL_V1.maximumLineCharacters)},1)).toMatch(/"truncated":\d+/u);
});

test("a malformed switch in a real process is told once and never breaks the command",()=>{
 const run=Bun.spawnSync([process.execPath,"-e",`const m=await import(${JSON.stringify(join(here,"..","🟨️.mjs"))});console.log(m.diagnoseV1("graph",()=>({event:"x"})));`],{env:{...process.env,SEMIO_DIAGNOSTICS:"verbose"}});
 expect(run.exitCode).toBe(0);
 expect(run.stdout.toString().trim()).toBe("false");
 expect(run.stderr.toString()).toContain("Unknown diagnostics area verbose");
 const on=Bun.spawnSync([process.execPath,"-e",`const m=await import(${JSON.stringify(join(here,"..","🟨️.mjs"))});m.diagnoseV1("cache",()=>({event:"hit"}));`],{env:{...process.env,SEMIO_DIAGNOSTICS:"cache"}});
 expect(on.stderr.toString()).toMatch(/^\[diagnostic:cache\] \{"at":\d+,"event":"hit"\}\n$/u);
});

const scan=()=>{
 const found:{path:string;line:number}[]=[],seen=new Set<string>(),needle=Buffer.from(marker);
 const walk=(directory:string)=>{
  for(const entry of readdirSync(directory,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:1)){
   const path=join(directory,entry.name);
   if(entry.isDirectory()){if(!law.skipDirectories.includes(entry.name))walk(path);continue;}
   if(!entry.isFile()||lstatSync(path).isSymbolicLink()||statSync(path).size>law.maximumFileBytes)continue;
   const real=realpathSync(path);if(seen.has(real))continue;seen.add(real);
   const bytes=readFileSync(path);if(bytes.indexOf(needle)<0)continue;
   let number=0,start=0;
   while(start<=bytes.length){const end=bytes.indexOf(10,start),stop=end<0?bytes.length:end;number++;if(bytes.subarray(start,stop).indexOf(needle)>=0)found.push({path:relative(framework,path).split(sep).join("/"),line:number});if(end<0)break;start=end+1;}
  }
 };
 for(const root of law.roots)walk(join(framework,root));
 return found;
};

test("no temporary debug line survives in landed build system code; only the pinned recorded producer log carries it",()=>{
 expect(python,"python3 is required for the independent model").toBeTruthy();
 const model=JSON.parse(Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),"scan",schemaPath,framework],{maxBuffer:1<<26}).stdout.toString()) as {path:string;line:number}[];
 const ours=scan();
 const key=(row:{path:string;line:number})=>row.path+":"+row.line;
 expect(ours.map(key).sort()).toEqual(model.map(key).sort());
 const allowed=(path:string)=>law.recordedData.some(row=>path.endsWith(row.suffix));
 expect(ours.filter(row=>!allowed(row.path)),"a "+marker.trim()+" line is temporary: remove it, or move permanent output to the diagnostics channel (SEMIO_DIAGNOSTICS)").toEqual([]);
 for(const row of law.recordedData){expect(ours.some(hit=>hit.path.endsWith(row.suffix)),"stale allowance "+row.suffix).toBe(true);expect(row.reason.en.length>0&&row.reason.de.length>0).toBe(true);}
},180000);
