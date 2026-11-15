/** 🪶️ Contract: a generation row that the file system alone can refute is checked before the parser material is imported, and the rows that need the parsers are the only ones that wait for them. */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{PARSER_FREE_HOLD_KINDS_V1,splitHoldRowsV1}from"../../🕰️host/🟦️.ts";
import{graphSourceV1}from"../📖️source/🟦️.ts";

const kinds=["file","analysis","exists","lstat","readdir","resolve","fileExists","directoryExists","readdirNames","stamp","field","registry"];

test("the split keeps every row exactly once and the order of each part",()=>{
 const rows=kinds.flatMap((kind,index)=>[{kind,path:"a"+index},{kind,path:"b"+index}]),{free,parsed}=splitHoldRowsV1(rows);
 expect(free.length+parsed.length).toBe(rows.length);
 expect(free.every(row=>PARSER_FREE_HOLD_KINDS_V1.has(row.kind))).toBe(true);
 expect(parsed.every(row=>!PARSER_FREE_HOLD_KINDS_V1.has(row.kind))).toBe(true);
 expect(rows.filter(row=>PARSER_FREE_HOLD_KINDS_V1.has(row.kind))).toEqual(free);
 expect(rows.filter(row=>!PARSER_FREE_HOLD_KINDS_V1.has(row.kind))).toEqual(parsed);
 expect(parsed.some(row=>row.kind==="analysis")).toBe(true);
});

test("the split equals an independent Python partition",()=>{
 const python=spawnSync("python3",["-I","-c","import json,sys;k=json.loads(sys.argv[1]);f=set(json.loads(sys.argv[2]));print(json.dumps([[x for x in k if x in f],[x for x in k if x not in f]]))",JSON.stringify(kinds),JSON.stringify([...PARSER_FREE_HOLD_KINDS_V1])],{encoding:"utf8"});
 expect(python.status).toBe(0);
 const{free,parsed}=splitHoldRowsV1(kinds.map(kind=>({kind})));
 expect(JSON.parse(python.stdout)).toEqual([free.map(row=>row.kind),parsed.map(row=>row.kind)]);
});

test("the host refutes the file system rows first and imports the parsers only for the rest",()=>{
 const source=graphSourceV1(),at=source.indexOf("const holdsByStat="),body=source.slice(at,source.indexOf("const lease=",at));
 const split=body.indexOf("splitHoldRowsV1(plain)"),freeCheck=body.indexOf("observationsHold(free,"),prepared=body.indexOf("await analysesPrepared()"),parsedCheck=body.indexOf("observationsHold(parsed,");
 expect(at).toBeGreaterThan(0);expect(split).toBeGreaterThan(0);expect(freeCheck).toBeGreaterThan(split);expect(prepared).toBeGreaterThan(freeCheck);expect(parsedCheck).toBeGreaterThan(prepared);
 expect(body.match(/analysesPrepared\(\)/gu)?.length).toBe(1);
});
