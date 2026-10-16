import{test,expect}from"bun:test";
import{NativeArtifactCacheReceiverLedgerV1}from"../📋️model/📨️receiver/🟦️.ts";
import{readNativeArtifactCachePolicyV1}from"../⚙️policy/🟦️.ts";
import corpus from"./🔣️.json";
test("neutral Core cache receiver rejects own-frame close after pre-enrollment",async()=>{const policy=readNativeArtifactCachePolicyV1(corpus.smallPolicy),ledger=new NativeArtifactCacheReceiverLedgerV1(policy);let refused=false;await ledger.run("read",async()=>{try{await ledger.close();}catch{refused=true;}});expect(refused).toBe(true);await ledger.close();});

import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{nativeArtifactCacheModelV1}from"../📋️model/🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
test("neutral Core cache separately authored consumer policy agrees with Ajv and SQLite",()=>{const validate=new Ajv({strict:true}).compile(schema.definitions.Policy),db=new Database(":memory:");try{for(const request of[corpus.productionPolicy,corpus.smallPolicy]){expect(validate(request)).toBe(true);const policy=readNativeArtifactCachePolicyV1(request),model=nativeArtifactCacheModelV1(policy);expect((db.query("SELECT ? = ? + ? AND ? = ? + 1048576 AS coherent").get(policy.chargeBytes,policy.maximumBackingBytes,policy.parentMetadataBytes,policy.maximumBackingBytes,policy.filesystemBytes)as{coherent:number}).coherent).toBe(1);for(const availableBytes of[policy.chargeBytes+policy.initialParentFloorBytes-1,policy.chargeBytes+policy.initialParentFloorBytes]){const oracle=(db.query("SELECT ? >= ? + ? AS admitted").get(availableBytes,policy.chargeBytes,policy.initialParentFloorBytes)as{admitted:number}).admitted===1;expect(model.artifactCacheCreationAdmittedV1({version:1,availableBytes})).toBe(oracle);}}expect(corpus.productionPolicy.chargeBytes).toBe(4294967296);expect(corpus.productionPolicy.maximumSlots).toBe(8);expect(corpus.productionPolicy.minimumAvailableBytes).toBe(1073741824);expect(corpus.productionPolicy.chargeBytes+corpus.productionPolicy.initialParentFloorBytes).toBe(8589934592);}finally{db.close();}});
test("neutral Core cache small consumer uses the same clean slot and pressure model",()=>{const policy=readNativeArtifactCachePolicyV1(corpus.smallPolicy),model=nativeArtifactCacheModelV1(policy);let state=model.readNativeArtifactCacheCatalogV1({version:1,sequence:0,slots:[]});for(let i=0;i<2;i++){const begin=model.beginNativeArtifactCacheV1(state,String(i+1).repeat(64),"a".repeat(64));state=model.commitNativeArtifactCacheV1(begin.state,begin.slot,String(i+1).repeat(64),"a".repeat(64),"b".repeat(64));}const observations=state.slots.map(row=>({slot:row.slot,digest:row.digest,fingerprint:"c".repeat(64),allocatedBytes:1048576,dev:1,ino:row.slot+10})),plan=model.planNativeArtifactCachePressureV1({version:1,state,context:"f".repeat(64),content:"a".repeat(64),availableBytes:policy.minimumAvailableBytes-1048576,observations});expect(plan).toEqual({slot:0,evict:[0]});const invalid=structuredClone(state);invalid.slots[0]!.slot=2;expect(()=>model.readNativeArtifactCacheCatalogV1(invalid)).toThrow();});
test("neutral Core cache policy rejects accessors and captures public mutation",()=>{const request=structuredClone(corpus.smallPolicy),captured=readNativeArtifactCachePolicyV1(request),model=nativeArtifactCacheModelV1(captured);request.maximumSlots=1;expect(model.selectNativeArtifactCacheSlotV1({version:1,state:{version:1,sequence:1,slots:[{slot:0,context:"a".repeat(64),content:"b".repeat(64),state:"clean",sequence:1,digest:"c".repeat(64),retirement:null}]},context:"f".repeat(64),content:"a".repeat(64),active:false})).toEqual({slot:1,kind:"create"});let touched=false;const accessor=Object.defineProperty(structuredClone(corpus.smallPolicy),"chargeBytes",{get(){touched=true;return 1;}});expect(()=>readNativeArtifactCachePolicyV1(accessor)).toThrow();expect(touched).toBe(false);expect(Object.isFrozen(captured)).toBe(true);});
test("neutral Core cache tiny receiver drains detached success Error and undefined",async()=>{for(const marker of[Error("cache child"),undefined]){const ledger=new NativeArtifactCacheReceiverLedgerV1(readNativeArtifactCachePolicyV1(corpus.smallPolicy));let release!:()=>void;const barrier=new Promise<void>(yes=>{release=yes;});ledger.run("read",async()=>{throw marker;});const sibling=ledger.run("read",()=>barrier);let settled=false;const closed=ledger.close().then(()=>({failed:false,error:undefined as unknown}),error=>({failed:true,error}));void closed.then(()=>{settled=true;});await Promise.resolve();await Promise.resolve();expect(settled).toBe(false);release();await sibling;const result=await closed;expect(result.failed).toBe(true);expect(Object.is(result.error,marker)).toBe(true);}});

