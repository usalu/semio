/** 🧰️ Tests for storage programs. */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{readdirSync,readFileSync}from"node:fs";
import{join,sep}from"node:path";
import{validateJsonSchemaSubset}from"../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
import{nativeStorageWindowsProgramPathV1,pinNativeStorageProgramV1}from"../🟦️.ts";
const storage=join(import.meta.dir,"../..");
const withLanguage=<T>(language:string,run:()=>T):T=>{const before=process.env.SEMIO_LANGUAGE;process.env.SEMIO_LANGUAGE=language;try{return run();}finally{if(before===undefined)delete process.env.SEMIO_LANGUAGE;else process.env.SEMIO_LANGUAGE=before;}};
test("the corpus is schema-valid and the Windows path derives from SystemRoot, whatever PATH says",()=>{
 expect(validateJsonSchemaSubset(schema.definitions.Corpus,fixture,schema)).toEqual([]);
 for(const row of fixture.resolutions)expect(nativeStorageWindowsProgramPathV1(row.program as "powershell",row.environment),row.id).toBe(row.expected);
});
test("a missing program, a relative name and an invalid SystemRoot are refused in each language",()=>{
 const host={executable:()=>false};
 expect(()=>withLanguage("en",()=>pinNativeStorageProgramV1("/usr/bin/hdiutil",host))).toThrow("is required for native storage but is missing");
 expect(()=>withLanguage("de",()=>pinNativeStorageProgramV1("/usr/bin/hdiutil",host))).toThrow("Systemprogramm");
 for(const name of["hdiutil","./hdiutil","bin/codesign","powershell.exe"])expect(()=>withLanguage("en",()=>pinNativeStorageProgramV1(name,{executable:()=>true})),name).toThrow("absolute path");
 expect(pinNativeStorageProgramV1("/usr/bin/hdiutil",{executable:path=>path==="/usr/bin/hdiutil"})).toBe("/usr/bin/hdiutil");
 for(const root of[undefined,"","relative","C:","\\\\server\\share","C:\\bad|name","C:\\Windows\\"])expect(()=>nativeStorageWindowsProgramPathV1("powershell",root===undefined?{}:{SystemRoot:root}),String(root)).toThrow("SystemRoot");
});
test("no storage source starts a program by a bare name and no schema names one",()=>{
 const files=readdirSync(storage,{recursive:true,encoding:"utf8"}).filter(path=>/\.(ts|json)$/u.test(path)&&!path.split(sep).some(part=>part.includes("🧪️tests")||part==="node_modules"));
 expect(files.length).toBeGreaterThan(100);
 const call=/\b(?:captureOwnedCommandV1|spawnSync|spawn|execFileSync|execFile|runOwnedCommand)\(\s*["'`]([^"'`]+)["'`]/gu,key=/"program"\s*:\s*"([^"]+)"/gu,offenders:string[]=[];
 for(const file of files){const text=readFileSync(join(storage,file),"utf8");for(const pattern of[call,key])for(const match of text.matchAll(pattern))if(!match[1]!.startsWith("/"))offenders.push(file+": "+match[1]);}
 expect(offenders).toEqual([]);
});
test.skipIf(process.platform!=="darwin")("every macOS system program native storage runs exists and an independent python agrees that it is executable",()=>{
 const paths=["/usr/bin/hdiutil","/usr/bin/codesign","/usr/sbin/sysctl","/sbin/mount","/usr/sbin/lsof"].map(path=>pinNativeStorageProgramV1(path));
 const run=spawnSync("/usr/bin/python3",["-I",join(import.meta.dir,"🐍️.py"),...paths],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 expect(run.stdout.trim().split("\n")).toEqual(paths.map(path=>path+" ok"));
});
