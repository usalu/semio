import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {slotIdV1} from "../../../../🪢️slots/🟦️.ts";
import {RemoteCacheStoreV1} from "../../../💾️store/🟦️.ts";
import {declareV1,openTreeDigestsV1} from "../../🍂️staleness/🟦️.ts";

const scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`rcache-tomb-${label}-`));scratch.push(path);return path;};
const body=(bytes:Uint8Array)=>(async function*(){yield bytes;})();
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const sha=(value:string)=>createHash("sha256").update(value).digest("hex");
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const [path,text] of Object.entries(files)){const full=join(root,path);mkdirSync(dirname(full),{recursive:true});writeFileSync(full,text);}};
const open=async(folder:string,workspace:string)=>RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**30,maximumEntryBytes:2**22,maximumAgeMilliseconds:0,graceMilliseconds:150,staleness:{mode:"enforce",tree:await openTreeDigestsV1(workspace),examples:5,recheckMilliseconds:0,manifestNames:["unit.json"]}});
type Built=Readonly<{source?:readonly [string,string];externs?:readonly string[];produces:string}>;
const put=(store:RemoteCacheStoreV1,key:string,built:Built)=>{const bytes=new TextEncoder().encode(JSON.stringify({hash:sha(built.produces)}));return store.put("main","nx",key,body(bytes),{mode:"keep",declaredLength:bytes.length,slot:{slot:slotIdV1("rust-unit",[key,"build","",""]),declaration:declareV1({kind:"rust-unit",toolchain:sha("t1"),sources:built.source===undefined?[]:[{path:built.source[0],sha256:sha(built.source[1])}],externs:(built.externs??[]).map(sha)})}});};
const live=async(store:RemoteCacheStoreV1,key:string)=>(await store.read("main","nx",key))!==undefined;
const tombstones=(store:RemoteCacheStoreV1)=>JSON.parse(readFileSync(join(store.root,".retention","retired.json"),"utf8")) as unknown[];

test("a registry unit retired as unreached leaves no tombstone: a current unit that later consumes its blob from a machine's own cache stays live",async()=>{
 const tree=directory("tree"),store=await open(directory("unreached"),tree);write(tree,{"app/lib.rs":"app v1","tool/lib.rs":"tool v1"});
 await put(store,"app",{source:["app/lib.rs","app v1"],externs:["blob:serde"],produces:"blob:app"});
 await put(store,"tool",{source:["tool/lib.rs","tool v1"],externs:["blob:other"],produces:"blob:tool"});
 await put(store,"serde",{produces:"blob:serde"});await put(store,"other",{produces:"blob:other"});
 write(tree,{"app/lib.rs":"app v2"});await sleep(200);await store.collect();
 expect(await live(store,"app"),"edited").toBe(false);expect(await live(store,"serde"),"no live consumer left").toBe(false);
 await put(store,"app-v2",{source:["app/lib.rs","app v2"],externs:["blob:serde"],produces:"blob:app2"});
 await sleep(200);await store.collect();
 expect(await live(store,"app-v2"),"its sources are the tree's and nothing it consumed is stale: serde left because nobody consumed it, not because its content went stale").toBe(true);
 expect(store.stalenessStatus()).toMatchObject({stale:{},unreached:0});
 expect(tombstones(store).length,"only the edited unit's blob is remembered").toBe(1);
 await store.close();
});

test("a tombstone holds only while its cause holds: once the edited file is back to the retired content, a unit consuming the retired blob is live again; while it differs, such a unit is stale",async()=>{
 const tree=directory("tree-back"),store=await open(directory("revert"),tree);write(tree,{"core/lib.rs":"core v1","mid/lib.rs":"mid v1"});
 await put(store,"core",{source:["core/lib.rs","core v1"],produces:"blob:core"});
 await put(store,"mid",{source:["mid/lib.rs","mid v1"],externs:["blob:core"],produces:"blob:mid"});
 write(tree,{"core/lib.rs":"core v2"});await sleep(200);await store.collect();
 expect(await live(store,"core")).toBe(false);expect(await live(store,"mid")).toBe(false);
 expect(tombstones(store)).toEqual(expect.arrayContaining([{blob:sha("blob:core"),path:"core/lib.rs",sha256:sha("core v1")}]));
 await put(store,"late",{source:["mid/lib.rs","mid v1"],externs:["blob:core"],produces:"blob:late"});
 await sleep(200);await store.collect();
 expect(await live(store,"late"),"consumes the retired core while core/lib.rs still differs").toBe(false);
 write(tree,{"core/lib.rs":"core v1"});
 await put(store,"mid-again",{source:["mid/lib.rs","mid v1"],externs:["blob:core"],produces:"blob:mid"});
 await sleep(200);await store.collect();
 expect(await live(store,"mid-again"),"core/lib.rs is back to the content the retired blob was built from").toBe(true);
 expect(store.stalenessStatus()).toMatchObject({stale:{}});
 await store.close();
});