import{readFileSync}from"node:fs";
import ts from"typescript";
test("neutral Core cache closed policy capture agrees with language-neutral SQLite records",()=>{const validate=new Ajv({strict:true}).compile(schema.definitions.PolicyCapture),db=new Database(":memory:");try{for(const row of corpus.policyCaptures){expect(validate(row)).toBe(true);const oracle=(db.query("SELECT ? IN ('object','null') AND NOT ? AND NOT ? AND NOT ? AS allowed").get(row.prototype,Number(row.accessor),Number(row.proxy),Number(row.symbol))as{allowed:number}).allowed===1;expect(oracle).toBe(row.expected);let request:object=Object.assign(row.prototype==="null"?Object.create(null):row.prototype==="array"?[]:{},corpus.smallPolicy),touched=false;if(row.accessor)Object.defineProperty(request,"chargeBytes",{get(){touched=true;return corpus.smallPolicy.chargeBytes;}});if(row.symbol)Object.defineProperty(request,Symbol("foreign"),{value:1});if(row.proxy)request=new Proxy(request,{ownKeys(target){touched=true;return Reflect.ownKeys(target);}});let accepted=true;try{readNativeArtifactCachePolicyV1(request);}catch{accepted=false;}expect(accepted).toBe(oracle);expect(touched).toBe(false);}}finally{db.close();}});
test("neutral Core cache production implementation has no repository reverse edge",()=>{const db=new Database(":memory:");try{db.exec("CREATE TABLE imports(path TEXT)");for(const path of["../🟦️.ts","../💾️volume/🟦️.ts","../📍️layout/🟦️.ts","../⚙️policy/🟦️.ts","../📋️model/🟦️.ts","../📋️model/📨️receiver/🟦️.ts","../📏️inventory/🟦️.ts","../📂️descriptors/🟦️.ts"]){const source=readFileSync(new URL(path,import.meta.url),"utf8"),tree=ts.createSourceFile(path,source,ts.ScriptTarget.Latest,true);for(const node of tree.statements)if(ts.isImportDeclaration(node)&&ts.isStringLiteral(node.moduleSpecifier))db.query("INSERT INTO imports VALUES(?)").run(node.moduleSpecifier.text);}expect((db.query("SELECT COUNT(*) AS n FROM imports WHERE path LIKE '%products%' OR path LIKE '%🛍️products%' OR path LIKE '%repo%' OR path LIKE '%compiler/💾️state%'").get()as{n:number}).n).toBe(0);const volume=readFileSync(new URL("../💾️volume/🟦️.ts",import.meta.url),"utf8");expect(volume).toContain("requireNativePlatformImageV1(parent,requested)");expect(volume).toContain("readNativeStorageReservationPolicyV1(allocation,allocationControl)");expect(volume).toContain("policy.initialParentFloorBytes!==issuedPolicy.maximumWorkspaceBytes");expect(volume.indexOf("await allocation.recheck()")).toBeLessThan(volume.indexOf('const owner=policy.owner'));expect(volume).not.toContain("receivers=new Set<NativeImageStorageV1>([parent])");}finally{db.close();}});

import{assertNativeCacheDescriptorCustodyV1,decodeLsofPathV1}from"../📂️descriptors/🟦️.ts";
import{join}from"node:path";
test("neutral Core cache actual NUL descriptor wire excludes foreign readers",()=>{const root=join(process.cwd(),"descriptor-root"),validate=new Ajv({strict:true}).compile(schema.definitions.DescriptorCase);for(const row of corpus.descriptors){expect(validate(row)).toBe(true);let accepted=true;try{assertNativeCacheDescriptorCustodyV1({status:0,signal:null,reason:"exit",stdout:row.stdout.replaceAll("{root}",root),stderr:""},root,{pid:71,paths:row.guardPaths.map(path=>path.replaceAll("{root}",root))},row.format==="nul-pcfatn"?"nul-pcfatn":"newline-pn");}catch{accepted=false;}expect(accepted).toBe(row.accepted);}});

