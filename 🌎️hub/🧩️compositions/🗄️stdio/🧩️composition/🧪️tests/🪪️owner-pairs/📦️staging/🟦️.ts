import {expect,test} from "bun:test";
import {resolve} from "node:path";
import fixture from "../🔣️.json";
const root=resolve(import.meta.dir,"../../../../../../..");
for(const row of fixture.rows)test(row.artifact+" fresh component publishes both actual roles and source schema",async()=>{
 const path=resolve(root,"🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions",row.family,"🧩️component/🔣️.json");
 const descriptor=await Bun.file(path).json();expect(descriptor.manifest.pluginId).toBe(row.owner);
 for(const role of row.roles){const apps=descriptor.manifest.apps.filter((app:any)=>app.id===role.app);expect(apps.length).toBe(1);expect(apps[0].role).toBe(role.role);expect(apps[0].io.artifactSchema).toBe(row.documentSchema);}
});
