import {checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {posix,win32} from "node:path";
import type {CargoCompilationUnitV1} from "../🕸️units/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
type Unit=Pick<CargoCompilationUnitV1,"packageName"|"name"|"platform"|"profileName"|"kind"|"operation">;
const policy=schema.definitions.Policy.const;
/** 📏️ Models every known selected Cargo metadata, native-library and codegen-object suffix before compiler writes. */
export function cargoCompilerIntermediatePathsV1(buildDirectory:string,units:readonly unknown[],flavor:"posix"|"win32"=process.platform==="win32"?"win32":"posix"):readonly string[]{
 const path=flavor==="win32"?win32:posix,hash="0".repeat(policy.fingerprintCharacters),recipe="0".repeat(policy.recipeCharacters),codegen="0".repeat(policy.codegenUnitCharacters),disambiguator="0".repeat(policy.codegenDisambiguatorCharacters),paths=new Set<string>();
 for(const value of units){if(!value||typeof value!=="object"||Array.isArray(value))throw Error("Complete selected Cargo path unit required");const source=value as Record<string,unknown>,projected={packageName:source.packageName,name:source.name,platform:source.platform,profileName:source.profileName,kind:source.kind,operation:source.operation};if(validateJsonSchemaSubset(schema.definitions.Unit,projected,schema).length)throw Error("Closed selected Cargo path unit required");const unit=projected as Unit;if(["build-script-run","documentation"].includes(unit.kind)||["doc","doctest","run-custom-build"].includes(unit.operation))continue;for(const value of[unit.packageName,unit.name,unit.profileName])if(!value||value==="."||value===".."||/[\\/\0]/u.test(value))throw Error("Closed selected Cargo path component required");
  const target=unit.platform?(path.isAbsolute(unit.platform)?path.basename(unit.platform).replace(/\.json$/u,""):unit.platform):"";if(target==="."||target===".."||/[\\/\0]/u.test(target))throw Error("One selected Cargo target directory required");const profile=["dev","test"].includes(unit.profileName)?"debug":unit.profileName,name=unit.name.replaceAll("-","_"),out=path.join(buildDirectory,target,profile,"build",unit.packageName,hash,"out");
  for(const extension of policy.extensions){paths.add(path.join(out,"lib"+name+"-"+hash+"."+extension));paths.add(path.join(buildDirectory,"t",recipe,target,profile,"lib"+name+"."+extension));}
  paths.add(path.join(out,name+"-"+hash+"."+codegen+"."+disambiguator+".rcgu.o"));for(const extension of["","exe","pdb"])paths.add(path.join(buildDirectory,"t",recipe,target,profile,unit.name+(extension?"."+extension:"")));
 }
 return Object.freeze([...paths].sort());
}
/** 🚧️ Refuses either portable character measure over the unchanged materialized-path bound. */
export function requireCargoSelectedPathEnvelopeV1(buildDirectory:string,units:readonly unknown[],flavor:"posix"|"win32"=process.platform==="win32"?"win32":"posix"):readonly string[]{const paths=cargoCompilerIntermediatePathsV1(buildDirectory,units,flavor);for(const path of paths)if(path.includes("\0")||path.length>policy.maximumCharacters||[...path].length>policy.maximumCharacters)throw Error("Selected Cargo compiler path exceeds 256 characters: "+path);return paths;}

/** 🧵️ Checks one selected unit at a time under the received control before any compiler writer runs. */
export async function requireCargoSelectedPathEnvelopeControlledV1(buildDirectory:string,units:readonly unknown[],control:ResourceLeaseControl):Promise<void>{
 const started=performance.now(),failures=new Set<string>();let refusedUnits=0;checkResourceLeaseControl(control);if(!control.publish||!control.yieldContinuation)throw Error("Issued Cargo path validation progress required");
 for(let index=0;index<units.length;index++){checkResourceLeaseControl(control);const paths=cargoCompilerIntermediatePathsV1(buildDirectory,[units[index]]),invalid=paths.filter(path=>path.includes("\0")||path.length>policy.maximumCharacters||[...path].length>policy.maximumCharacters).sort((a,b)=>b.length-a.length||a.localeCompare(b));if(invalid.length){refusedUnits++;if(failures.size<policy.maximumDiagnosticPaths)failures.add(invalid[0]!);}else for(const path of paths)requireMaterializedPathV1(path);if((index+1)%32===0){await control.publish({resource:"cargo:path-envelope",mode:"shared",elapsedMs:performance.now()-started});checkResourceLeaseControl(control);await control.yieldContinuation();}}
 checkResourceLeaseControl(control);if(refusedUnits)throw Error("Selected Cargo compiler path exceeds 256 characters in "+refusedUnits+" selected units (showing at most "+policy.maximumDiagnosticPaths+" paths):\n"+[...failures].join("\n"));
}
