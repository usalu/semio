import{lstatSync,statfsSync,mkdirSync,openSync,closeSync,writeSync,fsyncSync,readFileSync,readdirSync,writeFileSync,rmSync,constants}from"node:fs";
import{join,resolve,dirname,win32}from"node:path";
import{randomBytes}from"node:crypto";
import{validateJsonSchemaSubset}from"../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{nativeDirectoryProtocolV1,nativeDirectorySchemaV1,observeNativeDirectoryIdentityV1,readNativeDirectoryIdentityV1,sameNativeDirectoryIdentityV1,type NativeDirectoryIdentityV1,type NativeDirectoryLayoutV1,type NativeDirectoryPlatformPortV1,type NativeDirectoryRecoveryV1,type NativeDirectoryStateV1}from"./📋️contract/🟦️.ts";
import{createNativeDirectoryExchangePortV1}from"./🔀️exchange/🟦️.ts";
import{createNativeDirectoryJournaledPortV1}from"./🪟️journal/🟦️.ts";
import{nativeWindowsSystemHostPortV1}from"./🪟️host/🟦️.ts";
import type{NativeWindowsHostPortV1}from"./🪟️host/🟦️.ts";
import{measureNativeDirectoryUsageV1,measureNativeDirectoryUsageAsyncV1,nativeDirectoryEntryBoundV1,nativeDirectoryUsageV1,createNativeDirectoryUsageV1,type NativeDirectoryUsageV1}from"./📏️usage/🟦️.ts";
export{nativeDirectoryProtocolV1,observeNativeDirectoryIdentityV1,readNativeDirectoryIdentityV1,measureNativeDirectoryUsageV1,measureNativeDirectoryUsageAsyncV1,nativeDirectoryEntryBoundV1,nativeDirectoryUsageV1,createNativeDirectoryUsageV1};
export type{NativeDirectoryUsageV1};
export type{NativeDirectoryIdentityV1,NativeDirectoryLayoutV1,NativeDirectoryPlatformPortV1,NativeDirectoryRecoveryV1,NativeDirectoryStateV1};
const schema=nativeDirectorySchemaV1,protocol=nativeDirectoryProtocolV1(),same=sameNativeDirectoryIdentityV1;

/** 🧭️ Joins the canonical anchor, volume, root and receipt names beneath one partition directory. */
export function nativeDirectoryLayoutV1(directory:string):NativeDirectoryLayoutV1{if(directory!==resolve(directory)||directory.length>256)throw Error("Canonical bounded directory partition path required");return Object.freeze({directory,anchor:join(directory,protocol.anchor),volume:join(directory,protocol.volume),root:join(directory,protocol.root),receipt:join(directory,protocol.receipt)});}

let systemPort:NativeDirectoryPlatformPortV1|undefined;
/** 🧲️ Selects the platform port once: the kernel exchange on Linux and macOS, the journaled write-through swap on Windows; any other host is a precise refusal, never a fallback. */
export function nativeDirectorySystemPortV1():NativeDirectoryPlatformPortV1{
 if(systemPort)return systemPort;
 if(process.platform==="darwin"||process.platform==="linux")return systemPort=createNativeDirectoryExchangePortV1();
 if(process.platform==="win32")return systemPort=createNativeDirectoryJournaledPortV1(nativeWindowsSystemHostPortV1());
 throw Error("Directory partitions require Linux, macOS or Windows");
}

