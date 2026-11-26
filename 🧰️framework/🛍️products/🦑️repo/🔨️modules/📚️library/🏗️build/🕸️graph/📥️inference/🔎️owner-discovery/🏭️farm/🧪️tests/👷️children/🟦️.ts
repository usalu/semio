/** 👷️ Contract: an owner discovery child answers the same admission facts and the same observed rows as the discovery in this process (the oracle), refuses to serve another admission format or another owner registry than its parent holds (the parent then discovers the owner itself), and the farm is off by environment and never granted below two children.
 * @see ../../🟨️.mjs */
import {afterAll,expect,test} from "bun:test";
import {mkdir,mkdtemp,rm,writeFile} from "node:fs/promises";
import {dirname,join,resolve} from "node:path";
import fixture from "../../../../../../🧪️tests/🧫️fixtures/🕸️graph/🔣️.json";
import {readBuildOwnerRegistryV1,readBuildGraphSelectionV1} from "../../../../../🚪️admission/🟦️.ts";
import {loadedGraphAdmissionFormatV1} from "../../../../../🔐️authority/🟦️.ts";
import {ownerRegistryDigestV1} from "../../../../../🧾️registry-reads/🟨️.mjs";
import {captureObservedSourceInputsV1} from "../../../../../../👁️observation/🟨️.mjs";
import {closeOwnerFactsFarmV1,ownerFactsFarmV1,ownerFarmOffV1,ownerFarmSizeV1,OWNER_FARM_V1} from "../../🟨️.mjs";

const producer=await import("../../../../🟨️.mjs"),row=fixture.ownerAdmission,registry=readBuildOwnerRegistryV1(row.registry),selection=readBuildGraphSelectionV1(row.selection);
const files={"app/📋️project.json":JSON.stringify({name:"app",targets:{test:{executor:"repo:script",cache:false,inputs:["^api"],dependsOn:["^build"],options:{command:"repo ./📜️script.ts test",cwd:"app"},metadata:{semio:{execution:{version:1,kind:"source"}}}}}}),"app/📜️script.ts":"import {thing} from \"../library/api.ts\";export const current = thing;\n","library/📋️project.json":JSON.stringify({name:"lib",namedInputs:{api:["{projectRoot}/types/**/*"]},targets:{}}),"library/api.ts":"export const thing = {value:1};\n","library/types/data.json":"{\"version\":1}\n","📋️build-owners.json":JSON.stringify(row.registry)};
const roots:string[]=[];
const lab=async()=>{const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"owner-farm-"));roots.push(root);for(const[path,bytes]of Object.entries(files)){await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),bytes);}return root;};
afterAll(async()=>{for(const root of roots){closeOwnerFactsFarmV1(root);await rm(root,{recursive:true,force:true});}});
const ready=async(farm:NonNullable<ReturnType<typeof ownerFactsFarmV1>>)=>{for(let waited=0;!farm.ready()&&waited<120000;waited+=250)await new Promise(done=>setTimeout(done,250));expect(farm.ready()).toBe(true);};
const request=(root:string,over:Record<string,unknown>={})=>({owner:"app",targets:["test"],inputs:[],selection,registryDigest:ownerRegistryDigestV1(registry),format:loadedGraphAdmissionFormatV1,epoch:"e1",...over});

test("a child answers the facts and the rows of the discovery in this process",async()=>{
 const root=await lab(),farm=ownerFactsFarmV1(root,{size:2})!;
 expect(farm.size).toBe(2);await ready(farm);
 const owner=registry.owners.find(each=>each.name==="app")!,oracle=await captureObservedSourceInputsV1(()=>producer.discoverBuildOwnerAdmissionFactsV1(owner,["test"],registry,{workspaceRoot:root,selection,nativeCache:new Map(),nativeClosures:new Map()},[]),undefined,false);
 const answer=await farm.discover(request(root)) as {facts:unknown;rows:{kind:string;path:string}[]};
 expect(answer.facts).toEqual(JSON.parse(JSON.stringify(oracle.value)));
 const key=(each:{kind:string;path:string})=>each.kind+"\0"+each.path,mine=new Set(answer.rows.map(key));
 expect(mine.size).toBeGreaterThan(0);
 for(const each of oracle.observations)if(each.kind==="file"||each.kind==="readdir"||each.kind==="readdirNames")expect(mine.has(key(each))).toBe(true);
 expect(answer.rows.some(each=>each.path.endsWith("library/api.ts"))).toBe(true);
 const lib=registry.owners.find(each=>each.name==="lib")!,viaInput=await captureObservedSourceInputsV1(()=>producer.discoverBuildOwnerAdmissionFactsV1(lib,[],registry,{workspaceRoot:root,selection,nativeCache:new Map(),nativeClosures:new Map()},["api"]),undefined,false);
 const again=await farm.discover(request(root,{owner:"lib",targets:[],inputs:["api"],epoch:"e2"})) as {facts:unknown};
 expect(again.facts).toEqual(JSON.parse(JSON.stringify(viaInput.value)));
},180000);

test("a child refuses another admission format and another owner registry, and an unknown owner",async()=>{
 const root=await lab(),farm=ownerFactsFarmV1(root,{size:2})!;await ready(farm);
 await expect(farm.discover(request(root,{format:"another"}))).rejects.toMatchObject({childCode:"OWNER_FARM_FORMAT"});
 await expect(farm.discover(request(root,{registryDigest:"another"}))).rejects.toMatchObject({childCode:"OWNER_FARM_REGISTRY"});
 await expect(farm.discover(request(root,{owner:"absent"}))).rejects.toMatchObject({childCode:"OWNER_FARM_OWNER"});
 await expect(farm.discover(request(root))).resolves.toBeDefined();
},180000);

test("the farm is off by environment and below two children",()=>{
 expect(ownerFarmOffV1({[OWNER_FARM_V1.environment]:OWNER_FARM_V1.disabledValue})).toBe(true);
 expect(ownerFactsFarmV1("/absent",{environment:{[OWNER_FARM_V1.environment]:OWNER_FARM_V1.disabledValue},size:2})).toBeUndefined();
 expect(ownerFarmSizeV1({cpus:2,availableBytes:1e12})).toBe(0);
 expect(ownerFarmSizeV1({cpus:64,availableBytes:1e12})).toBe(OWNER_FARM_V1.maximumChildren);
 expect(ownerFarmSizeV1({cpus:64,availableBytes:OWNER_FARM_V1.childBytes*3.5})).toBe(3);
 expect(ownerFarmSizeV1({cpus:64,availableBytes:OWNER_FARM_V1.childBytes*1.5})).toBe(0);
});
