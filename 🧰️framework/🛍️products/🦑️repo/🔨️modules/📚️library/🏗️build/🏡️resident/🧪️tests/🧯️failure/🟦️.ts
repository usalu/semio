/** 🧯️ An EPERM of the resident service below a macOS-protected folder is named as the missing file access consent in both languages, with a reference and the log; every other failure keeps its former refusal. */
import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {accessDeniedOfV1,deniedPlaceOfV1,failureBlockV1,failureDetailV1,failureMessageV1,failureRefusalV1,mirrorFailureV1,heavyRefusalOfV1,leaseRefusalOfV1,leaseTimeoutRefusalOfV1,serviceFactsV1,wantedRefusalOfV1} from "../../🏛️service/🧯️failure/🟨️.mjs";
import {ResourceLeaseTimeoutErrorV1} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {LeaseOrderErrorV1,type LeaseViolationV1} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

const perm=Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM"}),home="/Users/worker",logs:string[]=[];
const refuse=(error:unknown,options:{root:string;platform?:string})=>failureRefusalV1(error,{log:(event,detail)=>{logs.push(event+" "+detail);},logPath:"/log",home,platform:"darwin",...options});

test("EPERM below Documents, Desktop or Downloads of the user on macOS is the access consent, anywhere else it is internal",()=>{
  for(const folder of ["Documents","Desktop","Downloads"])expect(accessDeniedOfV1(perm,{root:`${home}/${folder}/semio/deep`,platform:"darwin",home})).toBe(folder);
  for(const root of [`${home}/semio`,"/Users/Shared/semio",`${home}/DocumentsOther/semio`,"/Users/other/Documents/semio"])expect(accessDeniedOfV1(perm,{root,platform:"darwin",home}),root).toBeUndefined();
  expect(accessDeniedOfV1(perm,{root:`${home}/Documents/semio`,platform:"linux",home})).toBeUndefined();
  expect(accessDeniedOfV1(new Error("Relative filesystem fstatat failed (ENOENT)"),{root:`${home}/Documents/semio`,platform:"darwin",home})).toBeUndefined();
  expect(accessDeniedOfV1(new Error("Relative filesystem fstatat failed (EPERM)"),{root:`${home}/Documents/semio`,platform:"darwin",home})).toBe("Documents");
});

test("the refusal carries folder, reference and log, keeps the retry advice of the caller and the log holds the failure under the same reference",()=>{
  const refusal=refuse(perm,{root:`${home}/Documents/semio`});
  expect(refusal).toMatchObject({code:"consent",key:"access-denied",retry:"later",fields:{folder:"Documents",log:"/log"}});
  expect(refusal.fields.identity).toContain("semio/launcher/bin/repo");
  expect(logs.at(-1)).toContain(refusal.fields.reference);
  expect(refuse(perm,{root:"/Users/Shared/semio"})).toMatchObject({code:"internal",key:"internal",retry:"later"});
  expect(refuse(Object.assign(new Error("database or disk is full"),{code:"ENOSPC"}),{root:`${home}/Documents/semio`}).code).toBe("disk");
});

