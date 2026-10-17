/** 🌍️ Consumer-owned GIS receiver wiring. */
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
test("GIS receiving schema compilers select the canonical framework schema oracle and import no plugin",()=>{
  const owner=new URL("../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts",import.meta.url);
  for(const name of ["🌉️component-cold-map-patch","💡️inference-control","📇️native-codecs"]){
    const path=new URL(`../${name}/🟦️.ts`,import.meta.url),source=readFileSync(path,"utf8");
    for(const link of source.matchAll(/from "(\.[^"]+)"/g)){expect(()=>readFileSync(new URL(link[1]!,path))).not.toThrow();expect(link[1]!).not.toContain("✏️s/");}
    const match=source.match(/import \{ compileSchemaScopeExport \} from "([^"]+)"/);
    if(name==="📇️native-codecs"){expect(match).not.toBeNull();expect(new URL(match![1]!,path).href).toBe(owner.href);}else expect(match).toBeNull();
  }
});
