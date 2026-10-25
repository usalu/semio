/** 📌️ Contract: marker-only folder discovery lists the topmost directories whose whole tree is zero-byte taxonomy markers, never a diff facet, a protected tree or a build directory; marker names are re-derived from the taxonomy file independently and the corpus is language-agnostic with `🐍️.py` walking the same workspace. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {afterAll} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {sep} from "node:path";
import {cleanCollectMarkerOnlyFolderRemovals,cleanDirectoryTreeIsOnlyEmptyMarkers,cleanEmptyMarkerFilenames,cleanIsEmptyMarkerFile} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const posix=(path:string):string=>path.split(sep).join("/");
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const world=():string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"folders-")));roots.push(root);for(const node of corpus.nodes){const target=join(root,...node.path.split("/"));if(node.kind==="directory")mkdirSync(target,{recursive:true});else{mkdirSync(dirname(target),{recursive:true});writeFileSync(target,(node as {text?:string}).text??"");}}return root;};
const root=world();
const absolute=(path:string):string=>join(root,...path.split("/"));
const taxonomyPath=fileURLToPath(new URL("../../../🔣️taxonomy.json",import.meta.url));
const names=cleanEmptyMarkerFilenames();
test("marker names are the taxonomy marker kinds plus the ticket important leaf",()=>{
 const taxonomy=JSON.parse(readFileSync(taxonomyPath,"utf8")) as {fileKinds:Record<string,{emoji:string;extensionChains:string[];role:string}>};
 const derived=Object.values(taxonomy.fileKinds).filter(kind=>kind.role==="marker").map(kind=>kind.emoji+kind.extensionChains[0]!);
 expect([...names].sort()).toEqual([...derived,corpus.marker.expectedNames.find(name=>name.endsWith("\ud83d\udcdd\ufe0f.md"))!].sort());expect([...names].sort()).toEqual([...corpus.marker.expectedNames].sort());
});
for(const row of corpus.marker.files)test("empty marker file "+row.path,()=>{expect(cleanIsEmptyMarkerFile(absolute(row.path),names)).toBe(row.expected);});
for(const row of corpus.marker.trees)test("only empty markers below "+row.path,()=>{expect(cleanDirectoryTreeIsOnlyEmptyMarkers(absolute(row.path),names)).toBe(row.expected);});
for(const row of corpus.marker.collect)test("topmost marker-only folders with protected "+JSON.stringify(row.prefixes),()=>{expect(cleanCollectMarkerOnlyFolderRemovals(root,row.prefixes.map(absolute)).map(item=>posix(item.path))).toEqual(row.expected);expect(cleanCollectMarkerOnlyFolderRemovals(root,row.prefixes.map(absolute)).every(item=>item.kind==="marker-only-folder")).toBe(true);});
test("a taxonomy without marker kinds only keeps the ticket important leaf",()=>{expect<unknown>([...cleanEmptyMarkerFilenames({fileKinds:{x:{emoji:"x",extensionChains:[".x"],role:"specification"}}} as never)]).toEqual([corpus.marker.expectedNames.find(name=>name.endsWith(".md")&&!name.includes("empty"))]);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
