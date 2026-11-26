import {expect,test} from "bun:test";
import {readdirSync} from "node:fs";
import {resolve} from "node:path";
import {admitCompositionContributionV1} from "../../📋️contract/🟦️.ts";
const root=resolve(import.meta.dir,"../../../../../..");
const hub=resolve(root,"🌎️hub/🧩️compositions/🗄️stdio");
const artifacts=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
const read=(path:string)=>Bun.file(path).text();
const rows=async()=>Promise.all(readdirSync(artifacts,{withFileTypes:true}).filter(entry=>entry.isDirectory()).map(async entry=>{const file=Bun.file(resolve(artifacts,entry.name,"🧩️composition/🔣️.json"));return await file.exists()?admitCompositionContributionV1(JSON.parse(await file.text())):undefined;})).then(all=>all.filter(row=>row!==undefined));
test("every runtime-hosted artifact is a definition-only catalog contribution and its host hosts the runtime with its own catalog topic",async()=>{
 const hosted=(await rows()).filter(row=>row.runtimeHost!==undefined);
 expect(hosted.map(row=>[row.artifact,row.runtimeHost]).sort()).toEqual([["ifc","stdio-bim"],["semio","stdio-semio"]]);
 const catalog=await read(resolve(hub,"🤖️generated/🧩️composition/🦀️.rs"));
 const hosts=catalog.match(/fn selected_runtime_hosts\(\)[^\n]*/)![0];
 const selected=catalog.match(/fn selected_contributions\(\)[^\n]*/)![0];
 const runtime=catalog.match(/fn selected_runtime_contributions\(\)[^\n]*/)![0];
 const owners=JSON.parse(await read(resolve(hub,"🧩️composition/🔣️.json"))).owners;
 for(const row of hosted){
  const crate=row.package.replaceAll("-","_"),catalogCrate=row.catalogPackage!.replaceAll("-","_");
  expect(hosts).toContain(`(${JSON.stringify(row.artifact)}, ${JSON.stringify(row.runtimeHost)})`);
  expect(selected).toContain(`definition_only_assembly(${JSON.stringify(row.artifact)}, ${crate}::definition()?)`);
  expect(selected).not.toContain(catalogCrate+"::contribution()");
  expect(runtime).toContain(catalogCrate+"::contribution()");
  const owner=owners.find((owner:any)=>owner.id===row.runtimeHost);expect(owner.main).toBe(false);
  const plugin=await read(resolve(hub,owner.root));
  expect(plugin).toContain(`runtime_host_artifact(${catalogCrate}::assembly()?.into_runtime()?)`);
  expect(plugin).toContain(`hosted_artifact_catalog_contribution_for(${JSON.stringify(row.runtimeHost)}, ${JSON.stringify("semio:"+row.runtimeHost)}`);
  const catalogManifest=await read(resolve(hub,"🔌️plugin/📇️catalog/📦️packages/🦀️rust/Cargo.toml"));
  expect(catalogManifest).toContain(`${row.package} = { workspace = true }`);
 }
});
test("a runtime host must be a declared deployable owner that owns every app of the artifact",()=>{
 const row=JSON.parse(JSON.stringify({schema:"semio.stdio.composition-contribution/v1",artifact:"alpha",package:"neutral-alpha",order:0,selections:["full"],apps:[],nativeFactories:[],openTargets:[],catalogPackage:"neutral-alpha-catalog",symbolPackages:[],playgrounds:[],mutationDescriptors:[],mutationCoordinates:[]}));
 expect(admitCompositionContributionV1({...row,runtimeHost:"documents"}).runtimeHost).toBe("documents");
 expect(()=>admitCompositionContributionV1({...row,runtimeHost:"documents",catalogPackage:null})).toThrow();
 expect(()=>admitCompositionContributionV1({...row,runtimeHost:"Not A Host"})).toThrow();
});
