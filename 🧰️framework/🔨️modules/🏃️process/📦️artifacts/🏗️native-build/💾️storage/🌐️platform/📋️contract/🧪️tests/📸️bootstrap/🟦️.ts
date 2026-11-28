import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {loadProductFunctionsSyncV1} from "../../../../../../../🧪️testing/🧩️extracted-function/🟦️.ts";
import {spawnSync} from "node:child_process";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {createHash} from "node:crypto";
import {withArtifactReadFileSyncV1,withArtifactReadDirectorySyncV1} from "../../../../../../../../📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../../../../../🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1,requireIssuedResourceControlRefinementV1} from "../../../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {acquireResourceLease,captureResourceLeaseWriterProofV1,checkResourceLeaseControl,requireMaterializedPathV1,requireResourceLeaseWriterProofV1,resourceLeasePathV1} from "../../../../../../../🔒️leases/🟦️.ts";
import {openNativePlatformStorageV1} from "../../../🟦️.ts";
import {nativeImageBackingAllocatedBytesV1} from "../../../../🪶️extent/🟦️.ts";
import {nativeStorageBackendOfDirectoryV1} from "../../../../🔌️backends/📋️contract/🟦️.ts";
import {readNativeImageRetirementFileV1} from "../../../../📄️retirement-file/🟦️.ts";
import {nativeImageAttachedV1,nativeImageFilesystemBytesV1,nativeImageRootV1,nativePhysicalIdentityTokenV1,nativeProtectedMaintenanceProtocolV1,readNativeProtectedMaintenanceRequestV1,requireNativeImageEnvelopeV1} from "../../🟦️.ts";
import fixedSchema from "../../../../🧱️fixed/🧬️schema/🔣️.json" with {type:"json"};
import {nativeDirectoryAnchorBytesV1,nativeDirectoryLayoutV1,observeNativeDirectoryIdentityV1} from "../../../../📁️directory/🟦️.ts";
import corpus from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema.json" with {type:"json"};
import work from "./🔣️work.json" with {type:"json"};

declareBuildTestEnvelopeV1({owner:"test:native-maintenance-proof-cost",maximumElapsedMilliseconds:180000,capabilities:{artifacts:true}});

function compile(name:string,_helpers:readonly string[],ports:Record<string,unknown>){
 return loadProductFunctionsSyncV1(new URL("../../../../🧱️fixed/🟦️.ts",import.meta.url),[name],ports)[name];
}
const healthyBytes=8588886016;