/** 💾️ Flushes one directory entry table through the platform port: fsync on POSIX, FlushFileBuffers on Windows. */
export function flushNativeDirectoryV1(path:string,port:NativeDirectoryPlatformPortV1=nativeDirectorySystemPortV1()):void{port.flush(path);}
/** 🔒️ Windows census substitute: each path must open with share mode zero, which fails while any other handle to that directory exists; file handles below are fenced by the partition lease and by the legacy rename, which refuses sharing violations. */
export function requireNativeDirectoryUnusedV1(paths:readonly string[],host:NativeWindowsHostPortV1=nativeWindowsSystemHostPortV1()):void{
 for(const path of paths){const result=host.openExclusive(path);if(result.ok)continue;throw Error(result.error===protocol.windowsErrors.sharing?protocol.refusals.windowsInUse+": "+path:"Directory could not be probed (Win32 error "+result.error+"): "+path);}
}
/** 🧪️ Builds the closed child-process environment for the owned partition script: fixed PATH and locale on Linux and macOS, the validated Windows system directory on Windows. */
export function nativeDirectoryChildEnvironmentV1(directory:string,environment:Readonly<Record<string,string|undefined>>=process.env,platform:string=process.platform):Readonly<Record<string,string>>{
 if(platform==="linux"||platform==="darwin")return Object.freeze({...protocol.childEnvironment[platform]});
 if(platform!=="win32")throw Error("Actual fixed platform required");
 const root=environment.SystemRoot;if(typeof root!=="string"||!new RegExp(protocol.exchange.windows.systemRoot.pattern,"u").test(root)||win32.normalize(root)!==root)throw Error("Canonical Windows SystemRoot required");
 return Object.freeze({SystemRoot:root,WINDIR:root,PATH:win32.join(root,protocol.exchange.windows.systemRoot.library),TEMP:directory,TMP:directory});
}
/** 💽️ Reports the bytes available to the caller on the host filesystem holding a path: statfs on POSIX, GetDiskFreeSpaceExW on Windows. */
export function nativeDirectoryHostAvailableBytesV1(path:string,host?:NativeWindowsHostPortV1):number{
 if(process.platform!=="win32"){const row=statfsSync(path);return row.bavail*row.bsize;}
 const result=(host??nativeWindowsSystemHostPortV1()).availableBytes(path);if(!result.ok)throw Error("Host free space could not be read (Win32 error "+result.error+")");return result.bytes;
}

/** 🔁️ Swaps two same-device sibling-or-nested directory names through the platform port and proves both identities moved. */
export function exchangeNativeDirectoriesV1(first:string,second:string,port:NativeDirectoryPlatformPortV1=nativeDirectorySystemPortV1()):Readonly<{first:NativeDirectoryIdentityV1;second:NativeDirectoryIdentityV1}>{
 for(const path of[first,second])if(path!==resolve(path)||path.length>256)throw Error("Canonical bounded exchange paths required");
 if(first===second)throw Error("Two distinct directories required");
 const before={first:observeNativeDirectoryIdentityV1(first),second:observeNativeDirectoryIdentityV1(second)};
 if(before.first.dev!==before.second.dev||before.first.ino===before.second.ino)throw Error("Two distinct same-device directories required");
 port.swap(first,second);
 const after={first:observeNativeDirectoryIdentityV1(first),second:observeNativeDirectoryIdentityV1(second)};
 if(!same(after.first,before.second)||!same(after.second,before.first))throw Error("Ambiguous directory exchange: names do not hold the swapped identities");
 port.flush(dirname(first));if(dirname(second)!==dirname(first))port.flush(dirname(second));
 return Object.freeze(after);
}