test("the text exists in English and German with the same placeholders and names the settings, the identity and the alternative",()=>{
  const texts=schema.$defs?.Texts ?? (schema as any).definitions?.Texts;
  const all=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8"));
  const find=(node:any):any=>node&&typeof node==="object"?(node.en&&node.de&&node.en["access-denied"]?node:Object.values(node).map(find).find(Boolean)):undefined;
  const table=find(all) ?? find(texts);
  const holes=(value:string)=>[...value.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
  expect(holes(table.de["access-denied"])).toEqual(holes(table.en["access-denied"]));
  expect(table.en["access-denied"]).toContain("Files & Folders");
  expect(table.de["access-denied"]).toContain("Dateien & Ordner");
  expect(table.en["access-denied"]).toContain("{identity}");
});

test("the graph-wanted refusal keeps its numbers through values, message text and cause, and every other failure is not one",()=>{
  const values={wanted:703,arrived:0,seconds:902,missing:703,local:192},fields={wanted:"703",arrived:"0",seconds:"902",missing:"703",local:"192",heavy:""};
  const message="unserved build.graph.wanted-expired "+JSON.stringify(values);
  expect(wantedRefusalOfV1(Object.assign(new Error(message),{name:"RepoError",key:"build.graph.wanted-expired",values}))).toEqual(fields);
  expect(wantedRefusalOfV1(new Error(message))).toEqual(fields);
  expect(wantedRefusalOfV1(new Error("outer",{cause:new Error(message)}))).toEqual(fields);
  expect(wantedRefusalOfV1(new Error("secret"))).toBeUndefined();
  expect(refuse(new Error(message),{root:"/Users/Shared/semio"})).toEqual({code:"unserved",key:"host-graph-wanted-expired",fields,retry:"later"});
});

test("a graph-wanted refusal that waited for heavy projects keeps the sentence naming them through values and message text",()=>{
  const heavy=", among them the too heavy project(s) workspace (about 6200 MB) (this machine can give about 4790 MB per project)",values={wanted:2,arrived:0,seconds:300,missing:2,local:192,heavy},message="unserved build.graph.wanted-expired "+JSON.stringify(values);
  const fields={wanted:"2",arrived:"0",seconds:"300",missing:"2",local:"192",heavy};
  expect(wantedRefusalOfV1(Object.assign(new Error(message),{name:"RepoError",key:"build.graph.wanted-expired",values}))).toEqual(fields);
  expect(wantedRefusalOfV1(new Error(message))).toEqual(fields);
  const shown=refuse(new Error(message),{root:"/Users/Shared/semio"});
  expect(shown.fields).toEqual(fields);
  expect(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8").match(/"host-graph-wanted-expired": "[^"]*\{heavy\}/gu)).toHaveLength(2);
});

test("a graph read that stopped projects too heavy for the machine keeps its words through values, message text and cause, and its refusal is retried later in both languages",()=>{
  const values={named:"@semio-tech/s-dev-composition (about 28000 MB)",limit:2396,producers:2},message="unserved build.graph.owner-too-heavy "+JSON.stringify(values),fields={named:values.named,limit:"2396",producers:"2"};
  expect(heavyRefusalOfV1(Object.assign(new Error(message),{name:"RepoError",key:"build.graph.owner-too-heavy",values}))).toEqual(fields);
  expect(heavyRefusalOfV1(new Error(message))).toEqual(fields);
  expect(heavyRefusalOfV1(new Error("outer",{cause:new Error(message)}))).toEqual(fields);
  expect(heavyRefusalOfV1(new Error("secret"))).toBeUndefined();expect(wantedRefusalOfV1(new Error(message))).toBeUndefined();
  expect(refuse(new Error(message),{root:"/Users/Shared/semio"})).toEqual({code:"unserved",key:"host-graph-owner-too-heavy",fields,retry:"later"});
  const text=readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8");
  expect(text.match(/"host-graph-owner-too-heavy": "[^"]*\{named\}[^"]*\{limit\}[^"]*\{producers\}/gu)).toHaveLength(2);
});

test("every EPERM in the service log carries a where line with the refused root and entry, the code, and the route, pid and parent of the service; an EPERM that names nothing says so",()=>{
  const bare=Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM"}),named=Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM",root:`${home}/Documents/semio`,entry:".cache"}),lines:string[]=[];
  expect(deniedPlaceOfV1(named)).toBe(`root=${home}/Documents/semio entry=.cache code=EPERM`);
  expect(deniedPlaceOfV1(new Error("outer",{cause:named}))).toBe(deniedPlaceOfV1(named));
  expect(deniedPlaceOfV1(bare)).toBeUndefined();expect(deniedPlaceOfV1("text")).toBeUndefined();
  expect(serviceFactsV1({environment:{SEMIO_SERVICE_ROUTE:"launchd"},pid:11,ppid:1})).toBe("route=launchd pid=11 ppid=1");
  expect(serviceFactsV1({environment:{SEMIO_SERVICE_ROUTE:"direct"},pid:12,ppid:300})).toBe("route=direct pid=12 ppid=300");
  expect(serviceFactsV1({environment:{},pid:13,ppid:2})).toBe("route=foreground pid=13 ppid=2");
  expect(serviceFactsV1({environment:{SEMIO_SERVICE_ROUTE:"cron"},pid:13,ppid:2})).toBe("route=foreground pid=13 ppid=2");
  const record=(error:unknown,root=`${home}/Documents/semio`)=>failureRefusalV1(error,{log:(event,detail)=>{lines.push(event+" "+detail);},logPath:"/log",home,platform:"darwin",root,environment:{SEMIO_SERVICE_ROUTE:"direct"},pid:4242,ppid:77});
  const one=record(named),two=record(bare);
  expect(lines.at(-2)).toContain(`where: root=${home}/Documents/semio entry=.cache code=EPERM route=direct pid=4242 ppid=77`);
  expect(lines.at(-2)!.startsWith("request-failed "+one.fields.reference+" ")).toBe(true);expect(two.fields.reference).toMatch(/^[0-9a-f]{8}$/u);
  expect(lines.at(-1)).toContain("where: root=? entry=? code=EPERM route=direct pid=4242 ppid=77");
  expect(one.fields).toMatchObject({place:`${home}/Documents/semio/.cache`,route:"direct",pid:"4242",ppid:"77"});
  expect(two.fields).toMatchObject({place:"?",route:"direct",pid:"4242",ppid:"77"});
  const outside=record(named,"/Users/Shared/semio");
  expect(outside.code).toBe("internal");expect(lines.at(-1)).toContain("where: root=");expect(lines.at(-1)).toContain("route=direct pid=4242 ppid=77");
  record(Object.assign(new Error("Relative filesystem fstatat failed (ENOENT)"),{code:"ENOENT"}));
  expect(lines.at(-1)).not.toContain("where:");
});