import{receiveScriptProcessInvocation}from"../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{scriptCompilerAdmissionControlV1}from"../../../🏗️build/🎟️capacity/🟦️.ts";
import{captureOwnedCommandV1}from"../../../🎛️owned-execution/🟦️.ts";
test("neutral Core cache descriptor wire agrees with independent Python and SQLite",async()=>{await receiveScriptProcessInvocation(process.env,async original=>{const root=join(process.cwd(),"descriptor-root"),control=scriptCompilerAdmissionControlV1(original,"descriptor:oracle",30000),input=corpus.descriptors.map(row=>({...row,stdout:row.stdout.replaceAll("{root}",root),guardPaths:row.guardPaths.map(path=>path.replaceAll("{root}",root))})),program=String.raw`
import codecs,json,re,sqlite3,sys
rows=json.loads(sys.argv[1]);root=sys.argv[2];output=[]
def decode(value):
 raw=value.encode('utf8')
 if re.search(rb'\\(?![bfnrt\\]|x[0-9a-fA-F]{2})',raw):raise ValueError('escape')
 raw=re.sub(rb'\^([@-_?])',lambda m:bytes([127 if m[1]==b'?' else m[1][0]-64]),raw)
 text=codecs.escape_decode(raw)[0].decode('utf8')
 if '\0' in text:raise ValueError('nul')
 return text
for row in rows:
 accepted=True;db=sqlite3.connect(':memory:');db.execute('create table files(pid integer,path text)')
 try:
  wire=row['stdout'];pid=0;pids=set();count=0
  if not wire or not wire.endswith('\n') or len(wire.encode())>16777216:raise ValueError('wire')
  if row['format']=='nul-pcfatn':
   if not wire.endswith('\0\n'):raise ValueError('set')
   sets=wire[:-2].split('\0\n')
   if len(sets)>262144:raise ValueError('sets')
   for group in sets:
    fields=group.split('\0');count+=len(fields)
    if not fields or any(not f or len(f.encode())>65536 for f in fields):raise ValueError('field')
    ids=[f[0] for f in fields]
    if len(ids)!=len(set(ids)) or any(len(f)==1 for f in fields):raise ValueError('duplicate')
    if ids[0]=='p':
     if any(x!='c' for x in ids[1:]):raise ValueError('process')
     value=fields[0][1:]
     if not re.fullmatch('[1-9][0-9]*',value):raise ValueError('pid')
     pid=int(value)
     if pid>9007199254740991 or pid in pids:raise ValueError('pid')
     pids.add(pid)
    elif ids[0]=='f' and pid:
     if any(x not in 'atn' for x in ids[1:]) or 'n' not in ids:raise ValueError('file')
     db.execute('insert into files values(?,?)',(pid,decode(fields[ids.index('n')][1:])))
    else:raise ValueError('start')
  else:
   if '\0' in wire:raise ValueError('nul')
   for field in wire[:-1].split('\n'):
    count+=1
    if not field or len(field.encode())>65536:raise ValueError('field')
    if field[0]=='p':
     if not re.fullmatch('[1-9][0-9]*',field[1:]):raise ValueError('pid')
     pid=int(field[1:])
     if pid>9007199254740991 or pid in pids:raise ValueError('pid')
     pids.add(pid)
    elif field[0]=='n' and pid and len(field)>1:db.execute('insert into files values(?,?)',(pid,decode(field[1:])))
    else:raise ValueError('field')
  if count>524288 or not pids:raise ValueError('complete')
  allowed=row['guardPaths']
  if any(re.search(r'[\\\x00-\x1f\x7f]|\^[@-_?]',path) for path in [root]+allowed):raise ValueError('ambiguous guard')
  db.execute('create table guards(path text primary key)');db.executemany('insert into guards values(?)',[(x,) for x in allowed])
  foreign=db.execute("select count(*) from files where (path=? or substr(path,1,length(?))=?) and (pid<>71 or path not in (select path from guards))",(root,root+'/',root+'/')).fetchone()[0]
  absent=db.execute('select count(*) from guards where path not in (select path from files where pid=71)').fetchone()[0]
  accepted=not foreign and not absent
 except (ValueError,UnicodeError,sqlite3.Error):accepted=False
 finally:db.close()
 output.append(accepted)
print(json.dumps(output,separators=(',',':')))
`;const oracle:unknown=JSON.parse(await captureOwnedCommandV1("python3",["-c",program,JSON.stringify(input),root],process.cwd(),{control}));expect(oracle).toEqual(corpus.descriptors.map(row=>row.accepted));});});

