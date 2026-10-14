import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {specificCatalogPublicationV1} from "../🟦️.ts";
import {composeSpecificOsCatalogV1} from "../../🟦️.ts";
import {readGeneratedCatalogProjection} from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import {parse as parseJsonc} from "jsonc-parser";
test("the actual S consumer admits only its explicit owner publication",()=>{
 const request=specificCatalogPublicationV1(),root=fileURLToPath(new URL("../../🤖️generated/",import.meta.url)),projection=readGeneratedCatalogProjection(root);
 expect(request.contributions.map(row=>row.ownerRoot)).toEqual(["✏️s","🌎️hub"]);
 const supplied=composeSpecificOsCatalogV1("https://composition.example/",{maxBytes:2097152,maxRows: 512,maxEdges:4096,maxWork:65536,deadlineMs:Date.now()+30000,now:()=>Date.now(),cancelled:()=>false,progress:()=>{}});
 expect(supplied.rows.targets.map(row=>row.pluginId).sort()).toEqual(projection.entries.map(row=>row.pluginId).sort());
 expect(projection.entries.length).toBeGreaterThan(0);expect(projection.playgrounds.length).toBeGreaterThan(0);
 for(const row of projection.entries)expect(request.contributions.some(owner=>owner.packageRoots.some(root=>row.cratePath.startsWith(root+"/")))).toBe(true);
 console.log("[DEBUG] actual S catalog consumer",{targets:supplied.rows.targets.length,playgrounds:projection.playgrounds.length,hosts:supplied.rows.hosts.length});
});
test("the exact canonical publication routes survive both launch sources",()=>{
 const root=new URL("../../../../../",import.meta.url),seed=parseJsonc(readFileSync(new URL(".vscode/🧩️launch.seed.jsonc",root),"utf8")),launch=parseJsonc(readFileSync(new URL(".vscode/launch.json",root),"utf8"));
 for(const target of ["generate","check-generated","preview-generated","verify-catalog","test-catalog-publication"]){const command="bun repo run @semio-tech/s-dev-composition:"+target,authored=seed.configurations.filter((row:{command:string})=>row.command===command),published=launch.configurations.filter((row:{command:string})=>row.command===command);expect(authored.length).toBe(1);expect(published).toEqual(authored);}
});

import {catalogNxContributionV1} from "../🟦️.ts";
import {cacheInternals,libraryBootstrap} from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs";
test("the actual Nx session producer binds S authority and explicit installed inventory",async()=>{
 await libraryBootstrap;
 const workspace=fileURLToPath(new URL("../../../../../",import.meta.url)),root=fileURLToPath(new URL("../../🤖️generated/",import.meta.url)),sources=JSON.parse(readFileSync(new URL("🗂️sources.json",new URL("../../🤖️generated/",import.meta.url)),"utf8")),manifests=sources.map((row:{cratePath:string})=>row.cratePath+"/Cargo.toml");
 expect(await cacheInternals.workspaceCatalogNxContributionsV1(workspace)).toEqual([catalogNxContributionV1()]);
 const sessions=await cacheInternals.playgroundSessionTargets(manifests,workspace,[catalogNxContributionV1()]);
 expect(Object.keys(sessions).length).toBeGreaterThan(0);
 for(const target of Object.values(sessions) as {dependsOn:string[];options:{env:{SEMIO_PLUGIN_CATALOG:string}}}[]){expect(target.dependsOn).toEqual(["@semio-tech/s-dev-composition:generate"]);expect(target.options.env.SEMIO_PLUGIN_CATALOG).toBe(root.replace(/\/$/u,""));}
 expect(await cacheInternals.playgroundSessionTargets(manifests,"owner-deleted",[])).toEqual({});
 expect(cacheInternals.playgroundPreparationTargets(manifests,"owner-deleted","general",{},[])).toEqual({});
 expect(cacheInternals.collectPlaygroundCatalog(manifests,"owner-deleted",[])).toEqual([]);
 console.log("[DEBUG] actual Nx session publication",{sessions:Object.keys(sessions).length,emptyProviders:0});
});
