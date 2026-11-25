/** 🦀️ Contract: the Cargo part of the module closure graph (crate, sources and file leaf nodes, `./🟨️.mjs`) gives every owner that has only a `Cargo.toml` a closure digest that an edit of one crate's Rust sources moves for exactly the owners whose dependency closure contains that crate, as an independent Python reading of the manifests (`tomllib`, plain recursion) says; an edit of the lock file moves every owner of its Cargo workspace, an edit of a file no crate reads moves none; a rebuild that reuses the nodes whose stamps hold equals a build from scratch; another checkout trusts exactly the leaves whose bytes it has; declared globs of non-module inputs are nodes whose matches equal Python's `glob`. */
import{expect,setDefaultTimeout,test}from"bun:test";
import{appendFileSync,cpSync,mkdirSync,mkdtempSync,realpathSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{createClosureGraphV1}from"../../🟨️.mjs";
import{globMatchesV1,globNodesV1,leafContentHoldsV1}from"../../🍃️leaves/🟨️.mjs";
import{cargoCrateNodesV1,cargoNodeStampHoldsV1,cargoOwnerRootV1}from"../🟨️.mjs";
import{requireTestToolV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

setDefaultTimeout(180000);
const oracle=join(import.meta.dir,"🐍️.py"),python=requireTestToolV1("python");
const TREE:Record<string,string>={
 "Cargo.toml":'[workspace]\nmembers=["a","b","c","d","e"]\nresolver="2"\n[workspace.dependencies]\nb={path="b"}\n',
 "Cargo.lock":"# lock v1\n",
 "a/Cargo.toml":'[package]\nname="a"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nb={workspace=true}\n[dev-dependencies]\ne={path="../e"}\n',
 "a/src/lib.rs":"mod inner;\npub fn a(){}\n","a/src/inner.rs":"pub fn inner(){}\n","a/README.md":"readme\n",
 "a/assets/one.json":"{}\n","a/assets/deep/two.json":"[]\n","a/assets/skip.txt":"x\n",
 "b/Cargo.toml":'[package]\nname="b"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nc={path="../c"}\n',"b/src/lib.rs":"pub fn b(){}\n",
 "c/Cargo.toml":'[package]\nname="c"\nversion="0.1.0"\nedition="2021"\n',"c/src/lib.rs":"mod deep;\npub fn c(){}\n","c/src/deep.rs":"pub fn deep(){}\n",
 "d/Cargo.toml":'[package]\nname="d"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nc={path="../c"}\n',"d/src/main.rs":"fn main(){}\n",
 "e/Cargo.toml":'[package]\nname="e"\nversion="0.1.0"\nedition="2021"\n',"e/src/lib.rs":"pub fn e(){}\n",
 "f/Cargo.toml":'[package]\nname="f"\nversion="0.1.0"\nedition="2021"\n[workspace]\n[dependencies]\nc={path="../c"}\n',"f/src/lib.rs":"pub fn f(){}\n","f/build.rs":"fn main(){}\n","f/Cargo.lock":"# f lock\n","f/data.bin":"bytes\n",
};
const OWNERS=["a","b","c","d","e","f"];
const past=(root:string)=>{const at=new Date(Date.now()-3600000);for(const path of Object.keys(TREE))utimesSync(join(root,path),at,at);};
const fixture=()=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"closure-cargo-")));for(const[path,text]of Object.entries(TREE)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}past(root);return root;};
const build=(root:string,held?:(key:string)=>any)=>{const nodes=new Map<string,any>(),built=new Set<string>(),graph=createClosureGraphV1();for(const owner of OWNERS){const before=new Set(nodes.keys());cargoCrateNodesV1(root,owner,{nodes,held:held??(()=>undefined)});for(const key of nodes.keys())if(!before.has(key))built.add(key);}graph.update(nodes.values());return{graph,nodes};};
const digests=(root:string,graph:ReturnType<typeof createClosureGraphV1>)=>Object.fromEntries(OWNERS.map(owner=>[owner,graph.closureDigest([cargoOwnerRootV1(root,owner)])!]));
const ask=(root:string,edits:string[])=>{const run=Bun.spawnSync([python,"-I",oracle],{stdin:Buffer.from(JSON.stringify({root,owners:OWNERS,edits:edits.map(path=>({path}))}))});expect(run.exitCode,run.stderr.toString()).toBe(0);return JSON.parse(run.stdout.toString()) as{reach:Record<string,string[]>;moved:string[][]};};
const moved=(before:Record<string,string>,after:Record<string,string>)=>OWNERS.filter(owner=>before[owner]!==after[owner]).sort();

