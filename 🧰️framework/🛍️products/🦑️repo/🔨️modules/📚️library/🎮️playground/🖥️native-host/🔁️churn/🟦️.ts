import {randomBytes} from "node:crypto";
import {writeSync} from "node:fs";
import {relative,resolve,isAbsolute,sep} from "node:path";
import {ConcurrentEditErrorV1,churnProtocolV1,concurrentEditOfV1,concurrentEditV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {OwnedCommandFailure} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {formatStampProtocolV1} from "../../../🏗️build/🧷️format-stamps/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

export const nativeHostSourceEditProtocolV1=Object.freeze(schema.$defs.Protocol.const);
const {environment,marker,requestBytes,maximumCharacters}=nativeHostSourceEditProtocolV1;
const requestPattern=new RegExp(schema.properties.request.pattern,"u");
type SourceEdit=Readonly<{version:1;request:string;reason:"input";path:string}>;

function readSourceEdit(value:unknown,workspace:string):SourceEdit|undefined {
  if(validateJsonSchemaSubset(schema,value).length)return;
  const row=value as SourceEdit;
  if(!isAbsolute(row.path)||resolve(row.path)!==row.path||/[\u0000\r\n]/u.test(row.path))return;
  const path=relative(workspace,row.path);
  if(!path||path===".."||path.startsWith(".."+sep)||isAbsolute(path))return;
  return row;
}

/** 📡️ Takes a private launch identifier before any Cargo child can inherit it and reports only terminal typed Source drift. */
export function takeNativeHostSourceEditReporterV1(env:NodeJS.ProcessEnv,workspace:string):((error:unknown)=>void)|undefined {
  const request=env[environment];
  delete env[environment];
  if(!request||!requestPattern.test(request))return;
  return error=>{
    if(!(error instanceof ConcurrentEditErrorV1))return;
    const edit=concurrentEditOfV1(error),buildRank=churnProtocolV1().profiles.build.rank;
    if(!edit||(edit.rank!==buildRank-1&&edit.rank!==buildRank))return;
    const row=readSourceEdit({version:1,request,reason:edit.reason,path:edit.path},workspace);
    if(row)writeSync(2,marker+JSON.stringify(row)+"\n");
  };
}

/** 🛰️ Restores one unsignalled terminal source edit for a fresh Original after the caller retires the failed receipt. */
export function createNativeHostSourceEditReceiverV1(workspace:string) {
  const request=randomBytes(requestBytes).toString("hex");
  let seen=false,source:SourceEdit|undefined;
  return Object.freeze({
    environment:Object.freeze({[environment]:request}),
    observe(line:string,stream:"stdout"|"stderr"):void {
      if(stream!=="stderr"||!line.startsWith(marker))return;
      if(seen||line.length>maximumCharacters){seen=true;source=undefined;return;}
      seen=true;
      try{const row=readSourceEdit(JSON.parse(line.slice(marker.length)),workspace);if(row?.request===request)source=row;}catch{}
    },
    restore(error:unknown):ConcurrentEditErrorV1|undefined {
      if(!source||!(error instanceof OwnedCommandFailure)||error.outcome.status!==formatStampProtocolV1().exitCode||error.outcome.signal!==null)return;
      return concurrentEditV1(source.reason,source.path,"Native host Cargo Source changed","build");
    },
  });
}
