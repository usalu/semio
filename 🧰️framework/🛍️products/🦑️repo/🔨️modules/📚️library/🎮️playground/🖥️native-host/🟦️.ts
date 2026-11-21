import {lstatSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {parseStoreReceiptV1,storeReceiptOutputV1} from "../../🏗️build/🧾️store-receipt/🟨️.mjs";
import { admitPlaygroundNativeHostV1 as admit, parsePlaygroundNativeHostV1 as parse, nativeHostSourceFactsV1 as facts,nativeHostArtifactPathV1 as artifact, declaredPlaygroundHostInputPathsV1 as inputs,nativeHostScriptAdmissionV1 } from "./🟨️.mjs";

import type { PlaygroundNativeHostV1, NativeHostSourceViewV1 } from "./🟨️.mjs";
export type { PlaygroundNativeHostV1, NativeHostSourceViewV1, NativeHostScriptAdmissionV1 } from "./🟨️.mjs";

/** 🖥 Binds an optional owner executable to authored Cargo and portable project source identities. */
export function admitPlaygroundNativeHostV1(value:unknown,view:NativeHostSourceViewV1,context?:string):PlaygroundNativeHostV1|undefined {
  return admit(value,(cratePath:string)=>facts(cratePath,view),context);
}

/** 📜 Decodes the exact authored native producer table. */
export function parsePlaygroundNativeHostV1(block:string,fieldName:"nativeHost"|"mcpHost"="nativeHost"):unknown {return parse(block,fieldName);}

/** 🧾 Admits declared producer identities and returns their exact authored input closure. */
export function declaredPlaygroundHostInputPathsV1(manifest:string,view:NativeHostSourceViewV1):readonly string[] {return inputs(manifest,view);}

/** 📏️ Binds a genuine textual source view to explicit bounded script admission. */
export function nativeHostTextSourceViewV1(view:Pick<NativeHostSourceViewV1,"kind"|"readText">):NativeHostSourceViewV1 {
  return {kind:path=>view.kind(path),readText:path=>view.readText(path),admitScript:path=>nativeHostScriptAdmissionV1(view.readText(path))};
}

/** 📂 Supplies local source facts while refusing filesystem indirection. */
export function nativeHostFilesystemViewV1(repoRoot:string):NativeHostSourceViewV1 {
  return nativeHostTextSourceViewV1({kind:(path)=>{try {const stat=lstatSync(resolve(repoRoot,path));return stat.isSymbolicLink()?"symlink":stat.isDirectory()?"directory":stat.isFile()?"file":null;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}},readText:(path)=>new TextDecoder("utf-8",{fatal:true}).decode(readFileSync(resolve(repoRoot,path)))});
}

/** 🏗️ Resolves the authored self-contained host task without expanding a graph or interpreting a shell command. */
export function nativeHostTaskV1(value:unknown,mode:"prototype"|"release",view:NativeHostSourceViewV1){
  let projectSource:string|undefined;
  const host=admitPlaygroundNativeHostV1(value,{...view,readText:path=>{
    const source=view.readText(path);
    if(path.endsWith("/📋️project.json"))projectSource=source;
    return source;
  }});
  if(!host||!projectSource)throw Error("Declared native host source required");
  const target=JSON.parse(projectSource).targets[host.target],configuration=target.configurations[mode];
  const recipe={command:target.options.command,arguments:configuration?.args??target.options.args??[],environment:{...target.options.env,...configuration?.env}};
  const outputs=configuration?.outputs??target.outputs;
  const hasDependencies=(value:unknown)=>value!==undefined&&(!Array.isArray(value)||value.length!==0);
  if(!configuration||configuration.command!==undefined||configuration.cwd!==undefined||configuration.options!==undefined||
    hasDependencies(configuration.dependsOn)||hasDependencies(target.dependsOn)||recipe.environment.SEMIO_BUILD_MODE!==mode||
    target.options.forwardAllArgs===false&&recipe.arguments.length>0||
    validateJsonSchemaSubset(schema.$defs.NativeHostTaskV1,recipe).length||
    !Array.isArray(outputs)||outputs.length!==1||outputs[0]!==storeReceiptOutputV1(host.target)){
    throw Error("Authored native host task requires a self-contained profiled script and its sole receipt");
  }
  return Object.freeze({host,script:host.cratePath+"/📜️script.ts",arguments:Object.freeze([...recipe.command.split(" ").slice(2),...recipe.arguments]),outputs:Object.freeze([outputs[0]])});
}

/** 🚀 Selects the declared producer artifact through the store receipt its last build recorded; a producer that was not built has no receipt and is refused. */
export function playgroundNativeHostArtifactV1(repoRoot:string,value:unknown,profile:"prototype"|"release",platform:NodeJS.Platform=process.platform):string|undefined {
  const view=nativeHostFilesystemViewV1(repoRoot),host=admitPlaygroundNativeHostV1(value,view);
  return host?resolve(repoRoot,artifact(host,profile,facts(host.cratePath,view),platform,parseStoreReceiptV1(view.readText(`${host.cratePath}/${storeReceiptOutputV1(host.target).slice("{projectRoot}/".length)}`)))):undefined;
}
