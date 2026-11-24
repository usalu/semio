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

/** ⏰️ How long an offloaded action may execute on the fleet before the unit compiles here instead: `factor` times the unit's recorded local cost (`predictedMilliseconds`), at least the floor and at most the cap; a unit without a record takes the unknown value. A linked wasm unit and a command (`linked`) have their own unknown value and cap, a component final runs 10 to 31 minutes. */
export function offloadDeadlineMillisecondsV1(predictedMilliseconds:number|undefined,linked:boolean):number{
 const deadline=OFFLOAD_POLICY_V1.deadline,cap=linked?deadline.linkedCapMilliseconds:deadline.capMilliseconds,unknown=linked?deadline.linkedUnknownMilliseconds:deadline.unknownMilliseconds;
 if(predictedMilliseconds===undefined||!Number.isFinite(predictedMilliseconds)||predictedMilliseconds<0)return Math.min(cap,unknown);
 return Math.min(cap,Math.max(deadline.floorMilliseconds,Math.ceil(predictedMilliseconds*deadline.factor)));
}

/** 💾️ The megabytes that placement reserves for a unit: the predicted peak of a wasm linked unit (the measured peaks of the release's component finals are 0.4 to 6.5 GiB; this is the cold bound that the family estimate replaces once enough crates were measured), 0 for every other unit (the static part: whether the unit is a linked wasm unit; the size a unit is placed with is `offloadUnitPeakMegabytesV1`). */
export const offloadPeakMegabytesV1=(types:readonly string[],target:string|undefined):number=>offloadWasmTargetV1(target)&&types.some(type=>OFFLOAD_POLICY_V1.wasm.crateTypes.includes(type))?OFFLOAD_POLICY_V1.wasm.peakMegabytes:0;

/** 👪️ The peak a crate of the family is expected to need before it was measured: the `peak.family.quantilePercent` quantile (linear between closest ranks, R-7) of the recorded peaks of the family's crates, one sample per crate; `undefined` while fewer than `peak.family.minimumCrates` crates were measured (the cold bound applies).
 * @see ../../🧪️tests/🏋️unit-admission/🐍️.py */
export function offloadFamilyPeakMegabytesV1(samples:readonly number[]):number|undefined{
 const family=OFFLOAD_POLICY_V1.peak.family,sorted=samples.filter(value=>Number.isFinite(value)&&value>0).sort((left,right)=>left-right);
 if(sorted.length<family.minimumCrates)return undefined;
 const position=(sorted.length-1)*family.quantilePercent/100,lower=Math.floor(position);
 return Math.ceil(lower+1>=sorted.length?sorted[lower]!:sorted[lower]!+(position-lower)*(sorted[lower+1]!-sorted[lower]!));
}

/** 🕰️ The compile time a crate of the family is expected to need before it was measured: the same quantile as `offloadFamilyPeakMegabytesV1` over the recorded local compile times (milliseconds) of the family's crates, `undefined` while fewer than `peak.family.minimumCrates` crates were measured (the seed `finals.seedMilliseconds` applies).
 * @see ../../🧪️tests/🏁️finals/🐍️.py */
export const offloadFamilyMillisecondsV1=(samples:readonly number[]):number|undefined=>offloadFamilyPeakMegabytesV1(samples);

/** 🏷️ Whether a crate belongs to the family whose peaks estimate a crate that was never measured (its name ends in `peak.family.crateSuffix`). */
export const offloadFamilyCrateV1=(crate:string):boolean=>crate.endsWith(OFFLOAD_POLICY_V1.peak.family.crateSuffix);

/** 🏋️ The megabytes placement reserves for one unit: the peak measured here for the crate (`learned`) plus the margin of the schema, between its bounds; else for a linked wasm unit the estimate of its family (`family`, see `offloadFamilyPeakMegabytesV1`) plus the same margin, else the unknown default (`peak.unknownMegabytes`, for a linked wasm unit the cold bound `wasm.peakMegabytes`), so a worker admits the unit by its own size instead of one constant.
 * @see ../../🧪️tests/🏋️unit-admission/🐍️.py */