test("the refusal text names the place and the route of the service in both languages with the same placeholders",()=>{
  const all=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8"));
  const find=(node:any):any=>node&&typeof node==="object"?(node.en&&node.de&&node.en["access-denied"]?node:Object.values(node).map(find).find(Boolean)):undefined;
  const table=find(all);
  for(const language of ["en","de"])for(const name of ["place","route","pid","ppid","reference","log","folder","identity"])expect(table[language]["access-denied"],language+name).toContain("{"+name+"}");
});

test("a failure whose stack lost its message is logged with its name, message, code and project",()=>{
  const lost=Object.assign(new Error("Observation leaf changed before delivery"),{code:"BUILD_INFERENCE_LEAF_CHANGED",semioProject:"libs/alpha",stack:"Error\n    at leafChangedV1 (x.mjs:1:1)"}),lines:string[]=[];
  const detail=failureDetailV1(lost);
  expect(detail.split("\n")[0]).toBe("Error: Observation leaf changed before delivery code=BUILD_INFERENCE_LEAF_CHANGED project=libs/alpha");expect(detail).toContain("at leafChangedV1");
  const intact=new Error("plain");expect(failureDetailV1(intact)).toBe(intact.stack);expect(failureDetailV1("text")).toBe("text");
  failureRefusalV1(lost,{log:(event,line)=>{lines.push(event+" "+line);},logPath:"/log"});
  expect(lines[0]).toContain("BUILD_INFERENCE_LEAF_CHANGED");expect(lines[0]).toContain("project=libs/alpha");
});

test("an EPERM that two reads of the relative call agree on, for a place that reading by path reaches, is no missing macOS consent: the service keeps serving and says what happened",()=>{
  const reported=Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM",root:`${home}/Documents/semio/.cache/b`,directory:`${home}/Documents/semio/.cache/b/g`,entry:".reset.json",reads:2}),lines:string[]=[];
  const run=(error:unknown,inspect:(path:string)=>unknown)=>failureRefusalV1(error,{log:(event,detail)=>{lines.push(event+" "+detail);},logPath:"/log",home,platform:"darwin",root:`${home}/Documents/semio`,environment:{SEMIO_SERVICE_ROUTE:"direct"},pid:4242,ppid:77,inspect});
  const absent=run(reported,()=>{throw Object.assign(new Error("gone"),{code:"ENOENT"});});
  expect(absent).toMatchObject({code:"unserved",key:"host-relative-call-refused",retry:"later",fields:{place:`${home}/Documents/semio/.cache/b/g/.reset.json`,reads:"2",found:"ENOENT",log:"/log"}});
  expect(lines.at(-1)).toContain(`where: root=${home}/Documents/semio/.cache/b directory=${home}/Documents/semio/.cache/b/g entry=.reset.json code=EPERM reads=2 route=direct pid=4242 ppid=77 pathcheck=ENOENT`);
  expect(lines.at(-1)).toContain(absent.fields.reference);
  expect(run(reported,()=>({})).fields).toMatchObject({found:"present"});
  expect(run(reported,()=>{throw Object.assign(new Error("no"),{code:"EACCES"});}).fields).toMatchObject({found:"EACCES"});
});

