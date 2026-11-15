import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash,createPrivateKey,createPublicKey,diffieHellman,generateKeyPairSync} from "node:crypto";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {SEALED_HEADER_V1,SEALED_REPLY_HEADER_V1,SEALED_ROUTE_V1,SealedErrorV1,createSealedFetchV1,createSealedHandlerV1,exchangePrivateKeyV1,exchangePublicKeyV1,openRequestV1,requireSealedShapeV1,sealRequestV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

const ajv=new Ajv({strict:true,allowUnionTypes:true}),python=testPythonV1(),oracle=join(import.meta.dirname,"🐍️oracle","🐍️.py"),fixturePath=join(import.meta.dirname,"🧫️fixtures","🔣️.json"),scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const ed25519=(seed:Buffer)=>Buffer.concat([Buffer.from("302e020100300506032b657004220420","hex"),seed]).toString("base64url");
const edPublic=(privateKey:string)=>createPublicKey(createPrivateKey({key:Buffer.from(privateKey,"base64url"),format:"der",type:"pkcs8"})).export({type:"spki",format:"der"}).toString("base64url");
const identity=()=>{const pair=generateKeyPairSync("ed25519"),privateKey=pair.privateKey.export({type:"pkcs8",format:"der"}).toString("base64url");return {privateKey,publicKey:edPublic(privateKey)};};
const aad=(label:string,group:string,extra:Buffer)=>Buffer.concat([Buffer.from(label),Buffer.from([0]),Buffer.from(group),Buffer.from([0]),extra]);
const importKey=(key:string)=>crypto.subtle.importKey("raw",Buffer.from(key,"hex"),"AES-GCM",false,["decrypt","encrypt"]);

test("the vectors are valid against the schema, the independent Python oracle (standard library only) derives the same server key, request key and IV, reply key and IV from the seeds, and the converted public key is the public key of the converted private key",()=>{
 const validate=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Fixture"});
 expect(validate(fixture),JSON.stringify(validate.errors?.[0])).toBe(true);
 if(python===undefined)return;
 const run=Bun.spawnSync([python,"-I",oracle,"check",fixturePath],{stdout:"pipe",stderr:"pipe"});
 expect(run.stdout.toString()+run.stderr.toString()).toBe("ok\n");
 for(const vector of fixture.vectors){
  const privateKey=ed25519(Buffer.from(vector.originSeed,"hex"));
  expect(exchangePublicKeyV1(edPublic(privateKey)).toString("hex"),vector.name).toBe(vector.serverX25519);
  expect(createPublicKey(exchangePrivateKeyV1(privateKey)).export({type:"spki",format:"der"}).subarray(12).toString("hex"),vector.name).toBe(vector.serverX25519);
 }
});

test("the ciphertext of every vector opens with the keys of the oracle in an independent AES-GCM implementation (WebCrypto), carries exactly the declared request and reply, and sealing again from the same seeds gives the same bytes",async()=>{
 for(const vector of fixture.vectors){
  const head=vector.request.head,plain=async(ciphertext:string,key:string,iv:string,additional:Buffer)=>Buffer.from(await crypto.subtle.decrypt({name:"AES-GCM",iv:Buffer.from(iv,"hex"),additionalData:additional,tagLength:128},await importKey(key),Buffer.from(ciphertext,"hex")));
  const stamp=Buffer.alloc(8);stamp.writeBigUInt64BE(BigInt(vector.timestamp));
  const opened=await plain(vector.requestCiphertext,vector.requestKey,vector.requestIv,aad("semio-fleet-sealed/1",vector.group,Buffer.concat([stamp,Buffer.from(vector.nonce,"base64url")])));
  const length=opened.readUInt32BE(0);
  expect(JSON.parse(opened.subarray(4,4+length).toString("utf8")),vector.name).toEqual(head);
  expect(opened.subarray(4+length).toString("hex")).toBe(vector.request.bodyHex);
  const requestHash=createHash("sha256").update(Buffer.from(vector.requestCiphertext,"hex")).digest(),answer=await plain(vector.replyCiphertext,vector.replyKey,vector.replyIv,aad("semio-fleet-sealed/1 reply",vector.group,requestHash));
  const replyLength=answer.readUInt32BE(0);
  expect(JSON.parse(answer.subarray(4,4+replyLength).toString("utf8"))).toEqual(vector.reply.head);
  expect(answer.subarray(4+replyLength).toString("hex")).toBe(vector.reply.bodyHex);
  const sealed=sealRequestV1({serverPublic:Buffer.from(vector.serverX25519,"hex"),group:vector.group,now:vector.timestamp,request:{method:head.method,target:head.target,headers:head.headers,body:Buffer.from(vector.request.bodyHex,"hex")},seeds:{ephemeral:Buffer.from(vector.askerSeed,"hex"),nonce:Buffer.from(vector.nonce,"base64url")}});
  expect(sealed.headers[SEALED_HEADER_V1]).toBe(vector.requestHeader);expect(Buffer.from(sealed.body).toString("hex")).toBe(vector.requestCiphertext);
 }
});

test("the shared secret of the one-time key and the converted identity key is the one that the OpenSSL command line derives",()=>{
 const openssl=Bun.which("openssl");
 if(openssl===undefined)return;
 const directory=mkdtempSync(join(tmpdir(),"sealed-openssl-"));scratch.push(directory);
 const server=identity(),asker=generateKeyPairSync("x25519"),serverPrivate=exchangePrivateKeyV1(server.privateKey),serverPublic=createPublicKey(serverPrivate);
 writeFileSync(join(directory,"asker.pem"),asker.privateKey.export({type:"pkcs8",format:"pem"}));writeFileSync(join(directory,"server.pem"),serverPublic.export({type:"spki",format:"pem"}));
 const derived=Bun.spawnSync([openssl,"pkeyutl","-derive","-inkey",join(directory,"asker.pem"),"-peerkey",join(directory,"server.pem"),"-hexdump"],{stdout:"pipe",stderr:"pipe"});
 expect(derived.exitCode,derived.stderr.toString()).toBe(0);
 const text=derived.stdout.toString().split("\n").map(line=>line.replace(/^[0-9a-f]{4} - /u,"").slice(0,47).replace(/[\s-]/gu,"")).join("");
 expect(text).toBe(Buffer.from(diffieHellman({privateKey:asker.privateKey,publicKey:serverPublic})).toString("hex"));
 expect(text).toBe(Buffer.from(diffieHellman({privateKey:serverPrivate,publicKey:createPublicKey(asker.privateKey)})).toString("hex"));
});

const requestOf=(target="/semio/cas/"+"a".repeat(64))=>({method:"GET",target,headers:{authorization:"Semio-Fleet x"}});

test("a sealed request opens only with the identity key of the server of that group, within the clock skew, once, and a changed byte anywhere is refused",()=>{
 const server=identity(),privateKey=exchangePrivateKeyV1(server.privateKey),publicKey=exchangePublicKeyV1(server.publicKey),now=1_800_000_000_000;
 const sealed=sealRequestV1({serverPublic:publicKey,group:"g1",now,request:{...requestOf(),body:new TextEncoder().encode("payload of project 🦀️secret")}});
 const header=sealed.headers[SEALED_HEADER_V1]!;
 const opened=openRequestV1({serverPrivate:privateKey,group:"g1",header,body:sealed.body,now});
 expect(opened.request).toMatchObject({method:"GET",target:requestOf().target});expect(Buffer.from(opened.request.body!).toString()).toBe("payload of project 🦀️secret");
 const code=(change:()=>unknown):string=>{try{change();return "accepted";}catch(error){return error instanceof SealedErrorV1?error.code:String(error);}};
 expect(code(()=>openRequestV1({serverPrivate:exchangePrivateKeyV1(identity().privateKey),group:"g1",header,body:sealed.body,now})),"another server").toBe("decrypt");
 expect(code(()=>openRequestV1({serverPrivate:privateKey,group:"g2",header,body:sealed.body,now})),"another group").toBe("decrypt");
 expect(code(()=>openRequestV1({serverPrivate:privateKey,group:"g1",header,body:sealed.body,now:now+61_000})),"late").toBe("clock");
 expect(code(()=>openRequestV1({serverPrivate:privateKey,group:"g1",header,body:sealed.body,now:now-61_000})),"early").toBe("clock");
 for(let index=0;index<sealed.body.length;index+=Math.max(1,Math.floor(sealed.body.length/37))){const copy=Buffer.from(sealed.body);copy[index]=copy[index]!^1;expect(code(()=>openRequestV1({serverPrivate:privateKey,group:"g1",header,body:copy,now})),`byte ${index}`).toBe("decrypt");}
 const parts=header.split(".");
 for(const changed of[`${parts[0]}.${parts[1]}.${Number(parts[2])+1}.${parts[3]}`,`${parts[0]}.${parts[1]}.${parts[2]}.${"A".repeat(22)}`,`${parts[0]}.${generateKeyPairSync("x25519").publicKey.export({type:"spki",format:"der"}).subarray(12).toString("base64url")}.${parts[2]}.${parts[3]}`])expect(code(()=>openRequestV1({serverPrivate:privateKey,group:"g1",header:changed,body:sealed.body,now}))).toBe("decrypt");
 for(const bad of["","1.x.y.z","2."+parts.slice(1).join("."),"nonsense",null])expect(code(()=>openRequestV1({serverPrivate:privateKey,group:"g1",header:bad,body:sealed.body,now}))).toBe("malformed");
 expect(code(()=>openRequestV1({serverPrivate:privateKey,group:"g1",header,body:new Uint8Array(3),now}))).toBe("malformed");
});

test("a reply opens only for the request it answers and only when the server that holds the key wrote it; a changed or foreign reply is refused",()=>{
 const server=identity(),privateKey=exchangePrivateKeyV1(server.privateKey),publicKey=exchangePublicKeyV1(server.publicKey),now=1_800_000_000_000;
 const one=sealRequestV1({serverPublic:publicKey,group:"g1",now,request:requestOf()}),two=sealRequestV1({serverPublic:publicKey,group:"g1",now,request:requestOf("/semio/cas/"+"b".repeat(64))});
 const answer=openRequestV1({serverPrivate:privateKey,group:"g1",header:one.headers[SEALED_HEADER_V1]!,body:one.body,now}).seal({status:200,headers:{"content-type":"application/octet-stream"},body:new TextEncoder().encode("the blob")});
 expect(Buffer.from(one.open(answer.header,answer.body).body).toString()).toBe("the blob");
 expect(()=>two.open(answer.header,answer.body),"the reply of another request").toThrow(SealedErrorV1);
 const copy=Buffer.from(answer.body);copy[5]=copy[5]!^1;expect(()=>one.open(answer.header,copy)).toThrow(SealedErrorV1);
 expect(()=>openRequestV1({serverPrivate:exchangePrivateKeyV1(identity().privateKey),group:"g1",header:one.headers[SEALED_HEADER_V1]!,body:one.body,now}),"an impostor cannot even open the request, so it cannot write a reply").toThrow(SealedErrorV1);
 expect(()=>one.open("1.garbage",answer.body)).toThrow(SealedErrorV1);expect(()=>one.open(null,answer.body)).toThrow(SealedErrorV1);
});

test("the handler answers the sealed route only, refuses a replayed message, a message from the wrong clock and one above the bound, and the fetch gives the same answer as a plain call",async()=>{
 const server=identity(),handled:{target:string;authorization:string|null}[]=[];
 const handler=createSealedHandlerV1({privateKey:exchangePrivateKeyV1(server.privateKey),group:"g1",maximumBytes:1024,handle:async request=>{handled.push({target:new URL(request.url).pathname,authorization:request.headers.get("authorization")});return new Response(`answer for ${new URL(request.url).pathname} ${await request.text()}`,{status:request.method==="POST"?201:200,headers:{"content-type":"text/plain","x-semio-time":"123","set-cookie":"leaks=1"}});}});
 const call=(input:RequestInfo|URL,init?:RequestInit)=>handler(new Request(String(input),init)).then(response=>response??new Response("passed on\n",{status:404}));
 expect((await handler(new Request("http://x/other")))).toBeUndefined();
 const fetchSealed=createSealedFetchV1({group:"g1",serverKey:()=>server.publicKey,fetch:call as typeof fetch});
 const get=await fetchSealed("http://x/semio/cas/"+"c".repeat(64),{headers:{authorization:"Semio-Fleet signed","x-secret-extra":"1"}});
 expect(get.status).toBe(200);expect(await get.text()).toBe("answer for /semio/cas/"+"c".repeat(64)+" ");expect(get.headers.get("content-type")).toBe("text/plain");expect(get.headers.get("set-cookie")).toBeNull();
 expect(handled[0]).toEqual({target:"/semio/cas/"+"c".repeat(64),authorization:"Semio-Fleet signed"});
 const post=await fetchSealed("http://x/semio/claim/ac/"+"d".repeat(64),{method:"POST",body:"hello"});
 expect(post.status).toBe(201);expect(await post.text()).toContain("hello");
 const head=await fetchSealed("http://x/semio/cas/"+"e".repeat(64),{method:"HEAD"});expect(head.status).toBe(200);expect(await head.text()).toBe("");
 const sealed=sealRequestV1({serverPublic:exchangePublicKeyV1(server.publicKey),group:"g1",now:Date.now(),request:requestOf()}),send=()=>handler(new Request("http://x"+SEALED_ROUTE_V1,{method:"POST",headers:sealed.headers as Record<string,string>,body:sealed.body as BodyInit}));
 const first=await send();expect(first!.status).toBe(200);expect(first!.headers.get(SEALED_REPLY_HEADER_V1)).toMatch(/^1\./u);
 expect((await send())!.status,"the same message again").toBe(401);
 const late=sealRequestV1({serverPublic:exchangePublicKeyV1(server.publicKey),group:"g1",now:Date.now()-120_000,request:requestOf()});
 const refused=await handler(new Request("http://x"+SEALED_ROUTE_V1,{method:"POST",headers:late.headers as Record<string,string>,body:late.body as BodyInit}));expect(refused!.status).toBe(401);expect(Number(refused!.headers.get("x-semio-time"))).toBeGreaterThan(0);
 const large=await fetchSealed("http://x/semio/claim/ac/"+"f".repeat(64),{method:"POST",body:new Uint8Array(4096)});expect(large.status).toBe(413);
 expect((await handler(new Request("http://x"+SEALED_ROUTE_V1,{method:"POST",headers:{[SEALED_HEADER_V1]:"nonsense"},body:"x"})))!.status).toBe(400);
 expect((await handler(new Request("http://x"+SEALED_ROUTE_V1)))!.status).toBe(405);
 const unknown=await createSealedFetchV1({group:"g1",serverKey:()=>undefined,fetch:(()=>{throw Error("must not send");}) as unknown as typeof fetch})("http://x/semio/cas/"+"a".repeat(64));
 expect(unknown.status).toBe(503);expect(unknown.headers.get("x-semio-sealed-error")).toBe("server-key-unknown");
});

test("the head grammar rejects what a sealed request must never carry",()=>{
 expect(()=>requireSealedShapeV1("RequestHead",{method:"TRACE",target:"/x",headers:{}},"head")).toThrow();
 expect(()=>requireSealedShapeV1("RequestHead",{method:"GET",target:"x",headers:{}},"head")).toThrow();
 expect(()=>requireSealedShapeV1("RequestHead",{method:"GET",target:"/x",headers:{"Bad Name":"1"}},"head")).toThrow();
 expect(()=>requireSealedShapeV1("ReplyHead",{status:99,headers:{}},"head")).toThrow();
 expect(()=>requireSealedShapeV1("TransportPolicy",{crossGroup:"plain",inGroup:"sealed"},"policy")).toThrow();
 expect(()=>requireSealedShapeV1("TransportPolicy",{crossGroup:"sealed",inGroup:"trusted-lan"},"policy")).not.toThrow();
});
