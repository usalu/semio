import type {BuildGraphPartStubV1,BuildGraphPartV1,BuildGraphRowV1,BuildGraphV1,BuildUnavailableProjectV1} from "../📋️model/🟦️.ts";
import {pinnedAuthorityReadV1} from "../../📥️inputs/🔏️revision/🟨️.mjs";
import * as observationV1 from "../../👁️observation/🟨️.mjs";
import {createProjectFileIndexV1,createWithheldRootIndexV1} from "../🧭️file-owner/🟨️.mjs";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {readBuildOwnerRegistryV1,readBuildGraphSelectionV1,type BuildGraphSelectionV1,type BuildOwnerRegistryV1,type BuildOwnerAdmissionV1} from "../🚪️admission/🟦️.ts";
import {readFile,lstat} from "node:fs/promises";
import {join,dirname} from "node:path";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {advanceScriptInvocation,requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {discoverTestOwnersV1,selectTestFeaturesV1} from "../🎯️test-owners/🟨️.mjs";
import {CLAIMS_POLICY_V1} from "../🕰️generations/🧩️claims/🟨️.mjs";
import {withholdDanglingTestProjectsV1} from "../🕰️generations/🔌️load/🟨️.mjs";
import {deriveEnvironmentV1} from "../🕰️generations/🌍️environment/🟨️.mjs";
import {createEdgeReadsRecorderV1} from "../🕰️generations/🧵️edge-reads/🟨️.mjs";
import {createRowLogV1,type RowLogListV1} from "../🕰️generations/🗃️row-log/🟨️.mjs";
import {repositoryBuildGraphWriterDirectoryV1} from "../../📥️inputs/💾️storage/🟨️.mjs";
import {withClosurePassV1} from "../📥️inference/🧭️script-inputs/🟨️.mjs";
import type {EnvironmentStoredPartsV1} from "../🕰️generations/🧵️edge-reads/🟨️.mjs";
import {requireBuildOwnerRegistryAuthorityV1,graphDeclarationV1,javascriptManagerV1} from "../🔐️authority/🟦️.ts";
import {buildGraphMembersV1,workspaceBuildFilesV1,portableInputs} from "../📁️workspace/🟦️.ts";
import {neutralBuildTargetV1,readBuildGraphV1} from "../📜️contract/🟦️.ts";
import {setupShareV1,edgeConsumersOfPartV1,edgeRefreshOwnersV1} from "../🤝️prefetch/🟦️.ts";
import {taggedOwnersRetainedV1,TEST_TAG_V1,POLICY_TAG_V1} from "../🏷️retention/🟦️.ts";



/** 🛢️ Where the rows of one graph slice are spilled: the inference data directory of the running graph writer (a disk, never a memory file system), else a directory of the host temporary files. */
function rowLogDirectoryV1(root:string):string{
  try{return join(repositoryBuildGraphWriterDirectoryV1(root,"inference"),"rl");}
  catch{return join(tmpdir(),"semio-row-logs");}
}

/** 🚧️ The digest of the bytes of one workspace file as the snapshot observes them; a file that vanished while the graph was built has the digest of no bytes. */
function observedFileDigestV1(root:string,file:string):string{
  try{return createHash("sha256").update(observationV1.observedFs.readFileSync(join(root,file))).digest("hex");}
  catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return createHash("sha256").digest("hex");throw error;}
}