test("neutral Core cache descriptor admission has finite fields and invocation-bound formats",()=>{const root=join(process.cwd(),"descriptor-root"),observe=(stdout:string)=>({status:0,signal:null,reason:"exit"as const,stdout,stderr:""}),guard={pid:71,paths:[]},maximum=schema.definitions.DescriptorPolicy.const.maximumFieldBytes;expect(decodeLsofPathV1("\\b\\f\\n\\r\\t^A^?\\x41")).toBe("\b\f\n\r\t\x01\x7fA");expect(decodeLsofPathV1("literal^x")).toBe("literal^x");expect(()=>assertNativeCacheDescriptorCustodyV1(observe("p71\n"+"a".repeat(schema.definitions.DescriptorPolicy.const.maximumBytes)),root,guard,"newline-pn")).toThrow("Complete compiler descriptor inventory required");expect(()=>assertNativeCacheDescriptorCustodyV1(observe("p71\0\nf1\0n/"+"a".repeat(maximum-2)+"\0\n"),root,guard,"nul-pcfatn")).not.toThrow();expect(()=>assertNativeCacheDescriptorCustodyV1(observe("p71\0\nf1\0n/"+"a".repeat(maximum-1)+"\0\n"),root,guard,"nul-pcfatn")).toThrow();expect(()=>assertNativeCacheDescriptorCustodyV1(observe("p71\0\n"+"f1\0ar\0tREG\0n/outside\0\n".repeat(131072)),root,guard,"nul-pcfatn")).toThrow();expect(()=>assertNativeCacheDescriptorCustodyV1(observe("p71\nn/outside\n"),root,{pid:71,paths:["/foreign"]},"newline-pn")).toThrow();expect(()=>assertNativeCacheDescriptorCustodyV1(observe("p71\nn/outside\n"),root+"\\x41",guard,"newline-pn")).toThrow();for(const status of[1,null])expect(()=>assertNativeCacheDescriptorCustodyV1({...observe("p71\n"),status},root,guard,"newline-pn")).toThrow();for(const reason of["timeout","cancelled","output-limit","spawn-error"]as const)expect(()=>assertNativeCacheDescriptorCustodyV1({...observe("p71\n"),reason},root,guard,"newline-pn")).toThrow();expect(()=>assertNativeCacheDescriptorCustodyV1({...observe("p71\n"),stderr:"partial"},root,guard,"newline-pn")).toThrow();expect(()=>Reflect.apply(assertNativeCacheDescriptorCustodyV1,undefined,[observe("p71\n"),root,guard,"auto"])).toThrow();});
test("neutral Core cache all actual descriptor callers bind their selected wire protocol",()=>{const processRoot=new URL("../../../",import.meta.url),workspace=new URL("../../",processRoot),paths=[new URL("📦️artifacts/💾️cache/💾️volume/🟦️.ts",processRoot),new URL("🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts",workspace),new URL("../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/build-maintenance/📜️script.ts",workspace)],expected=["nul-pcfatn","newline-pn","newline-pn"],flags=["-F0pcfatn","-Fpn","-Fpn"],db=new Database(":memory:");try{db.exec("create table protocols(owner integer,flag text,format text)");for(let i=0;i<paths.length;i++){const path=paths[i];if(!path)throw Error("Exact descriptor Source required");const source=readFileSync(path,"utf8"),tree=ts.createSourceFile(path.pathname,source,ts.ScriptTarget.Latest,true);let count=0;const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="assertNativeCacheDescriptorCustodyV1"){const format=node.arguments[3];expect(format&&ts.isStringLiteral(format)?format.text:null).toBe(expected[i]??null);count++;}ts.forEachChild(node,visit);};visit(tree);expect(count).toBe(1);expect(source).toContain(JSON.stringify(flags[i]));db.query("insert into protocols values(?,?,?)").run(i,flags[i]??"",expected[i]??"");}expect((db.query("select count(*) as n from protocols where (flag='-F0pcfatn' and format='nul-pcfatn') or (flag='-Fpn' and format='newline-pn')").get()as{n:number}).n).toBe(3);}finally{db.close();}});
