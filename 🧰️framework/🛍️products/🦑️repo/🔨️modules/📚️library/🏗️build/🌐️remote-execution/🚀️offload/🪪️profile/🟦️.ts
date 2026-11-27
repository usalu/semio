import {createHash} from "node:crypto";

const CODEGEN:ReadonlySet<string>=new Set(["opt-level","lto","codegen-units","debuginfo","target-cpu","target-feature"]);
const OFF:ReadonlySet<string>=new Set(["no","off","false","n"]);
const FAT:ReadonlySet<string>=new Set(["","yes","on","true","y","fat"]);

/** 🪪️ The compile-relevant profile of one rustc command line as a canonical text: `opt-level`, `lto` (`off`, `fat`, `thin`, `-` when unset), `codegen-units`, `debuginfo` (`-g` is 2, `-O` is opt-level 2), the target triple, `target-cpu` and the sorted `target-feature` set, `opt-level` and `debuginfo` default to rustc's `0` and every other field to `-` when the command line does not say; the last spelling of a flag wins, any other flag (crate name, metadata, paths, externs) is not part of it.
 * @see ./🧪️tests/🔣️.json */
export function unitProfileIdentityV1(args:readonly string[]):string{
 const values=new Map<string,string>(),features=new Set<string>();let target="-";
 const apply=(item:string):void=>{
  const at=item.indexOf("="),name=at<0?item:item.slice(0,at),value=at<0?"":item.slice(at+1);
  if(!CODEGEN.has(name))return;
  if(name==="target-feature"){for(const feature of value.split(","))if(feature!=="")features.add(feature);}
  else if(name==="lto")values.set(name,OFF.has(value)?"off":FAT.has(value)?"fat":value);
  else values.set(name,value);
 };
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  if((argument==="-C"||argument==="--codegen")&&index+1<args.length)apply(args[++index]!);
  else if(argument.startsWith("--codegen="))apply(argument.slice(10));
  else if(argument.startsWith("-C")&&argument.length>2)apply(argument.slice(2));
  else if(argument==="--target"&&index+1<args.length)target=args[++index]!;
  else if(argument.startsWith("--target="))target=argument.slice(9);
  else if(argument==="-O")values.set("opt-level","2");
  else if(argument==="-g")values.set("debuginfo","2");
 }
 const of=(name:string,fallback="-")=>values.get(name)??fallback;
 return`opt-level=${of("opt-level","0")};lto=${of("lto")};codegen-units=${of("codegen-units")};debuginfo=${of("debuginfo","0")};target=${target};target-cpu=${of("target-cpu")};target-feature=${features.size===0?"-":[...features].sort().join(",")}`;
}

/** 🔑️ The short key of {@link unitProfileIdentityV1} (10 hex digits of its SHA-256) that qualifies the peak evidence of a crate. */
export const unitProfileKeyV1=(args:readonly string[]):string=>createHash("sha256").update(unitProfileIdentityV1(args)).digest("hex").slice(0,10);

/** 🗝️ The name under which the peak evidence of `crate` compiled with this profile is kept: the profile key before the crate, so the crate suffix of a family (`_component`) still ends it. */
export const unitPeakKeyV1=(crate:string,profile:string|readonly string[]):string=>`${typeof profile==="string"?profile:unitProfileKeyV1(profile)}_${crate}`;

/** 🎚️ The rustc arguments that one resolved Cargo profile (the settings of the profile policy: opt-level, debug, codegen units and lto, where `false` passes no lto flag) puts on a compile for `target`, so a caller that knows the profile but no command line (the component pass) gets the very key of {@link unitProfileKeyV1}. */
export const unitProfileArgumentsV1=(settings:Readonly<{optLevel:string;debug:number;codegenUnits:number;lto:string|false;target:string}>):readonly string[]=>Object.freeze(["-C","opt-level="+settings.optLevel,"-C","debuginfo="+String(settings.debug),"-C","codegen-units="+String(settings.codegenUnits),...settings.lto===false?[]:["-C","lto="+settings.lto],"--target",settings.target]);
