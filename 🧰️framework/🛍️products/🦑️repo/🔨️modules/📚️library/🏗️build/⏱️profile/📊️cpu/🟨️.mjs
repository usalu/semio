import {posix,win32,join} from "node:path";
import {existsSync,mkdirSync} from "node:fs";
import {fileURLToPath} from "node:url";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const protocol=schema.definitions.Protocol.const;
let sequence=0;

/** 📊️ Returns only the fixed CPU profiler flags for one explicit, bounded diagnostic request. */
export function repoCpuProfileArgumentsV1(environment,context){
 const wire=environment[protocol.environmentVariable];
 if(wire===undefined)return Object.freeze([]);
 if(typeof wire!=="string"||Buffer.byteLength(wire)>protocol.maximumRequestBytes)throw Error("Bounded CPU profile request required / Begrenzte CPU-Profilanfrage erforderlich");
 const request=JSON.parse(wire);
 if(!request||typeof request!=="object"||Array.isArray(request)||Object.keys(request).sort().join(",")!=="directory,version"||request.version!==schema.definitions.Request.properties.version.const)throw Error("Closed CPU profile request required / Geschlossene CPU-Profilanfrage erforderlich");
 const directory=request.directory,system=context.platform==="win32"?win32:posix;
 if(!["linux","darwin","win32"].includes(context.platform)||!Number.isSafeInteger(context.pid)||context.pid<1||context.pid>protocol.maximumProcessId||!Number.isSafeInteger(context.sequence)||context.sequence<1||context.sequence>protocol.maximumSequence)throw Error("Finite CPU profile process identity required / Endliche CPU-Prozessidentität erforderlich");
 if(typeof directory!=="string"||!system.isAbsolute(directory)||system.resolve(directory)!==directory||directory.includes("\0"))throw Error("Explicit canonical CPU artifact directory required / Explizites kanonisches CPU-Artefaktverzeichnis erforderlich");
 const name="repo-"+context.pid+"-"+context.sequence+protocol.artifactSuffix;
 if(system.join(directory,name).length>protocol.maximumPathCharacters)throw Error("Bounded CPU profile output path required / Begrenzter CPU-Ausgabepfad erforderlich");
 return Object.freeze([protocol.enabledFlag,protocol.intervalFlag+"="+protocol.samplingIntervalMicroseconds,protocol.directoryFlag+"="+directory,protocol.nameFlag+"="+name]);
}

/** 📁️ Prepares the caller's explicit diagnostic directory before the qualified driver starts its captured command. */
export function prepareRepoCpuProfileArgumentsV1(environment){
 const args=repoCpuProfileArgumentsV1(environment,{platform:process.platform,pid:process.pid,sequence:sequence+1});
 if(!args.length)return args;
 const directory=args[2].slice(protocol.directoryFlag.length+1),name=args[3].slice(protocol.nameFlag.length+1);
 mkdirSync(directory,{recursive:true});
 if(existsSync(join(directory,name)))throw Error("Fresh CPU profile output required / Neue CPU-Profilausgabe erforderlich");
 sequence++;
 return args;
}

/** 🍞️ Selects the fixed sampler preload because Bun test does not emit CLI CPU profiles. */
export function prepareRepoCpuTestProfileArgumentsV1(environment){
 const enabled=repoCpuProfileArgumentsV1(environment,{platform:process.platform,pid:process.pid,sequence:sequence+1});if(!enabled.length)return Object.freeze([]);
 const entry=fileURLToPath(new URL("./🍞️bun/📜️script.mjs",import.meta.url));if(entry.length>protocol.maximumPathCharacters)throw Error("Bounded CPU sampler source path required");return Object.freeze([protocol.testPreloadFlag,entry]);
}
