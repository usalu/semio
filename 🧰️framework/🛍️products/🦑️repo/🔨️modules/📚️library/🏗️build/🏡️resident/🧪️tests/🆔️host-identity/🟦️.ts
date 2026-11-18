/** 🆔️ Contract: one graph service per (implementation digest, checkout root, machine state root); the socket path is derived below the per-user runtime base, never moved to a shared folder, and a base that cannot hold it fails with a plain text in both languages.
 * @see ../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
const shortBase=process.platform==="win32"?tmpdir():"/tmp";
import {join} from "node:path";
import {HOST_SOCKET_BYTES_V1,HostRuntimePathErrorV1,checkoutRootOfV1,hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";

const digest="a".repeat(64),base=mkdtempSync(join(shortBase,"hid-")),own=(extra:Record<string,string>={})=>({XDG_RUNTIME_DIR:base,...extra});

test("two machine state roots on one checkout get two services, and the same identity always gets the same one",()=>{
 const first=hostEndpointsV1(digest,own({SEMIO_MACHINE_STATE_DIRECTORY:"/state/one"}),"linux","501",base),second=hostEndpointsV1(digest,own({SEMIO_MACHINE_STATE_DIRECTORY:"/state/two"}),"linux","501",base),none=hostEndpointsV1(digest,own(),"linux","501",base);
 expect(new Set([first.tag,second.tag,none.tag]).size).toBe(3);
 expect(new Set([first.endpoint,second.endpoint,none.endpoint]).size).toBe(3);
 expect(new Set([first.log,second.log,none.log]).size).toBe(3);
 expect(hostEndpointsV1(digest,own({SEMIO_MACHINE_STATE_DIRECTORY:"/state/one"}),"linux","501",base)).toEqual(first);
 expect(first.identity).toEqual({implementation:digest,root:realpathSync(base),state:"/state/one"});
});

test("two checkouts and two code digests get two services",()=>{
 const one=mkdtempSync(join(shortBase,"hid-one-")),two=mkdtempSync(join(shortBase,"hid-two-"));
 try{
  const a=hostEndpointsV1(digest,own(),"linux","501",one),b=hostEndpointsV1(digest,own(),"linux","501",two),c=hostEndpointsV1("b".repeat(64),own(),"linux","501",one);
  expect(new Set([a.tag,b.tag,c.tag]).size).toBe(3);
 }finally{rmSync(one,{recursive:true,force:true});rmSync(two,{recursive:true,force:true});}
});

test("the tag is the sha256 an independent implementation computes",()=>{
 const found=hostEndpointsV1(digest,own({SEMIO_MACHINE_STATE_DIRECTORY:"/state/one"}),"linux","501",base);
 const python=spawnSync("python3",["-I","-c","import hashlib,sys;print(hashlib.sha256(('semio-repo-host-v2\\0'+sys.argv[1]+'\\0'+sys.argv[2]+'\\0'+sys.argv[3]).encode()).hexdigest()[:12])",digest,found.identity.root,"/state/one"],{encoding:"utf8"});
 expect(python.stdout.trim()).toBe(found.tag);
 expect(createHash("sha256").update("semio-repo-host-v2\0"+digest+"\0"+found.identity.root+"\0/state/one").digest("hex").slice(0,12)).toBe(found.tag);
});

test("an overlong base never falls back to a shared folder: it uses the compact spelling below the same base, or fails in both languages",()=>{
 const long="/t/"+"x".repeat(63);
 const compact=hostEndpointsV1(digest,{XDG_RUNTIME_DIR:long},"linux","501",base);
 expect(compact.directory.startsWith(long)).toBe(true);expect(compact.directory).not.toContain("/tmp/semio-repo");
 expect(Buffer.byteLength(compact.endpoint)).toBeLessThanOrEqual(HOST_SOCKET_BYTES_V1);
 const tooLong="/t/"+"y".repeat(120),failure:any=(()=>{try{hostEndpointsV1(digest,{XDG_RUNTIME_DIR:tooLong},"linux","501",base);}catch(error){return error;}})();
 expect(failure).toBeInstanceOf(HostRuntimePathErrorV1);expect(failure.code).toBe("runtime-path");
 expect(failure.message).toContain("no usable socket path");expect(failure.message).toContain("keinen nutzbaren Socket-Pfad");
 expect(failure.message).toContain(tooLong);
});

test("every endpoint of a usable base fits the socket limit and names its tag",()=>{
 for(const state of["","/a","/very/long/state/root/"+"z".repeat(60)]){
  const found=hostEndpointsV1(digest,own(state===""?{}:{SEMIO_MACHINE_STATE_DIRECTORY:state}),"linux","501",base);
  expect(Buffer.byteLength(found.endpoint)).toBeLessThanOrEqual(HOST_SOCKET_BYTES_V1);
  for(const path of[found.endpoint,found.descriptor,found.lock,found.log])expect(path).toContain(found.tag);
 }
});

test("on Windows the pipe and files carry the same identity tag",()=>{
 const one=hostEndpointsV1(digest,{LOCALAPPDATA:"C:\\Users\\me\\AppData\\Local",SEMIO_MACHINE_STATE_DIRECTORY:"/state/one"},"win32","501",base),two=hostEndpointsV1(digest,{LOCALAPPDATA:"C:\\Users\\me\\AppData\\Local"},"win32","501",base);
 expect(one.endpoint).not.toBe(two.endpoint);expect(one.endpoint).toContain(one.tag);
});

test("a directory inside a checkout belongs to the checkout, so every subdirectory reaches the same service",()=>{
 const top=mkdtempSync(join(shortBase,"hid-top-")),inner=join(top,"a","b");
 try{
  mkdirSync(join(top,".git"));mkdirSync(inner,{recursive:true});
  expect(checkoutRootOfV1(inner)).toBe(realpathSync(top));
  expect(hostEndpointsV1(digest,own(),"linux","501",inner)).toEqual(hostEndpointsV1(digest,own(),"linux","501",top));
 }finally{rmSync(top,{recursive:true,force:true});}
});