export function offloadUnitPeakMegabytesV1(learned:number|undefined,types:readonly string[],target:string|undefined,family?:number):number{
 const peak=OFFLOAD_POLICY_V1.peak,margin=(megabytes:number)=>Math.min(peak.maximumMegabytes,Math.max(peak.minimumMegabytes,Math.ceil(megabytes*(100+peak.marginPercent)/100)));
 if(learned!==undefined&&Number.isFinite(learned)&&learned>0)return margin(learned);
 if(family!==undefined&&Number.isFinite(family)&&family>0&&offloadPeakMegabytesV1(types,target)>0)return margin(family);
 return Math.max(peak.unknownMegabytes,offloadPeakMegabytesV1(types,target));
}

/** 💎️ What makes a unit expensive and where the number comes from. */
export type OffloadExpenseV1=Readonly<{expensive:boolean;milliseconds:number|undefined;megabytes:number|undefined;source:"record"|"family"|"seed"|"none"}>;

/** 💎️ Whether a unit is expensive enough to prefer a worker with headroom over a free slot of this machine (`finals`): its predicted local compile time (the recorded one, else for a linked wasm unit the family estimate, else the seed) reaches `finals.minimumMilliseconds`, or its placed peak (`placedMegabytes`, counted only for a unit with a learned peak or a linked wasm unit: the unknown default of an ordinary unit says nothing) reaches `finals.minimumMegabytes`. A unit with neither signal is never expensive.
 * @see ../../🧪️tests/🏁️finals/🐍️.py */
export function offloadExpenseV1(input:Readonly<{recordedMilliseconds?:number;familyMilliseconds?:number;learnedMegabytes?:number;placedMegabytes:number;linked:boolean}>):OffloadExpenseV1{
 const finals=OFFLOAD_POLICY_V1.finals,source=input.recordedMilliseconds!==undefined?"record" as const:!input.linked?"none" as const:input.familyMilliseconds!==undefined?"family" as const:"seed" as const;
 const milliseconds=source==="record"?input.recordedMilliseconds:source==="family"?input.familyMilliseconds:source==="seed"?finals.seedMilliseconds:undefined,megabytes=input.learnedMegabytes!==undefined||input.linked?input.placedMegabytes:undefined;
 return Object.freeze({expensive:milliseconds!==undefined&&milliseconds>=finals.minimumMilliseconds||megabytes!==undefined&&megabytes>=finals.minimumMegabytes,milliseconds,megabytes,source});
}

/** 🧠️ The peak a crate is known by after one more measurement: a larger one at once (an under-estimate is corrected by the first compile that shows it), a smaller one only halfway, so one lucky run never shrinks it.
 * @see ../../🧪️tests/🏋️unit-admission/🐍️.py */
export const offloadLearnPeakMegabytesV1=(before:number|undefined,measured:number):number=>before===undefined||measured>=before?Math.ceil(measured):Math.ceil((before+measured)/2);

/** 💀️ What a crate is known by after a worker killed its unit (exit `peak.killedExit`) while it was placed with `placed` megabytes: `killedFactor` times that, so the next placement asks for a machine that has the room; `undefined` for any other exit.
 * @see ../../🧪️tests/🏋️unit-admission/🐍️.py */
export const offloadKilledPeakMegabytesV1=(exitCode:number,placed:number,learned:number|undefined):number|undefined=>exitCode===OFFLOAD_POLICY_V1.peak.killedExit?Math.min(OFFLOAD_POLICY_V1.peak.maximumMegabytes,Math.max(learned??0,placed*OFFLOAD_POLICY_V1.peak.killedFactor)):undefined;

/** 🎛️ How a build treats offloading: `distribute` places every unit worth a round trip on any ready machine of the fleet (this machine's slots are one more worker), `pin` places every unit on the one named machine, `never` stays local. */
export type OffloadModeV1="distribute"|"pin"|"never";
/** 🧭️ Why a unit stays here or its offload failed. */
export type OffloadReasonV1="not-portable"|"no-endpoint"|"no-credential"|"unmapped-path"|"inputs-too-large"|"unreadable-input"|"no-idle-machine"|"queue-timeout"|"infrastructure"|"toolchain-missing"|"path-dependent-output"|"worker-policy"|"incomplete-inputs"|"crate-type"|"source-kind"|"cheap"|"local-slot"|"unmeasured"|"final-wait"|"pin-failed"|"cancelled";

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