/** 🔠️ Reports whether a partition holds case-sensitive content or was declared fold-safe because its Windows host cannot set the per-directory flag without the optional WSL feature. */
export function nativeDirectoryCaseModeV1(directory:string):"sensitive"|"fold-safe"{return readNativeDirectoryAnchorFileV1(directory).caseFold?"fold-safe":"sensitive";}
const foldKey=(name:string,map:(text:string)=>string)=>[...name.normalize("NFC")].map(char=>{const folded=map(char);return[...folded].length===1?folded:char;}).join("");
/** 🪞️ Groups the names a case-folding filesystem would merge: names equal after Unicode normalization and simple lower or upper case mapping; deliberately a superset of what NTFS or APFS merges, so a fold-safe partition never admits a pair that could alias. */
export function nativeDirectoryFoldCollisionsV1(names:readonly string[]):readonly(readonly string[])[]{
 const classes=new Map<string,Set<string>>(),parent=new Map<string,string>(),find=(name:string):string=>{let root=name;while(parent.get(root)!==root)root=parent.get(root)!;return root;};
 for(const name of names)parent.set(name,name);
 for(const map of[(text:string)=>text.toLowerCase(),(text:string)=>text.toUpperCase()])for(const name of names){const key=foldKey(name,map),seen=classes.get(key)??new Set<string>();for(const other of seen)parent.set(find(name),find(other));seen.add(name);classes.set(key,seen);}
 const groups=new Map<string,string[]>();for(const name of names){const root=find(name),group=groups.get(root)??[];if(!group.includes(name))group.push(name);groups.set(root,group);}
 return Object.freeze([...groups.values()].filter(group=>group.length>1).map(group=>Object.freeze([...group].sort())).sort((a,b)=>a[0]!<b[0]!?-1:1));
}
/** 🔠️ Proves that two names differing only by case stay distinct beneath the directory. */
export function probeNativeDirectoryCaseSensitivityV1(directory:string):boolean{
 const base=join(directory,".case-"+randomBytes(4).toString("hex"));mkdirSync(base,{mode:0o700});
 try{writeFileSync(join(base,protocol.probe[0]!),"");try{writeFileSync(join(base,protocol.probe[1]!),"",{flag:"wx"});}catch(error){if((error as NodeJS.ErrnoException).code==="EEXIST")return false;throw error;}return readdirSync(base).length===2;}finally{rmSync(base,{recursive:true,force:true});}
}
/** 📂️ Proves that a fresh volume directory beneath the host can hold case-sensitive content once the platform prepared it. */
export function probeNativeDirectoryCaseCapabilityV1(directory:string,port:NativeDirectoryPlatformPortV1=nativeDirectorySystemPortV1()):boolean{
 const scratch=join(directory,".capability-"+randomBytes(4).toString("hex"));mkdirSync(scratch,{mode:0o700});
 try{port.prepareVolume(scratch);return probeNativeDirectoryCaseSensitivityV1(scratch);}finally{rmSync(scratch,{recursive:true,force:true});}
}
/** 🧫️ Proves on a scratch pair beneath the directory that the host filesystem swaps directory names. */
export function probeNativeDirectoryExchangeV1(directory:string,port:NativeDirectoryPlatformPortV1=nativeDirectorySystemPortV1()):void{
 const base=join(directory,".exchange-"+randomBytes(4).toString("hex"));mkdirSync(base,{mode:0o700});
 try{const first=join(base,"a"),second=join(base,"b");mkdirSync(first);mkdirSync(second);exchangeNativeDirectoriesV1(first,second,port);}finally{rmSync(base,{recursive:true,force:true});}
}

/** ⚖️ Bounds each payload request by its filesystem envelope and the journal and complete measured tree by their physical ceiling. */
export function admitNativeDirectoryCapacityV1(value:unknown):Readonly<{remainingBytes:number}>{
 if(validateJsonSchemaSubset(schema.definitions.Admission,value,schema).length)throw Error("Closed directory capacity request required");
 const row=value as Readonly<{filesystemBytes:number;maximumPhysicalBytes:number;reservedBytes:number;usedBytes:number;requestBytes:number}>;
 if(row.requestBytes>row.filesystemBytes||row.usedBytes+row.requestBytes>row.maximumPhysicalBytes||row.reservedBytes+row.requestBytes>row.maximumPhysicalBytes)throw Error(protocol.refusals.capacity);
 return Object.freeze({remainingBytes:Math.min(row.maximumPhysicalBytes-row.usedBytes,row.maximumPhysicalBytes-row.reservedBytes)-row.requestBytes});
}