test("an edit of one crate's sources moves exactly the owners whose dependency closure holds it, the lock moves its workspace, an unread file moves nothing",()=>{
 const root=fixture();
 try{
  const first=build(root),before=digests(root,first.graph);
  for(const owner of OWNERS)expect(before[owner]).toMatch(/^[0-9a-f]{64}$/u);
  expect(new Set(Object.values(before)).size).toBe(OWNERS.length);
  const reached=ask(root,[]).reach;
  for(const owner of OWNERS){const crates=first.graph.reach([cargoOwnerRootV1(root,owner)]).keys.filter(key=>JSON.parse(key)[0]==="crate").map(key=>(JSON.parse(key)[1] as string).slice("{workspaceRoot}/".length)).sort();expect(crates).toEqual(reached[owner]!);}
  const edits=["c/src/deep.rs","a/src/inner.rs","e/src/lib.rs","b/src/lib.rs","Cargo.lock","f/Cargo.lock"],expected=ask(root,edits).moved;
  let current=before;
  edits.forEach((path,at)=>{
   appendFileSync(join(root,path),"// edit "+at+"\n");past(root);
   const next=build(root,key=>{const held=first.nodes.get(key);return held&&cargoNodeStampHoldsV1(root,held)?held:undefined;}),after=digests(root,next.graph);
   expect(moved(current,after)).toEqual(expected[at]!);
   expect(after).toEqual(digests(root,build(root).graph));
   first.nodes.clear();for(const[key,node]of next.nodes)first.nodes.set(key,node);current=after;
  });
  writeFileSync(join(root,"a/README.md"),"other\n");writeFileSync(join(root,"a/src/unused.rs"),"pub fn unused(){}\n");past(root);
  expect(moved(current,digests(root,build(root).graph))).toEqual([]);
  writeFileSync(join(root,"f/data.bin"),"changed\n");past(root);
  expect(moved(current,digests(root,build(root).graph))).toEqual(["f"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("another checkout trusts exactly the leaves whose bytes it has, and globs of non-module inputs match as Python's glob",()=>{
 const root=fixture(),copy=realpathSync(mkdtempSync(join(tmpdir(),"closure-cargo-copy-")));
 try{
  cpSync(root,copy,{recursive:true});appendFileSync(join(copy,"c/src/deep.rs"),"// worker edit\n");
  const{nodes}=build(root),leaves=[...nodes.values()].filter(node=>node.kind==="file"),untrusted=leaves.filter(leaf=>!leafContentHoldsV1(copy,leaf)).map(leaf=>leaf.name);
  expect(untrusted).toEqual(["{workspaceRoot}/c/src/deep.rs"]);
  for(const pattern of["**/*.json","*.json","deep/*","**/*"]){
   const run=Bun.spawnSync([python,"-I","-c","import glob,json,os,sys;os.chdir(sys.argv[1]);print(json.dumps(sorted(p for p in glob.glob(sys.argv[2],recursive=True) if os.path.isfile(p))))",join(root,"a/assets"),pattern]);
   expect(globMatchesV1(root,"a/assets",pattern).files).toEqual((JSON.parse(run.stdout.toString()) as string[]).map(path=>"a/assets/"+path).sort());
  }
  const graph=createClosureGraphV1(),before=globNodesV1(root,"a/assets","**/*.json");graph.update(before);const key=before[0]!.key,first=graph.closureDigest([key]);
  writeFileSync(join(root,"a/assets/skip.txt"),"y\n");graph.update(globNodesV1(root,"a/assets","**/*.json"));expect(graph.closureDigest([key])).toBe(first);
  writeFileSync(join(root,"a/assets/deep/two.json"),"[1]\n");graph.update(globNodesV1(root,"a/assets","**/*.json"));const second=graph.closureDigest([key]);expect(second).not.toBe(first);
  writeFileSync(join(root,"a/assets/deep/three.json"),"{}\n");graph.update(globNodesV1(root,"a/assets","**/*.json"));expect(graph.closureDigest([key])).not.toBe(second);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(copy,{recursive:true,force:true});}
});
