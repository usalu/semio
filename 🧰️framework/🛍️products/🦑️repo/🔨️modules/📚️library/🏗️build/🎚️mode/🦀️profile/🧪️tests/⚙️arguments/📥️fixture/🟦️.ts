import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

/** 🧫️ Holds bounded Source bytes for one real Cargo argument observation. */
export interface CargoProfileFixtureV1{readonly files:readonly Readonly<{path:string;content:string}>[];readonly crates:readonly string[]}
type ProfileCase={name:string;profile:"prototype"|"release";settings:Record<string,string|number|boolean>};

/** 📦️ Authors the same closed profile for one root or an explicit two-rlib dependency graph. */
export function createCargoProfileFixtureV1(input:unknown,context:unknown):CargoProfileFixtureV1{
 if(validateJsonSchemaSubset(schema.properties.cases.items,input).length||typeof context!=="string"||!schema.properties.contexts.const.includes(context))throw Error("Closed Cargo profile fixture and context required");const row=input as ProfileCase,dependency=context==="dependency",settings=Object.entries(row.settings).map(([key,value])=>key+"="+JSON.stringify(value)).join("\n")+"\n",profile="[profile."+row.profile+"]\n",library='[lib]\npath="lib.rs"\ncrate-type=["rlib"]\n',files=[
  {path:"Cargo.toml",content:'[package]\nname="profile_arguments"\nversion="0.0.0"\nedition="2021"\n[workspace]\nresolver="2"\nmembers='+JSON.stringify(dependency?["leaf"]:[])+"\n"+library+(dependency?'[dependencies]\nprofile_dependency={path="leaf"}\n':"")+profile+(row.profile==="prototype"?'inherits="dev"\n':"")+settings},
  {path:"lib.rs",content:'#![no_std]\npub const VALUE:u8=7;\npub fn value()->u8{'+(dependency?"profile_dependency::value()+":"")+'VALUE}\n'},
  {path:".cargo/config.toml",content:profile+settings},
  ...(dependency?[{path:"leaf/Cargo.toml",content:'[package]\nname="profile_dependency"\nversion="0.0.0"\nedition="2021"\n'+library},{path:"leaf/lib.rs",content:'#![no_std]\npub fn value()->u8{11}\n'}]:[])
 ];if(files.reduce((bytes,file)=>bytes+Buffer.byteLength(file.content),0)>16384)throw Error("Bounded Cargo profile fixture Source required");return Object.freeze({files:Object.freeze(files.map(file=>Object.freeze(file))),crates:Object.freeze(dependency?["profile_dependency","profile_arguments"]:["profile_arguments"])});
}
