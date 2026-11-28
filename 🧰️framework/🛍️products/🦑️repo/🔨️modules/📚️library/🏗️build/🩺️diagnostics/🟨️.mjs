import {writeSync} from "node:fs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🩺️ The declared numbers of the diagnostics channel: the schema file is the single source.
 * @see ./🧬️schema/🔣️.json */
export const DIAGNOSTICS_PROTOCOL_V1=Object.freeze({...schema.definitions.Protocol.const,silent:Object.freeze([...schema.definitions.Protocol.const.silent])});
/** 🗺️ Every diagnostics area with its English and German description. */
export const DIAGNOSTICS_AREAS_V1=Object.freeze(Object.fromEntries(Object.entries(schema.definitions.Areas.const).map(([area,text])=>[area,Object.freeze({...text})])));
/** 🎛️ The environment variable that names the areas to print: `SEMIO_DIAGNOSTICS=graph,cache` or `all`; unset, empty, `0`, `off`, `false` and `none` print nothing. */
export const DIAGNOSTICS_VARIABLE_V1=DIAGNOSTICS_PROTOCOL_V1.variable;

/** 🔎️ Reads the enabled areas from an environment; an unknown area is refused in both languages instead of silently printing nothing. */
export function readDiagnosticAreasV1(environment=process.env){
 const raw=String(environment?.[DIAGNOSTICS_VARIABLE_V1]??"").trim().toLowerCase();
 if(DIAGNOSTICS_PROTOCOL_V1.silent.includes(raw))return Object.freeze([]);
 const known=Object.keys(DIAGNOSTICS_AREAS_V1),names=raw.split(",").map(name=>name.trim()).filter(Boolean);
 if(names.includes(DIAGNOSTICS_PROTOCOL_V1.everything))return Object.freeze(known);
 const unknown=names.filter(name=>!known.includes(name));
 if(unknown.length)throw Error(`Unknown diagnostics area ${unknown.join(",")} in ${DIAGNOSTICS_VARIABLE_V1} (known: ${known.join("|")}|all) / Unbekannter Diagnosebereich ${unknown.join(",")} in ${DIAGNOSTICS_VARIABLE_V1} (bekannt: ${known.join("|")}|all)`);
 return Object.freeze([...new Set(names)].sort());
}

/** 🧾️ Renders one diagnostic line `[diagnostic:<area>] {json}` with the milliseconds since the process start first; a record that cannot be written as JSON or exceeds the line bound is replaced by a marker row, never thrown. */
export function renderDiagnosticLineV1(area,record,now=0){
 let body;
 try{body=JSON.stringify({at:Math.round(now),...record});}catch(error){body=JSON.stringify({at:Math.round(now),unserializable:String(error?.message??error)});}
 if(body.length>DIAGNOSTICS_PROTOCOL_V1.maximumLineCharacters)body=JSON.stringify({at:Math.round(now),truncated:body.length});
 return DIAGNOSTICS_PROTOCOL_V1.linePrefix+area+DIAGNOSTICS_PROTOCOL_V1.lineSuffix+body+"\n";
}

/** 🩺️ Creates one diagnostics channel: `diagnose(area, build)` calls `build` and prints its record only when the area is enabled, so a disabled channel costs one set lookup and builds nothing. */
export function createDiagnosticsV1({environment=process.env,now=()=>performance.now(),write=text=>{try{writeSync(2,text);}catch{}}}={}){
 const areas=new Set(readDiagnosticAreasV1(environment)),known=DIAGNOSTICS_AREAS_V1;
 return Object.freeze({
  areas:Object.freeze([...areas].sort()),
  enabled:area=>areas.has(area),
  diagnose(area,build){
   if(!Object.hasOwn(known,area))throw Error(`Unknown diagnostics area ${area} / Unbekannter Diagnosebereich ${area}`);
   if(!areas.has(area))return false;
   write(renderDiagnosticLineV1(area,build(),now()));return true;
  },
 });
}

let shared;
/** 🌐️ The process wide channel, created on first use from the environment; a malformed variable is told once on stderr and then counts as off, a diagnostics switch never breaks a command. */
export function diagnosticsV1(){
 if(shared===undefined){
  try{shared=createDiagnosticsV1();}
  catch(error){try{writeSync(2,String(error?.message??error)+"\n");}catch{}shared=createDiagnosticsV1({environment:{}});}
 }
 return shared;
}
/** 🧪️ Drops the process wide channel so a test can create another one. */
export function resetDiagnosticsV1(){shared=undefined;}
/** 🩺️ Prints the record `build()` returns when `area` is enabled in `SEMIO_DIAGNOSTICS`; the record is a plain JSON object (`{event:"...",...}`) and is never built otherwise. */
export function diagnoseV1(area,build){return diagnosticsV1().diagnose(area,build);}