test("an EPERM that reading by path confirms stays the missing consent and names the directory of the refused entry; an error without the reads of the relative call is judged as before",()=>{
  const reported=Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM",root:`${home}/Documents/semio/.cache/b`,directory:`${home}/Documents/semio/.cache/b/g`,entry:".reset.json",reads:2}),checked:string[]=[];
  const options={log:()=>{},logPath:"/log",home,platform:"darwin",root:`${home}/Documents/semio`};
  const denied=failureRefusalV1(reported,{...options,inspect:path=>{checked.push(path);throw Object.assign(new Error("no"),{code:"EPERM"});}});
  expect(denied).toMatchObject({code:"consent",key:"access-denied",fields:{place:`${home}/Documents/semio/.cache/b/g/.reset.json`}});
  expect(checked).toEqual([`${home}/Documents/semio/.cache/b/g/.reset.json`]);
  checked.length=0;
  const plain=Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM",root:`${home}/Documents/semio`,entry:".cache"});
  expect(failureRefusalV1(plain,{...options,inspect:path=>{checked.push(path);return{};}})).toMatchObject({code:"consent",key:"access-denied"});
  expect(checked).toEqual([]);
  expect(deniedPlaceOfV1(reported)).toBe(`root=${home}/Documents/semio/.cache/b directory=${home}/Documents/semio/.cache/b/g entry=.reset.json code=EPERM reads=2`);
});

test("the default check reads the place by path with the access of the service: absent and present entries of a real scratch directory",()=>{
  const fakeHome=mkdtempSync(join(tmpdir(),"relative-refused-")),root=join(fakeHome,"Documents","semio"),directory=join(root,".cache","b","g");
  try{
    mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"image.json"),"{}");
    const run=(entry:string)=>failureRefusalV1(Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM",root:join(root,".cache","b"),directory,entry,reads:2}),{log:()=>{},logPath:"/log",home:fakeHome,platform:"darwin",root});
    expect(run(".reset.json")).toMatchObject({code:"unserved",key:"host-relative-call-refused",fields:{found:"ENOENT",place:join(directory,".reset.json")}});
    expect(run("image.json").fields.found).toBe("present");
  }finally{rmSync(fakeHome,{recursive:true,force:true});}
});