/** 📏️ Charges every admission against a running estimate of the real tree and re-measures the tree periodically, on a commit, and before any refusal; the measurements are shared with every other reader of the tree, never overlap and never block the event loop. */
export type NativeDirectoryMeterV1=Readonly<{admit(request:unknown,signal?:AbortSignal):Promise<void>}>;
export function createNativeDirectoryMeterV1(input:Readonly<{root:string;filesystemBytes:number;maximumPhysicalBytes:number;measure?:(root:string,signal?:AbortSignal)=>number|Promise<number>;usage?:NativeDirectoryUsageV1;now?:()=>number}>):NativeDirectoryMeterV1{
 if(validateJsonSchemaSubset(schema.definitions.MeterLimits,{filesystemBytes:input.filesystemBytes,maximumPhysicalBytes:input.maximumPhysicalBytes},schema).length)throw Error("Closed directory meter limits required");
 const limits=protocol.meter,now=input.now??Date.now,usage=input.measure?undefined:input.usage??nativeDirectoryUsageV1(input.root,input.maximumPhysicalBytes),measure=(signal?:AbortSignal):Promise<number>=>usage?usage.fresh(signal):Promise.resolve(input.measure!(input.root,signal));
 let measured:number|undefined,measuredAt=0,admitted=0;
 const refresh=async(signal?:AbortSignal)=>{const startedAt=now(),before=admitted,bytes=await measure(signal);measured=bytes;measuredAt=startedAt;admitted=Math.max(0,admitted-before);return bytes;};
 const stale=()=>measured===undefined||now()-measuredAt>=limits.intervalMilliseconds;
 const gate=(used:number,requestBytes:number)=>admitNativeDirectoryCapacityV1({filesystemBytes:input.filesystemBytes,maximumPhysicalBytes:input.maximumPhysicalBytes,reservedBytes:0,usedBytes:used,requestBytes});
 return Object.freeze({admit:async(value:unknown,signal?:AbortSignal)=>{
  if(validateJsonSchemaSubset(schema.definitions.MeterRequest,value,schema).length)throw Error("Closed directory meter request required");
  const request=value as Readonly<{mode:"reserve"|"commit"|"recheck";requestBytes:number}>;
  if(request.mode==="commit"){gate(await refresh(signal),0);return;}
  if(request.mode==="recheck"){if(stale())await refresh(signal);gate(measured!+admitted,0);return;}
  if(measured===undefined||admitted+request.requestBytes>=limits.intervalBytes||stale())await refresh(signal);
  if(measured!+admitted+request.requestBytes>input.maximumPhysicalBytes)await refresh(signal);
  gate(measured!+admitted,request.requestBytes);admitted+=request.requestBytes;
 }});
}

/** 🪪️ The immutable partition anchor: its envelope, its empty placeholder and, only when the host cannot make the volume case sensitive, the declared case-folding mode. */
export type NativeDirectoryAnchorV1=Readonly<{filesystemBytes:number;maximumPhysicalBytes:number;underlying:NativeDirectoryIdentityV1;caseFold?:"insensitive"}>;
/** 📜️ Serializes the immutable partition anchor that stands in for the former image file and names its empty placeholder. */
export function nativeDirectoryAnchorBytesV1(input:NativeDirectoryAnchorV1):Buffer{
 const row={version:1,kind:protocol.kind,filesystemBytes:input.filesystemBytes,maximumPhysicalBytes:input.maximumPhysicalBytes,underlying:{dev:input.underlying.dev,ino:input.underlying.ino,mode:input.underlying.mode},...(input.caseFold?{caseFold:input.caseFold}:{})};
 if(validateJsonSchemaSubset(schema.definitions.Anchor,row,schema).length)throw Error("Closed directory anchor required");return Buffer.from(JSON.stringify(row)+"\n");
}
/** 📖️ Admits only canonical bounded anchor bytes. */
export function readNativeDirectoryAnchorV1(bytes:Uint8Array):NativeDirectoryAnchorV1{
 if(bytes.byteLength>protocol.maximumAnchorBytes)throw Error("Bounded directory anchor required");const text=new TextDecoder("utf-8",{fatal:true}).decode(bytes),row=JSON.parse(text);
 if(validateJsonSchemaSubset(schema.definitions.Anchor,row,schema).length||nativeDirectoryAnchorBytesV1(row).toString()!==text)throw Error("Canonical directory anchor required");return Object.freeze({filesystemBytes:row.filesystemBytes,maximumPhysicalBytes:row.maximumPhysicalBytes,underlying:readNativeDirectoryIdentityV1(row.underlying),...(row.caseFold?{caseFold:row.caseFold as "insensitive"}:{})});
}
/** 📎️ Reads the anchor beside a partition directory without following links. */
export function readNativeDirectoryAnchorFileV1(directory:string):NativeDirectoryAnchorV1{
 const layout=nativeDirectoryLayoutV1(directory),row=lstatSync(layout.anchor);if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1||row.size<1||row.size>protocol.maximumAnchorBytes)throw Error("Single bounded directory anchor required");
 const descriptor=openSync(layout.anchor,constants.O_RDONLY|(constants.O_NOFOLLOW??0));try{return readNativeDirectoryAnchorV1(readFileSync(descriptor));}finally{closeSync(descriptor);}
}

