"use strict";
/** 🔌️ Reads the manager descriptors for plain JavaScript consumers (inference components, scripts) that cannot import the TypeScript registry; the descriptors stay the single definition. */
const catalog=require("./🔣️catalog.json"),policy=require("./🔣️policy.json");
const descriptors=Object.freeze({
 "🍞️bun":require("./🍞️bun/🔣️.json"),
 "🐍️uv":require("./🐍️uv/🔣️.json"),
 "🐹️go":require("./🐹️go/🔣️.json"),
 "📦️npm":require("./📦️npm/🔣️.json"),
 "🔷️dotnet":require("./🔷️dotnet/🔣️.json"),
 "🦀️cargo":require("./🦀️cargo/🔣️.json"),
});
const protocolEnvironment="SEMIO_PACKAGE_MANAGER_{ECOSYSTEM}";

/** 🧾️ Summarizes one descriptor into the values consumers need. */
function summarizeV1(descriptor){
 const primary=descriptor.locks.find(lock=>lock.role==="primary");
 return Object.freeze({
  id:descriptor.id,label:descriptor.label,ecosystem:descriptor.ecosystem,primaryLockName:primary.name,lockFileNames:Object.freeze(descriptor.locks.map(lock=>lock.name)),
  manifestFileNames:Object.freeze([...descriptor.manifests]),scopeConfigName:descriptor.configurations.scope[0]??null,
  configurationNames:role=>Object.freeze([...descriptor.configurations[role]]),generationDirectory:descriptor.generationDirectory,hostEnvironment:Object.freeze([...descriptor.runtime.hostEnvironment]),graphComponent:descriptor.graphComponent,ownedModules:Object.freeze([...descriptor.ownedModules]),
 });
}
/** 🆔️ Reads one manager by id. */
function managerV1(id){
 const folder=catalog.adapters[id];
 if(!folder)throw Error("Registered package manager required: "+id);
 const descriptor=descriptors[folder];
 if(!descriptor)throw Error("Registered package manager descriptor required: "+folder);
 return summarizeV1(descriptor);
}
/** 🥾️ Selects the manager of one ecosystem: the policy table, overridable per ecosystem through the neutral environment variable. */
function selectedManagerV1(ecosystem,environment=process.env){
 const id=environment[protocolEnvironment.replace("{ECOSYSTEM}",ecosystem.toUpperCase().replace(/-/gu,"_"))]||policy.selection[ecosystem];
 if(!id)throw Error("Selected package manager required: "+ecosystem);
 const manager=managerV1(id);
 if(manager.ecosystem!==ecosystem)throw Error("Package manager of the selected ecosystem required: "+ecosystem+" "+id);
 return manager;
}
/** 🗃️ Lists every registered manager. */
const allManagersV1=()=>Object.keys(catalog.adapters).sort().map(managerV1);
/** 🔏️ Lists the absolute path of every file whose content decides what the accessor answers, so revisioned consumers can include them in their implementation revision. */
const sourceFilesV1=()=>{
 const path=require("node:path");
 return Object.freeze([__filename,...["🔣️catalog.json","🔣️policy.json",...Object.values(catalog.adapters).map(folder=>folder+"/🔣️.json")].map(name=>path.join(__dirname,name))]);
};
module.exports={managerV1,selectedManagerV1,allManagersV1,sourceFilesV1};
