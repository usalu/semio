import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {createFleetRegistryV1,createFleetVerifierV1,ensureFleetIdentityV1,fleetContentHashV1,fleetIdentityDirectoryV1,fleetInterfaceAddressV1,fleetPeerAllowedV1,fleetRegistrationPathV1,fleetSignedFetchV1,fleetTargetV1,loadFleetSignerV1,requireFleetIdentityShapeV1,rotateFleetIdentityV1,syncFleetIdentityV1,FLEET_TIME_HEADER_V1,type FleetRegistrationV1,type FleetRoleV1} from "../🟦️.ts";

const scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`identity-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const posix=process.platform!=="win32";
const ajv=new Ajv({strict:true,allowUnionTypes:true}),validator=(definition:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`});

function machine(storeRoot:string,originId:string,group="g1"):string{
 const root=directory(originId);mkdirSync(join(root,".cache","f"),{recursive:true});
 writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId,groupId:group,storeRoot,key:"k"}));
 mkdirSync(join(storeRoot,"v1","origins",originId),{recursive:true});return root;
}
const registration=async(root:string,storeRoot:string)=>{const result=await syncFleetIdentityV1({root});expect(result.error).toBeUndefined();return result.registration!;};
const asRead=(registry:ReturnType<typeof createFleetRegistryV1>)=>registry;
const registry=(storeRoot:string,extra:{pinsPath?:string}={})=>createFleetRegistryV1({storeRoot,group:"g1",ttlMilliseconds:0,...extra});
const verifier=(storeRoot:string,audience:"cache"|"execution"="cache",extra:Partial<Parameters<typeof createFleetVerifierV1>[0]>={})=>createFleetVerifierV1({registry:registry(storeRoot),audience,adminOrigin:"g1-server",startedAtMs:0,...extra});
const request=(root:string,role:FleetRoleV1,input:{audience?:"cache"|"execution";method?:string;target?:string;content?:string;now?:number;nonce?:string}={})=>{
 const signer=loadFleetSignerV1(root,role)!,method=input.method??"GET",target=input.target??"/semio/ac/"+"0".repeat(64);
 const headers=signer.sign({audience:input.audience??"cache",method,target,content:input.content,...(input.now===undefined?{}:{now:input.now}),...(input.nonce===undefined?{}:{nonce:input.nonce})});
 return {method,target,authorization:headers.authorization,content:headers["x-semio-content-sha256"]};
};

test("the identity schema agrees with an independent JSON-schema implementation",async()=>{
 const store=directory("store"),root=machine(store,"g1-m01"),sample=await registration(root,store);
 expect(validator("Registration")(sample),JSON.stringify(validator("Registration").errors)).toBe(true);expect(()=>requireFleetIdentityShapeV1("Registration",sample,"x")).not.toThrow();
 const broken:unknown[]=[{...sample,extra:1},{...sample,sequence:0},{...sample,origin:"short"},{...sample,keys:{...sample.keys,extra:sample.origin}},{...sample,keys:{"cache-read":sample.origin}},{...sample,signature:"x"},{...sample,originId:"../x"}];
 for(const value of broken){expect(validator("Registration")(value),JSON.stringify(value).slice(0,80)).toBe(false);expect(()=>requireFleetIdentityShapeV1("Registration",value,"x")).toThrow();}
 const pins={version:1,pins:{"g1-m01":{publicKey:sample.origin,pinnedAtMs:1}}};
 expect(validator("Pins")(pins)).toBe(true);expect(()=>requireFleetIdentityShapeV1("Pins",pins,"x")).not.toThrow();expect(validator("Pins")({...pins,pins:{a:{publicKey:"x",pinnedAtMs:1}}})).toBe(false);
 const rows={version:1,since:1,total:1,classes:{unit:1},rows:[{namespace:"semio",kind:"ac",key:"a".repeat(64),keptOrigin:"g1-m01",keptSha256:"b".repeat(64),refusedOrigin:"g1-m02",refusedSha256:"c".repeat(64),sort:"unit",differing:["a.rlib"],atMs:5}]};
 expect(validator("Divergences")(rows)).toBe(true);expect(()=>requireFleetIdentityShapeV1("Divergences",rows,"x")).not.toThrow();expect(validator("Divergences")({...rows,rows:[{...rows.rows[0],kind:"x"}]})).toBe(false);
 const tally={version:1,since:1,total:3,pairs:{"g1-m01>g1-m02":{hits:3,units:2}}};
 expect(validator("Consumption")(tally)).toBe(true);expect(()=>requireFleetIdentityShapeV1("Consumption",tally,"x")).not.toThrow();expect(validator("Consumption")({...tally,pairs:{"bad":{hits:1,units:1}}})).toBe(false);expect(()=>requireFleetIdentityShapeV1("Consumption",{...tally,total:-1},"x")).toThrow();
});

