import {lstatSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
import { admitPlaygroundNativeHostV1 as admit, parsePlaygroundNativeHostV1 as parse, nativeHostSourceFactsV1 as facts,nativeHostArtifactPathV1 as artifact, declaredPlaygroundHostInputPathsV1 as inputs } from "./🟨️.mjs";

import type { PlaygroundNativeHostV1, NativeHostSourceViewV1 } from "./🟨️.mjs";
export type { PlaygroundNativeHostV1, NativeHostSourceViewV1 } from "./🟨️.mjs";

/** 🖥 Binds an optional owner executable to authored Cargo and Nx source identities. */
export function admitPlaygroundNativeHostV1(value:unknown,view:NativeHostSourceViewV1):PlaygroundNativeHostV1|undefined {
  return admit(value,(cratePath:string)=>facts(cratePath,view));
}

/** 📜 Decodes the exact authored native producer table. */
export function parsePlaygroundNativeHostV1(block:string,fieldName:"nativeHost"|"mcpHost"="nativeHost"):unknown {return parse(block,fieldName);}

/** 🧾 Admits declared producer identities and returns their exact authored input closure. */
export function declaredPlaygroundHostInputPathsV1(manifest:string,view:NativeHostSourceViewV1):readonly string[] {return inputs(manifest,view);}

/** 📂 Supplies local source facts while refusing filesystem indirection. */
export function nativeHostFilesystemViewV1(repoRoot:string):NativeHostSourceViewV1 {
  return {kind:(path)=>{try {const stat=lstatSync(resolve(repoRoot,path));return stat.isSymbolicLink()?"symlink":stat.isDirectory()?"directory":stat.isFile()?"file":null;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}},readText:(path)=>new TextDecoder("utf-8",{fatal:true}).decode(readFileSync(resolve(repoRoot,path)))};
}

/** 🚀 Selects the declared producer artifact from its actual Nx publication contract. */
export function playgroundNativeHostArtifactV1(repoRoot:string,value:unknown,profile:"dev"|"release",platform:NodeJS.Platform=process.platform):string|undefined {
  const view=nativeHostFilesystemViewV1(repoRoot),host=admitPlaygroundNativeHostV1(value,view);
  return host?resolve(repoRoot,artifact(host,profile,facts(host.cratePath,view),platform)):undefined;
}