test("maintenance uses the actual canonical backing kind and preserves opaque receipt and physical CAS fences",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of corpus){
  const folder={dev:1n,ino:2n,mode:16832n,isDirectory:()=>true,isSymbolicLink:()=>false};
  const backing={dev:1n,ino:3n,nlink:1n,size:BigInt(healthyBytes),blocks:BigInt(healthyBytes/512),isFile:()=>true,isSymbolicLink:()=>false};
  const root={...folder,dev:row.backend==="mounted-image"?4n:1n,ino:5n};
  const receipt=Buffer.from(JSON.stringify({version:1,filesystemBytes:healthyBytes,maximumPhysicalBytes:8589934592}));
  const filename=row.backend==="mounted-image"?"fixed.dmg":"fixed.dir",image="/asset/"+filename;
  const request={directory:{dev:1,ino:2,mode:16832},backing:{dev:1,ino:row.fault==="backing"?30:3,size:healthyBytes,allocatedBytes:healthyBytes},root:{dev:Number(root.dev),ino:row.fault==="root"?50:5,mode:16832},receiptSha256:row.fault==="receipt"?"0".repeat(64):createHash("sha256").update(receipt).digest("hex")};
  const options={directory:"/asset",leaseDirectory:"/leases",filesystemBytes:38653657088,maximumPhysicalBytes:row.fault==="capacity"?healthyBytes-1:38654705664};
  const observed:string[]=[];
  let geometry=0,checks=0;
  const run=compile("protectedMaintenancePhysicalFactsV1",["fixedBackingIdentityV1"],{
   join,process:{platform:row.platform},requireNativeImageEnvelopeV1:()=>{},nativeImageRootV1:()=>"/asset/m",
   nativeStorageBackendOfDirectoryV1:(path:string)=>{expect(path).toBe(options.directory);return{backingFilename:filename};},
   lstatSync:(path:string)=>{observed.push(path);if(path==="/asset")return folder;if(path==="/asset/m")return root;if(path===image)return backing;throw Error("Foreign or absent physical backing");},
   readNativeImageRetirementFileV1:()=>({bytes:receipt}),nativeProtectedMaintenanceProtocolV1:()=>({maximumReceiptBytes:4096}),protectedMaintenancePublicationSnapshotV1:()=>null,
   nativePhysicalIdentityTokenV1,nativeImageBackingAllocatedBytesV1:()=>BigInt(healthyBytes),nativeImageAttachedV1:(path:string)=>path===image&&row.fault!=="detached",
   nativeImageFilesystemBytesV1:()=>{geometry++;return row.fault==="geometry"?Number.NaN:healthyBytes;},createHash
  });
  let facts:{image:string}|undefined;
  try{facts=run(options,request,()=>{checks++;if(row.fault==="custody")throw Error("Actual custody refused");});}catch{}
  const oracle=db.query("select ?='none' accepted, ? in ('none','geometry','receipt') geometry").get(row.fault,row.fault) as {accepted:number;geometry:number};
  expect(facts!==undefined,row.id).toBe(row.accept);
  expect(facts!==undefined,row.id).toBe(Boolean(oracle.accepted));
  expect(geometry,row.id).toBe(oracle.geometry);
  if(facts)expect(facts.image).toBe(image);
  if(row.fault!=="custody")expect(observed,row.id).toContain(image);
  expect(checks).toBeGreaterThan(0);
 }}finally{db.close();}
});

