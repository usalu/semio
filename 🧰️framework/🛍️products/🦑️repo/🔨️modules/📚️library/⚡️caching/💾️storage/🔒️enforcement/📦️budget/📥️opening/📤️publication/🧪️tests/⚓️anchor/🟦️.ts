import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {mkdir,mkdtemp,readFile,writeFile} from "node:fs/promises";
import {lstatSync} from "node:fs";
import {join,resolve} from "node:path";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import {withBuildTestInvocationV1} from "@semio-tech/process/testing/invocation";
import {createIssuedResourceControlV1} from "@semio-tech/process/routing";
import {openNativeFixedPlatformV1} from "@semio-tech/process/artifacts/native-build/storage/fixed";
import {publishObservedNativePlatformImageV1} from "@semio-tech/process/artifacts/native-build/storage/publication/platform";
import {captureNativeImagePublicationWitnessV1,requireCurrentCapturedNativeImageWitnessV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🟦️.ts";
import {requireResourceLeaseWriterProofV1,resourceLeasePathV1} from "@semio-tech/process/leases";
import {acquireNativeStorageV1} from "@semio-tech/process/artifacts/native-build/storage";
import {requireNativeImagePublicationSettledV1} from "@semio-tech/process/artifacts/native-build/storage/platform/contract";
import cuts from "./🔣️.json" with {type:"json"};
import fixtureSchema from "./🧬️schema.json" with {type:"json"};

declareLibraryBuildTestEnvelopeV1({suite:"storage-opening-processes",maximumElapsedMilliseconds:300000});

/** 🧪️ Exercises actual private directory Source and fresh owner recovery without sharing any live repository image. */
test("directory envelope actual owner preserves funded payload through durable interruption cuts",()=>withBuildTestInvocationV1(async original=>{
 expect(new Ajv({strict:true}).validate(fixtureSchema,cuts)).toBe(true);
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Actual issued test artifact directory required");
 const base=await mkdtemp(join(resolve(artifacts),"a-"));let index=0;
 for(const vector of cuts.cases){
  const directory=join(base,(index++).toString(16)),leaseDirectory=join(base,"l"),oldBounds=cuts.before,newBounds=cuts.after;
  for(const path of[directory,leaseDirectory])if(path.length>256)throw Error("Bounded private fixture path required");
  await mkdir(leaseDirectory,{recursive:true});
  const opening=createIssuedResourceControlV1(original,"anchor-fixture:opening:"+vector.id,60000),image=await openNativeFixedPlatformV1({directory,leaseDirectory,...oldBounds,control:opening});
  const payload=Buffer.from(cuts.payload,"utf8"),policy={version:1 as const,root:image.root,leaseDirectory,capacityBytes:oldBounds.maximumPhysicalBytes,maximumWorkspaceBytes:oldBounds.filesystemBytes,maximumArtifactBytes:oldBounds.filesystemBytes};
  const reservation=await acquireNativeStorageV1({policy,control:opening,port:image.port,owner:"fixture:anchor",kind:"artifact-publication",bytes:4096});
  await writeFile(join(image.root,"payload"),payload);await reservation.commit();
  if(vector.cut===null){const proof=await image.withExclusiveMaintenance({version:1,wait:false},async(_port,custody)=>{
   if(custody.kind!=="kernel-ex")throw Error("Actual private image kernel writer required");const actual=custody.control,resource="native-fixed-image:"+directory,check=()=>requireResourceLeaseWriterProofV1(custody.proof,{directory:leaseDirectory,resource,path:resourceLeasePathV1(leaseDirectory,resource)},actual),witness=await captureNativeImagePublicationWitnessV1(image.root,actual,oldBounds.filesystemBytes,check,{platform:"posix"}),db=new Database(":memory:");
   try{for(const row of cuts.witnessCases){const value=row.mode==="copied"?{...witness}:witness,path=row.mode==="foreign-root"?join(base,"foreign"):image.root,phase=row.mode==="foreign-control"?opening:actual;let accepted=true;try{requireCurrentCapturedNativeImageWitnessV1(value,path,phase);}catch{accepted=false;}expect(accepted,row.mode).toBe(row.accepted);expect(accepted).toBe(Boolean((db.query("select ?1 and ?2 and ?3 accepted").get(value===witness?1:0,path===image.root?1:0,phase===actual?1:0)as{accepted:number}).accepted));}}finally{db.close();}
  });if(!proof.acquired)throw Error("Private image writer fixture unexpectedly busy");}
  const oldAnchor=await readFile(image.image),oldIdentity=lstatSync(image.image,{bigint:true}),journal=await readFile(join(image.root,"storage-events.jsonl"));
  await image.close();
  const marker=Object.freeze(Error("anchor-cut:"+vector.id)),first=createIssuedResourceControlV1(original,"anchor-fixture:publish:"+vector.id,60000);
  let failure:unknown;
  try{await publishObservedNativePlatformImageV1({directory,leaseDirectory,...newBounds,maximumLogicalBytes:newBounds.filesystemBytes,control:first,onProgress:async event=>{if(event.stage===vector.cut)throw marker;}});}catch(error){failure=error;}
  if(vector.cut===null)expect(failure,vector.id).toBeUndefined();else{const errors=failure instanceof AggregateError?failure.errors:[failure];expect(errors[0],vector.id).toBe(marker);}
  if(vector.pending)expect(()=>requireNativeImagePublicationSettledV1(directory),vector.id).toThrow();
  if(vector.cut!==null){
   const recovery=createIssuedResourceControlV1(original,"anchor-fixture:recover:"+vector.id,60000);
   await publishObservedNativePlatformImageV1({directory,leaseDirectory,...newBounds,maximumLogicalBytes:newBounds.filesystemBytes,control:recovery});
  }
  requireNativeImagePublicationSettledV1(directory);
  const reopening=createIssuedResourceControlV1(original,"anchor-fixture:reopen:"+vector.id,60000),reopened=await openNativeFixedPlatformV1({directory,leaseDirectory,...newBounds,control:reopening});
  try{
   expect(await readFile(join(reopened.root,"payload")),vector.id).toEqual(payload);expect(await readFile(join(reopened.root,"storage-events.jsonl")),vector.id).toEqual(journal);
   const anchor=JSON.parse((await readFile(reopened.image)).toString()),receipt=JSON.parse((await readFile(join(directory,"image.json"))).toString()),identity=lstatSync(reopened.image,{bigint:true});
   expect(anchor.filesystemBytes).toBe(newBounds.filesystemBytes);expect(anchor.maximumPhysicalBytes).toBe(newBounds.maximumPhysicalBytes);expect(anchor.underlying).toEqual(JSON.parse(oldAnchor.toString()).underlying);expect(anchor.caseFold).toBe(JSON.parse(oldAnchor.toString()).caseFold);
   expect(identity.ino,vector.id).not.toBe(oldIdentity.ino);expect(receipt.dev).toBe(Number(identity.dev));expect(receipt.ino).toBe(Number(identity.ino));expect(receipt.size).toBe(Number(identity.size));
  }finally{await reopened.close();}
 }
}),300000);