test("the identity is created once, one private key per role in its own 0600 file, and only public keys reach the store",async()=>{
 const store=directory("store"),root=machine(store,"g1-m01"),first=await registration(root,store);
 const again=await ensureFleetIdentityV1({root,originId:"g1-m01",group:"g1"});expect(again.created).toBe(false);expect(again.registration).toEqual(first);
 expect(Object.keys(first.keys).sort()).toEqual(["cache-read","cache-write","submit","worker"]);
 const published=readFileSync(fleetRegistrationPathV1(store,"g1-m01"),"utf8");expect(JSON.parse(published)).toEqual(first);
 const directoryPath=fleetIdentityDirectoryV1(root);
 for(const name of["origin","cache-read","cache-write","submit","worker"]){
  const secret=readFileSync(join(directoryPath,`${name}.key`),"utf8").trim();
  expect(published.includes(secret),`${name} private key must not be published`).toBe(false);
  if(posix)expect(statSync(join(directoryPath,`${name}.key`)).mode&0o777).toBe(0o600);
 }
 if(posix)expect(statSync(directoryPath).mode&0o777).toBe(0o700);
 const server=machine(store,"g1-server"),admin=await registration(server,store);expect(Object.keys(admin.keys).sort()).toEqual(["admin","cache-read","cache-write","submit","worker"]);
 expect(loadFleetSignerV1(root,"admin")).toBeUndefined();
});

test("a signature made by this module verifies with OpenSSL, an independent implementation, and a wrong message does not",async()=>{
 const store=directory("store"),root=machine(store,"g1-m01"),sample=await registration(root,store),work=directory("openssl");
 const check=spawnSync("openssl",["version"]);if(check.status!==0){console.warn("openssl not available, cross-check skipped");return;}
 const signer=loadFleetSignerV1(root,"cache-write")!,input={audience:"cache" as const,method:"PUT",target:"/semio/ac/"+"1".repeat(64),content:fleetContentHashV1("body"),now:1700000000000,nonce:"A".repeat(22)};
 const signature=signer.sign(input).authorization.split(" s=")[1]!;
 writeFileSync(join(work,"key.der"),Buffer.from(sample.keys["cache-write"]!,"base64url"));writeFileSync(join(work,"sig.bin"),Buffer.from(signature,"base64url"));
 const message=(target:string)=>["semio-fleet-v1","cache","g1-m01","cache-write",String(input.now),input.nonce,"PUT",target,input.content].join("\n");
 const run=(target:string)=>{writeFileSync(join(work,"msg.txt"),message(target));return spawnSync("openssl",["pkeyutl","-verify","-pubin","-inkey",join(work,"key.der"),"-keyform","DER","-rawin","-in",join(work,"msg.txt"),"-sigfile",join(work,"sig.bin")]);};
 expect(run(input.target).status,String(run(input.target).stderr)).toBe(0);expect(run("/semio/ac/"+"2".repeat(64)).status).not.toBe(0);
});