test.skipIf(process.platform!=="darwin")("borrowed private Darwin backing samples geometry once and retains every distinct writer fence",()=>withBuildTestInvocationV1(async original=>{
 const scratch=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"vp-"))),directory=join(scratch,"i"),leaseDirectory=join(scratch,"l"),root=join(directory,"m");
 mkdirSync(leaseDirectory);
 const control=createIssuedResourceControlV1(original,"cache:physical-proof-cost",60000);
 const options={directory,leaseDirectory,filesystemBytes:work.filesystemBytes,maximumPhysicalBytes:work.maximumPhysicalBytes,control};
 const detach=()=>spawnSync("repo",["tools","exec","--light","hdiutil","--","detach",root],{encoding:"utf8"});
 const db=new Database(":memory:");
 try{
  const image=await openNativePlatformStorageV1(options);
  try{
   const outcome=await image.withExclusiveMaintenance({version:1,wait:false},async(_port,custody)=>{
    if(custody.kind!=="kernel-ex")throw Error("Actual original Darwin writer required");
    const anchor=(path:string)=>{const row=lstatSync(path,{bigint:true});return{dev:nativePhysicalIdentityTokenV1(row.dev),ino:nativePhysicalIdentityTokenV1(row.ino),mode:Number(row.mode)};};
    const file=lstatSync(image.image,{bigint:true}),receipt=join(directory,"image.json"),request={version:1,purpose:"mounted-parent-maintenance",wait:false,directory:anchor(directory),root:anchor(root),backing:{dev:nativePhysicalIdentityTokenV1(file.dev),ino:nativePhysicalIdentityTokenV1(file.ino),size:Number(file.size),allocatedBytes:Number(nativeImageBackingAllocatedBytesV1(file))},receiptSha256:createHash("sha256").update(readFileSync(receipt)).digest("hex")};
    const counts={geometry:0,proof:0,receipt:0};
    const capture=compile("captureNativeProtectedMaintenancePhysicalV1",["fixedBackingIdentityV1","protectedMaintenancePublicationSnapshotV1","protectedMaintenancePhysicalFactsV1"],{
     dirname,join,lstatSync,process,schema:fixedSchema,createHash,nativeImageRootV1,nativeStorageBackendOfDirectoryV1,nativePhysicalIdentityTokenV1,nativeImageBackingAllocatedBytesV1,nativeImageAttachedV1,readNativeProtectedMaintenanceRequestV1,nativeProtectedMaintenanceProtocolV1,requireNativeImageEnvelopeV1,requireIssuedResourceControlRefinementV1,checkResourceLeaseControl,resourceLeasePathV1,requireMaterializedPathV1,withArtifactReadFileSyncV1,withArtifactReadDirectorySyncV1,
     readNativeImageRetirementFileV1:(...args:Parameters<typeof readNativeImageRetirementFileV1>)=>{counts.receipt++;return readNativeImageRetirementFileV1(...args);},
     nativeImageFilesystemBytesV1:(...args:Parameters<typeof nativeImageFilesystemBytesV1>)=>{counts.geometry++;return nativeImageFilesystemBytesV1(...args);},
     requireResourceLeaseWriterProofV1:(...args:Parameters<typeof requireResourceLeaseWriterProofV1>)=>{counts.proof++;return requireResourceLeaseWriterProofV1(...args);}
    });
    const actual=capture({...options,control:custody.control},request,custody.proof);
    counts.geometry=counts.proof=counts.receipt=0;
    for(let index=0;index<work.iterations;index++)actual.check(custody.control);
    const expected=db.query("select ?*? geometry, ?*? proof, ? receipt").get(work.iterations,work.optimized.geometry,work.iterations,work.optimized.proof,work.iterations) as typeof counts;
    expect(counts).toEqual(expected);
    expect(nativeImageFilesystemBytesV1(root,options.filesystemBytes)).toBeGreaterThan(0);
   });
   expect(outcome.acquired).toBe(true);
  }finally{await image.close();}
 }finally{
  db.close();
  if(existsSync(root)&&existsSync(join(directory,"fixed.dmg"))&&lstatSync(root).dev!==lstatSync(join(directory,"fixed.dmg")).dev)expect(detach().status).toBe(0);
  rmSync(scratch,{recursive:true,force:true});
 }
}),100000);

