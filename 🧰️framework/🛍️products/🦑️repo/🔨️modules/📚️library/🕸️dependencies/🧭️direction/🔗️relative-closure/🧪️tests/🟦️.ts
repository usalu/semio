/** 🔗️ Relative closure law: the portable corpus agrees with TypeScript's own import preprocessor, and the build CLI entry reaches no workspace package specifier. */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import ts from "typescript";
import {readRelativeClosureV1} from "../🟦️.ts";
import corpus from "./🧫️fixtures/🔣️.json";

const root=resolve(import.meta.dir,"../../../../../../../../..");
const sources=/\.(?:[cm]?[jt]s|[jt]sx)$/u;

function materialize(files:Readonly<Record<string,string>>):string{
  const directory=mkdtempSync(join(tmpdir(),"relative-closure-"));
  for(const [path,text] of Object.entries(files)){mkdirSync(dirname(join(directory,path)),{recursive:true});writeFileSync(join(directory,path),text);}
  return directory;
}

function oracle(directory:string,entry:string):{files:string[];workspace:string[]}{
  const seen=new Set<string>(),workspace=new Set<string>(),queue=[entry];
  while(queue.length){
    const path=queue.pop()!;
    if(seen.has(path))continue;
    seen.add(path);
    const text=ts.sys.readFile(join(directory,path))!;
    for(const row of ts.preProcessFile(text,true,true).importedFiles){
      if(row.fileName.startsWith("@semio-tech/"))workspace.add(`${path} -> ${row.fileName}`);
      if(!row.fileName.startsWith("."))continue;
      const target=join(dirname(path),row.fileName);
      if(sources.test(target)&&ts.sys.fileExists(join(directory,target)))queue.push(target);
    }
  }
  return {files:[...seen].sort(),workspace:[...workspace].sort()};
}

type Case=Readonly<{id:string;entry:string;files:Readonly<Record<string,string>>;closure:readonly string[];workspace:readonly string[]}>;

test("portable closure corpus agrees with the expected closure and TypeScript's import preprocessor",()=>{
  for(const row of corpus.cases as unknown as readonly Case[]){
    const directory=materialize(row.files),ours=readRelativeClosureV1(directory,row.entry),theirs=oracle(directory,row.entry);
    expect(ours.files,row.id).toEqual(row.closure);
    expect(ours.workspaceSpecifiers,row.id).toEqual(row.workspace);
    expect({files:ours.files,workspace:ours.workspaceSpecifiers},row.id).toEqual(theirs);
  }
});

test("the build CLI entry reaches no workspace package specifier, so it boots without node_modules",()=>{
  const closure=readRelativeClosureV1(root,corpus.bootstrapEntry);
  expect(closure.files.length).toBeGreaterThan(100);
  expect(closure.workspaceSpecifiers).toEqual([]);
});
