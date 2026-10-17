import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {execFileSync} from "node:child_process";
import {chmodSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import Ajv from "ajv";
import stableStringify from "fast-json-stable-stringify";
import {applyPatch} from "fast-json-patch";
import {Database} from "bun:sqlite";
import * as discovery from "../../🔍️discovery/🟦️.ts";
import * as normalization from "../../🧹️normalization/🟦️.ts";
import examples from "../🧫️fixtures/🏠️owner-projection-policy/🔣️.json";
import policySchema from "../../🧬️schema/🏠️owner-projection-policy/🔣️.json";
import approvalSchema from "../../🧬️schema/🔖️readme-current-source-revision/🔣️.json";

type TaxonomyProgress=Parameters<NonNullable<Parameters<typeof normalization.inventoryTaxonomy>[0]["progress"]>>[0];

const library=resolve(import.meta.dir,"../.."),sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),tuple=(text:string)=>({sha256:sha(text),mode:"0644" as const,size:Buffer.byteLength(text)});
const validatePolicy=new Ajv({strict:true}).compile(policySchema),validateApprovals=new Ajv({strict:true}).compile(approvalSchema);
function policy(count:number){return {schemaVersion:1,kind:"owner-projection-policy",ownerEvidence:{documentation:{kind:"ordinary-owner-doc",evidencePaths:[]}},referenceOwners:{"markdown-relative-reference-adapter":{kind:"reference-adapter",ownerPath:"repo-lib normalization reference graph"}},generatorOwners:{},entries:examples.owners.slice(0,count).map(sourcePath=>({sourcePath,destinationPath:dirname(sourcePath)+"/📃️readme/📝️.md",ownerEvidenceId:"documentation",disposition:"owner-documentation-relocate",fixedContractId:null,projectionContractId:"exact-owner-readme-projection",referenceOwnerIds:["markdown-relative-reference-adapter"],generatorOwnerId:null,preimage:tuple(examples.bodies.baseline)}))};}
function installed(value:unknown){const artifact=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifact)throw Error("Original ticket artifact storage required");const root=mkdtempSync(join(artifact,"op-")),path="🖼️assets/policy.json",bytes=JSON.stringify(value),taxonomy=JSON.parse(readFileSync(join(library,"🔣️taxonomy.json"),"utf8"));mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),bytes);chmodSync(join(root,path),0o644);const contract=taxonomy.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"] as any;delete contract.expectedCounts;contract.authorityCatalogPath=path;contract.authorityCatalogSha256=sha(bytes);contract.referenceOwnerIds=["markdown-relative-reference-adapter"];contract.generatorOwnerIds=[];return {root,path,taxonomy,contract};}

for(const row of examples.policyCases)test("owner projection policy "+row.id,()=>{
 const value:any=policy(row.rows);if("prefix" in row){value.entries[0].sourcePath=row.prefix+value.entries[0].sourcePath;value.entries[0].destinationPath=row.prefix+value.entries[0].destinationPath;}if("extra" in row&&row.extra!==undefined)value[row.extra]={};if("entryExtra" in row&&row.entryExtra!==undefined)value.entries[0][row.entryExtra]={};if("oldCases" in row){value.cases=value.entries;delete value.entries;}if("duplicate" in row)value.entries[1]=structuredClone(value.entries[0]);
 const shape=Boolean(validatePolicy(value)),independent=shape&&new Set((value.entries??[]).map((entry:any)=>entry.sourcePath)).size===(value.entries??[]).length;expect(independent).toBe(row.accepted);
 const state=installed(value);try{let accepted=true;try{discovery.semanticExactOwnedFileCatalog(state.root,state.taxonomy);}catch{accepted=false;}expect(accepted).toBe(row.accepted);}finally{rmSync(state.root,{recursive:true,force:true});}
});

test("current production owner policy contains only normative records and derives its census",()=>{
 const root=resolve(library,"../../../../.."),taxonomy=discovery.loadTaxonomy(),contract=taxonomy.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
 if(contract.contractKind!=="exact-owner-path-catalog")throw Error("Actual owner projection contract required");
 const snapshot=discovery.semanticOwnedInputFileSnapshot(root,contract.authorityCatalogPath)!;
 const document=JSON.parse(Buffer.from(snapshot.bytes).toString()),parsed=discovery.semanticExactOwnedFileCatalog(root,taxonomy)!;
 expect(validatePolicy(document),JSON.stringify(validatePolicy.errors)).toBe(true);expect(snapshot.contentHash).toBe(contract.authorityCatalogSha256);
 expect(Object.keys(document).sort()).toEqual(["entries","generatorOwners","kind","ownerEvidence","referenceOwners","schemaVersion"]);expect(Object.hasOwn(contract,"expectedCounts")).toBe(false);
 const sql=new Database(":memory:");try{sql.exec("CREATE TABLE owner(source TEXT,fixed INTEGER,references_count INTEGER)");for(const entry of parsed.entries)sql.query("INSERT INTO owner VALUES(?,?,?)").run(entry.sourcePath,Number(entry.disposition==="fixed"),entry.referenceOwnerIds.length);const counts=sql.query<{total:number;fixed:number;projected:number;readme:number;license:number;referenceBindings:number},[]>("SELECT count(*) AS total,sum(fixed) AS fixed,sum(1-fixed) AS projected,sum(source LIKE '%/README.md') AS readme,sum(source LIKE '%/LICENSE.md') AS license,sum(references_count) AS referenceBindings FROM owner").get();expect<typeof counts>(discovery.ownerProjectionPolicyCensus(parsed)).toEqual(counts);}finally{sql.close();}
});