test("protected maintenance validates each immutable envelope once under real custody",()=>withBuildTestInvocationV1(async original=>{
 const scratch=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"ve-"))),leaseDirectory=join(scratch,"l");
 const control=createIssuedResourceControlV1(original,"cache:physical-envelope-source",60000);
 const fixture=(name:string,filesystemBytes:number,maximumPhysicalBytes:number)=>{
  const directory=join(scratch,name),layout=nativeDirectoryLayoutV1(directory);
  mkdirSync(directory);mkdirSync(layout.volume);mkdirSync(layout.root);
  writeFileSync(layout.anchor,nativeDirectoryAnchorBytesV1({filesystemBytes,maximumPhysicalBytes,underlying:observeNativeDirectoryIdentityV1(layout.volume)}),{mode:0o600});
  const receipt=Buffer.from(JSON.stringify({version:1,filesystemBytes,maximumPhysicalBytes}));
  writeFileSync(layout.receipt,receipt,{mode:0o600});
  const identity=(path:string)=>{const stat=lstatSync(path,{bigint:true});return{dev:nativePhysicalIdentityTokenV1(stat.dev),ino:nativePhysicalIdentityTokenV1(stat.ino),mode:Number(stat.mode)};};
  const stat=lstatSync(layout.anchor,{bigint:true}),backing={dev:nativePhysicalIdentityTokenV1(stat.dev),ino:nativePhysicalIdentityTokenV1(stat.ino),size:Number(stat.size),allocatedBytes:Number(nativeImageBackingAllocatedBytesV1(stat))};
  const request={version:1,purpose:"mounted-parent-maintenance",wait:false,directory:identity(directory),root:identity(layout.root),backing,receiptSha256:createHash("sha256").update(receipt).digest("hex")};
  const association=Object.freeze({root:layout.root,image:layout.anchor,backingIdentity:JSON.stringify(Object.values(backing)),kernelIdentity:"private-directory-partition"});
  return {options:{directory,leaseDirectory,control,filesystemBytes,maximumPhysicalBytes},request,association,layout,receipt};
 };
 const db=new Database(":memory:");
 try{
  const parent=fixture("p",work.filesystemBytes,work.maximumPhysicalBytes),child=fixture("c",work.filesystemBytes+1048576,work.maximumPhysicalBytes+1048576),resource="native-fixed-image:"+parent.options.directory;
  const lease=await acquireResourceLease({directory:leaseDirectory,resource,mode:"exclusive",control,signal:control.signal,timeoutMs:0,maximumDatabaseBytes:16384});
  try{
   const proof=captureResourceLeaseWriterProofV1(lease,leaseDirectory,resource,control),counts={envelope:0,geometry:0,proof:0,receipt:0};
   const capture=compile("captureNativeProtectedMaintenancePhysicalV1",["fixedBackingIdentityV1","protectedMaintenancePublicationSnapshotV1","protectedMaintenancePhysicalFactsV1"],{
    dirname,join,lstatSync,process,schema:fixedSchema,createHash,nativeImageRootV1,nativeStorageBackendOfDirectoryV1,nativePhysicalIdentityTokenV1,nativeImageBackingAllocatedBytesV1,nativeImageAttachedV1,readNativeProtectedMaintenanceRequestV1,nativeProtectedMaintenanceProtocolV1,requireIssuedResourceControlRefinementV1,checkResourceLeaseControl,resourceLeasePathV1,requireMaterializedPathV1,withArtifactReadFileSyncV1,withArtifactReadDirectorySyncV1,
    requireNativeImageEnvelopeV1:(...args:Parameters<typeof requireNativeImageEnvelopeV1>)=>{counts.envelope++;requireNativeImageEnvelopeV1(...args);},
    readNativeImageRetirementFileV1:(...args:Parameters<typeof readNativeImageRetirementFileV1>)=>{counts.receipt++;return readNativeImageRetirementFileV1(...args);},
    nativeImageFilesystemBytesV1:(...args:Parameters<typeof nativeImageFilesystemBytesV1>)=>{counts.geometry++;return nativeImageFilesystemBytesV1(...args);},
    requireResourceLeaseWriterProofV1:(...args:Parameters<typeof requireResourceLeaseWriterProofV1>)=>{counts.proof++;return requireResourceLeaseWriterProofV1(...args);}
   });
   expect(()=>capture({...parent.options,filesystemBytes:parent.options.filesystemBytes+1,validatedEnvelope:true},parent.request,proof)).toThrow("Invalid fixed image allocation envelope");
   counts.envelope=counts.geometry=counts.proof=counts.receipt=0;
   const physical=capture(parent.options,parent.request,proof);
   parent.options.filesystemBytes++;
   for(let index=0;index<work.iterations;index++)physical.check(control);
   const expected=db.query("select ? envelope, (?+1)*? geometry, (?+1)*? proof, ?+1 receipt").get(work.optimized.envelope,work.iterations,work.optimized.geometry,work.iterations,work.optimized.proof,work.iterations) as {envelope:number;geometry:number;proof:number;receipt:number};
   expect(counts).toEqual(expected);
   const declaration={...child.options,current:child.request};
   expect(()=>physical.captureChild({...declaration,filesystemBytes:child.options.filesystemBytes+1,validatedEnvelope:true},control,[child.association])).toThrow("Invalid fixed image allocation envelope");
   expect(physical.captureChild(declaration,control,[child.association])).toBe(child.association);
   expect(counts.envelope).toBe(work.optimized.envelope+2);
   writeFileSync(parent.layout.receipt,Buffer.concat([parent.receipt,Buffer.from("x")]));
   expect(()=>physical.check(control)).toThrow("Exact current protected maintenance physical CAS required");
  }finally{lease.release();}
 }finally{db.close();rmSync(scratch,{recursive:true,force:true});}
}),100000);
