import {afterAll,expect,test} from "bun:test";
import {cpSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fleetFetchContextV1} from "../../🟦️.ts";
import {syncFleetIdentityV1} from "../../../../../🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {fleetUvPointerV1,fleetUvPortV1,fleetUvPublishV1,fleetUvRestoreV1,fleetUvWheelsV1} from "../🟦️.ts";

const uv=Bun.which("uv"),scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`fleet-uv-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const run=async(argv:string[],cwd:string,cache:string,extra:Record<string,string>={})=>{
 const child=Bun.spawn(argv,{cwd,stdout:"pipe",stderr:"pipe",env:{...process.env,UV_CACHE_DIR:cache,UV_PYTHON_DOWNLOADS:"never",UV_NO_CONFIG:"1",...extra}}),[out,err,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
 return {out,err,code};
};
const member=(store:string)=>{
 const root=directory("member");mkdirSync(join(root,".cache","f"),{recursive:true});
 writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:"g1-m01",groupId:"g1",storeRoot:store,key:"k"}));return root;
};

test.skipIf(uv===null)("a warm uv cache seeds an empty one through the fleet store: uv then needs no download for the locked wheels; its own listing agrees with our reader of uv.lock and of its pointers",async()=>{
 const store=directory("store"),project=directory("project"),donor=directory("donor"),fresh=directory("fresh"),venv=join(project,".venv");
 writeFileSync(join(project,"pyproject.toml"),'[project]\nname = "probe"\nversion = "0.0.0"\nrequires-python = ">=3.9"\ndependencies = ["six==1.16.0", "idna==3.7"]\n');
 const python=Bun.which("python3");expect(python).not.toBeNull();
 const locked=await run([uv!,"lock","--python",python!],project,donor);expect(locked.code,locked.err).toBe(0);
 const synced=await run([uv!,"sync","--locked","--python",python!],project,donor);expect(synced.code,synced.err).toBe(0);
 const lock=readFileSync(join(project,"uv.lock"),"utf8"),wheels=fleetUvWheelsV1(lock);
 expect(wheels.map(wheel=>wheel.name).sort()).toEqual(["idna","six"]);
 const listing=await run(["find",join(donor,"wheels-v6","pypi"),"-name","*.http"],project,donor);
 expect(listing.out.trim().split("\n").length,"uv itself cached one pointer per wheel").toBe(2);
 for(const wheel of wheels){const pointer=fleetUvPointerV1(readFileSync(join(donor,"wheels-v6","pypi",wheel.name,`${wheel.stem}.http`)));expect(pointer?.sha256,"the digest of uv's pointer equals the hash in uv.lock").toBe(wheel.sha256);}
 const publisher=member(store);await syncFleetIdentityV1({root:publisher});
 mkdirSync(join(store,"v1","origins","g1-m01"),{recursive:true});cpSync(join(publisher,".cache","f","identity","registration.json"),join(store,"v1","origins","g1-m01","fleet-auth.json"));
 const context=fleetFetchContextV1({},publisher,{storeRoot:store});
 expect(await fleetUvPublishV1(context,{cacheDirectory:donor,lock})).toBe(2);
 expect(await fleetUvPublishV1(context,{cacheDirectory:donor,lock}),"a second publish packs the same blobs").toBe(2);
 const seeds=()=>readdirSync(join(store,"v1","fleet","uv")).filter(name=>name.endsWith(".json"));
 expect(seeds().length).toBe(1);
 expect(seeds()[0],"the seed is named by the lock and the host class").toMatch(/^uv-[0-9a-f]{64}-(darwin|linux|win32)-[a-z0-9_-]+\.json$/u);
 expect(await fleetUvPublishV1(context,{cacheDirectory:donor,lock,host:{platform:"linux",arch:"x64",libc:"gnu"}})).toBe(2);
 expect(seeds().length,"another host class writes its own seed and replaces nobody's").toBe(2);
 expect(await fleetUvRestoreV1(context,{cacheDirectory:directory("windows"),lock,host:{platform:"win32",arch:"x64"}}),"a class nobody seeded restores nothing").toBe(0);
 const blobs=readdirSync(join(store,"v1","fleet","uv","blobs")).flatMap(folder=>folder.startsWith(".")?[]:readdirSync(join(store,"v1","fleet","uv","blobs",folder)));
 expect(blobs.length).toBeGreaterThan(2);
 const before=await run([uv!,"sync","--locked","--python",python!],project,directory("control"),{UV_OFFLINE:"1",UV_PROJECT_ENVIRONMENT:join(project,".venv-control")});
 expect(before.code===0,"an empty cache cannot sync offline").toBe(false);
 expect(await fleetUvRestoreV1(context,{cacheDirectory:fresh,lock})).toBe(2);
 expect(await fleetUvRestoreV1(context,{cacheDirectory:fresh,lock}),"nothing left to place").toBe(0);
 const offline=await run([uv!,"sync","--locked","--python",python!],project,fresh,{UV_OFFLINE:"1",UV_PROJECT_ENVIRONMENT:join(project,".venv-fresh")});
 expect(offline.code,offline.err).toBe(0);
 expect(existsSync(join(project,".venv-fresh"))).toBe(true);
 const imported=await run([join(project,".venv-fresh","bin","python"),"-c","import six,idna;print(six.__version__,idna.__version__)"],project,fresh);
 expect(imported.out.trim()).toBe("1.16.0 3.7");
 expect(existsSync(venv)).toBe(true);
 const lines:string[]=[],port=fleetUvPortV1(context,lock,line=>{lines.push(line);},{platform:"darwin",arch:"arm64"},{SEMIO_LANGUAGE:"en"});
 await port.before({UV_CACHE_DIR:directory("port")});await port.after({UV_CACHE_DIR:donor});
 const miss=fleetUvPortV1(context,"no seed for this lock",line=>{lines.push(line);},{platform:"darwin",arch:"arm64"},{SEMIO_LANGUAGE:"en"});await miss.before({UV_CACHE_DIR:directory("miss")});
 expect(lines).toEqual(["[setup] uv-cache: 2 wheels restored from the fleet, uv downloads only the rest","[setup] uv-cache: 2 wheels published to the fleet, class darwin-arm64","[setup] uv-cache: the fleet has no seed for this lock and class darwin-arm64, uv downloads the wheels"]);
},180000);

test("a seed of another lock, a tampered manifest and a machine without the store restore nothing",async()=>{
 const store=directory("store"),root=member(store);await syncFleetIdentityV1({root});
 const context=fleetFetchContextV1({},root,{storeRoot:store});
 expect(await fleetUvRestoreV1(context,{cacheDirectory:directory("c"),lock:"nothing published"})).toBe(0);
 expect(await fleetUvRestoreV1(fleetFetchContextV1({SEMIO_FLEET_FETCH:"off"},root),{cacheDirectory:directory("c"),lock:"x"})).toBe(0);
 expect(await fleetUvPublishV1(context,{cacheDirectory:directory("empty"),lock:'url = "https://files.pythonhosted.org/packages/aa/bb/x/six-1.16.0-py2.py3-none-any.whl", hash = "sha256:'+"a".repeat(64)+'"'})).toBeUndefined();
});