/** 🛟️ Reports whether a journaled swap was interrupted beside this partition. */
export function nativeDirectorySwapPendingV1(directory:string,port:NativeDirectoryPlatformPortV1=nativeDirectorySystemPortV1()):boolean{return port.pending(nativeDirectoryLayoutV1(directory).directory);}
/** 📁️ Completes or rolls back an interrupted swap from its intent receipt; only the exclusive lease holder may call it. */
export function recoverNativeDirectoryV1(directory:string,port:NativeDirectoryPlatformPortV1=nativeDirectorySystemPortV1()):NativeDirectoryRecoveryV1{return port.recover(nativeDirectoryLayoutV1(directory).directory);}

/** 🎯️ Derives the state from which of root and volume still holds the original empty placeholder named by the anchor. */
export function observeNativeDirectoryStateV1(directory:string,port:NativeDirectoryPlatformPortV1=nativeDirectorySystemPortV1()):NativeDirectoryStateV1{
 const layout=nativeDirectoryLayoutV1(directory);if(port.pending(directory))throw Error(protocol.refusals.interrupted);
 const held=readNativeDirectoryAnchorFileV1(directory).underlying,root=observeNativeDirectoryIdentityV1(layout.root),volume=observeNativeDirectoryIdentityV1(layout.volume);
 if(root.dev!==volume.dev||root.ino===volume.ino)throw Error("Two distinct same-device partition directories required");
 if(same(root,held))return"inactive";if(same(volume,held))return"attached";throw Error("Original underlying placeholder directory required");
}

/** 🗺️ Derives one stable mapping string from the attached volume identity and the immutable partition anchor. */
export function nativeDirectoryMappingV1(directory:string,port?:NativeDirectoryPlatformPortV1):string{
 if(observeNativeDirectoryStateV1(directory,port)!=="attached")throw Error("Attached directory partition required");const layout=nativeDirectoryLayoutV1(directory),file=lstatSync(layout.anchor);
 return JSON.stringify({kind:protocol.kind,anchor:[file.dev,file.ino,file.size],volume:observeNativeDirectoryIdentityV1(layout.root)});
}

