import custodySchema from "./🧬️schema/🔣️.json";
import {createHash} from "node:crypto";
import {existsSync,lstatSync,readFileSync,readdirSync,writeFileSync} from "node:fs";
import {dirname,isAbsolute,relative,resolve,sep} from "node:path";
import {fileURLToPath} from "node:url";

export type CargoPreparationInputV1 = Readonly<{path:string;kind:"file"|"directory"|"presence";sha256:string|null}>;
export type CargoPreparationResolutionV1 = Readonly<{source:string;specifier:string;selected:string}>;
export type CargoPreparationObservationV1 = Readonly<{version:1;root:string;script:string;command:readonly string[];sources:readonly CargoPreparationInputV1[];inputs:readonly CargoPreparationInputV1[];outputs:readonly CargoPreparationInputV1[];resolutions:readonly CargoPreparationResolutionV1[]}>;
const digest=(value:string|Buffer):string=>createHash("sha256").update(value).digest("hex");
const ordered=(a:string,b:string):number=>Buffer.compare(Buffer.from(a),Buffer.from(b));
function physical(root:string,path:string):void {
 const local=relative(root,path);if(!isAbsolute(path)||isAbsolute(local)||local===".."||local.startsWith(".."+sep))throw Error("Preparation input escapes its repository");
 let current=resolve(root);if(lstatSync(current).isSymbolicLink())throw Error("Preparation root follows a symlink");
 for(const part of local.split(sep).filter(Boolean)){current=resolve(current,part);try{if(lstatSync(current).isSymbolicLink())throw Error("Preparation input follows a symlink");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}}
}
/** 📷️ Hashes exact bytes, directory identity, or physical presence without reading directory children. */
export function cargoPreparationInputV1(root:string,path:string,kind:CargoPreparationInputV1["kind"]):CargoPreparationInputV1 {
 physical(root,path);if(!existsSync(path))return{path,kind,sha256:null};const info=lstatSync(path);
 if(kind==="presence")return{path,kind,sha256:digest(JSON.stringify(info.isFile()?"file":info.isDirectory()?"directory":"other"))};
 if(kind==="file"){if(!info.isFile())throw Error("Preparation file is not regular");return{path,kind,sha256:digest(readFileSync(path))};}
 if(!info.isDirectory())throw Error("Preparation directory is not physical");
 const entries=readdirSync(path,{withFileTypes:true}).map(row=>{if(row.isSymbolicLink())throw Error("Preparation directory contains a symlink");return[row.name,row.isDirectory()?"directory":row.isFile()?"file":"other"];}).sort((a,b)=>ordered(a[0]!,b[0]!));return{path,kind,sha256:digest(JSON.stringify(entries))};
}
/** 📚️ Binds the selected first-party program's eager import/reexport closure and its resolver configuration. */
export function cargoPreparationProgramSourcesV1(root:string,entry:string):Readonly<{sources:readonly CargoPreparationInputV1[];inputs:readonly CargoPreparationInputV1[];resolutions:readonly CargoPreparationResolutionV1[]}> {
 physical(root,entry);const result=Bun.spawnSync([process.execPath,fileURLToPath(new URL("../📜️script.ts",import.meta.url)),"capture-program",root,entry],{cwd:root,env:{...process.env,NX_WORKSPACE_ROOT:root},stdout:"pipe",stderr:"pipe"});if(result.exitCode!==0)throw Error(`Preparation program capture failed: ${result.stderr.toString()}`);const value=JSON.parse(result.stdout.toString());const row=admitCargoPreparationObservationV1({...value,version:1,root,script:entry,command:["capture-program"],outputs:[]});return{sources:row.sources,inputs:row.inputs,resolutions:row.resolutions};
}

