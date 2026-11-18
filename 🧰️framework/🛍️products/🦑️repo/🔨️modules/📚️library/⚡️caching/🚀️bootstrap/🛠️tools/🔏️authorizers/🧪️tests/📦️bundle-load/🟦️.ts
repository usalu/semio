/** 📦️ Contract: a module of the authorizer compiler closure never reads a data file by a path it takes from data at run time and resolves against its own location, because the single-file bundle moves every module to the bundle's directory (`.cache/b/t/m/nx/.stage-X/`) where the sibling no longer exists: the bundle loads from a directory far away from the source tree, and every remaining location-relative read is audited with its reason. https://bun.sh/docs/bundler */
import {expect,mock,test} from "bun:test";
import {mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import audited from "./🧫️audited.json";

const here=dirname(fileURLToPath(import.meta.url)),authorizers=resolve(here,"../.."),root=resolve(here,"../../../../../../../../../../.."),entry=relative(root,join(authorizers,"📜️script.ts")).replaceAll("\\","/");
const readers=/\b(readFileSync|readFile|openSync|createReadStream|existsSync|readdirSync|readdir|statSync|lstatSync|Bun\.file)\s*\(/gu,located=/import\.meta\.(url|dir|dirname)|__dirname/u,literalSibling=/^new URL\(\s*(["'])\.{1,2}\/[^"'\n]*\1\s*,\s*import\.meta\.url\s*\)/u;

function callsOf(code:string):{reader:string;args:string}[]{
 const rows:{reader:string;args:string}[]=[];
 for(const match of code.matchAll(readers)){let depth=1,at=match.index+match[0].length;const from=at;while(at<code.length&&depth>0&&at<from+700){depth+=Number(code[at]==="(")-Number(code[at]===")");at++;}rows.push({reader:match[1]!,args:code.slice(from,at-1).trim()});}
 return rows;
}
export function locationReadsV1(path:string,code:string):string[]{
 return callsOf(code).filter(row=>located.test(row.args)&&!row.args.includes("physicalSourceOriginV1")&&!literalSibling.test(row.args)).map(row=>path+" | "+row.reader);
}

test("a read that names its file through data and its own location is found",()=>{
 expect(locationReadsV1("a.mjs",'readFileSync(new URL(rules.policy,import.meta.url),"utf8")')).toEqual(["a.mjs | readFileSync"]);
 expect(locationReadsV1("a.mjs",'readFileSync(new URL("../x.json",import.meta.url),"utf8")')).toEqual([]);
 expect(locationReadsV1("a.mjs",'readFileSync(new URL(p,physicalSourceOriginV1(import.meta.url,m)))')).toEqual([]);
 expect(locationReadsV1("a.mjs",'readFileSync(join(import.meta.dirname,"../x.json"))')).toEqual(["a.mjs | readFileSync"]);
});

test("the closure of the authorizer compiler holds only audited location-relative reads",async()=>{
 const {censusNxAuthorizerSourcesV1}=await import("../../🟦️.ts"),census=await censusNxAuthorizerSourcesV1(root,entry,new AbortController().signal),found=new Set<string>();
 for(const source of census.sources){if(!/\.[cm]?[jt]sx?$/u.test(source.path))continue;for(const row of locationReadsV1(source.path,readFileSync(join(root,source.path),"utf8")))found.add(row);}
 const expected=audited.reads.map(row=>row.module+" | "+row.reader);
 expect([...found].sort(),"unaudited location-relative reads (move the data into an import with a JSON attribute or pass it in)").toEqual(expected.sort());
 for(const row of audited.reads)expect(row.reason.length,row.module).toBeGreaterThan(20);
},120000);

test("the compiler bundle loads from a directory far away from the source tree",async()=>{
 const routing=join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),leases=join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts"),real={routing:await import(routing),leases:await import(leases)};
 mock.module(routing,()=>({...real.routing,requireIssuedResourceControlV1:()=>{}}));mock.module(leases,()=>({...real.leases,checkResourceLeaseControl:()=>{}}));
 const {captureNxAuthorizerSourcesV1}=await import("../../🟦️.ts"),{snapshotNxAuthorizerSourcesV1,compileNxAuthorizerSnapshotV1}=await import("../../📦️snapshot/🟦️.ts");
 const control:any={signal:new AbortController().signal,publish:async()=>{},yieldContinuation:async()=>{}},sealed=await compileNxAuthorizerSnapshotV1(root,snapshotNxAuthorizerSourcesV1(await captureNxAuthorizerSourcesV1(root,entry,control.signal)),control,"runtime","program");
 const far=mkdtempSync(join(tmpdir(),"bundle-load-")),deep=join(far,"a","b","c","d");
 try{Bun.spawnSync(["mkdir","-p",deep]);const program=join(deep,".semio-authorizer-compiler.mjs");writeFileSync(program,sealed.bytes);
  const run=Bun.spawnSync([process.execPath,program],{cwd:deep,env:{PATH:process.env.PATH??""}}),text=run.stderr.toString()+run.stdout.toString();
  expect(text,"the bundle read a sibling of the source tree while loading").not.toContain("ENOENT");
  expect(text).toContain("Original parent Script process invocation required");
 }finally{rmSync(far,{recursive:true,force:true});}
},300000);
