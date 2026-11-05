import {posix,win32} from "node:path";
import {requireMaterializedPathV1} from "../../../🔒️leases/🟦️.ts";

/** 📦️ Declared Cargo output names from one selected compilation unit. */
export type CargoCompilerOutputUnitV1=Readonly<{packageName:string;crateName:string;profile:string;platform:string|null}>;

/** 🆔️ Reads Cargo's package identity without assuming its manifest directory is its name. */
export function cargoUnitPackageNameV1(identity:string):string{
 const [location,fragment=""]=identity.split("#");
 return decodeURIComponent(fragment.includes("@")?fragment.slice(0,fragment.lastIndexOf("@")):location!.split("/").at(-1)!);
}

/** 📏️ Quotes intermediates and retained targets before any selected compiler unit starts. */
export function cargoCompilerOutputPathsV1(buildDirectory:string,unit:CargoCompilerOutputUnitV1,flavor:"posix"|"win32"=process.platform==="win32"?"win32":"posix"):readonly string[]{
 const paths=flavor==="win32"?win32:posix;
 if(!paths.isAbsolute(buildDirectory)||![unit.packageName,unit.crateName,unit.profile].every(value=>typeof value==="string"&&/^[A-Za-z0-9_-]+$/u.test(value))||unit.platform!==null&&typeof unit.platform!=="string")throw Error("Closed Cargo compiler output names required");
 const target=unit.platform===null?"":paths.isAbsolute(unit.platform)?paths.basename(unit.platform).replace(/\.json$/u,""):unit.platform;
 return ["rmeta","rlib","dylib","dll","pdb","d"].flatMap(extension=>[
  paths.join(buildDirectory,target,unit.profile,"build",unit.packageName,"0".repeat(16),"out",`lib${unit.crateName}-${"0".repeat(16)}.${extension}`),
  paths.join(buildDirectory,"t","0".repeat(32),target,unit.profile,`lib${unit.crateName}.${extension}`),
 ]);
}

/** 🚧️ Enforces the existing physical path guard across the complete transitive unit plan. */
export function requireCargoCompilerOutputPathsV1(buildDirectory:string,units:readonly CargoCompilerOutputUnitV1[]):void{
 for(const unit of units)for(const path of cargoCompilerOutputPathsV1(buildDirectory,unit))requireMaterializedPathV1(path);
}
