import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The authored rustc offload policy.
 * @see ../🧬️schema/🔣️.json */
export const OFFLOAD_POLICY_V1=schema.policy;
const WASM_LINK_ARGUMENTS=OFFLOAD_POLICY_V1.wasm.linkArguments.map(pattern=>new RegExp(pattern,"u"));
const LINK_ARGUMENTS=OFFLOAD_POLICY_V1.link.arguments.map(pattern=>new RegExp(pattern,"u"));

/** 🕸️ Whether a `--target` is a WebAssembly target whose linked output may be offloaded. */
export const offloadWasmTargetV1=(target:string|undefined):boolean=>target!==undefined&&OFFLOAD_POLICY_V1.wasm.targets.includes(target);

/** 🔗️ Whether one `-C link-arg` value is a wasm-ld argument of the release (`--max-memory=<bytes>`, `-zstack-size=<bytes>`): the only linker arguments besides the macOS install name and object prefix; the one list that the requester and the worker both read. */
export const offloadWasmLinkArgumentV1=(value:string):boolean=>WASM_LINK_ARGUMENTS.some(pattern=>pattern.test(value));

/** 🔩️ Whether one `-C link-arg` value is a native linker argument of the release (`-Wl,-undefined,dynamic_lookup` of a Node addon): the closed list of the schema, which the requester and the worker both read; for a WebAssembly target the wasm-ld list applies instead. */
export const offloadLinkArgumentV1=(value:string):boolean=>LINK_ARGUMENTS.some(pattern=>pattern.test(value));

const LIBRARY=/^(?:([a-z]{1,16})(?::[-+](?:bundle|verbatim|whole-archive|as-needed)(?:,[-+](?:bundle|verbatim|whole-archive|as-needed))*)?=)?([A-Za-z0-9_][A-Za-z0-9_.+-]{0,127})(?::([A-Za-z0-9_][A-Za-z0-9_.+-]{0,127}))?$/u;
const SEARCH_DIRECTORY=new RegExp(`^${OFFLOAD_POLICY_V1.virtual.native.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&")}[0-9]{1,3}$`,"u");

/** 📚️ Why one `-l` value (`[KIND[:MODIFIERS]=]NAME[:RENAME]`) may not be linked on another machine, `undefined` when it may: a static library is a file of a shipped search directory (or of the SDK), so any plain name goes; a dynamic library and a framework only when the schema lists the name as one that the worker resolves against its own SDK. */
export function offloadLinkLibraryRefusalV1(spec:string):string|undefined{
 const found=LIBRARY.exec(spec);
 if(found===null)return `-l ${spec.slice(0,60)}`;
 const kind=found[1]??"dylib",name=found[2]!;
 if(found[1]!==undefined&&!OFFLOAD_POLICY_V1.link.libraryKinds.includes(kind))return `-l kind ${kind}`;
 if(kind==="static")return undefined;
 if(found[3]!==undefined)return `-l ${spec.slice(0,60)}`;
 return (kind==="framework"?OFFLOAD_POLICY_V1.link.frameworks:OFFLOAD_POLICY_V1.link.systemLibraries).includes(name)?undefined:`-l ${kind} ${name}`;
}

/** 🧭️ Why one `-L KIND=DIR` value (not `dependency=`) may not reach another machine, `undefined` when it may: the kind is declared and the directory is a native search directory shipped with the action (`.semio-offload/n<k>`), never a path of the worker. */
export function offloadSearchDirectoryRefusalV1(value:string):string|undefined{
 const equals=value.indexOf("="),kind=equals<0?"":value.slice(0,equals);
 return equals>0&&OFFLOAD_POLICY_V1.link.searchKinds.includes(kind)&&SEARCH_DIRECTORY.test(value.slice(equals+1))?undefined:`-L ${value.slice(0,60)}`;
}

/** 🏠️ Whether a search directory is one that every machine of the platform has by itself (the SDK and the system library directories of the schema): it is dropped from the command line of an offload, never shipped, and the worker finds the same names in its own SDK. */
export const offloadSystemDirectoryV1=(directory:string):boolean=>OFFLOAD_POLICY_V1.link.systemDirectories.some(prefix=>directory===prefix||directory.startsWith(prefix+"/"));

/** 🚦️ Why a unit of these crate types for this target never offloads, `undefined` when it may: `lib`, `rlib`, `proc-macro` and `bin` always, `cdylib` for a WebAssembly target of the policy and `cdylib`, `dylib` and `staticlib` for every native one (the linker of the worker is the one of the platform); the other WebAssembly targets link with a toolchain that the worker does not have. */
export function offloadCrateTypeRefusalV1(types:readonly string[],target:string|undefined):string|undefined{
 const linked=offloadWasmTargetV1(target)?OFFLOAD_POLICY_V1.wasm.crateTypes:target?.startsWith("wasm")===true?[]:OFFLOAD_POLICY_V1.link.crateTypes;
 for(const type of types.length>0?types:["bin"]){
  if(OFFLOAD_POLICY_V1.crateTypes.includes(type)||linked.includes(type))continue;
  return `--crate-type ${type}`;
 }
 return undefined;
}

/** 🔗️ Whether a unit of these crate types for this target is linked by the machine that runs it with the host's native linker (a program, a procedural macro or a native library; never a WebAssembly target, never a plain library): such a unit needs a machine with the same SDK. */
export const offloadLinksNativelyV1=(types:readonly string[],target:string|undefined):boolean=>!offloadWasmTargetV1(target)&&target?.startsWith("wasm")!==true&&(types.length>0?types:["bin"]).some(type=>type==="bin"||type==="proc-macro"||OFFLOAD_POLICY_V1.link.crateTypes.includes(type));

