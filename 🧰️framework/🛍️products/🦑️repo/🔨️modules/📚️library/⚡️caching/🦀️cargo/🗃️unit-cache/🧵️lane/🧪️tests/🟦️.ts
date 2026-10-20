import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import {linuxAncestryV1,parseProcStatV1,psAncestryV1,resolveLaneIdV1,systemAncestryV1,windowsAncestryV1,WINDOWS_PROCESS_LISTING_V1} from "../🟦️.ts";

const python=(code:string,...args:string[])=>Bun.spawnSync([testPythonV1()??"python3","-c",code,...args],{stdout:"pipe",stderr:"pipe"});
const hasPython=testPythonV1()!==null;

test("the stat line is read after the last parenthesis, so a command name with spaces or parentheses cannot shift the fields, as an independent python parse agrees",()=>{
 const check=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["name","text","parent","started"],properties:{name:{type:"string"},text:{type:"string"},parent:{type:["integer","null"]},started:{type:["string","null"]}}});
 for(const row of fixture.stat){
  expect(check(row),row.name+JSON.stringify(check.errors)).toBe(true);
  const parsed=parseProcStatV1(row.text);expect(parsed?.parent??null,row.name).toBe(row.parent);expect(parsed?.started??null,row.name).toBe(row.started);
  if(hasPython&&row.parent!==null){const answer=python("import sys\nt=sys.argv[1]\nf=t[t.rindex(')')+2:].split()\nprint(f[1],f[19])",row.text).stdout.toString().trim();expect(answer).toBe(row.parent+" "+row.started!.slice(5));}
 }
});

test("the Linux chain is walked through /proc alone, ends at an unreadable process, and names the agent client so its sessions get their own lane",()=>{
 const files=fixture.linuxChain.files as Record<string,string>,rows=linuxAncestryV1(path=>files[path]??null,fixture.linuxChain.first);
 expect(rows).toEqual(fixture.linuxChain.rows);
 const lane=resolveLaneIdV1({environment:{},ancestry:()=>rows});expect(lane).toMatch(/^a[0-9a-z]{1,10}$/u);
 expect(resolveLaneIdV1({environment:{},ancestry:()=>linuxAncestryV1(path=>files[path]??null,fixture.linuxChain.first)})).toBe(lane);
 const other:Record<string,string>={...files,"/proc/200/stat":files["/proc/200/stat"]!.replace(" 2000 "," 2001 ")};
 expect(resolveLaneIdV1({environment:{},ancestry:()=>linuxAncestryV1(path=>other[path]??null,fixture.linuxChain.first)}),"a restarted client is a new lane").not.toBe(lane);
 expect(linuxAncestryV1(()=>null,300)).toEqual([]);expect(linuxAncestryV1(path=>files[path]??null,1)).toEqual([]);
});

test("the Windows chain comes from one process listing, stops at a recycled parent id and names the client by its image",()=>{
 const text=JSON.stringify(fixture.windows.listing),rows=windowsAncestryV1(text,fixture.windows.first);
 expect(rows.map(row=>row.pid)).toEqual(fixture.windows.pids);
 expect(windowsAncestryV1(text,fixture.windows.orphanFirst).map(row=>row.pid)).toEqual(fixture.windows.orphanPids);
 expect(resolveLaneIdV1({environment:{},ancestry:()=>rows})).toMatch(/^a[0-9a-z]{1,10}$/u);
 expect(resolveLaneIdV1({environment:{},ancestry:()=>windowsAncestryV1(JSON.stringify(fixture.windows.listing.slice(0,1)),fixture.windows.first)})).toBe("d");
 expect(windowsAncestryV1(JSON.stringify(fixture.windows.single),77).map(row=>row.pid)).toEqual([77]);
 expect(windowsAncestryV1("not json",77)).toEqual([]);expect(windowsAncestryV1("[{\"pid\":\"x\"}]",77)).toEqual([]);
 for(const word of["Win32_Process","ConvertTo-Json -Compress","ToUniversalTime","CommandLine"])expect(WINDOWS_PROCESS_LISTING_V1).toContain(word);
});

test("the macOS ps chain keeps its meaning behind the injected runner",()=>{
 const table:Record<number,string>={50:" 40 Mon Oct 20 10:00:00 2026 /Applications/Claude.app/claude\n",40:"  1 Mon Oct 20 09:00:00 2026 launchd\n"};
 expect(psAncestryV1(pid=>table[pid]??null,50)).toEqual([{pid:50,started:"Mon Oct 20 10:00:00 2026",command:"/Applications/Claude.app/claude"},{pid:40,started:"Mon Oct 20 09:00:00 2026",command:"launchd"}]);
 expect(psAncestryV1(()=>null,50)).toEqual([]);
});

test.skipIf(process.platform!=="linux")("on a real Linux host the /proc chain equals the ps chain",()=>{
 const rows=systemAncestryV1("linux"),ps=Bun.spawnSync(["ps","-o","ppid=","-p",String(process.pid)],{stdout:"pipe"}).stdout.toString().trim();
 expect(rows[0]!.pid).toBe(Number(ps));expect(rows.every(row=>row.command.length>0)).toBe(true);
});
test.skipIf(process.platform!=="darwin")("on a real macOS host the ps chain starts at the parent process",()=>{
 const rows=systemAncestryV1("darwin");expect(rows[0]!.pid).toBe(process.ppid);
});
test.skipIf(process.platform!=="win32")("on a real Windows host the CIM listing starts at the parent process",()=>{
 const rows=systemAncestryV1("win32");expect(rows[0]!.pid).toBe(process.ppid);
});