test("attack: unsigned, wrong origin, forged, tampered, replayed, stale, wrong audience and wrong role are all refused; the honest request passes",async()=>{
 const store=directory("store"),a=machine(store,"g1-m01"),b=machine(store,"g1-m02");await registration(a,store);await registration(b,store);
 const check=verifier(store);
 const honest=request(a,"cache-write",{method:"PUT",target:"/semio/ac/"+"3".repeat(64),content:fleetContentHashV1("x")});
 const accepted=await check.verify(honest);expect(accepted).toMatchObject({ok:true,origin:"g1-m01",role:"cache-write"});
 expect(await check.verify(honest),"replay of the very same request").toMatchObject({ok:false,reason:"replay"});
 expect(await check.verify({...request(a,"cache-write"),authorization:undefined})).toMatchObject({ok:false,reason:"no-signature"});
 expect(await check.verify({...request(a,"cache-write"),authorization:"Bearer abcdefghijklmnopqrstuvwxyz"})).toMatchObject({ok:false,reason:"no-signature"});
 const other=request(a,"cache-write",{target:"/semio/ac/"+"4".repeat(64)});
 expect(await check.verify({...other,target:"/semio/ac/"+"5".repeat(64)}),"signed for another target").toMatchObject({ok:false,reason:"bad-signature"});
 expect(await check.verify({...request(a,"cache-write"),method:"PUT"}),"signed for GET, sent as PUT").toMatchObject({ok:false,reason:"bad-signature"});
 expect(await check.verify({...request(a,"cache-write",{method:"PUT",content:fleetContentHashV1("good")}),content:fleetContentHashV1("evil")}),"another body hash").toMatchObject({ok:false,reason:"bad-signature"});
 const stolen=request(a,"cache-write").authorization!.replace("o=g1-m01","o=g1-m02");
 expect(await check.verify({...request(a,"cache-write"),authorization:stolen}),"origin name swapped").toMatchObject({ok:false,reason:"bad-signature"});
 expect(await check.verify({...request(a,"cache-write"),authorization:request(a,"cache-write").authorization!.replace("r=cache-write","r=cache-read")}),"role swapped").toMatchObject({ok:false,reason:"bad-signature"});
 expect(await check.verify({...request(b,"cache-read"),authorization:request(b,"cache-read").authorization!.replace("r=cache-read","r=cache-write")}),"a read key presented as write key").toMatchObject({ok:false,reason:"bad-signature"});
 expect(await check.verify(request(a,"cache-read",{audience:"execution"})),"signature for the other service").toMatchObject({ok:false});
 expect(await check.verify(request(a,"submit",{audience:"execution"})),"an execution role at the cache").toMatchObject({ok:false,reason:"malformed"});
 expect(await check.verify(request(a,"cache-write",{now:Date.now()-120000})),"two minutes old").toMatchObject({ok:false,reason:"clock"});
 expect(await check.verify(request(a,"cache-write",{now:Date.now()+120000})),"two minutes ahead").toMatchObject({ok:false,reason:"clock"});
 expect(await check.verify(request(a,"cache-write",{nonce:"short"})),"short nonce").toMatchObject({ok:false,reason:"malformed"});
 const early=verifier(store,"cache",{startedAtMs:Date.now()+30000});
 expect(await early.verify(request(a,"cache-write")),"signed before the server started").toMatchObject({ok:false,reason:"before-start"});
 const sample=JSON.parse(readFileSync(fleetRegistrationPathV1(store,"g1-m01"),"utf8")) as FleetRegistrationV1;
 const forged=machine(store,"g1-m03");await registration(forged,store);
 writeFileSync(fleetRegistrationPathV1(store,"g1-m03"),JSON.stringify({...sample,originId:"g1-m03"}));
 expect(await verifier(store).verify(request(forged,"cache-write")),"a copied registration under another origin").toMatchObject({ok:false});
 const stranger=machine(directory("elsewhere"),"g1-m99");await registration(stranger,directory("elsewhere2"));
 expect(await verifier(store).verify(request(stranger,"cache-write")),"an origin the store does not know").toMatchObject({ok:false});
 expect(check.denied().replay).toBe(1);
});

test("an admin signature is only accepted from the admin origin, and a registration cannot add one to another origin",async()=>{
 const store=directory("store"),member=machine(store,"g1-m01"),server=machine(store,"g1-server");await registration(member,store);await registration(server,store);
 const check=verifier(store);
 expect(await check.verify(request(server,"admin",{audience:"cache"}))).toMatchObject({ok:true,origin:"g1-server",role:"admin"});
 expect(await verifier(store,"cache",{adminOrigin:"g1-other"}).verify(request(server,"admin"))).toMatchObject({ok:false,reason:"admin-origin"});
 const forgedAdmin=JSON.parse(readFileSync(fleetRegistrationPathV1(store,"g1-server"),"utf8")) as FleetRegistrationV1;
 const memberRegistration=JSON.parse(readFileSync(fleetRegistrationPathV1(store,"g1-m01"),"utf8")) as FleetRegistrationV1;
 writeFileSync(fleetRegistrationPathV1(store,"g1-m01"),JSON.stringify({...memberRegistration,keys:{...memberRegistration.keys,admin:forgedAdmin.keys.admin}}));
 expect(await verifier(store).verify(request(member,"cache-write")),"edited registration breaks its own signature").toMatchObject({ok:false});
});