/** 🧾️ The crate types a compiler command line names (`--crate-type <t>` and `--crate-type=<t>`, a comma list split). */
export const offloadCrateTypesOfV1=(args:readonly string[]):readonly string[]=>args.flatMap((argument,index)=>argument==="--crate-type"?(args[index+1]??"").split(","):argument.startsWith("--crate-type=")?argument.slice(13).split(","):[]);

/** 🎯️ The `--target` of a compiler command line, `undefined` for the host. */
export const offloadTargetOfV1=(args:readonly string[]):string|undefined=>args.flatMap((argument,index)=>argument==="--target"?[args[index+1]??""]:argument.startsWith("--target=")?[argument.slice(9)]:[])[0];

/** ⏳️ How long the action of a unit may run on a worker: the long limit of a wasm linked unit (a component final runs 10 to 31 minutes), the ordinary one for every other. */
export const offloadActionSecondsV1=(types:readonly string[],target:string|undefined):number=>offloadPeakMegabytesV1(types,target)>0?OFFLOAD_POLICY_V1.wasm.actionSeconds:OFFLOAD_POLICY_V1.timeouts.actionSeconds;

/** 💾️ The megabytes that placement reserves for a unit: the predicted peak of a wasm linked unit (the measured peaks of the release are 1.2 to 4.7 GiB, the cold bound of the admission broker 6 GiB), 0 for every other unit, which keeps the machine's own admission as the only memory check. */
export const offloadPeakMegabytesV1=(types:readonly string[],target:string|undefined):number=>offloadWasmTargetV1(target)&&types.some(type=>OFFLOAD_POLICY_V1.wasm.crateTypes.includes(type))?OFFLOAD_POLICY_V1.wasm.peakMegabytes:0;

/** 🎛️ How a build treats offloading: `distribute` places every unit worth a round trip on any ready machine of the fleet (this machine's slots are one more worker), `pin` places every unit on the one named machine, `never` stays local. */
export type OffloadModeV1="distribute"|"pin"|"never";
/** 🧭️ Why a unit stays here or its offload failed. */
export type OffloadReasonV1="not-portable"|"no-endpoint"|"no-credential"|"unmapped-path"|"inputs-too-large"|"unreadable-input"|"no-idle-machine"|"queue-timeout"|"infrastructure"|"toolchain-missing"|"path-dependent-output"|"worker-policy"|"incomplete-inputs"|"crate-type"|"source-kind"|"cheap"|"pin-failed"|"cancelled";

type Environment=Readonly<Record<string,string|undefined>>;

const PIN_ORIGIN=/^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/u;
/** 🎚️ Parses one setting text: `never`, `distribute` or `pin:<origin>`; `undefined` for anything else. */
export function offloadParseSettingV1(value:string|undefined):Readonly<{mode:OffloadModeV1;pin?:string}>|undefined{
 const prefix=OFFLOAD_POLICY_V1.mode.pinPrefix;
 if(value==="never"||value==="distribute")return{mode:value};
 if(value!==undefined&&value.startsWith(prefix)&&PIN_ORIGIN.test(value.slice(prefix.length)))return{mode:"pin",pin:value.slice(prefix.length)};
 return undefined;
}

/** 🎚️ Reads the setting from the environment; an unknown value is the default, never a failure. */
export const offloadSettingV1=(environment:Environment):Readonly<{mode:OffloadModeV1;pin?:string}>=>offloadParseSettingV1(environment[OFFLOAD_POLICY_V1.mode.environment])??{mode:OFFLOAD_POLICY_V1.mode.default as OffloadModeV1};

/** 🎚️ The mode of the environment (see `offloadSettingV1`). */
export const offloadModeV1=(environment:Environment):OffloadModeV1=>offloadSettingV1(environment).mode;

/** 📌️ The origin that every unit is pinned to, `undefined` outside the `pin` mode. */
export const offloadPinV1=(environment:Environment):string|undefined=>offloadSettingV1(environment).pin;

/** 💬️ Renders one catalogue text in one language. */
export function offloadTextV1(language:"en"|"de",key:string,variables:Readonly<Record<string,string|number>>={}):string{
 const message=(schema.messages[language] as Readonly<Record<string,string>>)[key];
 if(message===undefined)throw Error(`Declared offload text required: ${key}`);
 return message.replace(/\{(\w+)\}/gu,(_,name:string)=>String(variables[name]??""));
}

/** 🌍️ One text in every language, one line per language. */
export const offloadTextsV1=(key:string,variables:Readonly<Record<string,string|number>>={}):string=>(["en","de"] as const).map(language=>offloadTextV1(language,key,variables)).join("\n");

/** 🚩️ Takes `--offload=never|distribute|pin:<origin>` (or `--offload <setting>`) out of a command line before the delimiter `--`: the setting text and the arguments without it. An unknown or missing setting is refused in English and German; the flag may be given once. */
export function extractOffloadFlagV1(args:readonly string[]):Readonly<{args:readonly string[];mode?:string}>{
 const delimiter=args.indexOf("--"),head=delimiter<0?args:args.slice(0,delimiter),tail=delimiter<0?[]:args.slice(delimiter),kept:string[]=[];
 let mode:string|undefined;
 for(let index=0;index<head.length;index++){
  const argument=head[index]!;
  if(argument!=="--offload"&&!argument.startsWith("--offload=")){kept.push(argument);continue;}
  const value=argument==="--offload"?head[++index]:argument.slice("--offload=".length);
  if(mode!==undefined||offloadParseSettingV1(value)===undefined)throw Error(offloadTextsV1("flag.invalid"));
  mode=value!;
 }
 return Object.freeze({args:Object.freeze([...kept,...tail]),...mode===undefined?{}:{mode}});
}
