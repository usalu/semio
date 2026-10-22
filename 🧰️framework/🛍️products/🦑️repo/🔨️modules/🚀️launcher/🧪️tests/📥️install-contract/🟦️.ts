import{expect,test}from"bun:test";
import{chmodSync,cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{createHash}from"node:crypto";
import{fileURLToPath}from"node:url";

const installerSource=fileURLToPath(new URL("../../",import.meta.url)),repositoryRoot=fileURLToPath(new URL("../../../../../../../",import.meta.url));
const pinsRelative="🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json",launcherRelative="🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher";
const platform=process.platform==="darwin"?"darwin":"linux",arch=process.arch==="arm64"?"arm64":"x64",goArch=arch==="arm64"?"arm64":"amd64",version="1.26.9",archive=`go${version}.${platform}-${goArch}.tar.gz`;
const supported=process.platform==="darwin"||process.platform==="linux";
const fakeGo=`#!/bin/sh
case "$1" in
version) echo "go version go${version} ${platform}/${goArch}" ;;
build) while [ "$#" -gt 0 ]; do [ "$1" = "-o" ] && out="$2"; shift; done; printf '#!/bin/sh\\necho stand-in launcher\\n' >"$out"; chmod 755 "$out" ;;
*) exit 2 ;;
esac
`;

function synthetic(root:string):{bytes:Uint8Array;sha256:string}{
  const stage=join(root,"stage");mkdirSync(join(stage,"go/bin"),{recursive:true});writeFileSync(join(stage,"go/bin/go"),fakeGo);chmodSync(join(stage,"go/bin/go"),0o755);
  const made=spawnSync("tar",["-czf",join(root,archive),"-C",stage,"go"]);expect(made.status).toBe(0);
  const bytes=new Uint8Array(readFileSync(join(root,archive)));return{bytes,sha256:createHash("sha256").update(bytes).digest("hex")};
}

function scaffold(root:string,sha256:string):string{
  const checkout=join(root,"checkout"),pins=join(checkout,pinsRelative);mkdirSync(join(pins,".."),{recursive:true});
  writeFileSync(pins,`{"definitions":{"Pins":{"const":{"assets":[\n{"tool":"go","version":"${version}","platform":"${platform}","arch":"${arch}","url":"https://dl.google.com/go/${archive}","sha256":"${sha256}"}\n]}}}}\n`);
  cpSync(join(installerSource),join(checkout,launcherRelative),{recursive:true,filter:path=>!path.includes("/🧪️tests")});
  return join(checkout,launcherRelative,"📥️install/🐚️.sh");
}

async function install(script:string,root:string,mirror:string,extra:Record<string,string>={}){
  const child=Bun.spawn(["bash",script,join(root,"bin")],{env:{PATH:"/usr/bin:/bin",HOME:join(root,"home"),SEMIO_GO_ROOT:join(root,"go"),SEMIO_GO_MIRROR:mirror,...extra},stderr:"pipe",stdout:"ignore"});
  const stderr=await new Response(child.stderr).text();
  return{status:await child.exited,stderr};
}

const leftovers=(root:string)=>existsSync(join(root,"go"))?readdirSync(join(root,"go")).filter(name=>name.startsWith(".partial")||name.endsWith(".lock")):[];

test.skipIf(!supported)("the installer fetches the pinned Go once, checks it before unpacking and reuses it, in both languages",async()=>{
  const root=mkdtempSync(join(tmpdir(),"semio-install-"));
  try{
    const{bytes,sha256}=synthetic(root),script=scaffold(root,sha256);let served=0;
    const server=Bun.serve({port:0,hostname:"127.0.0.1",fetch:request=>{if(new URL(request.url).pathname!==`/${archive}`)return new Response("missing",{status:404});served++;return new Response(bytes);}});
    try{
      const mirror=`http://127.0.0.1:${server.port}`;
      const first=await install(script,root,mirror);
      expect(first.status).toBe(0);
      expect(first.stderr).toContain(`Downloading Go ${version}`);expect(first.stderr).toContain(`Lade Go ${version} herunter`);
      expect(first.stderr).toContain("Installed the repo launcher");expect(first.stderr).toContain("Repo-Launcher installiert");
      expect(existsSync(join(root,"go",`v${version}`,"bin/go"))).toBe(true);expect(existsSync(join(root,"bin/repo"))).toBe(true);expect(leftovers(root)).toEqual([]);
      const independent=spawnSync("shasum",["-a","256",join(root,archive)],{encoding:"utf8"});if(independent.status===0)expect(independent.stdout.split(" ")[0]).toBe(sha256);
      expect(served).toBe(1);
      const second=await install(script,root,mirror);
      expect(second.status).toBe(0);expect(second.stderr).not.toContain("Downloading");expect(served).toBe(1);
    }finally{server.stop(true);}
  }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test.skipIf(!supported)("a wrong archive and a foreign mirror are refused before anything is unpacked or installed",async()=>{
  const root=mkdtempSync(join(tmpdir(),"semio-install-"));
  try{
    const{sha256}=synthetic(root),script=scaffold(root,sha256);
    const server=Bun.serve({port:0,hostname:"127.0.0.1",fetch:()=>new Response("not the pinned archive")});
    try{
      const wrong=await install(script,root,`http://127.0.0.1:${server.port}`);
      expect(wrong.status).not.toBe(0);expect(wrong.stderr).toContain("Checksum mismatch");expect(wrong.stderr).toContain("Prüfsumme");
      expect(existsSync(join(root,"go",`v${version}`))).toBe(false);expect(existsSync(join(root,"bin/repo"))).toBe(false);expect(leftovers(root)).toEqual([]);
    }finally{server.stop(true);}
    const foreign=await install(script,root,"http://example.com/go");
    expect(foreign.status).not.toBe(0);expect(foreign.stderr).toContain("SEMIO_GO_MIRROR must be");expect(foreign.stderr).toContain("SEMIO_GO_MIRROR muss");expect(leftovers(root)).toEqual([]);
  }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test.skipIf(!supported)("a cancelled download exits with 130 and leaves no partial directory, lock or launcher",async()=>{
  const root=mkdtempSync(join(tmpdir(),"semio-install-"));
  try{
    const{sha256}=synthetic(root),script=scaffold(root,sha256);
    let release:()=>void=()=>{};const stalled=new Promise<void>(resolve=>{release=resolve;});let started:()=>void=()=>{};const began=new Promise<void>(resolve=>{started=resolve;});
    const server=Bun.serve({port:0,hostname:"127.0.0.1",idleTimeout:0,fetch:()=>new Response(new ReadableStream({async start(controller){controller.enqueue(new Uint8Array(1024));started();await stalled;controller.close();}}))});
    try{
      const child=Bun.spawn(["bash",script,join(root,"bin")],{env:{PATH:"/usr/bin:/bin",HOME:join(root,"home"),SEMIO_GO_ROOT:join(root,"go"),SEMIO_GO_MIRROR:`http://127.0.0.1:${server.port}`},stderr:"pipe",stdout:"ignore"});
      await began;
      child.kill("SIGINT");
      await Bun.sleep(300);
      server.stop(true);release();
      const code=await child.exited,text=await new Response(child.stderr).text();
      expect(code).toBe(130);expect(text).toContain("Cancelled");expect(text).toContain("Abgebrochen");
      expect(leftovers(root)).toEqual([]);expect(existsSync(join(root,"bin/repo"))).toBe(false);
    }finally{release();server.stop(true);}
  }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test("both installers read the Go pin from the shared table, hold no version literal and say every message in English and German",()=>{
  const shell=readFileSync(join(installerSource,"📥️install/🐚️.sh"),"utf8"),windows=readFileSync(join(installerSource,"📥️install/🔵️.ps1"),"utf8");
  for(const text of[shell,windows]){expect(text).not.toContain(version);expect(text).toContain("SEMIO_GO_MIRROR");expect(text).toContain("SEMIO_GO_ROOT");expect(text).toContain("GOTOOLCHAIN");}
  expect(shell).toContain("sha256_of");expect(windows).toContain("Get-FileHash");
  const shellSays=[...shell.matchAll(/(?:say|fail) "((?:[^"\\]|\\.)*)" "((?:[^"\\]|\\.)*)"/gu)],windowsSays=[...windows.matchAll(/(?:Write-Say|Stop-Say) "((?:[^"\\]|\\.)*)" "((?:[^"\\]|\\.)*)"/gu)];
  expect(shellSays.length).toBeGreaterThan(8);expect(windowsSays.length).toBeGreaterThan(8);
  for(const[,english,german]of[...shellSays,...windowsSays])expect(english).not.toBe(german);
  const pins=JSON.parse(readFileSync(join(repositoryRoot,pinsRelative),"utf8")).definitions.Pins.const.assets.filter((row:{tool:string})=>row.tool==="go");
  expect(pins.map((row:{platform:string;arch:string})=>`${row.platform}-${row.arch}`).sort()).toEqual(["darwin-arm64","darwin-x64","linux-arm64","linux-x64","win32-arm64","win32-x64"]);
  for(const row of pins)expect(row.url).toStartWith("https://dl.google.com/go/");
});