test("actual current source approval binds no-follow source facts without testing inputs",()=>{
 const root=resolve(library,"../../../../.."),taxonomy=discovery.loadTaxonomy(),contract=taxonomy.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
 if(contract.contractKind!=="exact-owner-path-catalog"||!contract.currentSourceRevisions)throw Error("Actual current approval required");
 const parsed=discovery.semanticExactOwnedFileCatalog(root,taxonomy)!,declarations=discovery.parseCurrentSourceApprovals(contract.currentSourceRevisions);
 expect(validateApprovals(declarations)).toBe(true);
 for(const [id,row]of Object.entries(declarations)){const source=discovery.semanticOwnedInputFileSnapshot(root,row.sourcePath)!,approval=discovery.semanticOwnedInputFileSnapshot(root,row.approvalPath)!,document=JSON.parse(Buffer.from(approval.bytes).toString());expect(approval.contentHash).toBe(row.approvalSha256);expect(source.contentHash).toBe(row.currentPreimage.sha256);expect(source.size).toBe(row.currentPreimage.size);expect(document.kind).toBe("current-source-approval");expect(Object.keys(document).sort()).toEqual(["baseline","currentPreimage","kind","policy","schemaVersion","sourcePath"]);expect(row.approvalPath.split("/")).not.toContain("🧫️fixtures");const actual=discovery.semanticExactOwnedFileCurrentPreimageAuthority(parsed,contract,declarations,{path:row.sourcePath,nodeKind:source.nodeKind,contentHash:source.contentHash,mode:source.mode,size:source.size,expectations:[approval]});expect(actual.disposition).toBe("revised");expect(actual.sourcePath).toBe(row.sourcePath);expect(actual.revisionId).toBe(id);}
});

test("actual scoped normalization preserves reserved documentation and observes cancellation",()=>{
 const root=resolve(library,"../../../../.."),taxonomy=discovery.loadTaxonomy(),contract=taxonomy.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
 if(contract.contractKind!=="exact-owner-path-catalog"||!contract.currentSourceRevisions)throw Error("Actual owner contract required");const sourcePath=Object.values(contract.currentSourceRevisions)[0]!.sourcePath,progress:TaxonomyProgress[]=[];
 const inventory=normalization.inventoryTaxonomy({repoRoot:root,scope:sourcePath,workers:1,progress:event=>progress.push(event)}),entry=inventory.entries.find(entry=>entry.sourcePath===sourcePath)!;
 expect(entry.normalizedPath).toBe(sourcePath);expect(entry.violations.filter(row=>row.code==="owner-leaf-authority-invalid")).toEqual([]);expect(progress.length).toBeGreaterThan(0);
 const baseline=execFileSync("git",["rev-parse","HEAD"],{cwd:root,encoding:"utf8"}).trim(),plan=normalization.planTaxonomy(inventory,{baselineCommit:baseline,excludedTreeDigests:[],progress:event=>progress.push(event)});expect(normalization.parseTaxonomyPlan(plan)).toEqual(plan);expect(plan.evidenceRemovals).toEqual([]);expect(plan.regenerations).toEqual([]);const {planDigest,...digestible}=plan;expect(planDigest).toBe(sha(stableStringify(digestible)));const invalid={...plan,evidenceRemovals:[{operationId:"0".repeat(24),sourcePath:"evidence/CON.ts",preimage:{nodeKind:"file",contentHash:"0".repeat(64),mode:420,size:0},authority:examples.retiredSentinelAuthority,rationaleRule:"redundant-ticket-evidence-v1",ownerId:"test"}]};expect(()=>normalization.parseTaxonomyPlan(invalid)).toThrow(/kind is invalid/u);
 const artifact=process.env.SEMIO_TEST_ARTIFACT_DIR!;const directory=mkdtempSync(join(artifact,"oc-")),cancel=join(directory,"cancel.json");writeFileSync(cancel,"{}\n");try{expect(()=>normalization.inventoryTaxonomy({repoRoot:root,scope:sourcePath,cancelFile:cancel,workers:1})).toThrow(/cancel/iu);}finally{rmSync(directory,{recursive:true,force:true});}
},30_000);