/** 📚️ Assembles all owned inference contributions without installing or invoking Nx. */
export async function discoverBuildGraphV1(root:string,selection:BuildGraphSelectionV1,registry:BuildOwnerRegistryV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,suppliedFiles?:string[],admission?:BuildOwnerAdmissionV1,compact=false):Promise<BuildGraphV1> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);selection=readBuildGraphSelectionV1(selection);registry=readBuildOwnerRegistryV1(registry);
  requireBuildOwnerRegistryAuthorityV1(registry,JSON.parse(pinnedAuthorityReadV1(join(root,"📋️build-owners.json"),()=>observationV1.observedFs.readFileSync(join(root,"📋️build-owners.json"),"utf8"))));
  if(selection.kind!=="all"&&!admission)throw Error("Closed selected owner admission required");
  const files=suppliedFiles??(selection.kind==="all"?buildGraphMembersV1(await workspaceBuildFilesV1(root,invocation)):admission!.files);
  const [owner,tests,policies]=await Promise.all([import("../📥️inference/🟨️.mjs"),import("../../../../🧪️test/📦️packages/🟦️typescript/🟨️.mjs"),import("../📥️inference/🛡️policy/🟨️.mjs")]);checkResourceLeaseControl(phaseControl);
  const policy=graphDeclarationV1("../🔣️policy.json"),options={analyzeLockfile:true,catalogProviders:selection.kind==="all"?registry.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})):admission!.catalogProviders,...(selection.kind!=="all"?{ownerAdmission:admission}:{})};
  const primary=await owner.discoverBuildProjectsV1(files.filter(file=>file.endsWith("📋️project.json")||file.endsWith("Cargo.toml")||javascriptManagerV1.lockFileNames().includes(file)||file.endsWith(".patch")),options,{workspaceRoot:root,invocation,phaseControl,registry});
  if(invocation)await advanceScriptInvocation(invocation,"build:infer","running");
  const contributions=[...primary,...await tests.discoverTestBuildProjectsV1(files.filter(file=>file.endsWith(".feature")),{}, {workspaceRoot:root,invocation,phaseControl})];
  const policyOwners=Object.assign({},...contributions.map(([,contribution])=>contribution.projects??{}));
  contributions.push(...await policies.default.createNodesV2[1](files.filter(file=>file.endsWith("📜️script.ts")),{}, {workspaceRoot:root,projects:policyOwners}));
  const projects:Record<string,any>={},externalNodes:Record<string,unknown>={},unavailable=new Map<string,BuildUnavailableProjectV1>(),configs:Record<string,string>={};
  for(const[file,contribution]of contributions){
    Object.assign(externalNodes,contribution.externalNodes);
    for(const row of (contribution.unavailable??[]) as BuildUnavailableProjectV1[])unavailable.set(row.root,row);
    for(const[name,project]of Object.entries(contribution.projects??{}) as [string,any][]){
      if(projects[name]&&projects[name].root!==project.root)throw Error(`Duplicate build project ${name}`);
      projects[name]={...projects[name],...project,name,targets:{...projects[name]?.targets,...project.targets}};
      configs[name]??=file;
    }
  }
  withholdDanglingTestProjectsV1({projects,configs,unavailable,dangling:tests.danglingTestDependenciesV1(projects),digest:file=>observedFileDigestV1(root,file)});
  const ownerOf=createProjectFileIndexV1(projects),projectFileMap:Record<string,{file:string}[]>=Object.fromEntries(Object.keys(projects).map(name=>[name,[]]));
  const withheldDepth=createWithheldRootIndexV1(unavailable.keys());
  for(const file of files){const owner=ownerOf(file);if(owner!==undefined&&withheldDepth(file)<=projects[owner]!.root.length){const entries=projectFileMap[owner];if(!entries)throw Error("Discovered project file owner is missing");entries.push({file});}}
  const context={workspaceRoot:root,projects,fileMap:{projectFileMap},invocation,phaseControl,selection,registry,completeSourceImports:selection.kind!=="all"};
  const testEdges:unknown=await tests.discoverTestBuildDependenciesV1({},context);if(!Array.isArray(testEdges))throw Error("Test dependency DATA array required");const edges=[...await owner.discoverBuildDependenciesV1(options,context),...testEdges];
  const metadata=await owner.createMetadata({nodes:Object.fromEntries(Object.entries(projects).map(([name,data])=>[name,{name,type:"lib",data}]))},options,context);
  if(!metadata||typeof metadata!=="object"||Array.isArray(metadata))throw Error("Graph metadata DATA object required");
  for(const[name,value]of Object.entries(metadata)){if(!value||typeof value!=="object"||!("metadata"in value)||!value.metadata||typeof value.metadata!=="object"||Array.isArray(value.metadata))throw Error("Project metadata DATA object required");if("semio"in value.metadata&&value.metadata.semio!==undefined&&(!value.metadata.semio||typeof value.metadata.semio!=="object"||Array.isArray(value.metadata.semio)))throw Error("Project semio metadata DATA object required");projects[name].metadata={...projects[name].metadata,...value.metadata,semio:{...projects[name].metadata?.semio,...("semio"in value.metadata&&value.metadata.semio&&typeof value.metadata.semio==="object"?value.metadata.semio:{})}};}
  const neutral=Object.fromEntries(Object.entries(projects).map(([name,project])=>[name,{...project,targets:Object.fromEntries(Object.entries(project.targets??{}).map(([target,value])=>[target,neutralBuildTargetV1(value as Record<string,any>,`${name}:${target}`)]))}]));
  const dependencies=[...new Map(edges.map(edge=>[JSON.stringify(edge),edge])).entries()].sort(([a],[b])=>a.localeCompare(b)).map(([,edge])=>edge);
  if(invocation)await advanceScriptInvocation(invocation,"build:graph","complete");
  const compactGraph=({version:1,projects:neutral,dependencies,namedInputs:policy.namedInputs,externalNodes,...(unavailable.size?{unavailable:[...unavailable.values()].sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0)}:{}),admission:{selection,registry:createHash("sha256").update(JSON.stringify(registry)).digest("hex"),closure:createHash("sha256").update(JSON.stringify(admission??files)).digest("hex")}});
  return compact?compactGraph as unknown as BuildGraphV1:readBuildGraphV1({...compactGraph,projects:await portableInputs(root,neutral,invocation,phaseControl)});
}


