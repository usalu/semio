import { withRepositoryCacheStoreV1 } from "@semio-tech/repo-lib/storage";
import { withRepositoryArtifactGroupV1, withRepositoryArtifactGroupTaskV1, withArtifactScratchV1, withArtifactDestinationTaskV1, withRepositoryArtifactDestinationV1, stageRepositoryArtifacts, readNativeCallerAuthorityV1 } from "@semio-tech/repo-lib/artifacts";
import {BundleScript} from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {runtimeCatalogBuildInventoryV1} from "@semio-tech/plugin-registry/catalog-view";
import {builtPluginModulesRootIn} from "@semio-tech/framework-os-dev/activation";
import {readArtifactBytesV1} from "../🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";
import { artifactsStorageSchema as publicationSchema } from "@semio-tech/repo-lib/artifacts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {join} from "node:path";
import {writeFile} from "node:fs/promises";
import {componentModuleOwnerV1} from "@semio-tech/plugin-registry/deployment";
const policy=publicationSchema.definitions.Protocol.const,protocol=schema.definitions.Protocol.const;
export interface PluginWasmSizeBreakdown{readonly totalBytes:number;readonly codeBytes:number;readonly dataBytes:number;readonly nameBytes:number;readonly otherCustomBytes:number;readonly functionCount:number;readonly memoryInitialPages:number|null;readonly memoryMaxPages:number|null}
export type PluginWasmSizeRow=PluginWasmSizeBreakdown&Readonly<{pluginId:string;file:string}>;
/** 🔢️ Reads one finite Wasm u32 with a caller-owned section boundary. */
export function readULEB128(bytes:Uint8Array,offset:number,end:number=bytes.length):Readonly<{value:number;next:number}>{
 if(!Number.isSafeInteger(offset)||offset<0||!Number.isSafeInteger(end)||end>bytes.length||end<offset)throw Error("Finite Wasm integer extent required");let value=0;for(let count=0;count<5;count++){if(offset>=end)throw Error("Truncated Wasm integer");const byte=bytes[offset++]!;if(count===4&&(byte&0xf0))throw Error("Wasm u32 overflow");value+=(byte&0x7f)*2**(count*7);if(!(byte&0x80))return{value,next:offset};}throw Error("Wasm u32 continuation overflow");
}
/** 📏️ Accounts finite core-module section bytes without opening files or requiring external runtime types. */
export function analyzePluginWasmModule(input:Uint8Array):PluginWasmSizeBreakdown{
 if(!(input instanceof Uint8Array)||input.length>policy.maximumOutputBytes||input.length<8||!Buffer.from(input.subarray(0,8)).equals(Buffer.from([0,97,115,109,1,0,0,0])))throw Error("Finite core Wasm bytes required");const bytes=input,totalBytes=bytes.length;let offset=8,codeBytes=0,dataBytes=0,nameBytes=0,otherCustomBytes=0,functionCount=0,memoryInitialPages:number|null=null,memoryMaxPages:number|null=null;
 while(offset<bytes.length){const id=bytes[offset++]!,section=readULEB128(bytes,offset),start=section.next,end=start+section.value;if(end>bytes.length||id>13)throw Error("Finite Wasm section required");if(id===10){codeBytes+=section.value;functionCount=readULEB128(bytes,start,end).value;}else if(id===11)dataBytes+=section.value;else if(id===5){const count=readULEB128(bytes,start,end);if(count.value){const flags=readULEB128(bytes,count.next,end);if(flags.value>1)throw Error("Core u32 memory limits required");const minimum=readULEB128(bytes,flags.next,end);memoryInitialPages=minimum.value;memoryMaxPages=flags.value===1?readULEB128(bytes,minimum.next,end).value:null;}}else if(id===0){const name=readULEB128(bytes,start,end);if(name.next+name.value>end)throw Error("Finite Wasm custom-section name required");if(new TextDecoder("utf-8",{fatal:true}).decode(bytes.subarray(name.next,name.next+name.value))==="name")nameBytes+=section.value;else otherCustomBytes+=section.value;}offset=end;
 }return{totalBytes,codeBytes,dataBytes,nameBytes,otherCustomBytes,functionCount,memoryInitialPages,memoryMaxPages};
}
/** 📊️ Measures only exact retained component products and publishes a separate selected-mode finite report. */
export class PluginSizeScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Plugin size takes no arguments");const mode=readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode,phase=scriptCompilerAdmissionControlV1(this.invocation,"browser:size",0),modules=builtPluginModulesRootIn(this.repoRoot,mode),catalog=runtimeCatalogBuildInventoryV1(),inventory=[...catalog.plugins,...catalog.extensions].filter(row=>row.directoryName!==undefined);if(!inventory.length)throw Error("Authored component inventory required");
  await withRepositoryCacheStoreV1(this.repoRoot,"products",this.invocation,phase,(store,storePhase)=>withRepositoryArtifactGroupV1({workspace:this.repoRoot,invocation:this.invocation,storage:store.storage,control:store.control,phaseControl:storePhase,requests:inventory.map(row=>({version:1,path:join(modules,row.directoryName!),owner:componentModuleOwnerV1(row.cratePath),maximumBytes:policy.maximumOutputBytes+policy.maximumManifestBytes,maximumFiles:policy.maximumFiles}))},group=>withRepositoryArtifactGroupTaskV1(group,this.invocation,async artifacts=>{
   const rows:PluginWasmSizeRow[]=[],encoded:string[]=[];let reportBytes=2;
   for(const declaration of inventory){const member=artifacts.artifacts.find(row=>row.path===join(modules,declaration.directoryName!));if(!member)throw Error("Exact retained component measurement required");for(const file of member.files.filter(name=>/^[^/]+\.core\d*\.wasm$/.test(name))){const row=Object.freeze({pluginId:declaration.pluginId,file,...analyzePluginWasmModule(await readArtifactBytesV1(join(member.path,file),policy.maximumOutputBytes,member.control.signal))}),text=JSON.stringify(row),extent=Buffer.byteLength(text)+(rows.length?1:0);if(rows.length>=policy.maximumFiles||extent>policy.maximumManifestBytes-reportBytes)throw Error("Size report record or byte grant exceeded");reportBytes+=extent;rows.push(row);encoded.push(text);}}
   rows.sort((a,b)=>b.totalBytes-a.totalBytes);const content="["+encoded.join(",")+"]";await artifacts.recheck();await withArtifactScratchV1({workspace:this.repoRoot,invocation:this.invocation,storage:store.storage,control:store.control,phaseControl:artifacts.control,request:{version:1,mode,owner:"browser/size-report"}},scratch=>withArtifactDestinationTaskV1(scratch,this.invocation,async sink=>{const path=join(sink.directory,protocol.report);await writeFile(path,content,{flag:"wx",signal:sink.control.signal});await withRepositoryArtifactDestinationV1({workspace:this.repoRoot,invocation:this.invocation,storage:store.storage,control:store.control,phaseControl:sink.control,directory:join(store.root,"browser",mode,protocol.namespace)},destination=>stageRepositoryArtifacts(destination,protocol.owner,new Map([[protocol.report,path]])));}));console.log(`Plugin size ${mode}: ${rows.length} core modules, ${rows.reduce((sum,row)=>sum+row.totalBytes,0)} bytes`);for(const row of rows)console.log(`  ${row.pluginId}/${row.file}: ${row.totalBytes} bytes, code ${row.codeBytes}, data ${row.dataBytes}`);
  })));
 }
}
