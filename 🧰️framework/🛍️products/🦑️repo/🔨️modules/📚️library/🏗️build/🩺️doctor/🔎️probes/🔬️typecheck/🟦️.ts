import {realpathSync} from "node:fs";
import type {TypecheckFactsV1} from "../../🟦️.ts";
import type {DoctorProbeV1} from "../📋️contract/🟦️.ts";

type ReportApi=Readonly<{
 typecheckCacheDirectoryV1(root:string,environment:Readonly<Record<string,string|undefined>>):string;
 readTypecheckLibraryReportV1(cacheDirectory:string):Readonly<{atMs:number;errors:number;product:number;tests:number;areas:readonly Readonly<{area:string;errors:number;product:number}>[]}>|undefined;
 runTypecheckLibraryV1(root:string,language:"en"|"de",ports:Readonly<{cacheDirectory:string;signal:AbortSignal;timeoutMs:number}>):Promise<unknown>;
}>;

/** 🔬️ The whole-library type check as a report: the stored measurement of the machine cache, measured again (minutes) only when this area is asked for by name and the stored one is not fresh; it never blocks anything. @see ../../../🔬️typecheck/🟦️.ts */
export const typecheckProbeV1:DoctorProbeV1=Object.freeze({
 area:"typecheck",phase:1,
 async read(ports,context){
  const api=await ports.optional("typecheck") as ReportApi|undefined;
  if(!api||typeof api.readTypecheckLibraryReportV1!=="function")return {unavailable:"The type check module is not installed in this checkout"};
  const cache=api.typecheckCacheDirectoryV1(realpathSync(ports.root),ports.environment),policy=context.policy.typecheck;
  let report=api.readTypecheckLibraryReportV1(cache);
  const age=():number=>report===undefined?Infinity:Math.max(0,Math.floor((ports.now()-report.atMs)/1000));
  if(context.requested?.has("typecheck")&&age()>=policy.freshSeconds){
   await api.runTypecheckLibraryV1(ports.root,"en",{cacheDirectory:cache,signal:ports.signal,timeoutMs:Math.max(1000,context.deadlineMs-ports.now())});
   report=api.readTypecheckLibraryReportV1(cache);
  }
  if(report===undefined)return {unavailable:"No whole-library type check was measured on this machine yet; run repo doctor --only=typecheck"};
  const facts:TypecheckFactsV1={errors:report.errors,product:report.product,tests:report.tests,ageSeconds:age(),areas:report.areas};
  return {sections:{typecheck:facts}};
 },
});