/** 🔗️ Infers only the projects whose roots are in `part.roots` (all of them when cold) plus the test and policy contributions, recomputes the edges of every other owner when the environment moved, and returns the pieces owner by owner. Everything an owner consumed is recorded in its own project record or in the capture around this call, so a later edit finds exactly its consumers. */
export async function discoverBuildGraphPartV1(root:string,registry:BuildOwnerRegistryV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,files:readonly string[],part:{cold:boolean;roots:ReadonlySet<string>;stubs:Record<string,BuildGraphPartStubV1>;previousEnv:string|null;previousParts?:EnvironmentStoredPartsV1;previousOwners?:readonly (readonly [string,string,string|null])[];changed?:readonly string[];ownerDone?:()=>void;ownerRunning?:(running:readonly {name:string;seconds:number;kind?:string;rows?:number}[],decision?:unknown)=>unknown}):Promise<BuildGraphPartV1> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);registry=readBuildOwnerRegistryV1(registry);
  requireBuildOwnerRegistryAuthorityV1(registry,JSON.parse(pinnedAuthorityReadV1(join(root,"📋️build-owners.json"),()=>observationV1.observedFs.readFileSync(join(root,"📋️build-owners.json"),"utf8"))));
  const [owner,tests,policies]=await Promise.all([import("../📥️inference/🟨️.mjs"),import("../../../../🧪️test/📦️packages/🟦️typescript/🟨️.mjs"),import("../📥️inference/🛡️policy/🟨️.mjs")]);checkResourceLeaseControl(phaseControl);
  const rowLog=createRowLogV1<BuildGraphRowV1>({path:join(rowLogDirectoryV1(root),"rl-"+process.pid+"-"+Date.now().toString(36)+".rows")}),edgeReads=createEdgeReadsRecorderV1(),nodeRows:Record<string,RowLogListV1<BuildGraphRowV1>>={},derivedFacts=new Map<string,{readonly row:BuildGraphRowV1;readonly support:Iterable<BuildGraphRowV1>}>(),context={workspaceRoot:root,invocation,phaseControl,registry,setupShare:setupShareV1(invocation)},stubs=Object.entries(part.stubs),realRoots=new Set(stubs.filter(([,stub])=>!stub.tag).map(([,stub])=>stub.root));
  const read=async(path:string)=>{try{return await readFile(join(root,path),"utf8");}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return undefined;throw error;}},stat=async(path:string)=>{try{const status=await lstat(join(root,path));return status.isFile()?{size:status.size,mtimeMs:status.mtimeMs}:undefined;}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return undefined;throw error;}};
  const captured=await withClosurePassV1(()=>observationV1.captureObservedSourceInputsV1(async()=>{
    const options={analyzeLockfile:true,catalogProviders:registry.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})),snapshot:true,...(part.cold?{}:{ownerRoots:part.roots}),...(part.ownerRunning?{onOwnerRunning:part.ownerRunning}:{}),rowLog,onOwnerObservations:(name:string,rows:Iterable<BuildGraphRowV1>&{length:number},derived?:{readonly row:BuildGraphRowV1;readonly support:Iterable<BuildGraphRowV1>&{length:number}}[])=>{nodeRows[name]=Array.isArray(rows)?rowLog.append(rows):rows as RowLogListV1<BuildGraphRowV1>;part.ownerDone?.();for(const fact of derived??[])derivedFacts.set(fact.row.path+"\0"+fact.row.fact,{row:fact.row,support:Array.isArray(fact.support)?rowLog.append(fact.support):fact.support});}};
    const primary=await owner.discoverBuildProjectsV1(files.filter(file=>file.endsWith("📋️project.json")||file.endsWith("Cargo.toml")||javascriptManagerV1.lockFileNames().includes(file)||file.endsWith(".patch")),options,context);
    if(invocation)await advanceScriptInvocation(invocation,"build:infer","running");
    const projects:Record<string,any>={},externalNodes:Record<string,unknown>={},unavailable=new Map<string,BuildUnavailableProjectV1>(),tags:Record<string,string>={},configs:Record<string,string>={},primaryNames=new Set<string>(),testNames=new Set<string>();
    const merge=(list:[string,any][],primaryList:boolean)=>list.forEach(([file,contribution])=>{
      Object.assign(externalNodes,contribution.externalNodes);
      for(const row of (contribution.unavailable??[]) as BuildUnavailableProjectV1[])unavailable.set(row.root,row);
      for(const[name,project]of Object.entries(contribution.projects??{}) as [string,any][]){
        if(projects[name]&&projects[name].root!==project.root)throw Error(`Duplicate build project ${name}`);
        projects[name]={...projects[name],...project,name,targets:{...projects[name]?.targets,...project.targets}};
        if(primaryList)primaryNames.add(name);
        configs[name]??=file;
      }
    });
    merge(primary as [string,any][],true);
    const kept=stubs.filter(([name,stub])=>!stub.tag&&stub.kind==="project"&&!projects[name]&&!part.roots.has(stub.root));
    const listed=[...kept.map(([name,stub])=>({name,root:stub.root,nativeRoot:stub.nativeRoot,config:stub.config})),...Object.entries(projects).map(([name,project])=>({name,root:project.root,nativeRoot:project.metadata?.nativeRoot as string|undefined,config:configs[name]}))];
    const env=await deriveEnvironmentV1({projects:listed,workspaceManifests:files.filter(file=>file==="Cargo.toml"||file.endsWith("/Cargo.toml")),lock:javascriptManagerV1.primaryLockName(),taxonomy:"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",read,stat});
    const tagged=stubs.filter(([,stub])=>stub.tag),retained=taggedOwnersRetainedV1({cold:part.cold,changed:part.changed,roots:part.roots,stubs:part.stubs,environment:env.digest,previousEnvironment:part.previousEnv});
    const incremental=!retained&&!part.cold&&part.changed!==undefined&&tagged.some(([,stub])=>stub.tag===TEST_TAG_V1)&&tagged.every(([,stub])=>stub.tag===TEST_TAG_V1||stub.tag===POLICY_TAG_V1)&&env.digest===part.previousEnv;
    const tagRows:BuildGraphRowV1[]=[],testRows:BuildGraphRowV1[]=[],factsInto=(sink:BuildGraphRowV1[])=>async<T>(run:()=>Promise<T>|T):Promise<T>=>{const taken=await observationV1.captureObservedSourceInputsV1(async()=>run(),undefined,false);sink.push(...(taken.observations as BuildGraphRowV1[]));return taken.value;},discoveredForTags=factsInto(tagRows),discoveredForTests=factsInto(testRows);
    const known=new Set(tagged.filter(([,stub])=>stub.tag===TEST_TAG_V1).map(([,stub])=>stub.root)),groups=retained?new Map<string,string[]>():selectTestFeaturesV1(files.filter(file=>file.endsWith(".feature")),{all:!incremental,roots:new Set([...part.roots,...(part.changed??[]).filter(path=>path.endsWith(".feature")).map(path=>dirname(path))]),known});
    const discovery=await observationV1.withObservationRoundV1(()=>discoverTestOwnersV1<BuildGraphRowV1>({tests:tests as never,capture:run=>observationV1.captureObservedSourceInputsV1(async()=>run(),undefined,false) as unknown as Promise<{value:any;observations:readonly BuildGraphRowV1[]}>,context,groups})),testContributions=discovery.contributions as [string,any][];for(const fact of discovery.derived)derivedFacts.set(fact.row.path+"\0"+fact.row.fact,fact);
    for(const[,contribution]of testContributions)for(const name of Object.keys(contribution.projects??{}))testNames.add(name);
    const policyOwners=Object.assign({},...[...primary,...testContributions].map(([,contribution])=>contribution.projects??{}));
    const policyContributions=await discoveredForTags(()=>policies.default.createNodesV2[1](files.filter(file=>file.endsWith("📜️script.ts")&&(part.cold||part.roots.has(dirname(file))||!retained&&!realRoots.has(dirname(file)))),{}, {workspaceRoot:root,projects:policyOwners}));
    merge(testContributions as [string,any][],false);merge(policyContributions as [string,any][],false);
    for(const name of Object.keys(projects))if(!primaryNames.has(name))tags[name]=testNames.has(name)?TEST_TAG_V1:POLICY_TAG_V1;
    if(!retained)await discoveredForTests(()=>withholdDanglingTestProjectsV1({projects,configs,unavailable,dangling:tests.danglingTestDependenciesV1({...Object.fromEntries(kept.map(([name,stub])=>[name,{root:stub.root}])),...projects}),digest:file=>observedFileDigestV1(root,file)}));
    const keptTagged=retained?tagged:incremental?tagged.filter(([,stub])=>stub.tag===TEST_TAG_V1&&!groups.has(stub.root)&&!part.roots.has(stub.root)):[],keptNames=new Set(keptTagged.map(([name])=>name));
    const consumers=part.cold||env.digest===part.previousEnv?undefined:edgeConsumersOfPartV1({previousEnvironment:part.previousEnv,previousParts:part.previousParts,previousOwners:part.previousOwners??[],parts:env.parts,stubs:part.stubs,inferred:new Set(Object.keys(projects)),roots:part.roots}),refresh=edgeRefreshOwnersV1({cold:part.cold,environment:env.digest,previousEnvironment:part.previousEnv,roots:part.roots,stubs:part.stubs,inferred:new Set(Object.keys(projects)),...(consumers?{consumers}:{})}),edgeNames=refresh.projects,processed=new Set([...Object.keys(projects),...edgeNames]);
    const contextProjects:Record<string,any>={...Object.fromEntries([...kept,...keptTagged.filter(([,stub])=>stub.kind==="project")].map(([name,stub])=>[name,{name,root:stub.root,targets:Object.fromEntries(stub.targets.map(target=>[target,{}])),...(stub.nativeRoot?{metadata:{nativeRoot:stub.nativeRoot}}:{})}])),...projects};
    const ownerOf=createProjectFileIndexV1(contextProjects),withheld=[...unavailable.keys(),...stubs.filter(([name,stub])=>(!stub.tag||keptNames.has(name))&&stub.kind==="unavailable"&&!part.roots.has(stub.root)).map(([,stub])=>stub.root)],projectFileMap:Record<string,{file:string}[]>=Object.fromEntries([...processed].map(name=>[name,[]]));
    const withheldDepth=createWithheldRootIndexV1(withheld);
    for(const file of files){const found=ownerOf(file);if(found!==undefined&&projectFileMap[found]&&withheldDepth(file)<=contextProjects[found].root.length)projectFileMap[found]!.push({file});}
    const streamed=new Set<string>(edgeNames),direct=Object.fromEntries([...processed].filter(name=>!streamed.has(name)).map(name=>[name,projectFileMap[name]!]));
    const graphContext={workspaceRoot:root,projects:contextProjects,fileMap:{projectFileMap:direct},invocation,phaseControl,selection:{kind:"all"},registry,edgeReads,completeSourceImports:false,moduleGraphProjects:new Set(Object.keys(projects)),edgeChunkOwners:CLAIMS_POLICY_V1.chunkOwners};
    const testEdges:unknown=retained?[]:await discoveredForTests(()=>tests.discoverTestBuildDependenciesV1({},graphContext));if(!Array.isArray(testEdges))throw Error("Test dependency DATA array required");
    const implementation=streamed.size?await owner.loadCurrentImplementationV1():undefined;await implementation?.libraryBootstrap;
    const streaming=await implementation?.createStreamingEdgesV1(options,{...graphContext,fileMap:{projectFileMap}}),edges=[...(await owner.discoverBuildDependenciesV1(options,graphContext)).filter((edge:{source:string})=>!streamed.has(edge.source)),...testEdges];
    const metadata=await owner.createMetadata({nodes:Object.fromEntries(Object.entries(contextProjects).map(([name,data])=>[name,{name,type:"lib",data}]))},options,graphContext);
    if(!metadata||typeof metadata!=="object"||Array.isArray(metadata))throw Error("Graph metadata DATA object required");
    for(const[name,value]of Object.entries(metadata)){if(!projects[name])continue;if(!value||typeof value!=="object"||!("metadata"in value)||!value.metadata||typeof value.metadata!=="object"||Array.isArray(value.metadata))throw Error("Project metadata DATA object required");projects[name].metadata={...projects[name].metadata,...value.metadata,semio:{...projects[name].metadata?.semio,...("semio"in value.metadata&&value.metadata.semio&&typeof value.metadata.semio==="object"?value.metadata.semio:{})}};}
    const neutral=Object.fromEntries(Object.entries(projects).map(([name,project])=>[name,{...project,targets:Object.fromEntries(Object.entries(project.targets??{}).map(([target,value])=>[target,neutralBuildTargetV1(value as Record<string,any>,`${name}:${target}`)]))}]));
    if(invocation)await advanceScriptInvocation(invocation,"build:graph","complete");
    const rows=[...unavailable.values()],testRowLists=new Map<object,BuildGraphRowV1[]>(),testNodeRows=(name:string)=>{const own=discovery.rows.get(name)??[];let list=testRowLists.get(own);if(!list)testRowLists.set(own,list=[...own,...testRows]);return list;};
    return {
      owners:[...Object.entries(neutral).map(([name,node]:[string,any])=>({name,root:node.root as string,nativeRoot:node.metadata?.nativeRoot as string|undefined,config:configs[name],...(tags[name]?{tag:tags[name]}:{}),...(tags[name]||!edgeReads.finish(name)?{}:{reads:edgeReads.finish(name)}),node,nodeRows:tags[name]===TEST_TAG_V1?testNodeRows(name):tags[name]?tagRows:nodeRows[name]})),...rows.filter(row=>row.project!==undefined).map(row=>({name:row.project!,root:row.root,config:row.source,...(tags[row.project!]?{tag:tags[row.project!]}:{}),unavailable:row,nodeRows:tags[row.project!]===TEST_TAG_V1?testNodeRows(row.project!):tags[row.project!]?tagRows:nodeRows[row.project!]}))],
      edgeOnly:[...edgeNames.map(name=>streamed.has(name)?{name,streamed:true as const}:{name}),...refresh.withheld.map(name=>({name}))],reEdge:streaming&&((name:string,fragment:{node?:any})=>streaming.edgesOf(name,fragment.node)),readsOf:(name:string)=>edgeReads.finish(name),carried:refresh.carried,derived:[...derivedFacts.values()],edges:edges as BuildGraphPartV1["edges"],externalNodes,parseUnavailable:rows.filter(row=>row.project===undefined),env,replaceTags:retained?[]:incremental?["policy",POLICY_TAG_V1]:["tests","policy",TEST_TAG_V1,POLICY_TAG_V1],
    };
  },undefined,false)).catch(error=>{rowLog.close();throw error;});
  return {...captured.value,rows:[...captured.observations] as BuildGraphRowV1[],dispose:()=>rowLog.close()};
}
