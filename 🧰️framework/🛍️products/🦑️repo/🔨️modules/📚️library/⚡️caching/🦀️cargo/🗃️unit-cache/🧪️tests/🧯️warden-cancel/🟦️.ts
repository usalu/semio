/** 🧯️ Contract: a compiler the host warden killed leaves one bilingual line in the build log that names the typed reason (for a unit over its ceiling: the unit, its peak and its ceiling and that it must be carved); a compiler that no cancel record names leaves nothing.
 * @see ../../📜️script.ts */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {reportWardenCancelV1} from "../../📜️script.ts";

const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const directory=():string=>{const root=mkdtempSync(join(tmpdir(),"unit-cancel-"));roots.push(root);mkdirSync(join(root,"warden","cancelled"),{recursive:true});return root;};
const record=(pid:number,over:object)=>({version:1,pid,start:"s",kind:"local-unit",owner:"o",command:"rustc --crate-name semio_plain",megabytes:1800,atMs:Date.now()-1000,reason:"ceiling",ancestors:[1],ceiling:{unit:"semio_plain|wasm32-wasip2",kind:"library",megabytes:1536,listed:false},...over});

test("a compiler killed for its ceiling leaves the unit, its peak and its ceiling in English and German; an unrelated pid leaves nothing",async()=>{
 const root=directory(),lines:string[]=[];
 writeFileSync(join(root,"warden","cancelled",`4711.aaaaaaaa.json`),JSON.stringify(record(4711,{})));
 await reportWardenCancelV1(4711,{directory:root,write:line=>lines.push(line)});
 expect(lines).toHaveLength(1);
 expect(lines[0]).toContain("[unit-cache] [warden] cancelled local-unit pid 4711 (1800 MB): unit semio_plain|wasm32-wasip2 peaked at 1800 MB, above its library ceiling of 1536 MB");
 expect(lines[0]).toContain("carve it");
 expect(lines[0]).toContain(" / [Wächter] abgebrochen local-unit pid 4711 (1800 MB): Einheit semio_plain|wasm32-wasip2 erreichte 1800 MB");
 await reportWardenCancelV1(4712,{directory:root,write:line=>lines.push(line)});
 expect(lines).toHaveLength(1);
});

test("a compiler killed for another reason leaves that reason; a missing directory or a record of another age never throws",async()=>{
 const root=directory(),lines:string[]=[];
 writeFileSync(join(root,"warden","cancelled",`4713.aaaaaaaa.json`),JSON.stringify(record(4713,{reason:"overdue",ceiling:undefined})));
 await reportWardenCancelV1(4713,{directory:root,write:line=>lines.push(line)});
 expect(lines[0]).toContain("suspended for too long");
 expect(lines[0]).toContain("zu lange angehalten");
 writeFileSync(join(root,"warden","cancelled",`4714.aaaaaaaa.json`),JSON.stringify(record(4714,{atMs:1})));
 await reportWardenCancelV1(4714,{directory:root,write:line=>lines.push(line)});
 await reportWardenCancelV1(4715,{directory:join(root,"missing"),write:line=>lines.push(line)});
 expect(lines).toHaveLength(1);
});
