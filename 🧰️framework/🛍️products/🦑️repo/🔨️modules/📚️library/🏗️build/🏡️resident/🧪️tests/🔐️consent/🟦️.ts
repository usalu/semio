/** 🔐️ The file access consent of the long-lived roles: which checkouts need it, the one identity it is given to, the probe that tells denied from waiting from fine, the receipts `repo preflight` reads and the bilingual refusals. Judged by Ajv (receipt schema) and by a real unreadable directory. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {spawnSync} from "node:child_process";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import type {AccessReceiptV1} from "../../🔐️consent/🟨️.mjs";
import {ACCESS_PROOF_VERSION_V1,createMountReproofV1,mountSignatureV1,forgetLaunchdRefusalV1,launchdRefusalV1,rememberLaunchdRefusalV1,sampleAccessInChildV1,sampleAccessV1,invalidateLauncherMarkerV1,launcherStateV1,launcherConsentedV1,launcherMarkerPathV1,writeLauncherMarkerV1,accessDirectoryV1,accessReceiptsLackingV1,consentIdentityV1,consentRefusalV1,probeConsentV1,protectedFolderOfV1,readAccessReceiptsV1,writeAccessReceiptV1} from "../../🔐️consent/🟨️.mjs";
import receiptSchema from "../../🔐️consent/🧬️schema/🔣️.json" with {type:"json"};
import resident from "../../🧬️schema/🔣️.json" with {type:"json"};

const home="/Users/worker",mac={platform:"darwin",home},base:AccessReceiptV1={version:1,role:"service",pid:4242,state:"denied",root:`${home}/Documents/semio`,folder:"Documents",identity:`${home}/.local/share/semio/launcher/bin/repo`,at:1_000_000},receipt=(over:Record<string,unknown>={}):AccessReceiptV1=>({...base,...over});

test("only a checkout below Documents, Desktop or Downloads of the user on macOS needs the consent",()=>{
  for(const folder of ["Documents","Desktop","Downloads"])expect(protectedFolderOfV1(`${home}/${folder}/semio/deep`,mac)).toBe(folder);
  for(const root of [`${home}/semio`,"/Users/Shared/semio",`${home}/DocumentsOther/semio`,"/Users/other/Documents/semio",`${home}/Documents/../semio`,home,"relative/Documents","",undefined])expect(protectedFolderOfV1(root as string,mac),String(root)).toBeUndefined();
  expect(protectedFolderOfV1(`${home}/Documents/semio`,{platform:"linux",home})).toBeUndefined();
  expect(protectedFolderOfV1(`${home}/Documents/semio`,{platform:"win32",home})).toBeUndefined();
});

test("the consent belongs to one identity, the launcher at its install location, whatever bun runtime runs below it",()=>{
  expect(consentIdentityV1({platform:"darwin",home,environment:{}})).toEqual({name:"repo",path:`${home}/.local/share/semio/launcher/bin/repo`});
  expect(consentIdentityV1({platform:"darwin",home,environment:{XDG_DATA_HOME:"/data"}})?.path).toBe("/data/semio/launcher/bin/repo");
  expect(consentIdentityV1({platform:"linux",home,environment:{}})).toBeUndefined();
  expect(consentIdentityV1({platform:"darwin",home,environment:{}})?.path).not.toContain("runtimes");
});

test("receipts lie outside every protected folder, one folder per checkout, and a checkout inside Documents never names its own receipts",()=>{
  const first=accessDirectoryV1(`${home}/Documents/semio`,{platform:"darwin",home,environment:{}}),second=accessDirectoryV1(`${home}/Documents/other`,{platform:"darwin",home,environment:{}});
  expect(first).not.toBe(second);
  for(const directory of [first,second])expect(protectedFolderOfV1(directory,mac),directory).toBeUndefined();
  expect(first.startsWith(`${home}/Library/Application Support/semio/access/`)).toBe(true);
  expect(accessDirectoryV1(`${home}/Documents/semio`,{platform:"linux",home,environment:{}}).startsWith(`${home}/.local/state/semio/access/`)).toBe(true);
  expect(accessDirectoryV1(`${home}/Documents/semio`,{platform:"linux",home,environment:{XDG_STATE_HOME:"/state"}}).startsWith("/state/semio/access/")).toBe(true);
  expect(accessDirectoryV1(`${home}/Documents/semio`,{platform:"win32",home,environment:{LOCALAPPDATA:"C:\\Local"}})).toContain("semio");
});

test("the probe tells a readable checkout, a refused one, one that waits for a dialog and one that needs no consent",async()=>{
  const root=`${home}/Documents/semio`;
  const listed:string[]=[];
  expect(await probeConsentV1(root,{...mac,list:async path=>{listed.push(path);return[];}})).toEqual({state:"ok",folder:"Documents"});
  expect(listed).toEqual([root]);
  for(const code of ["EPERM","EACCES"])expect(await probeConsentV1(root,{...mac,list:async()=>{throw Object.assign(new Error(code),{code});}})).toEqual({state:"denied",folder:"Documents",error:code,place:root});
  expect(await probeConsentV1(root,{...mac,list:async()=>{throw Object.assign(new Error("gone"),{code:"ENOENT"});}})).toEqual({state:"ok",folder:"Documents"});
  const started=Date.now();
  expect(await probeConsentV1(root,{...mac,timeoutMilliseconds:40,list:()=>new Promise(()=>{})})).toEqual({state:"waiting",folder:"Documents",error:"no answer"});
  expect(Date.now()-started).toBeLessThan(2000);
  let called=false;
  expect(await probeConsentV1("/Users/Shared/semio",{...mac,list:async()=>{called=true;return[];}})).toEqual({state:"unprotected",folder:""});
  expect(await probeConsentV1(root,{platform:"linux",home,list:async()=>{called=true;return[];}})).toEqual({state:"unprotected",folder:""});
  expect(called).toBe(false);
});

test("the probe reads below the root like the build does: a nested place the system refuses makes the checkout denied and names it, a vanished entry and a bounded sample do not",async()=>{
  const root=`${home}/Documents/semio`,info=(directory:boolean)=>({isDirectory:()=>directory});
  const tree:Record<string,string[]>={[root]:["a","deep",".cache","file"],[root+"/a"]:["x","y"],[root+"/.cache"]:["b"],[root+"/.cache/b"]:["g"],[root+"/.cache/b/g"]:["m"],[root+"/.cache/b/g/m"]:["leaf"]};
  const run=(refusals:Record<string,string>,limit?:number)=>{
    const statted:string[]=[],listed:string[]=[];
    const result=probeConsentV1(root,{...mac,...limit===undefined?{}:{limit},list:async(path:string)=>{listed.push(path);if(refusals["list:"+path])throw Object.assign(new Error(refusals["list:"+path]),{code:refusals["list:"+path]});if(!tree[path])throw Object.assign(new Error("gone"),{code:"ENOTDIR"});return tree[path];},stat:async(path:string)=>{statted.push(path);if(refusals["stat:"+path])throw Object.assign(new Error(refusals["stat:"+path]),{code:refusals["stat:"+path]});return info(Boolean(tree[path]));}});
    return result.then(value=>({value,statted,listed}));
  };
  expect((await run({})).value).toEqual({state:"ok",folder:"Documents"});
  const healthy=await run({});
  expect(healthy.statted).toContain(root+"/deep");expect(healthy.statted).toContain(root+"/.cache/b/g/m/leaf");
  expect(healthy.listed.indexOf(root+"/.cache"),"the cache images are read first").toBe(1);
  expect((await run({["stat:"+root+"/deep"]:"EPERM"})).value).toEqual({state:"denied",folder:"Documents",error:"EPERM",place:root+"/deep"});
  expect((await run({["list:"+root+"/a"]:"EPERM"})).value).toEqual({state:"denied",folder:"Documents",error:"EPERM",place:root+"/a"});
  expect((await run({["list:"+root+"/a"]:"EACCES",["stat:"+root+"/deep"]:"EACCES"})).value,"an ordinary permission below the root is no consent problem").toEqual({state:"ok",folder:"Documents"});
  expect((await run({["stat:"+root+"/.cache/b/g/m/leaf"]:"EPERM"})).value).toMatchObject({state:"denied",place:root+"/.cache/b/g/m/leaf"});
  expect((await run({["stat:"+root+"/deep"]:"ENOENT",["list:"+root+"/a"]:"ENOENT"})).value).toEqual({state:"ok",folder:"Documents"});
  const bounded=await run({},3);
  expect(bounded.statted.length,"a sample is bounded").toBeLessThanOrEqual(3);
  const many=Array.from({length:5000},(_,index)=>"n"+index),touched:string[]=[];
  expect(await sampleAccessV1(root,many,{list:async()=>[],stat:async(path:string)=>{touched.push(path);return info(false);}})).toBeUndefined();
  expect(touched.length).toBeLessThanOrEqual(400);
});

test("the mount boundary: a cache image directory that answers EPERM even for the reset marker it does not contain makes the checkout denied and names the marker; ENOENT for it is fine",async()=>{
  const root=`${home}/Documents/semio`,info=(directory:boolean)=>({isDirectory:()=>directory});
  const tree:Record<string,string[]>={[root]:["a",".cache"],[root+"/.cache"]:["b","f"],[root+"/.cache/b"]:["fixed.dmg","image.json","m"],[root+"/.cache/b/m"]:[],[root+"/.cache/f"]:[]};
  const run=(code?:string,at=root+"/.cache/b/.reset.json")=>{
    const statted:string[]=[];
    return probeConsentV1(root,{...mac,list:async(path:string)=>{if(!tree[path])throw Object.assign(new Error("gone"),{code:"ENOENT"});return tree[path];},stat:async(path:string)=>{statted.push(path);if(path===at){throw Object.assign(new Error(code??"ENOENT"),{code:code??"ENOENT"});}if(path.endsWith("/.reset.json"))throw Object.assign(new Error("ENOENT"),{code:"ENOENT"});return info(Boolean(tree[path]));}}).then(value=>({value,statted}));
  };
  const healthy=await run();
  expect(healthy.value).toEqual({state:"ok",folder:"Documents"});
  for(const directory of ["/.cache/b","/.cache/f","/.cache"])expect(healthy.statted,directory).toContain(root+directory+"/.reset.json");
  expect(healthy.statted.indexOf(root+"/.cache/b/.reset.json"),"the marker is asked before the listing entries of its directory").toBeLessThan(healthy.statted.indexOf(root+"/.cache/b/fixed.dmg"));
  expect((await run("EPERM")).value).toEqual({state:"denied",folder:"Documents",error:"EPERM",place:root+"/.cache/b/.reset.json"});
  expect((await run("EACCES")).value,"EACCES is no consent").toEqual({state:"ok",folder:"Documents"});
  expect((await run("EPERM",root+"/.cache/f/.reset.json")).value).toMatchObject({state:"denied",place:root+"/.cache/f/.reset.json"});
});

test("the refusal of the launchd route is remembered for this boot, for this checkout and these launcher bytes, bounded, and forgotten by new bytes, an older boot, another checkout or ten minutes",()=>{
  const sandbox=mkdtempSync(join(tmpdir(),"consent-memory-")),root=join(sandbox,"Documents","semio"),options={platform:"darwin",home:sandbox,environment:{}},base=1_800_000_000_000;
  try{
    let sha="a".repeat(64);const digest=()=>sha;
    expect(launchdRefusalV1(root,{...options,digest,now:()=>base,uptimeSeconds:()=>3600})).toBeUndefined();
    rememberLaunchdRefusalV1(root,{...options,digest,now:()=>base,place:root+"/.cache/b/.reset.json"});
    const ajv=new Ajv({strict:true}),validate=ajv.compile({...receiptSchema,$ref:"#/definitions/LaunchdRefusal"});
    const file=join(accessDirectoryV1(root,options),"launchd-refused.json");
    expect(validate(JSON.parse(readFileSync(file,"utf8"))),JSON.stringify(validate.errors)).toBe(true);
    expect(statSync(file).mode&0o077,"owner only").toBe(0);
    const at=(now:number,uptime=3600)=>launchdRefusalV1(root,{...options,digest,now:()=>now,uptimeSeconds:()=>uptime});
    expect(at(base+60_000)).toEqual({place:root+"/.cache/b/.reset.json",at:base});
    expect(at(base+9*60_000,24*3600),"booted before the record: still the same boot").toBeDefined();
    expect(at(base+3600_000,600),"the machine booted after the record: another boot").toBeUndefined();
    expect(at(base+11*60_000,30*3600),"older than the retry window: the route is probed again").toBeUndefined();
    expect(at(base-5*3600_000),"from the future").toBeUndefined();
    expect(launchdRefusalV1(join(sandbox,"Documents","other"),{...options,digest,now:()=>base,uptimeSeconds:()=>3600}),"another checkout").toBeUndefined();
    expect(launchdRefusalV1("/Users/Shared/semio",{...options,digest,now:()=>base,uptimeSeconds:()=>3600})).toBeUndefined();
    sha="b".repeat(64);
    expect(at(base+60_000),"the launcher was rebuilt to other bytes").toBeUndefined();
    sha="a".repeat(64);
    expect(at(base+60_000)).toBeDefined();
    forgetLaunchdRefusalV1(root,options);
    expect(at(base+60_000)).toBeUndefined();
  }finally{rmSync(sandbox,{recursive:true,force:true});}
});

test("a mount that appears after the start is seen: the device signature changes with an attach, the whole proof then runs again, once, and a refusal names the place and who found it",async()=>{
  const root=`${home}/Documents/semio`,devices:Record<string,number>={[root]:1,[root+"/.cache"]:1,[root+"/.cache/b"]:1};
  const stat=async(path:string)=>{if(!(path in devices))throw Object.assign(new Error("gone"),{code:"ENOENT"});return{dev:devices[path],isDirectory:()=>true};};
  const list=async(path:string)=>path===root+"/.cache/b"?Object.keys(devices).filter(name=>name.startsWith(root+"/.cache/b/")).map(name=>name.slice(root.length+"/.cache/b/".length)):[];
  const before=await mountSignatureV1(root,{stat,list});
  expect(await mountSignatureV1(root,{stat,list})).toBe(before);
  devices[root+"/.cache/b/m"]=7;
  const after=await mountSignatureV1(root,{stat,list});
  expect(after).not.toBe(before);expect(after).toContain(root+"/.cache/b/m:7");
  devices[root+"/.cache/b/m"]=1;
  const proofs:string[]=[];let refuse:{place:string;error:string}|undefined,child:{place:string;error:string}|undefined;
  const signature=async()=>JSON.stringify(devices);
  const reproof=createMountReproofV1({root,signature,probe:async(path:string)=>{proofs.push("probe "+path);return refuse?{state:"denied",folder:"Documents",error:refuse.error,place:refuse.place}:{state:"ok",folder:"Documents"};},childProbe:async(path:string)=>{proofs.push("child "+path);return child;}});
  await reproof.prime();
  expect(await reproof.check(),"nothing changed since the start").toEqual({changed:false,found:undefined});expect(proofs).toEqual([]);
  devices[root+"/.cache/b/m"]=9;
  expect(await reproof.check(),"a mount appeared and everything is readable").toEqual({changed:true,found:undefined});
  expect(proofs).toEqual(["probe "+root,"child "+root]);
  expect(await reproof.check(),"the proof is not repeated for the same mount").toEqual({changed:false,found:undefined});expect(proofs.length).toBe(2);
  devices[root+"/.cache/b/g"]=11;refuse={place:root+"/.cache/b/.reset.json",error:"EPERM"};
  expect((await reproof.check()).found).toEqual({place:root+"/.cache/b/.reset.json",error:"EPERM",by:"service"});
  devices[root+"/.cache/b/t"]=12;refuse=undefined;child={place:root+"/.cache/b/g/m",error:"EPERM"};
  expect((await reproof.check()).found).toEqual({place:root+"/.cache/b/g/m",error:"EPERM",by:"worker"});
  expect(ACCESS_PROOF_VERSION_V1).toBe(3);
});

test("a really unreadable directory below the root is an ordinary permission (EACCES), not the consent: the probe stays ok, and an independent reader (python) sees the same refusal",async()=>{
  const sandbox=mkdtempSync(join(tmpdir(),"consent-nested-")),root=join(sandbox,"Documents","semio"),nested=join(root,"packages","inner");
  mkdirSync(nested,{recursive:true});writeFileSync(join(nested,"file"),"x");
  try{
    expect((await probeConsentV1(root,{platform:"darwin",home:sandbox})).state).toBe("ok");
    chmodSync(nested,0o000);
    if(process.getuid?.()===0)return;
    expect(await probeConsentV1(root,{platform:"darwin",home:sandbox})).toEqual({state:"ok",folder:"Documents"});
    const python=spawnSync("python3",["-I","-c","import os,sys\ntry:\n os.listdir(sys.argv[1])\nexcept PermissionError as e:\n print(e.errno)\n sys.exit(3)",nested],{encoding:"utf8"});
    expect(python.status).toBe(3);expect(python.stdout.trim()).toBe("13");
  }finally{chmodSync(nested,0o700);rmSync(sandbox,{recursive:true,force:true});}
});

test("the sample also runs in a forked child like the one a slice load runs in: a refused nested place (EPERM) is named, a readable tree and a child that never answers are no refusal",async()=>{
  const sandbox=mkdtempSync(join(tmpdir(),"consent-child-")),root=join(sandbox,"Documents","semio"),nested=join(root,"packages","inner");
  mkdirSync(nested,{recursive:true});writeFileSync(join(nested,"file"),"x");
  try{
    expect(await sampleAccessInChildV1(root)).toBeUndefined();
    expect(await sampleAccessInChildV1(join(sandbox,"missing"))).toBeUndefined();
    const started=Date.now();
    expect(await sampleAccessInChildV1(root,{timeoutMilliseconds:60,spawn:((_script:string)=>({once:()=>undefined,kill:()=>undefined})) as never})).toBeUndefined();
    expect(Date.now()-started).toBeLessThan(2000);
    expect(await sampleAccessInChildV1(root,{spawn:(()=>{throw Error("no spawn");}) as never})).toBeUndefined();
    const refusing=join(sandbox,"refuse.mjs");
    writeFileSync(refusing,"process.send({place:process.argv[2]+'/packages/inner',error:'EPERM'},()=>process.exit(0));");
    expect(await sampleAccessInChildV1(root,{script:refusing})).toEqual({place:nested,error:"EPERM"});
    chmodSync(nested,0o000);
    if(process.getuid?.()===0)return;
    expect(await sampleAccessInChildV1(root),"EACCES is no consent problem in the child either").toBeUndefined();
  }finally{chmodSync(nested,0o700);rmSync(sandbox,{recursive:true,force:true});}
});

test("a receipt is validated by the schema and by the independent validator, is written atomically and read back",()=>{
  const ajv=new Ajv({strict:true,allErrors:true}),validate=ajv.compile({...receiptSchema,$ref:"#/definitions/Receipt"});
  expect(validate(receipt()),JSON.stringify(validate.errors)).toBe(true);
  for(const route of ["launchd","direct","foreground"])expect(validate(receipt({place:`${home}/Documents/semio/inner`,route,ppid:1})),route).toBe(true);
  expect(validate(receipt({route:"cron"}))).toBe(false);expect(validate(receipt({ppid:-1}))).toBe(false);expect(validate(receipt({place:7}))).toBe(false);
  for(const bad of [receipt({state:"maybe"}),receipt({role:"Service"}),receipt({role:""}),receipt({pid:0}),receipt({version:2}),{...receipt(),extra:1},receipt({at:-1})])expect(validate(bad),JSON.stringify(bad)).toBe(false);
  const directory=mkdtempSync(join(tmpdir(),"consent-receipts-"));
  try{
    writeAccessReceiptV1(directory,receipt());
    writeAccessReceiptV1(directory,receipt({role:"client",state:"ok",pid:7,folder:"Documents"}));
    expect(readdirSync(directory).sort()).toEqual(["client.json","service.json"]);
    expect(readAccessReceiptsV1(directory).map(row=>row.role).sort()).toEqual(["client","service"]);
    expect(()=>writeAccessReceiptV1(directory,receipt({role:"../x"}))).toThrow();
    expect(readdirSync(directory).sort()).toEqual(["client.json","service.json"]);
    writeFileSync(join(directory,"broken.json"),"{");writeFileSync(join(directory,"foreign.json"),JSON.stringify({version:1,role:"x"}));writeFileSync(join(directory,"note.txt"),"x");
    expect(readAccessReceiptsV1(directory).length).toBe(2);
    writeAccessReceiptV1(directory,receipt({state:"ok"}));
    expect(readAccessReceiptsV1(directory).find(row=>row.role==="service")?.state).toBe("ok");
    expect(readAccessReceiptsV1(join(directory,"missing"))).toEqual([]);
  }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the roles that lack the consent are the denied or waiting receipts that are recent, newest facts win, healthy and old ones are not listed",()=>{
  const now=10_000_000,rows=[receipt({role:"service",state:"denied",at:now-1000}),receipt({role:"client",state:"ok",at:now-1000}),receipt({role:"replica",state:"waiting",at:now-2000}),receipt({role:"old",state:"denied",at:now-7*3600_000}),receipt({role:"free",state:"unprotected",folder:"",at:now})];
  expect(accessReceiptsLackingV1(rows as never,{now}).map(row=>row.role)).toEqual(["replica","service"]);
  expect(accessReceiptsLackingV1(rows as never,{now,maximumAgeMilliseconds:10}).map(row=>row.role)).toEqual([]);
});

test("the refusal names folder, root and the identity, and distinguishes a refused read from a dialog that waits",()=>{
  const identity={name:"repo",path:`${home}/.local/share/semio/launcher/bin/repo`},root=`${home}/Documents/semio`;
  expect(consentRefusalV1({state:"denied",folder:"Documents"},{root,identity})).toEqual({code:"consent",key:"consent-denied",fields:{folder:"Documents",root,place:root,identity:identity.path},retry:"later"});
  expect(consentRefusalV1({state:"waiting",folder:"Desktop"},{root,identity})).toEqual({code:"consent",key:"consent-waiting",fields:{folder:"Desktop",root,place:root,identity:identity.path},retry:"later"});
  expect(consentRefusalV1({state:"denied",folder:"Documents"},{root,identity:undefined}).fields.identity).toBe("repo");
  expect(consentRefusalV1({state:"denied",folder:"Documents",place:root+"/inner"},{root,identity}).fields.place).toBe(root+"/inner");
  expect(()=>consentRefusalV1({state:"ok",folder:"Documents"},{root,identity})).toThrow();
});

test("the texts exist in both languages with the same placeholders and say what to click and where",()=>{
  const table=(resident as any).definitions.Policy.const.messages as Record<"en"|"de",Record<string,string>>;
  const holes=(value:string)=>[...value.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
  for(const key of ["consent-denied","consent-waiting","access-denied"]){
    expect(table.en[key],key).toBeTruthy();expect(table.de[key],key).toBeTruthy();
    expect(holes(table.de[key]!),key).toEqual(holes(table.en[key]!));
    expect(table.en[key]).not.toBe(table.de[key]);
  }
  for(const language of ["en","de"] as const){
    expect(table[language]["consent-denied"]).toContain("{folder}");expect(table[language]["consent-denied"]).toContain("{identity}");expect(table[language]["consent-denied"]).toContain("repo");
    expect(table[language]["consent-waiting"]).toContain("repo");
  }
  expect(table.en["consent-denied"]).toContain("System Settings");expect(table.en["consent-denied"]).toContain("Privacy & Security");expect(table.en["consent-denied"]).toContain("Files & Folders");expect(table.en["consent-denied"]).toContain("would like to access files in your");
  expect(table.de["consent-denied"]).toContain("Systemeinstellungen");expect(table.de["consent-denied"]).toContain("Datenschutz & Sicherheit");expect(table.de["consent-denied"]).toContain("Dateien & Ordner");
  expect((resident as any).definitions.RefusalCode.enum).toContain("consent");
  expect(existsSync(new URL("../../🔐️consent/🧬️schema/🔣️.json",import.meta.url))).toBe(true);
});

test("the launcher marker names the sha-256 of the launcher as it is now: consented, then not consented after the launcher was rebuilt to other bytes, and the probe script writes it for real",()=>{
  const sandbox=mkdtempSync(join(tmpdir(),"consent-marker-")),launcher=join(sandbox,".local","share","semio","launcher","bin","repo"),root=join(sandbox,"Documents","semio"),options={platform:"darwin",home:sandbox,environment:{}};
  mkdirSync(join(sandbox,".local","share","semio","launcher","bin"),{recursive:true});mkdirSync(root,{recursive:true});writeFileSync(launcher,"launcher-one");
  try{
    expect(launcherStateV1(root,options)).toBe("none");expect(launcherConsentedV1(root,options)).toBe(false);
    writeLauncherMarkerV1(root,options);
    expect(launcherConsentedV1(root,options)).toBe(true);
    expect(launcherMarkerPathV1(root,options).startsWith(join(sandbox,"Library","Application Support","semio","access"))).toBe(true);
    expect(launcherStateV1(root,options)).toBe("consented");
    writeFileSync(launcher,"launcher-two");
    expect(launcherConsentedV1(root,options)).toBe(false);expect(launcherStateV1(root,options),"replaced after it was allowed").toBe("changed");
    expect(launcherConsentedV1(root,{...options,platform:"linux"})).toBe(false);
    rmSync(launcherMarkerPathV1(root,options));writeFileSync(launcher,"launcher-three");
    if(process.platform==="darwin"){
      const probe=join(import.meta.dirname,"..","..","🔐️consent","🔎️probe","🟨️.mjs"),result=spawnSync(process.execPath,[probe,root],{env:{...process.env,HOME:sandbox,XDG_DATA_HOME:""},encoding:"utf8"});
      expect(result.status,result.stderr).toBe(0);
      expect(launcherConsentedV1(root,options)).toBe(true);
      expect(spawnSync(process.execPath,[probe],{env:{...process.env,HOME:sandbox},encoding:"utf8"}).status).toBe(2);
    }
  }finally{rmSync(sandbox,{recursive:true,force:true});}
});

test("a proof counts only for the exact launcher bytes and the exact checkout, and a receipt for other launcher bytes is ignored; a refused place below the checkout keeps the probe from writing the marker; an invalidated marker is gone",()=>{
  const sandbox=mkdtempSync(join(tmpdir(),"consent-bound-")),launcher=join(sandbox,".local","share","semio","launcher","bin","repo"),root=join(sandbox,"Documents","semio"),other=join(sandbox,"Documents","other"),options={platform:"darwin",home:sandbox,environment:{}};
  mkdirSync(join(sandbox,".local","share","semio","launcher","bin"),{recursive:true});mkdirSync(join(root,".cache","b","g","m"),{recursive:true});mkdirSync(other,{recursive:true});writeFileSync(launcher,"launcher-one");
  try{
    writeLauncherMarkerV1(root,options);
    expect(launcherStateV1(root,options)).toBe("consented");expect(launcherStateV1(other,options),"the marker of one checkout proves nothing for another").toBe("none");
    const marker=launcherMarkerPathV1(root,options);
    writeFileSync(marker,JSON.stringify({version:1,sha256:createHash("sha256").update("launcher-one").digest("hex"),root:other,at:1}));
    expect(launcherStateV1(root,options),"a marker written for another checkout").toBe("none");
    writeLauncherMarkerV1(root,options);
    invalidateLauncherMarkerV1(root,options);expect(existsSync(marker)).toBe(false);expect(launcherStateV1(root,options)).toBe("none");
    const stale=receipt({role:"service",state:"denied",launcherSha256:"a".repeat(64),at:5_000_000}),fresh=receipt({role:"client",state:"denied",launcherSha256:"b".repeat(64),at:5_000_000}),unbound=receipt({role:"launcher",state:"waiting",at:5_000_000});
    expect(accessReceiptsLackingV1([stale,fresh,unbound] as never,{now:5_000_100,launcherSha256:"b".repeat(64)}).map(row=>row.role)).toEqual(["client","launcher"]);
    expect(accessReceiptsLackingV1([stale,fresh] as never,{now:5_000_100}).map(row=>row.role)).toEqual(["client","service"]);
    if(process.platform==="darwin"&&process.getuid?.()!==0){
      const probe=join(import.meta.dirname,"..","..","🔐️consent","🔎️probe","🟨️.mjs"),env={...process.env,HOME:sandbox,XDG_DATA_HOME:""};
      chmodSync(join(root,".cache","b","g","m"),0o000);
      try{
        const refused=spawnSync(process.execPath,[probe,root],{env,encoding:"utf8"});
        expect(refused.status,refused.stderr).toBe(3);expect(refused.stderr).toContain(join(root,".cache","b","g","m"));expect(refused.stderr).toContain("verweigert");
        expect(launcherStateV1(root,options),"no marker while a place below the checkout is refused").toBe("none");
      }finally{chmodSync(join(root,".cache","b","g","m"),0o700);}
      expect(spawnSync(process.execPath,[probe,root],{env,encoding:"utf8"}).status).toBe(0);expect(launcherStateV1(root,options)).toBe("consented");
    }
  }finally{rmSync(sandbox,{recursive:true,force:true});}
});
