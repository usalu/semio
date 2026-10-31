import {readFile} from "node:fs/promises";
import {dirname,resolve} from "node:path";
import {readRustSourceClosureV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import corpus from "./🔣️.json";
const root=process.cwd();
export async function observe(manifest:string,cfg:readonly string[]=corpus.cfg){const path=resolve(root,manifest),document=Bun.TOML.parse(await readFile(path,"utf8")),library=document.lib;if(!library||typeof library!=="object"||!("path"in library)||typeof library.path!=="string")throw Error("Actual Cargo library entry required");return readRustSourceClosureV1([resolve(dirname(path),library.path)],root,cfg,async source=>{try{return await readFile(source)}catch(error){if(error instanceof Error&&"code"in error&&error.code==="ENOENT")return null;throw error;}},{manifestDirectory:dirname(path),generated:()=>{}});}
