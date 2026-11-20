import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {selectPackageManagerV1,packageManagerRegistryV1} from "../../../../../../📦️packages/🟦️typescript/💾️storage/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";
const javascriptManagerV1=selectPackageManagerV1("javascript"),javascriptLockV1=javascriptManagerV1.primaryLockName();
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
/** 🔗️ Keeps the external pins of a recipe: a `workspace:*` link names first-party code the captured sources resolve, never a package the installer fetches, and the lock holds no row for it. */
export function nxExternalPinsV1(dependencies:Readonly<Record<string,string>>):Record<string,string>{
 return Object.fromEntries(Object.entries(dependencies).filter(([,version])=>version!=="workspace:*"));
}

/** 🚫️ The typed refusal of a pinned dependency recipe or lock that is not closed: names the first violations of the recipe (`/type is not allowed`) or the lock shape in English and German, keeps the authored message, and is rendered by the CLI boundary as itself (class `nx-recipe`, never "Internal error"). */
export class NxRecipeRefusedV1 extends Error{
 readonly texts:Readonly<{en:string;de:string}>;readonly violations:readonly string[];
 constructor(violations:readonly string[]){
  super("Closed pinned dependency recipe required");this.name="NxRecipeRefused";this.violations=Object.freeze([...violations]);
  const shown=violations.slice(0,6).join("; ")+(violations.length>6?` (+${violations.length-6})`:"");
  this.texts=Object.freeze({
   en:violations.length?`The pinned dependency recipe of the build tools (package.json of the tools recipe) is not closed: ${shown}. Add the field to the Recipe definition of the recipe schema or remove it from package.json.`:`The pinned dependency lock of the build tools (bun.lock of the tools recipe) is not a JSON object. Restore the lock file.`,
   de:violations.length?`Das gepinnte Abhängigkeitsrezept der Build-Werkzeuge (package.json des Werkzeugrezepts) ist nicht geschlossen: ${shown}. Das Feld in die Recipe-Definition des Rezeptschemas aufnehmen oder aus package.json entfernen.`:`Die gepinnte Abhängigkeitssperre der Build-Werkzeuge (bun.lock des Werkzeugrezepts) ist kein JSON-Objekt. Die Sperrdatei wiederherstellen.`
  });
 }
}

/** 📐️ Captures exact dependency installation inputs separately from first-party SDK code. */
export function nxDependencyInstallFilesV1(recipe:unknown,lock:unknown):Map<string,Buffer>{
 const violations=validateJsonSchemaSubset(schema.definitions.Recipe,recipe,schema),lockClosed=!!lock&&typeof lock==="object"&&!Array.isArray(lock);
 if(violations.length||!lockClosed)throw new NxRecipeRefusedV1(violations.map(String));
 const value=recipe as {name:string;private:true;packageManager:string;dependencies:Record<string,string>},captured={...lock as Record<string,unknown>};delete captured.patchedDependencies;
 const files=new Map([["package.json",Buffer.from(JSON.stringify({name:value.name,private:value.private,packageManager:value.packageManager,dependencies:nxExternalPinsV1(value.dependencies)})+"\n")],[javascriptLockV1,Buffer.from(JSON.stringify(captured)+"\n")]]);
 if([...files.values()].reduce((total,bytes)=>total+bytes.byteLength,0)>schema.definitions.Protocol.const.maximumInputBytes)throw Error("Pinned dependency input byte ceiling exceeded");return files;
}
