/** 🔍️ Contract: the module-level resolver builds the module tree of every carved crate from its `#[path]` mounts, resolves `use` leaves and paths through explicit imports, globs and re-exports with visibility, records the foreign path (and cut flag) where a path crosses a crate boundary and reports macro-opaque modules; the corpus is language-agnostic and an independent Python model resolves every query again. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {Universe,isKeywordHead,mergeRep,repText,type Hit,type ModNode,type Rep} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const node=(u:Universe,ref:string):ModNode=>{const[head,...rest]=ref.split("::");let current=u.root(head!)!;for(const name of rest)current=current.children.get(name)![0]!;return current;};
const modPath=(n:ModNode):string=>{const names:string[]=[];for(let c:ModNode|null=n;c&&c.parent;c=c.parent)names.unshift(c.name);return [n.crate.ident,...names].join("::");};
const repShow=(r:Rep|null):string=>r?" => "+repText(r)+(r.cut?" cut":""):"";
const show=(h:Hit):string=>h.kind==="module"?"module "+modPath(h.node)+repShow(h.rep):"item "+modPath(h.node)+"::"+h.name+repShow(h.rep);
const build=(row:any):Universe=>{
 const u=new Universe("/virtual",new Map(Object.entries<string>(row.files)));for(const c of row.crates)u.addCrate(c.name,c.dir,c.root);
 u.cutEdge=b=>row.cut.includes(b.leaf.path.join("::"));u.apiLeaf=b=>row.api.includes(b.leaf.path.join("::"));for(const c of row.crates)u.root(c.name.replaceAll("-","_"));return u;
};
for(const row of corpus.cases)test("resolution: "+row.name,()=>{
 const u=build(row);
 for(const q of row.queries){
  const label=q.op+" "+JSON.stringify(q.path.length?q.path:q.name||q.needle);
  if(q.op==="path"){const r=u.resolvePath(node(u,q.at),q.path);expect(r===null?null:{consumed:r.consumed,hits:r.hits.map(show).sort()},label).toEqual(q.expect===null?null:{consumed:q.expect.consumed,hits:[...q.expect.hits].sort()});}
  else if(q.op==="member"){const at=node(u,q.at),from=node(u,q.from||q.at),hits=q.ignoreVisibility?u.memberIgnoringVisibility(at,q.name,from):u.member(at,q.name,from);expect(hits.map(show).sort(),label).toEqual([...q.expectHits].sort());}
  else if(q.op==="opaque")expect(u.opaqueModule(node(u,q.at)),label).toBe(q.expectFlag);
  else{const text=(row.files as Record<string,string|undefined>)[q.file]!,found=u.nodeAt(q.file,text.indexOf(q.needle));expect(found?modPath(found):null,label).toBe(q.expectNode);}
 }
 expect(u.ignoreVisibility).toBe(false);
});
for(const row of corpus.merge)test("mergeRep: "+row.name,()=>{expect(mergeRep(row.inner,row.outer,row.segment)).toEqual(row.expected);});
for(const row of corpus.texts)test("repText of "+row.expected,()=>{expect(repText(row.rep)).toBe(row.expected.replace(" cut",""));});
for(const row of corpus.keywords)test("isKeywordHead("+JSON.stringify(row.head)+")",()=>{expect(isKeywordHead(row.head)).toBe(row.expected);});
test("a universe names crates by extern identifier, reads through its overlay and loads roots lazily",()=>{
 const u=new Universe("/virtual",new Map(Object.entries<string>(corpus.universe.overlay)));
 for(const c of corpus.universe.crates)expect(u.addCrate(c.name,"x","x/lib.rs").ident).toBe(c.ident);
 expect([...u.crates.keys()].sort()).toEqual(corpus.universe.crates.map(c=>c.ident).sort());expect(u.loadedRoots()).toEqual([]);
 for(const r of corpus.universe.reads)expect(u.read(r.path)).toBe(r.expected);
 expect(u.root(corpus.universe.unknownRoot)).toBeNull();u.addCrate(corpus.universe.missingRoot,"nowhere","nowhere/lib.rs");expect(u.root(corpus.universe.missingRoot)).toBeNull();expect(u.loadedRoots()).toEqual([]);
 const loaded=u.root("my_crate")!;expect(loaded).not.toBeNull();expect(u.root("my_crate")).toBe(loaded);expect(u.loadedRoots()).toEqual(["my_crate"]);
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)throw Error("Independent Python oracle required");
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
