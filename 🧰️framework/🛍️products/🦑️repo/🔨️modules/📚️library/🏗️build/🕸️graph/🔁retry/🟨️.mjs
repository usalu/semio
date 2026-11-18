import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import{createHash}from"node:crypto";
import{resolve,relative,isAbsolute,sep}from"node:path";
import{createProjectFileIndexV1}from"../🧭️file-owner/🟨️.mjs";
import{registerSliceReportSourceV1}from"../../🧠️memory/📋️slice-report/🟨️.mjs";
const policy=schema.$defs.Policy.const,sealPolicy=schema.$defs.SealPolicy.const,workspaceOwner="(workspace)",digest=new RegExp(schema.$defs.Diagnostic.properties.before.anyOf[0].pattern,"u");
/** 🔁️ Projects only bounded source identities and hashes; raw observed facts and environment values never leave the owner. */
export function buildGraphRetryDiagnosticV1(root,reason,{before=null,after=null,stale=[]}={},currentDigest=()=>null){
 if(!schema.$defs.Diagnostic.properties.reason.enum.includes(reason)||[before,after].some(value=>value!==null&&!digest.test(value)))throw Error("Declared graph retry reason and content digests required");
 const facts=stale.slice(0,policy.maximumFacts).map(row=>{
  const path=resolve(root,row.path),distance=relative(root,path),inside=distance!==".."&&!distance.startsWith(".."+sep)&&!isAbsolute(distance)&&distance.length<=policy.maximumPathUnits;
  const encoded=JSON.stringify(row.fact);if(Buffer.byteLength(encoded)>policy.maximumFactBytes)throw Error("Bounded observed graph fact required");
  const expected=row.kind==="file"&&digest.test(row.fact?.sha256)?row.fact.sha256:createHash("sha256").update(encoded).digest("hex");let actual=null;
  if(inside&&row.kind==="file")try{const value=currentDigest(path);if(value!==null&&digest.test(value))actual=value;}catch{}
  return {kind:String(row.kind).slice(0,64),path:inside?distance.split(sep).join("/"):null,before:expected,after:actual};
 });
 return {version:1,reason,before,after,facts};
}
/** 🧮️ The verdict of a sealed read: the read is a set of per-owner records, each valid for the digest of exactly the rows that were observed for the files of that owner; a read seals when every record was valid at its observation, and `sealedWithLaterChanges` counts the observed rows that changed afterwards (those owners are stale in the NEXT read only). `stale` lists at most `maximumStalePaths` of them, `rows` every observed row, `projects` maps roots to owners.
 * @see ./🧬️schema/🔣️.json */
export function sealVerdictV1({rounds,staleCount,stale,membershipMoved,admissionMoved,projects,rows}){
 const index=createProjectFileIndexV1(projects??{}),ownerOf=path=>index(path)??workspaceOwner,groups=new Map();
 for(const row of rows){const owner=ownerOf(row.path),list=groups.get(owner);if(list)list.push(JSON.stringify([row.kind,row.path,row.fact]));else groups.set(owner,[JSON.stringify([row.kind,row.path,row.fact])]);}
 const owners=Object.fromEntries([...groups].sort(([left],[right])=>left<right?-1:left>right?1:0).map(([owner,list])=>[owner,createHash("sha256").update(list.sort().join("\n")).digest("hex")]));
 const later=[...new Set(stale.slice(0,sealPolicy.maximumStalePaths).map(row=>ownerOf(row.path)))].sort().slice(0,sealPolicy.maximumOwners),clean=staleCount===0&&!membershipMoved&&!admissionMoved;
 return {version:1,mode:clean?rounds===0?"quiet":"repaired":"sealed-with-later-changes",rounds,sealedWithLaterChanges:staleCount,laterChangedOwners:later,membershipMoved,admissionMoved,owners};
}
/** 📣️ Tells the slice report of the worker what the last sealed read of this process said (mode, rounds, how many observed rows changed later and whose, how many owners carry a digest), so the store of every worker shows whether a read seals and with how many later changes. */
export function reportSealV1(verdict){const said={mode:verdict.mode,rounds:verdict.rounds,sealedWithLaterChanges:verdict.sealedWithLaterChanges,laterChangedOwners:verdict.laterChangedOwners.slice(0,16),membershipMoved:verdict.membershipMoved,admissionMoved:verdict.admissionMoved,owners:Object.keys(verdict.owners).length};registerSliceReportSourceV1("seal",()=>said);}