/** 🌱️ Creates the volume and the anchor beside the empty root placeholder; nothing is attached. */
export function allocateNativeDirectoryV1(input:Readonly<{directory:string;filesystemBytes:number;maximumPhysicalBytes:number}>,probe:(directory:string)=>boolean=probeNativeDirectoryCaseSensitivityV1,port:NativeDirectoryPlatformPortV1=nativeDirectorySystemPortV1()):void{
 const layout=nativeDirectoryLayoutV1(input.directory),underlying=observeNativeDirectoryIdentityV1(layout.root);let caseFold:"insensitive"|undefined;
 if(readdirSync(layout.root).length)throw Error("Empty original placeholder directory required");
 mkdirSync(layout.volume,{mode:underlying.mode&0o7777});
 try{port.prepareVolume(layout.volume);if(!probe(layout.volume))throw Error(port.kind==="journaled"?protocol.refusals.caseUnverified:protocol.refusals.caseInsensitive);}catch(error){if(port.kind==="journaled"&&(protocol.foldSafeRefusals as readonly string[]).some(key=>(protocol.refusals as Readonly<Record<string,string>>)[key]===(error as Error)?.message))caseFold="insensitive";else{rmSync(layout.volume,{recursive:true,force:true});throw error;}}
 const bytes=nativeDirectoryAnchorBytesV1({filesystemBytes:input.filesystemBytes,maximumPhysicalBytes:input.maximumPhysicalBytes,underlying,...(caseFold?{caseFold}:{})}),descriptor=openSync(layout.anchor,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|(constants.O_NOFOLLOW??0),0o600);let failed=false,failure:unknown;
 try{if(writeSync(descriptor,bytes)!==bytes.length)throw Error("Complete anchor write required");fsyncSync(descriptor);}catch(error){failed=true;failure=error;}
 try{closeSync(descriptor);}catch(error){if(!failed)throw error;}if(failed)throw failure;
 port.flush(input.directory);
}
/** 🔗️ Attaches the volume as the root with one swap. */
export function activateNativeDirectoryV1(directory:string,port:NativeDirectoryPlatformPortV1=nativeDirectorySystemPortV1()):void{
 const layout=nativeDirectoryLayoutV1(directory);if(observeNativeDirectoryStateV1(directory,port)!=="inactive")throw Error("Inactive partition required for activation");
 exchangeNativeDirectoriesV1(layout.root,layout.volume,port);if(observeNativeDirectoryStateV1(directory,port)!=="attached")throw Error("Attached partition required after activation");
}
/** 🔓️ Detaches the volume by swapping the empty placeholder back into place. */
export function deactivateNativeDirectoryV1(directory:string,port:NativeDirectoryPlatformPortV1=nativeDirectorySystemPortV1()):void{
 const layout=nativeDirectoryLayoutV1(directory);if(observeNativeDirectoryStateV1(directory,port)!=="attached")throw Error("Attached partition required for deactivation");
 exchangeNativeDirectoriesV1(layout.root,layout.volume,port);if(observeNativeDirectoryStateV1(directory,port)!=="inactive")throw Error("Inactive partition required after deactivation");
}

/** 🌳️ Lists the roots of attached nested partitions at the declared depth beneath a root, in canonical order; an interrupted child is recovered first only when the caller holds the parent exclusively and says so. */
export function listNativeDirectoryChildrenV1(root:string,maximumEntries:number=protocol.maximumEntries,recover?:NativeDirectoryPlatformPortV1):readonly string[]{
 const found:string[]=[];let entries=0;
 const visit=(directory:string,depth:number)=>{for(const name of readdirSync(directory)){if(++entries>maximumEntries)throw Error("Nested partition scan entry bound exceeded");const path=join(directory,name),row=lstatSync(path);if(!row.isDirectory()||row.isSymbolicLink())continue;
  let anchored=false;try{anchored=lstatSync(join(path,protocol.anchor)).isFile();}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
  if(anchored){if(recover)recoverNativeDirectoryV1(path,recover);if(observeNativeDirectoryStateV1(path,recover)==="attached")found.push(join(path,protocol.root));continue;}
  if(depth<protocol.childDepth)visit(path,depth+1);}};
 visit(resolve(root),1);return Object.freeze(found.sort((left,right)=>left.length-right.length||left.localeCompare(right)));
}

/** 🛠️ Executes one closed partition action for the owned child process and returns its stdout text. */
export function runNativeDirectoryActionV1(argv:readonly string[],port?:NativeDirectoryPlatformPortV1,probe?:(directory:string)=>boolean):string{
 const[action,directory,...rest]=argv;if(!protocol.actions.includes(action as never)||directory===undefined)throw Error("Closed directory partition action required");
 if(action==="allocate"){if(rest.length!==2||rest.some(part=>!/^[0-9]+$/u.test(part)))throw Error("Closed allocation envelope required");allocateNativeDirectoryV1({directory,filesystemBytes:Number(rest[0]),maximumPhysicalBytes:Number(rest[1])},probe,port);return"";}
 if(rest.length)throw Error("Closed directory partition arguments required");
 if(action==="activate"){activateNativeDirectoryV1(directory,port);return"";}
 if(action==="deactivate"){deactivateNativeDirectoryV1(directory,port);return"";}
 return observeNativeDirectoryStateV1(directory,port)==="attached"?"attached\n":"";
}
