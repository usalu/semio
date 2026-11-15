import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {nxOutputsCacheableV1,projectBuildTargetForNxV1} from "../../../../🟨️.mjs";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {cacheLawCountsV1,cacheLawV1,type CacheLawProjectionV1} from "../🟦️.ts";

const projection:CacheLawProjectionV1={project:(target,name)=>projectBuildTargetForNxV1(target as never,name) as never,restorable:outputs=>nxOutputsCacheableV1(outputs as string[])};
const lenient:CacheLawProjectionV1={project:target=>target as never,restorable:()=>true};
const out="{workspaceRoot}/.cache/b/p/m/x";
const task=(extra:Record<string,unknown>)=>({execution:{kind:"command",command:"repo ./📜️script.ts run"},cache:true,outputs:[out],inputs:["default"],metadata:{semio:{cache:{version:1,class:"deterministic",reason:"pure-function-of-inputs"}}},...extra});
const graph=(targets:Record<string,unknown>)=>({version:1 as const,projects:{p:{name:"p",root:"p",targets:targets as never}},dependencies:[]});

test("the law classifies shared, guarded, uncacheable and each violation kind",()=>{
 const rows=cacheLawV1(graph({
  build:task({}),
  wasm:task({}),
  declared:task({cache:false,metadata:{semio:{cache:{version:1,class:"side-effecting",reason:"mutates-workspace"}}}}),
  identity:task({outputs:["{workspaceRoot}/.cache/b/p/m/home/Users"],execution:{kind:"command",command:"cp a /Users/ueli/b"}}),
  machine:task({inputs:["default",{env:"HOSTNAME"}]}),
  ignored:task({inputs:["{projectRoot}/🤖️generated/catalog.json"]}),
  covered:task({inputs:["{projectRoot}/🤖️generated/catalog.json",{runtime:"digest generated"}]}),
  odd:task({inputs:[7]}),
  check:task({outputs:[],metadata:{semio:{cache:{version:1,class:"deterministic",reason:"check-without-outputs"}}}}),
 }),projection),by=Object.fromEntries(rows.map(row=>[row.id.split(":")[1],row]));
 expect(by.build!.state).toBe("shared");expect(by.wasm!.state).toBe("guarded");expect(by.wasm!.reason!.length).toBeGreaterThan(10);
 expect(by.declared).toMatchObject({state:"uncacheable",reason:"side-effecting/mutates-workspace"});
 expect(by.identity!.findings).toContain("command-machine-identity");
 expect(by.machine!.findings).toContain("input-machine-environment");
 expect(by.ignored!.findings).toContain("input-gitignored-without-digest");expect(by.covered!.findings).not.toContain("input-gitignored-without-digest");
 expect(by.odd!.findings).toContain("input-uninterpretable");
 expect(by.check!.state).toBe("shared");
 const wide=cacheLawV1(graph({absolute:task({outputs:["/Users/someone/out"]}),empty:task({outputs:[]}),escape:task({outputs:["{workspaceRoot}/../x"]}),windows:task({outputs:["C:\\\\work\\\\out"]})}),lenient),wideBy=Object.fromEntries(wide.map(row=>[row.id.split(":")[1],row]));
 expect(wideBy.absolute!.findings).toEqual(expect.arrayContaining(["output-absolute","output-machine-identity"]));expect(wideBy.escape!.findings).toContain("output-escapes-workspace");expect(wideBy.empty!.findings).toEqual(["outputs-missing"]);expect(wideBy.windows!.findings).toContain("output-absolute");
 const downgraded=cacheLawV1(graph({absolute:task({outputs:["/Users/someone/out"]})}),projection);expect(downgraded[0]).toMatchObject({state:"uncacheable",reason:"outputs-not-restorable"});
 const counts=cacheLawCountsV1(rows);expect(counts.states.shared+counts.states.guarded+counts.states.uncacheable+counts.states.violation).toBe(rows.length);
 const ajv=new Ajv({strict:false}),validate=ajv.compile({$ref:"#/definitions/Row",definitions:schema.definitions});
 for(const row of rows)expect(validate(row),JSON.stringify(row)).toBe(true);
});

const file=process.env.SEMIO_CACHE_LAW_GRAPH;
test.skipIf(!file)("the whole generated task set has no violation (graph from SEMIO_CACHE_LAW_GRAPH, the output of `repo graph`)",()=>{
 const rows=cacheLawV1(JSON.parse(readFileSync(file!,"utf8")),projection),counts=cacheLawCountsV1(rows);
 console.log(JSON.stringify(counts));
 expect(rows.filter(row=>row.state==="violation").map(row=>row.id+" "+row.findings.join()).slice(0,50)).toEqual([]);
});
