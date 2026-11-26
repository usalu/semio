/** 🦀️ Law of cargo stand-ins: an admission read lists the Cargo manifest of an owner without its sibling project configuration, and the sibling is still a candidate, so the owner is declared at its root and the generator contract of that root is checked against the owner and holds, where it can only fail against a project built under the package name.
 * @see ../../🟨️.mjs `projectConfigCandidatesV1` */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join,relative,resolve}from"node:path";
import{findWorkspaceRoot}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{privateRepositoryWorkspaceV1}from"../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import{createScriptInputCache}from"../../../🟨️.mjs";
import{projectConfigCandidatesV1}from"../../🟨️.mjs";
import{projectWithDefaults}from"../../../🧱️project-assembly/🟨️.mjs";

await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference-projects",scratch:"checkout"});
const workspace=findWorkspaceRoot(import.meta.dir);

test("the sibling project file of a listed Cargo manifest is a candidate, so the owner holds the generator contract of its root and a package-named stand-in refuses it",async()=>{
 const directory=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!directory)throw Error("Stand-in fixtures require owned test artifacts");
 mkdirSync(directory,{recursive:true});
 const root=mkdtempSync(join(resolve(directory),"stand-in-")),write=(name:string,content:string)=>{mkdirSync(dirname(join(root,name)),{recursive:true});writeFileSync(join(root,name),content);};
 try{
  write("owner/Cargo.toml",'[package]\nname = "fixture-package"\nversion = "0.0.0"\n');
  write("owner/📋️project.json","{}");
  write("lone/Cargo.toml",'[package]\nname = "lone-package"\nversion = "0.0.0"\n');
  expect(projectConfigCandidatesV1(["owner/Cargo.toml","lone/Cargo.toml"],root)).toEqual(["lone/Cargo.toml","owner/Cargo.toml","owner/📋️project.json"]);
  const placement=relative(workspace,join(root,"owner")),contracts={fixture:{ownership:"owned",ownerPath:placement,target:"fixture-owner:generate",inputPatterns:[],outputRoots:[]}};
  const assemble=(json:Readonly<Record<string,unknown>>)=>projectWithDefaults(json,placement,join(root,"owner"),workspace,contracts,new Map(),undefined,createScriptInputCache(),undefined,"source",["generate"],[]);
  expect(Object.keys((await assemble({name:"fixture-owner",targets:{generate:{executor:"repo:noop",cache:false}}})).targets)).toEqual(["generate"]);
  await expect(assemble({name:"fixture-package",tags:["language:cargo","discovery:manifest"],targets:{}})).rejects.toThrow("Generator target has no project owner: fixture-owner:generate");
 }finally{rmSync(root,{recursive:true,force:true});}
},120_000);