/** 🧬️ Admits only complete current-format observations from the selected preparation producer. */
export function admitCargoPreparationObservationV1(value:unknown):CargoPreparationObservationV1 {
 const row=value as CargoPreparationObservationV1;if(!row||typeof row!=="object"||Object.keys(row).sort().join(",")!=="command,inputs,outputs,resolutions,root,script,sources,version"||row.version!==1||![row.root,row.script].every(path=>typeof path==="string"&&isAbsolute(path))||!Array.isArray(row.command)||!row.command.length||row.command.some(value=>typeof value!=="string"||!value))throw Error("Invalid preparation observation");
 for(const [key,inputs] of [["sources",row.sources],["inputs",row.inputs],["outputs",row.outputs]] as const){if(!Array.isArray(inputs)||(key==="sources"&&!inputs.length)||inputs.some(input=>!input||Object.keys(input).sort().join(",")!=="kind,path,sha256"||typeof input.path!=="string"||!isAbsolute(input.path)||!custodySchema.$defs.CargoPreparationInputV1.properties.kind.enum.includes(input.kind)||!(input.sha256===null||typeof input.sha256==="string"&&/^[0-9a-f]{64}$/.test(input.sha256)))||new Set(inputs.map(input=>JSON.stringify([input.path,input.kind]))).size!==inputs.length)throw Error("Invalid preparation inputs");if(key!=="inputs"&&inputs.some(input=>input.kind!=="file"||input.sha256===null))throw Error("Preparation source/output lacks exact file bytes");}
 if(!Array.isArray(row.resolutions)||row.resolutions.some(edge=>!edge||Object.keys(edge).sort().join(",")!=="selected,source,specifier"||![edge.source,edge.selected].every(path=>typeof path==="string"&&isAbsolute(path))||typeof edge.specifier!=="string"||!edge.specifier))throw Error("Invalid preparation program resolutions");
 return row;
}
/** 🔒️ Refuses changed producer source, input presence/bytes/rosters, or published output bytes. */
export function assertCargoPreparationObservationCurrentV1(value:unknown):void {
 const row=admitCargoPreparationObservationV1(value);
 for(const edge of row.resolutions){physical(row.root,edge.source);physical(row.root,edge.selected);}
 if(row.resolutions.length){const result=Bun.spawnSync([process.execPath,fileURLToPath(new URL("../📜️script.ts",import.meta.url)),"resolve-program"],{cwd:row.root,env:{...process.env,NX_WORKSPACE_ROOT:row.root},stdin:Buffer.from(JSON.stringify(row.resolutions)),stdout:"pipe",stderr:"pipe"});if(result.exitCode!==0)throw Error(`Preparation program resolution failed: ${result.stderr.toString()}`);const selected=JSON.parse(result.stdout.toString());if(!Array.isArray(selected)||selected.length!==row.resolutions.length)throw Error("Invalid current program resolution");for(const [index,edge] of row.resolutions.entries())if(selected[index]!==edge.selected)throw Error(`Preparation program resolution changed: ${edge.source} ${edge.specifier}`);}
 const outputs=new Set(row.outputs.map(input=>input.path));
 for(const input of [...row.sources,...row.inputs.filter(input=>!outputs.has(input.path)),...row.outputs])if(cargoPreparationInputV1(row.root,input.path,input.kind).sha256!==input.sha256)throw Error(`Preparation custody changed: ${input.path}`);
}
let active:{row:{version:1;root:string;script:string;command:string[]};sources:Map<string,CargoPreparationInputV1>;inputs:Map<string,CargoPreparationInputV1>;outputs:Map<string,CargoPreparationInputV1>}|undefined;
function capture():typeof active {
 if(!process.env.SEMIO_CARGO_PREPARATION_OBSERVATION)return;
 if(!active){const root=process.env.NX_WORKSPACE_ROOT,script=process.env.SEMIO_CARGO_PREPARATION_ACTIVE;if(!root||!script)throw Error("Preparation observation lacks its producer");active={row:{version:1,root,script,command:process.argv.slice(2)},sources:new Map(),inputs:new Map(),outputs:new Map()};for(const path of [script,fileURLToPath(import.meta.url)])active.sources.set(path,cargoPreparationInputV1(root,path,"file"));}
 return active;
}
/** 🧭️ Binds a production preparation implementation and its pure admission dependencies to current source bytes. */
export function observeCargoPreparationSourceV1(...paths:string[]):void {const state=capture();if(state)for(const path of paths){const input=cargoPreparationInputV1(state.row.root,path,"file"),previous=state.sources.get(path);if(previous&&previous.sha256!==input.sha256)throw Error(`Preparation source changed: ${path}`);state.sources.set(path,input);}}
/** 📥️ Retains the actual physical read/presence/directory authority of the active preparation operation. */
export function observeCargoPreparationInputV1(path:string,kind:CargoPreparationInputV1["kind"]="file",bytes?:string|Buffer):void {const state=capture();if(state){physical(state.row.root,path);const input=bytes===undefined?cargoPreparationInputV1(state.row.root,path,kind):{path,kind,sha256:digest(bytes)},key=JSON.stringify([path,kind]),previous=state.inputs.get(key);if(bytes!==undefined&&kind!=="file")throw Error("Preparation read bytes require a file");if(previous&&previous.sha256!==input.sha256&&!state.outputs.has(path))throw Error(`Preparation read changed: ${path}`);state.inputs.set(key,input);}}
/** 📤️ Retains the exact file bytes published by the active preparation operation. */
export function observeCargoPreparationOutputV1(path:string,bytes:string|Buffer):void {const state=capture();if(state){physical(state.row.root,path);state.outputs.set(path,{path,kind:"file",sha256:digest(bytes)});}}
/** 🧾️ Publishes complete operation custody only after all inputs and outputs remain current. */
export function completeCargoPreparationObservationV1():void {
 const state=capture();if(!state)return;const row={...state.row,resolutions:[],sources:[...state.sources.values()],inputs:[...state.inputs.values()],outputs:[...state.outputs.values()]};assertCargoPreparationObservationCurrentV1(row);writeFileSync(process.env.SEMIO_CARGO_PREPARATION_OBSERVATION!,JSON.stringify(row));active=undefined;
}