test("the refusal text exists in English and German with the same placeholders and says it is not the macOS consent",()=>{
  const all=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8"));
  const find=(node:any):any=>node&&typeof node==="object"?(node.en&&node.de&&node.en["host-relative-call-refused"]?node:Object.values(node).map(find).find(Boolean)):undefined;
  const table=find(all),holes=(value:string)=>[...value.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
  expect(holes(table.de["host-relative-call-refused"])).toEqual(holes(table.en["host-relative-call-refused"]));
  for(const name of ["place","reads","found","reference","log"])expect(table.en["host-relative-call-refused"]).toContain("{"+name+"}");
  expect(table.en["host-relative-call-refused"]).toContain("not the proven missing macOS file access consent");
  expect(table.de["host-relative-call-refused"]).toContain("nicht die belegte fehlende macOS-Dateizugriffserlaubnis");
});

test("g1-m02 round 27 try 2: the typed graph memory refusal (reference fb106d37, exit 75) reaches the client as a bilingual refusal that names the project and the numbers, not as the internal failure",async()=>{
 const {GraphMemoryRefusedErrorV1}=await import("../../../🧠️memory/🟨️.mjs");
 const error=new GraphMemoryRefusedErrorV1({message:"graph: @semio-tech/framework-renderer-wgpu needs 1788 MB but one project is granted 1756 MB here and 0 MB are free after 85 s",key:"@semio-tech/framework-renderer-wgpu",needBytes:1788*1048576,availableBytes:1756*1048576,freeBytes:0,seconds:85});
 const wrapped=Object.assign(new Error("slice failed"),{cause:error}),refusal=refuse(wrapped,{root:"/Users/Shared/semio"});
 expect(refusal).toEqual({code:"unserved",key:"host-graph-memory-refused",retry:"later",fields:{key:"@semio-tech/framework-renderer-wgpu",need:"1788",granted:"1756",free:"0",seconds:"85"}});
 expect(logs.at(-1)).toContain("graph-memory-refused");
 const all=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8")),find=(node:any):any=>node&&typeof node==="object"?(node.en&&node.de&&node.en["host-graph-memory-refused"]?node:Object.values(node).map(find).find(Boolean)):undefined,table=find(all),placeholders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(row=>row[1]).sort();
 for(const language of ["en","de"]){const text=table[language]["host-graph-memory-refused"] as string;expect(text,language).toBeTruthy();expect(placeholders(text)).toEqual(["free","granted","key","need","seconds"]);}
});

test("an unexpected failure carries its message in the refusal fields and in both texts, and its block lands in the origin folder of the store",()=>{
  const store=mkdtempSync(join(tmpdir(),"fail-store-")),error=Object.assign(new TypeError("x is not iterable"),{code:"E_BOOM"});
  try{
    const refusal=failureRefusalV1(error,{log:()=>undefined,logPath:"/log",root:"/Users/Shared/semio",environment:{SEMIO_FLEET_STORE_ROOT:store,SEMIO_FLEET_ORIGIN:"g1-test"}});
    expect(refusal).toMatchObject({code:"internal",key:"internal",fields:{log:"/log",message:"TypeError [E_BOOM]: x is not iterable"}});
    const all=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8")),find=(node:any):any=>node&&typeof node==="object"?(node.en&&node.de&&node.en["internal"]?node:Object.values(node).map(find).find(Boolean)):undefined,table=find(all);
    for(const language of ["en","de"])expect(table[language].internal).toContain("{message}");
    const folder=join(store,"v1","origins","g1-test"),text=readFileSync(join(folder,"service-failure.txt"),"utf8");
    expect(text).toContain("request-failed "+refusal.fields.reference);expect(text).toContain("message: x is not iterable");expect(text).toContain("code: E_BOOM");
    expect(text).toContain("en: The resident service met an unexpected failure (reference "+refusal.fields.reference+"): TypeError [E_BOOM]: x is not iterable");
    expect(text).toContain("de: Der residente Dienst ist auf einen unerwarteten Fehler gestoßen (Referenz "+refusal.fields.reference+"): TypeError [E_BOOM]: x is not iterable");
    expect(text).toContain("TypeError: x is not iterable");expect(text).toContain("\n    at ");
    expect(readFileSync(join(folder,"service-failure-"+refusal.fields.reference+".txt"),"utf8")).toBe(text);
  }finally{rmSync(store,{recursive:true,force:true});}
});

test("the block is bounded to 64 KiB and the message of the refusal to 300 characters, whatever the failure says",()=>{
  const huge=new Error("m".repeat(300000)),detail=failureDetailV1(huge),block=failureBlockV1({reference:"abcd0123",detail,error:huge,refusal:{key:"internal",fields:{reference:"abcd0123",log:"/log",message:failureMessageV1(huge)}}});
  expect(Buffer.byteLength(block)).toBeLessThanOrEqual(65536);expect(block.startsWith("request-failed abcd0123 ")).toBe(true);
  expect(failureMessageV1(huge).length).toBe(300);expect(failureMessageV1("plain text")).toBe("Error: plain text");expect(failureMessageV1("a\n  b")).toBe("Error: a b");
});

test("no store or origin in the environment writes nothing, an origin that is a path writes nothing, and only the newest 16 references are kept",()=>{
  const store=mkdtempSync(join(tmpdir(),"fail-keep-"));
  try{
    expect(mirrorFailureV1("x","aaaaaaaa",{environment:{}})).toBeUndefined();
    expect(mirrorFailureV1("x","aaaaaaaa",{environment:{SEMIO_FLEET_STORE_ROOT:store,SEMIO_FLEET_ORIGIN:"../x"}})).toBeUndefined();
    expect(mirrorFailureV1("x","aaaaaaaa",{environment:{SEMIO_FLEET_STORE_ROOT:store}})).toBeUndefined();
    expect(existsSync(join(store,"v1"))).toBe(false);
    const environment={SEMIO_FLEET_STORE_ROOT:store,SEMIO_FLEET_ORIGIN:"g1-keep"},folder=join(store,"v1","origins","g1-keep");
    for(let at=0;at<20;at++){const reference=at.toString(16).padStart(8,"0");mirrorFailureV1("block "+at,reference,{environment});const stamp=new Date(1700000000000+at*1000);utimesSync(join(folder,"service-failure-"+reference+".txt"),stamp,stamp);}
    mirrorFailureV1("block last","ffffffff",{environment});
    const names=readdirSync(folder).filter(name=>/^service-failure-/u.test(name));
    expect(names.length).toBe(16);expect(names).toContain("service-failure-ffffffff.txt");expect(names).not.toContain("service-failure-00000000.txt");
    expect(readFileSync(join(folder,"service-failure.txt"),"utf8")).toBe("block last");
  }finally{rmSync(store,{recursive:true,force:true});}
});

test("a read refused by the lease order is a typed refusal in both languages that names what was wanted, what was held, how long and who held it, not an unexpected failure",()=>{
  const row:LeaseViolationV1={version:1,kind:"hold-and-wait",held:"repository-graph-facts:/Users/ueli/Documents/semio:shard:53",heldMode:"exclusive",heldRank:4,heldClass:"graph",requested:"native-storage-mapping:5874ac7a",requestedMode:"shared",requestedRank:7,requestedClass:"image-mapping",waitedMilliseconds:17000,blockerPid:4242,blockerHeldSeconds:61,blockerCommand:"repo cache gc",atMs:1,pid:7,enforced:true};
  const error=new LeaseOrderErrorV1("Refusing to wait: x",row),fields={requested:row.requested,mode:"shared",held:row.held,seconds:"17",kind:"hold-and-wait",holderPid:"4242",holderCommand:"repo cache gc",holderSeconds:"61",bound:"-"};
  expect(leaseRefusalOfV1(error)).toEqual(fields);expect(leaseRefusalOfV1(new Error("outer",{cause:error}))).toEqual(fields);expect(leaseRefusalOfV1(new Error("secret"))).toBeUndefined();
  const shown=refuse(error,{root:"/Users/Shared/semio"});
  expect(shown).toEqual({code:"unserved",key:"host-lease-refused",fields,retry:"later"});
  const unregistered=refuse(new LeaseOrderErrorV1("x",{...row,blockerPid:undefined,blockerCommand:undefined,blockerHeldSeconds:undefined}),{root:"/Users/Shared/semio"});
  expect(unregistered).toMatchObject({code:"unserved",key:"host-lease-refused-unregistered",retry:"later"});
  const text=JSON.stringify(schema);
  for(const key of ["host-lease-refused","host-lease-refused-unregistered"])expect(text.match(new RegExp('"'+key+'":"[^"]*\\{seconds\\}[^"]*\\{requested\\}',"gu"))).toHaveLength(2);
  expect(text.match(/"host-lease-refused":"[^"]*\{holderPid\}[^"]*\{holderCommand\}[^"]*\{holderSeconds\}/gu)).toHaveLength(2);
  expect(logs.some(line=>line.startsWith("lease-refused "))).toBe(true);
});

