import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fleetSignedFetchV1,syncFleetIdentityV1} from "../../../../../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {graphFleetClientV1,publishGraphEntryOutcomeV1} from "../../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import {graphFleetSettingsV1} from "../../../../../../🪪️graph-state/🧶️nx/🟦️.ts";
import {ownerPoolIdV1,ownerPoolStateV1,publishOwnerPoolV1,readOwnerPoolPointerV1,readOwnerPoolV1,type OwnerPoolManifestV1,type OwnerPoolUnitV1} from "../../🟦️.ts";

/** 🔁️ A wave id is the digest of its owners, so every machine that reads the same owners publishes the same manifest key with a head, a session and cost sheet of its own: the cache keeps the first writer and must never be asked to refuse the others. */
const scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`owner-pool-sw-${label}-`));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop();for(const path of scratch)rmSync(path,{recursive:true,force:true});});
function machine(store:string,origin:string):string{
 const root=directory(origin);mkdirSync(join(root,".cache","f"),{recursive:true});
 writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:"g1",storeRoot:store,key:"k"}));
 mkdirSync(join(store,"v1","origins",origin),{recursive:true});return root;
}
async function world(){
 const store=directory("group"),a=machine(store,"g1-m01"),b=machine(store,"g1-m02");
 for(const root of[a,b])expect((await syncFleetIdentityV1({root})).error).toBeUndefined();
 const server=await startRemoteCacheServerV1({directory:directory("cache"),host:"127.0.0.1",port:0,maximumBytes:2**27,maximumEntryBytes:2**25,maximumAgeDays:30,defaultInstance:"semio",tokens:{},validation:"structure",fleet:{storeRoot:store,group:"g1",networks:["10.77.0.0/16"]}});
 servers.push(server);
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"semio"};
 const client=(root:string)=>graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:fleetSignedFetchV1({root,audience:"cache"})});
 const divergences=async()=>((await (await fleetSignedFetchV1({root:a,audience:"cache"})(`http://127.0.0.1:${server.port}/status`)).json()) as {divergences:{count:number;byClass:Record<string,number>}}).divergences;
 return{a,b,server,client,divergences};
}
const unitsOf=(count:number,megabytes:number):OwnerPoolUnitV1[]=>Array.from({length:count},(_all,index)=>({name:`owner-${String(index).padStart(4,"0")}`,kind:"k",expectedMilliseconds:60000,megabytes:megabytes+index,json:{name:`owner-${index}`}}));
const manifestOf=(tag:string,megabytes:number,createdMs:number):OwnerPoolManifestV1=>{
 const units=unitsOf(12,megabytes),partial={version:1 as const,identity:"identity-shared",globals:{registryDigest:"r",commandRevision:"c",late:false},publisher:"p:"+tag};
 return{head:{...partial,id:ownerPoolIdV1(partial,units),createdMs},session:Buffer.from(JSON.stringify({tag,rss:megabytes})),units};
};

test("control: writing one wave id from two machines by the plain entry publisher is a divergence the cache refuses (head, session and cost sheet differ by machine)",async()=>{
 const {a,b,client,divergences}=await world(),first=manifestOf("a",100,Date.now()),second=manifestOf("b",900,Date.now()+5);
 expect(second.head.id).toBe(first.head.id);
 const put=(root:string,manifest:OwnerPoolManifestV1)=>publishGraphEntryOutcomeV1({client:client(root),state:ownerPoolStateV1("manifest",manifest.head.id),owner:"o",documents:[{name:"head.br",bytes:Buffer.from(JSON.stringify(manifest.head))},{name:"session.br",bytes:manifest.session},{name:"units.br",bytes:Buffer.from(JSON.stringify(manifest.units))}]});
 expect((await put(a,first)).stored).toBe(true);expect((await put(b,second)).stored).toBe(false);
 expect((await divergences()).count).toBe(1);
});

test("one writer per wave: a second machine that publishes the same owners writes nothing, finds the first manifest valid and adds the wave to the pointer once; the cache counts no divergence",async()=>{
 const {a,b,client,divergences}=await world(),now=Date.now(),first=manifestOf("a",100,now),second=manifestOf("b",900,now+5);
 expect(await publishOwnerPoolV1({client:client(a),owner:"g1-m01:1",manifest:first,now:()=>now})).toBe(true);
 expect(await publishOwnerPoolV1({client:client(b),owner:"g1-m02:1",manifest:second,now:()=>now})).toBe(true);
 expect(await divergences()).toMatchObject({count:0});
 const read=await readOwnerPoolV1(client(b),first.head.id);
 expect(read?.head.publisher).toBe("p:a");expect(read?.units.length).toBe(12);
 const pointer=await readOwnerPoolPointerV1(client(b),()=>now);
 expect(pointer?.waves.map(wave=>wave.id)).toEqual([first.head.id]);
});

test("two machines that publish the same owners at the same moment end with one manifest, both report success and the cache counts no divergence",async()=>{
 const {a,b,client,divergences}=await world(),now=Date.now(),first=manifestOf("a",100,now),second=manifestOf("b",900,now+5);
 const both=await Promise.all([publishOwnerPoolV1({client:client(a),owner:"g1-m01:1",manifest:first,now:()=>now}),publishOwnerPoolV1({client:client(b),owner:"g1-m02:1",manifest:second,now:()=>now})]);
 expect(both).toEqual([true,true]);
 expect(await divergences()).toMatchObject({count:0});
 expect((await readOwnerPoolPointerV1(client(a),()=>now))?.waves.length).toBe(1);
});

test("a machine that renews its own wave writes it again (the manifest keeps its age) and a wave whose claim cannot be taken is published as before",async()=>{
 const {a,client,divergences}=await world(),now=Date.now(),first=manifestOf("a",100,now);
 expect(await publishOwnerPoolV1({client:client(a),owner:"g1-m01:1",manifest:first,now:()=>now})).toBe(true);
 const renewed={...first,head:{...first.head,createdMs:now+1000}};
 expect(await publishOwnerPoolV1({client:client(a),owner:"g1-m01:1",manifest:renewed,now:()=>now+1000})).toBe(true);
 expect((await readOwnerPoolV1(client(a),first.head.id))?.head.createdMs).toBe(now+1000);
 expect((await divergences()).count).toBe(0);
});