for(const row of examples.approvalCases)test("current source approval "+row.id,()=>{
 const catalog=policy(2),identity={authorityCatalogPath:"🖼️assets/policy.json",authorityCatalogSha256:sha(JSON.stringify(catalog))},owner=catalog.entries[row.owner]!;
 const document:any={schemaVersion:1,kind:"current-source-approval",sourcePath:owner.sourcePath,policy:{path:identity.authorityCatalogPath,sha256:identity.authorityCatalogSha256},baseline:{commit:"1".repeat(40),blob:"2".repeat(40),preimage:owner.preimage},currentPreimage:tuple(examples.bodies.current)};
 if("policyDrift" in row)document.policy.sha256="0".repeat(64);const bytes=Buffer.from(JSON.stringify(document)),approvalPath="🖼️assets/approval.json";
 const declaration:any={sourcePath:owner.sourcePath,baselineCommit:document.baseline.commit,baselineBlob:document.baseline.blob,baselinePreimage:owner.preimage,currentPreimage:document.currentPreimage,approvalPath,approvalSha256:sha(bytes)},registry:any={reviewed:declaration};
 if("ordinal" in row)declaration.catalogCaseIndex=31;if("duplicate" in row)registry.duplicate=structuredClone(declaration);if("reverse" in row)catalog.entries.reverse();
 const facts:any={path:owner.sourcePath,nodeKind:"file",contentHash:"sourceDrift" in row?"0".repeat(64):declaration.currentPreimage.sha256,mode:0o644,size:declaration.currentPreimage.size,expectations:[{path:approvalPath,nodeKind:"nodeKind" in row&&row.nodeKind!==undefined?row.nodeKind:"file",mode:0o644,ancestorNodeKinds:["parentKind" in row?row.parentKind:"directory"],bytes:"digestDrift" in row?Buffer.from(bytes.toString()+" "):bytes}]};
 if("missingOwner" in row)catalog.entries=catalog.entries.filter(entry=>entry.sourcePath!==owner.sourcePath);
 const actual=discovery.semanticExactOwnedFileCurrentPreimageAuthority(catalog as any,identity,registry,facts);if(row.disposition!==undefined)expect<typeof row.disposition>(actual.disposition).toBe(row.disposition);expect(actual.disposition==="revised").toBe(row.accepted);
 if(row.accepted){expect((actual as any).sourcePath).toBe(owner.sourcePath);expect(actual.revisionDigest).toHaveLength(64);expect(validateApprovals(registry)).toBe(true);const independent=applyPatch(catalog.entries,[],true).newDocument.find(entry=>entry.sourcePath===owner.sourcePath);expect(independent).toEqual(owner);expect(stableStringify(document)).toBe(stableStringify(JSON.parse(bytes.toString())));}
});

for(const row of examples.projectionCases)test("owner projection Source "+row.id,()=>{
 const root=resolve(library,"../../../../.."),taxonomy=discovery.loadTaxonomy(),catalog=discovery.semanticExactOwnedFileCatalog(root,taxonomy)!,entry=catalog.entries.find(entry=>row.owner==="fixed"?entry.disposition==="fixed":row.owner==="generated"?entry.generatorOwnerId!==null:entry.disposition==="owner-documentation-relocate"&&entry.generatorOwnerId===null)!;
 const facts={path:"unregistered" in row?"unregistered/README.md":"canonical" in row?entry.destinationPath:entry.sourcePath,nodeKind:"nodeKind" in row&&row.nodeKind!==undefined?row.nodeKind:"file",contentHash:"hashDrift" in row?"0".repeat(64):entry.preimage.sha256,mode:"modeDrift" in row?0o755:Number.parseInt(entry.preimage.mode,8),size:entry.preimage.size+Number("sizeDrift" in row),sourcePresent:!("canonical" in row),destinationPresent:"destination" in row||"canonical" in row,destinationPreimage:"convergent" in row?{contentHash:entry.preimage.sha256,mode:Number.parseInt(entry.preimage.mode,8),size:entry.preimage.size}:undefined,occupiedPaths:"collision" in row?[entry.destinationPath.toUpperCase()]:[]};
 const actual=discovery.semanticExactOwnedFileProjectionAuthority(catalog,facts),sql=new Database(":memory:");try{const independent=sql.query("SELECT CASE WHEN ? THEN 'none' WHEN ? THEN 'problem' WHEN ? THEN 'fixed' WHEN ? THEN CASE WHEN ? THEN 'regenerate' ELSE 'project' END ELSE 'canonical' END AS disposition").get(Number("unregistered" in row),Number(facts.nodeKind!=="file"||facts.sourcePresent&&(facts.contentHash!==entry.preimage.sha256||facts.mode!==Number.parseInt(entry.preimage.mode,8)||facts.size!==entry.preimage.size)||entry.disposition!=="fixed"&&facts.sourcePresent&&facts.destinationPresent&&!("convergent" in row)||"collision" in row),Number(entry.disposition==="fixed"),Number(facts.path===entry.sourcePath),Number(entry.generatorOwnerId!==null)) as {disposition:string};expect(independent.disposition).toBe(row.expected);expect<typeof independent.disposition>(actual.disposition).toBe(independent.disposition);}finally{sql.close();}
});
