/** 🪪️ A Windows relative directory or path refusal names exactly which compared NTFS facts moved, with both values and the directory; Windows-shaped facts are injected so the pure comparison and its texts run on every host, checked against hand-authored rows and an independent Python oracle. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {ArtifactWindowsRelativeFactsChangedErrorV1,artifactRelativeRefusalV1,artifactWindowsRelativeFactsChangeTextsV1,artifactWindowsRelativeFactsDifferencesV1,sameArtifactWindowsRelativeFactsV1,type ArtifactWindowsRelativeFactsRefusalV1,type ArtifactWindowsRelativeFactsV1} from "../../📁️descriptor/🟦️.ts";
import {filesystemBusyErrorV1,isConcurrentEditV1} from "../../🔁️churn/🟦️.ts";
import schema from "../../📁️descriptor/🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};

const policy=schema.definitions.RelativeWindowsFactsChange.const;
type Flat=Readonly<Record<string,string|number|null|undefined>>;
const factsOf=(flat:Flat):ArtifactWindowsRelativeFactsV1=>{const big=(field:string)=>BigInt(String(flat[field]));return{identity:{dev:big("dev"),ino:big("ino"),mode:big("mode"),size:big("size"),nlink:big("nlink"),mtimeNs:big("mtimeNs"),ctimeNs:big("ctimeNs")},kind:flat.kind as ArtifactWindowsRelativeFactsV1["kind"],volumeSerial:String(flat.volumeSerial),fileId:String(flat.fileId),attributes:Number(flat.attributes),security:String(flat.security),streams:String(flat.streams),...(flat.target===undefined||flat.target===null?{}:{target:String(flat.target)})};};
const pair=(row:(typeof fixture.rows)[number])=>[factsOf(fixture.base),factsOf({...fixture.base,...row.actualChanges})] as const;

test("every row names exactly the differing facts and renders both declared texts",()=>{
 for(const row of fixture.rows){
  const [expected,actual]=pair(row),differences=artifactWindowsRelativeFactsDifferencesV1(expected,actual,"role" in row?row.role:undefined);
  expect(differences,row.id).toEqual(row.differences);
  expect(sameArtifactWindowsRelativeFactsV1(expected,actual),row.id).toBe(row.differences.length===0);
  if(row.texts)expect(artifactWindowsRelativeFactsChangeTextsV1(row.refusal as ArtifactWindowsRelativeFactsRefusalV1,row.directory,differences),row.id).toEqual(row.texts);
 }
});

test("an independent Python oracle derives the same differences and texts",()=>{
 const run=spawnSync("python3",[fileURLToPath(new URL("./🐍️oracle.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url)),fileURLToPath(new URL("../../📁️descriptor/🧬️schema/🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual(Object.fromEntries(fixture.rows.map(row=>[row.id,{differences:row.differences,texts:row.texts}])));
});

test("the declared fields are exactly the facts the comparison reads, and moving any one of them alone is refused under its own name",()=>{
 const base=factsOf(fixture.base);
 expect([...policy.identityFields,...policy.fields].sort()).toEqual([...Object.keys(base.identity),...Object.keys(base).filter(key=>key!=="identity"),"target"].sort());
 for(const field of[...policy.identityFields,...policy.fields]){
  const moved=factsOf({...fixture.base,[field]:field==="kind"?"file":field==="attributes"?32:policy.identityFields.includes(field)?String(BigInt(String((fixture.base as Flat)[field]))+1n):"moved"});
  expect(sameArtifactWindowsRelativeFactsV1(base,moved),field).toBe(false);
  expect(artifactWindowsRelativeFactsDifferencesV1(base,moved).map(row=>row.field),field).toEqual([field]);
 }
});

test("the typed refusal keeps its declared sentence for retry policies and stays outside the concurrent-edit and busy classes like the plain error it replaces",()=>{
 const row=fixture.rows[0]!,[expected,actual]=pair(row),error=new ArtifactWindowsRelativeFactsChangedErrorV1("directory",row.directory,artifactWindowsRelativeFactsDifferencesV1(expected,actual));
 expect(error).toBeInstanceOf(Error);expect(error.message).toBe(row.texts!.en);expect(error.texts).toEqual(row.texts!);
 expect(error.refusal).toBe("Original Windows relative directory facts changed");expect(error.directory).toBe(row.directory);expect(error.differences).toEqual(row.differences);
 expect(artifactRelativeRefusalV1(error)).toBe(error.refusal);expect(artifactRelativeRefusalV1(Error("Current exact relative path descriptor changed"))).toBe("Current exact relative path descriptor changed");
 expect(artifactRelativeRefusalV1(new ArtifactWindowsRelativeFactsChangedErrorV1("pathDescriptor","C:\\released\\lease",[]))).toBe("Current exact Windows relative path descriptor changed");
 for(const candidate of[error,Error(error.refusal)]){expect(isConcurrentEditV1(candidate)).toBe(false);expect(filesystemBusyErrorV1(candidate,"win32")).toBe(false);}
});