test("a lease wait that ran out, of any lease class, is a typed refusal in both languages that names the resource, the mode, the seconds and the holder, not an unexpected failure",()=>{
  const error=new ResourceLeaseTimeoutErrorV1({directory:"/nowhere/leases",resource:"native-storage:ab12",mode:"exclusive"},62000);
  const fields={requested:"native-storage:ab12",mode:"exclusive",seconds:"62",holderPid:"?",holderCommand:"?",holderSeconds:"?"};
  expect(leaseTimeoutRefusalOfV1(error)).toEqual(fields);expect(leaseTimeoutRefusalOfV1(new Error("outer",{cause:error}))).toEqual(fields);expect(leaseTimeoutRefusalOfV1(new Error("secret"))).toBeUndefined();
  expect(refuse(error,{root:"/Users/Shared/semio"})).toEqual({code:"unserved",key:"host-lease-timeout-unregistered",fields,retry:"later"});
  const named={code:"SEMIO_LEASE_TIMEOUT",resource:"native-storage:ab12",mode:"shared",waitedMilliseconds:84000,holder:{pid:12936,command:"bun script.ts component",heldSeconds:610}};
  expect(leaseTimeoutRefusalOfV1(named)).toMatchObject({holderPid:"12936",holderCommand:"bun script.ts component",holderSeconds:"610"});
  const text=JSON.stringify(schema);
  for(const key of ["host-lease-timeout","host-lease-timeout-unregistered"])expect(text.match(new RegExp('"'+key+'":"[^"]*\\{seconds\\}[^"]*\\{requested\\}',"gu"))).toHaveLength(2);
  expect(text.match(/"host-lease-timeout":"[^"]*\{holderPid\}[^"]*\{holderCommand\}[^"]*\{holderSeconds\}/gu)).toHaveLength(2);
});
