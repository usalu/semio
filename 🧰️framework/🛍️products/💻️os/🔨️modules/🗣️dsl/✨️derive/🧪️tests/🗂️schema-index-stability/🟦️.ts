import {afterAll,expect,test} from "bun:test";
import fg from "fast-glob";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {mutationSchemaDocumentIndexProjection} from "../../🗂️schema-index/🟦️.ts";
import {writeProjectionIfChangedV1} from "../🔬️rust-package-commands/🟦️.ts";

/** 🧊️ The schema document index is a pure function of the tracked schema tree: the same tree gives the same bytes at any path, in any creation order, on any machine; machine-local directories named in the ignore list never enter it; an unchanged projection is not rewritten. */
const scratch=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR||tmpdir(),"r205-index-"));
afterAll(()=>{rmSync(scratch,{recursive:true,force:true});});

const FILES:readonly (readonly [string,string])[]=[
 ["🧰️framework/🔨️modules/alpha/🧬️schema/🔣️.json","https://x.example/alpha"],
 ["🧰️framework/🔨️modules/alpha/🧬️schema/sub/🔣️.json","https://x.example/alpha/sub"],
 ["🧰️framework/🔨️modules/beta/🧬️schema/🔣️.json","https://x.example/beta"],
 ["✏️s/🔌️plugins/p1/🧬️schema/🔣️.json","https://x.example/p1"],
 ["✏️s/🔌️plugins/p1/🧬️schema/🧬️mutations/m/🧬️schema/🔣️.json","https://x.example/p1/m"],
 ["✏️s/🔌️plugins/p1/🧪️tests/🧬️schema/🔣️.json","https://x.example/specimen"],
 ["✏️s/🔌️plugins/p2/🧬️schema/🔣️.json","https://x.example/p2"],
];
const GHOST:readonly (readonly [string,string])[]=[["temp/🔨️modules/ghost/🧬️schema/🔣️.json","https://x.example/ghost"],["local/🔌️plugins/ghost2/🧬️schema/🔣️.json","https://x.example/ghost2"]];

function tree(name:string,files:readonly (readonly [string,string])[],ignore?:string):string{
 const root=join(scratch,name);mkdirSync(root,{recursive:true});
 if(ignore!==undefined)writeFileSync(join(root,".gitignore"),ignore);
 for(const [path,id] of files){const target=join(root,path);mkdirSync(dirname(target),{recursive:true});writeFileSync(target,JSON.stringify({$id:id,type:"object"}));}
 return root;
}

test("the same tree gives byte-identical output at another absolute path and in the reverse creation order, with LF only and no path of the machine",()=>{
 const first=tree("a/ws",FILES),second=tree("b/deeper/still/ws",[...FILES].reverse());
 const left=mutationSchemaDocumentIndexProjection(first),right=mutationSchemaDocumentIndexProjection(second);
 expect(right).toBe(left);expect(mutationSchemaDocumentIndexProjection(first)).toBe(left);
 expect(left.includes("\r")).toBe(false);expect(left.endsWith("}\n")&&!left.endsWith("\n\n")).toBe(true);
 expect(left.includes(scratch)).toBe(false);expect(left.includes("/private/")||left.includes("/Users/")).toBe(false);
 const roots=Object.keys(JSON.parse(left).roots as Record<string,unknown>);expect(roots).toEqual([...roots].sort());
});

test("the documents are the ones an independent glob finds, without the specimen collection",async()=>{
 const root=tree("c/ws",FILES),projected=JSON.parse(mutationSchemaDocumentIndexProjection(root)).roots as Record<string,Record<string,string>>;
 expect(Object.keys(projected)).toEqual(["✏️s/🔌️plugins/p1","✏️s/🔌️plugins/p1/🧬️schema","✏️s/🔌️plugins/p2","🧰️framework/🔨️modules/alpha","🧰️framework/🔨️modules/beta"].filter(key=>key in projected));
 for(const [rootPath,documents] of Object.entries(projected)){
  const files=await fg("**/🧬️schema/**/*.json",{cwd:join(root,rootPath),onlyFiles:true,dot:false,followSymbolicLinks:false,ignore:["**/target/**","**/node_modules/**","**/dist/**","**/🗑️generated/**","**/📦️packages/**","**/🧪️tests/**"]});
  const ids=files.map(file=>JSON.parse(readFileSync(join(root,rootPath,file),"utf8")).$id as string).sort();
  const found=Object.keys(documents).sort();
  if(!rootPath.endsWith("🧬️schema"))expect(found,rootPath).toEqual(ids);
 }
 expect(JSON.stringify(projected).includes("specimen")).toBe(false);
});

test("a directory the ignore list names is machine-local: it never changes the bytes; without the entry the same directory would",()=>{
 const clean=mutationSchemaDocumentIndexProjection(tree("d/ws",FILES,"temp\nlocal\n# comment\n*.log\n/old\n"));
 const withGhost=mutationSchemaDocumentIndexProjection(tree("e/ws",[...FILES,...GHOST],"temp\nlocal\n# comment\n*.log\n/old\n"));
 expect(withGhost).toBe(clean);expect(clean.includes("ghost")).toBe(false);
 const unprotected=mutationSchemaDocumentIndexProjection(tree("f/ws",[...FILES,...GHOST]));
 expect(unprotected).not.toBe(clean);expect(unprotected.includes("ghost")).toBe(true);
});

test("an unchanged projection is not rewritten: same inode, same modification time; a changed one is",async()=>{
 const path=join(scratch,"index.json"),text=mutationSchemaDocumentIndexProjection(tree("g/ws",FILES));
 expect(writeProjectionIfChangedV1(path,text)).toBe("written");
 const before=statSync(path,{bigint:true});await new Promise(done=>setTimeout(done,20));
 expect(writeProjectionIfChangedV1(path,text)).toBe("unchanged");
 const after=statSync(path,{bigint:true});expect(after.ino).toBe(before.ino);expect(after.mtimeNs).toBe(before.mtimeNs);
 expect(writeProjectionIfChangedV1(path,text+" ")).toBe("written");expect(readFileSync(path,"utf8")).toBe(text+" ");
});

test("the committed index is content-stable: LF, one final newline, sorted keys, no absolute path",()=>{
 const text=readFileSync(new URL("../../🗂️schema-document-index.json",import.meta.url),"utf8");
 expect(text.includes("\r")).toBe(false);expect(text.endsWith("}\n")&&!text.endsWith("\n\n")).toBe(true);
 const index=JSON.parse(text) as {schema:string;roots:Record<string,Record<string,string>>};
 expect(index.schema).toBe("semio.dsl.mutation-schema-document-index/v1");
 const roots=Object.keys(index.roots);expect(roots).toEqual([...roots].sort());
 for(const [root,documents] of Object.entries(index.roots)){
  const ids=Object.keys(documents);expect(ids,root).toEqual([...ids].sort((left,right)=>left<right?-1:left>right?1:0));
  for(const path of Object.values(documents)){expect(path.startsWith("/")||/^[A-Za-z]:/u.test(path)||path.includes("\\")||path.includes("..")).toBe(false);}
 }
 expect(JSON.stringify(index,null,2)+"\n").toBe(text);
});