test("retiring an origin revokes it at once, re-admitting restores it, and a role key rotation retires only that role",async()=>{
 const store=directory("store"),a=machine(store,"g1-m01"),b=machine(store,"g1-m02");await registration(a,store);await registration(b,store);
 const live=registry(store);
 const check=createFleetVerifierV1({registry:live,audience:"cache",startedAtMs:0});
 expect(await check.verify(request(a,"cache-read"))).toMatchObject({ok:true});
 writeFileSync(join(store,"v1","origins","g1-m01","retired.json"),JSON.stringify({version:1,originId:"g1-m01",by:"g1-m02",hlc:"1",wallMs:Date.now(),reason:"gone",retired:true}));
 await new Promise(done=>setTimeout(done,1100));
 expect(await check.verify(request(a,"cache-read")),"retired").toMatchObject({ok:false,reason:"retired"});
 expect(await check.verify(request(b,"cache-read")),"others unaffected").toMatchObject({ok:true});
 writeFileSync(join(store,"v1","origins","g1-m01","retired.json"),JSON.stringify({version:1,originId:"g1-m01",by:"g1-m02",hlc:"2",wallMs:Date.now(),reason:"back",retired:false}));
 await new Promise(done=>setTimeout(done,1100));
 expect(await check.verify(request(a,"cache-read")),"re-admitted").toMatchObject({ok:true});
 const oldWriter=loadFleetSignerV1(a,"cache-write")!;
 const before=oldWriter.sign({audience:"cache",method:"PUT",target:"/semio/ac/"+"6".repeat(64),content:"-"});
 const rotated=await rotateFleetIdentityV1({root:a,originId:"g1-m01",group:"g1",roles:["cache-write"]});
 expect(rotated.sequence).toBe(2);await syncFleetIdentityV1({root:a});
 await new Promise(done=>setTimeout(done,1100));
 expect(await check.verify({method:"PUT",target:"/semio/ac/"+"6".repeat(64),authorization:before.authorization,content:"-"}),"the leaked old write key").toMatchObject({ok:false,reason:"bad-signature"});
 expect(await check.verify(request(a,"cache-write",{method:"PUT"})),"the new write key").toMatchObject({ok:true});
 expect(await check.verify(request(a,"cache-read")),"the untouched read key").toMatchObject({ok:true});
});

test("pinning: another origin key is refused without proof, accepted with the proof of the pinned key, and a machine that lost its keys needs a retire and re-admit",async()=>{
 const store=directory("store"),a=machine(store,"g1-m01"),pins=join(directory("pins"),"pins.json");await registration(a,store);
 const first=createFleetVerifierV1({registry:registry(store,{pinsPath:pins}),audience:"cache",startedAtMs:0});
 expect(await first.verify(request(a,"cache-read"))).toMatchObject({ok:true});
 const legit=JSON.parse(readFileSync(fleetRegistrationPathV1(store,"g1-m01"),"utf8")) as FleetRegistrationV1;
 const impostor=machine(directory("impostor-store"),"g1-m01");await ensureFleetIdentityV1({root:impostor,originId:"g1-m01",group:"g1"});
 const swap=JSON.parse(readFileSync(join(fleetIdentityDirectoryV1(impostor),"registration.json"),"utf8")) as FleetRegistrationV1;
 writeFileSync(fleetRegistrationPathV1(store,"g1-m01"),JSON.stringify(swap));
 const after=createFleetVerifierV1({registry:registry(store,{pinsPath:pins}),audience:"cache",startedAtMs:0});
 expect(await after.verify(request(impostor,"cache-read")),"a store writer replaces a live machine's registration").toMatchObject({ok:false,reason:"conflict"});
 writeFileSync(fleetRegistrationPathV1(store,"g1-m01"),JSON.stringify(legit));
 expect(await createFleetVerifierV1({registry:registry(store,{pinsPath:pins}),audience:"cache",startedAtMs:0}).verify(request(a,"cache-read")),"the real machine is unaffected").toMatchObject({ok:true});
 const proven=await rotateFleetIdentityV1({root:a,originId:"g1-m01",group:"g1",roles:[],origin:true});
 expect(proven.rotatedFrom?.publicKey).toBe(legit.origin);await syncFleetIdentityV1({root:a});
 expect(await createFleetVerifierV1({registry:registry(store,{pinsPath:pins}),audience:"cache",startedAtMs:0}).verify(request(a,"cache-read")),"origin key rotation signed by the pinned key").toMatchObject({ok:true});
 expect(JSON.parse(readFileSync(pins,"utf8")).pins["g1-m01"].publicKey).toBe(proven.origin);
 writeFileSync(fleetRegistrationPathV1(store,"g1-m01"),JSON.stringify(swap));
 expect(await createFleetVerifierV1({registry:registry(store,{pinsPath:pins}),audience:"cache",startedAtMs:0}).verify(request(impostor,"cache-read"))).toMatchObject({ok:false,reason:"conflict"});
 writeFileSync(join(store,"v1","origins","g1-m01","retired.json"),JSON.stringify({version:1,originId:"g1-m01",by:"g1-m02",hlc:"1",wallMs:Date.now()+10,reason:"reinstalled",retired:false}));
 expect(await createFleetVerifierV1({registry:registry(store,{pinsPath:pins}),audience:"cache",startedAtMs:0}).verify(request(impostor,"cache-read")),"after the owner re-admitted the origin").toMatchObject({ok:true});
});

