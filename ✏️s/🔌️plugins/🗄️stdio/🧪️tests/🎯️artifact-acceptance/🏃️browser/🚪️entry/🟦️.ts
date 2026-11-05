import {resolve,relative,join,sep,isAbsolute} from "node:path";
import {spawnedIdOfWindowInstanceV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪟️spawned-program/🟦️.ts";
/** 🚪️ Explicit entry paths and witnessed locales prevent natural imports from standing in for authored examples. */
import type {AcceptanceRow} from "../../📇️matrix/🟦️.ts";
export type EntryMode="natural-file"|"authored-example";
export function parseEntryMode(value:string|undefined):EntryMode{if(value!=="natural-file"&&value!=="authored-example")throw new Error("--entry must be natural-file or authored-example");return value;}
export function parseJourneyLocale(value:string|undefined):"en"|"de"{if(value!=="en"&&value!=="de")throw new Error("--locale must be en or de");return value;}
export function assertActualLocale(actual:string|null,wanted:"en"|"de"):void{if(actual!==wanted)throw new Error(`actual shell locale ${JSON.stringify(actual)} differs from required ${wanted}`);}
export function selectJourneyRows(rows:readonly AcceptanceRow[],entry:EntryMode,only:readonly string[]):AcceptanceRow[]{
 for(const id of only){const row=rows.find(row=>row.artifact===id);if(!row)throw new Error(`unknown requested artifact ${id}`);if(entry==="authored-example"&&!row.authoredExample)throw new Error(`artifact ${id} has no declared authored example journey`);}
 return rows.filter(row=>(only.length===0||only.includes(row.artifact))&&(entry==="natural-file"||row.authoredExample!==undefined));
}
/** 🪟️ Resolves actual shell ownership through the same first-party namespace protocol as the host. */
export function assertOwnedProgram(ids:readonly string[],programs:readonly {id:string;pluginId:string;appId:string}[],expected:{pluginId:string;appId:string}):void{
 if(ids.length===0)throw new Error("document has no owned windows");
 for(const id of ids){const owner=spawnedIdOfWindowInstanceV1(id,programs.map(program=>program.id));const program=programs.find(program=>program.id===owner);if(program?.pluginId!==expected.pluginId||program.appId!==expected.appId)throw new Error(`document window ${id} does not belong to ${expected.pluginId}/${expected.appId}`);}
}

/** 🎛️ Both languages require a successful real Settings control interaction. */
export function assertSeatedLocale(outcome:string,wanted:"en"|"de"):void{if(outcome!==wanted+"="+wanted)throw new Error("locale control did not seat "+wanted+": "+outcome);}

/** 📁️ Validates ticket custody, portable names and every planned output before a process or directory is created. */
export function validateJourneyOutput(root:string,output:string,tag:string,rows:readonly {artifact:string;extension:string;validPayloads?:readonly string[]}[]):string{
 const safe=(value:string)=>/^[a-z0-9][a-z0-9._-]{0,63}$/iu.test(value)&&!/[. ]$/u.test(value)&&!/^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/iu.test(value);
 if(!safe(tag))throw new Error("journey tag must be a short Windows-safe name");
 const ticketRoot=resolve(root,".🧬semio/🦑️repo/🎫️tickets"),destination=resolve(root,output),local=relative(ticketRoot,destination),parts=local.split(sep);
 if(isAbsolute(local)||local===".."||local.startsWith(".."+sep)||parts.length<5||!/^🎆️[0-9]{2}$/u.test(parts[0]!)||!/^🌙️[0-9]{2}$/u.test(parts[1]!)||!/^☀️[0-9]{2}$/u.test(parts[2]!)||!/^[A-Z0-9]+(?:-[A-Z0-9]+)*$/u.test(parts[3]!)||parts[4]!=="🗑️generated"||parts.slice(5).some(part=>!safe(part)))throw new Error("--out must be inside a repository ticket's generated directory");
 const directory=join(destination,tag),planned=[directory,join(directory,"journey.json"),join(directory,"table.md"),join(directory,"browser-tmp"),join(directory,"browser-tmp","playwright_chromiumdev_profile-XXXXXX","Default","Local Storage","leveldb","000003.log")];
 for(const row of rows){
  if(!/^[a-z0-9]+$/u.test(row.artifact)||!/^\.[a-z0-9]+$/u.test(row.extension))throw new Error("journey row has a nonportable artifact filename");
  planned.push(join(directory,row.artifact+"-failed.png"));
  const stages=["baseline","mutation","undo","redo","edited","reopened","reloaded","after-refusal",...(row.validPayloads??[]).map((_,index)=>"arbitrary-"+index)];
  for(const stage of stages)planned.push(join(directory,"exports",row.artifact+"-"+stage+row.extension));
 }
 if(planned.some(path=>path.length>256))throw new Error("planned journey path exceeds 256 UTF-16 code units");
 return directory;
}
