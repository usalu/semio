import {availableParallelism} from "node:os";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const policy=UNIT_CACHE_POLICY_V1;
/** 📍️ The wrapper script Cargo runs in place of rustc. */
export const UNIT_CACHE_SCRIPT_V1=fileURLToPath(new URL("../📜️script.ts",import.meta.url));

/** 🎯️ The package names a Cargo command line selects with `-p` or `--package`; their units compile incrementally in the lane instead of through the shared cache. */
export function focusFromCargoArgumentsV1(args:readonly string[]):string[]{
 const names:string[]=[],end=args.indexOf("--"),head=end<0?args:args.slice(0,end);
 for(let index=0;index<head.length;index++){
  const argument=head[index]!;
  if(argument==="-p"||argument==="--package"){const value=head[++index];if(value)names.push(value.replace(/@.*$/u,""));}
  else if(argument.startsWith("--package="))names.push(argument.slice("--package=".length).replace(/@.*$/u,""));
  else if(/^-p.+/u.test(argument))names.push(argument.slice(2).replace(/@.*$/u,""));
 }
 return[...new Set(names)].sort();
}

/** 🧩️ The environment block that routes every compiler call of one Cargo process through the unit cache: Cargo runs `bun <script> <rustc arguments>` and the script falls back to the memory-admitting scheduler for every miss. A launcher adds this after its storage admission and never before. */
export function unitCacheEnvironmentV1(input:Readonly<{cacheRoot:string;lane:string;laneRoot:string;scheduler:string;compiler:string;toolchain:string;focus:readonly string[];slots?:number;runtime?:string;features?:Readonly<{buildScripts?:boolean;brokerReplay?:boolean}>}>):Record<string,string>{
 return{
  RUSTC_WRAPPER:input.runtime??process.execPath,
  RUSTC:UNIT_CACHE_SCRIPT_V1,
  SEMIO_UNIT_CACHE_ROOT:input.cacheRoot,
  SEMIO_UNIT_CACHE_LANE:input.lane,
  SEMIO_UNIT_CACHE_LANE_ROOT:input.laneRoot,
  SEMIO_UNIT_CACHE_INNER:JSON.stringify([input.scheduler,input.compiler]),
  SEMIO_UNIT_CACHE_TOOLCHAIN:input.toolchain,
  SEMIO_UNIT_CACHE_SLOTS:String(input.slots??availableParallelism()),
  SEMIO_UNIT_CACHE_FOCUS:input.focus.join(","),
  ...(input.features?.buildScripts?{SEMIO_UNIT_CACHE_BUILD_SCRIPTS:"on"}:{}),
  ...(input.features?.brokerReplay?{SEMIO_UNIT_CACHE_BROKER_REPLAY:"on"}:{}),
 };
}

/** 🗺️ Where the cache root lives beside the compiler store's own directories. */
export function unitCacheRootV1(storeRoot:string):string{return join(storeRoot,"u");}

/** 📏️ The share of the image each part of the cache may use. */
export const UNIT_CACHE_SHARES_V1=Object.freeze({objects:policy.gc.casShare,lanes:policy.gc.laneShare,incremental:policy.gc.incrementalShare});

/** 🌐️ The language of operator messages: `SEMIO_LANGUAGE`, then the locale, English when neither names German. */
export function laneLanguageV1(environment:Readonly<Record<string,string|undefined>>):"en"|"de"{
 const value=(environment.SEMIO_LANGUAGE??environment.LC_ALL??environment.LC_MESSAGES??environment.LANG??"").toLowerCase();
 return value.startsWith("de")?"de":"en";
}

/** 💬️ One operator message with its placeholders filled. */
export function laneMessageV1(kind:keyof typeof schema.definitions.Messages.const,parameters:Readonly<Record<string,string|number>>,language:"en"|"de"):string{
 return schema.definitions.Messages.const[kind][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(parameters[name]??""));
}