test("a signing fetch signs method, target and body, sends no secret, and corrects its clock once when the server says so",async()=>{
 const store=directory("store"),root=machine(store,"g1-m01");await registration(root,store);
 const check=verifier(store),seen:string[]=[];let skewed=true;
 const server=async(input:RequestInfo|URL,init?:RequestInit):Promise<Response>=>{
  const url=new URL(String(input)),headers=new Headers(init?.headers),verdict=await check.verify({method:init?.method??"GET",target:fleetTargetV1(url),authorization:headers.get("authorization"),content:headers.get("x-semio-content-sha256")});
  seen.push(headers.get("authorization")??"");
  if(!verdict.ok)return new Response("no",{status:401,headers:{[FLEET_TIME_HEADER_V1]:String(verdict.serverTime)}});
  if(init?.body!==undefined&&headers.get("x-semio-content-sha256")!==fleetContentHashV1(init.body as string))return new Response("body",{status:400});
  return new Response(verdict.role);
 };
 const signed=fleetSignedFetchV1({root,audience:"cache",fetch:server as typeof fetch});
 expect(await (await signed("http://x/semio/ac/"+"7".repeat(64))).text()).toBe("cache-read");
 expect(await (await signed("http://x/semio/ac/"+"7".repeat(64),{method:"PUT",body:"payload"})).text()).toBe("cache-write");
 const secret=readFileSync(join(fleetIdentityDirectoryV1(root),"cache-write.key"),"utf8").trim();for(const header of seen)expect(header.includes(secret)).toBe(false);
 const behind=loadFleetSignerV1(root,"cache-read")!;behind.offset=-3600000;seen.length=0;
 expect(await (await signed("http://x/semio/ac/"+"8".repeat(64))).text(),"one retry with the corrected clock").toBe("cache-read");expect(seen.length).toBe(2);expect(Math.abs(behind.offset)).toBeLessThan(5000);skewed=false;
 const nobody=fleetSignedFetchV1({root:directory("empty"),audience:"cache",fetch:server as typeof fetch});
 expect((await nobody("http://x/semio/ac/"+"9".repeat(64))).status,"no identity: sent unsigned and refused").toBe(401);expect(skewed).toBe(false);
});

test("the fleet network check admits loopback and the fleet CIDR only, and finds the fleet interface",()=>{
 const net=["192.168.178.0/24"];
 for(const peer of["192.168.178.5","::ffff:192.168.178.200","127.0.0.1","::1"])expect(fleetPeerAllowedV1(peer,net),peer).toBe(true);
 for(const peer of["192.168.179.5","10.0.0.1","8.8.8.8","::ffff:10.1.1.1","fe80::1","",undefined,"192.168.178.256"])expect(fleetPeerAllowedV1(peer,net),String(peer)).toBe(false);
 expect(fleetPeerAllowedV1("10.9.9.9",["0.0.0.0/0"])).toBe(true);expect(fleetPeerAllowedV1("10.9.9.9",["10.9.9.9/32","bad"])).toBe(true);
 const interfaces={en0:[{address:"192.168.178.71",family:"IPv4",internal:false,netmask:"255.255.255.0",mac:"",cidr:null}],lo0:[{address:"127.0.0.1",family:"IPv4",internal:true,netmask:"",mac:"",cidr:null}]} as never;
 expect(fleetInterfaceAddressV1("192.168.178.0/24",interfaces)).toBe("192.168.178.71");expect(fleetInterfaceAddressV1("10.0.0.0/8",interfaces)).toBeUndefined();
});
