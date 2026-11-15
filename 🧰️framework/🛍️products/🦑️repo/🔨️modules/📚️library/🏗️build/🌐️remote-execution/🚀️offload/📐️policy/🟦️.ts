import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The authored rustc offload policy.
 * @see ../🧬️schema/🔣️.json */
export const OFFLOAD_POLICY_V1=schema.policy;
/** 🎛️ How a build treats offloading: `always` tries the fleet first, `auto` only when this machine would wait, `never` stays local. */
export type OffloadModeV1="always"|"auto"|"never";
/** 🧭️ Why a unit stays here or its offload failed. */
export type OffloadReasonV1="not-portable"|"no-endpoint"|"no-credential"|"unmapped-path"|"inputs-too-large"|"unreadable-input"|"no-idle-machine"|"queue-timeout"|"infrastructure"|"toolchain-missing"|"path-dependent-output"|"worker-policy"|"incomplete-inputs"|"cancelled";

type Environment=Readonly<Record<string,string|undefined>>;

/** 🎚️ Reads the mode from the environment; an unknown value is the default, never a failure. */
export function offloadModeV1(environment:Environment):OffloadModeV1{
 const value=environment[OFFLOAD_POLICY_V1.mode.environment];
 return value==="always"||value==="auto"||value==="never"?value:OFFLOAD_POLICY_V1.mode.default as OffloadModeV1;
}

/** 💬️ Renders one catalogue text in one language. */
export function offloadTextV1(language:"en"|"de",key:string,variables:Readonly<Record<string,string|number>>={}):string{
 const message=(schema.messages[language] as Readonly<Record<string,string>>)[key];
 if(message===undefined)throw Error(`Declared offload text required: ${key}`);
 return message.replace(/\{(\w+)\}/gu,(_,name:string)=>String(variables[name]??""));
}

/** 🌍️ One text in every language, one line per language. */
export const offloadTextsV1=(key:string,variables:Readonly<Record<string,string|number>>={}):string=>(["en","de"] as const).map(language=>offloadTextV1(language,key,variables)).join("\n");

/** 🚩️ Takes `--offload=always|auto|never` (or `--offload <mode>`) out of a command line before the delimiter `--`: the mode and the arguments without it. An unknown or missing mode is refused in English and German; the flag may be given once. */
export function extractOffloadFlagV1(args:readonly string[]):Readonly<{args:readonly string[];mode?:OffloadModeV1}>{
 const delimiter=args.indexOf("--"),head=delimiter<0?args:args.slice(0,delimiter),tail=delimiter<0?[]:args.slice(delimiter),kept:string[]=[];
 let mode:OffloadModeV1|undefined;
 for(let index=0;index<head.length;index++){
  const argument=head[index]!;
  if(argument!=="--offload"&&!argument.startsWith("--offload=")){kept.push(argument);continue;}
  const value=argument==="--offload"?head[++index]:argument.slice("--offload=".length);
  if(mode!==undefined||(value!=="always"&&value!=="auto"&&value!=="never"))throw Error(offloadTextsV1("flag.invalid"));
  mode=value;
 }
 return Object.freeze({args:Object.freeze([...kept,...tail]),...mode===undefined?{}:{mode}});
}
