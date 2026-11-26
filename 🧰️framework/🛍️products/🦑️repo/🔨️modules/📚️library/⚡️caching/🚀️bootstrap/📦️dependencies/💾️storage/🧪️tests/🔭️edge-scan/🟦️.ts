/** 🔭️ Law: the walk that re-observes the completed workspace exports of a produced generation reads every directory with one `scan()` (a constant number of system calls per entry) and no longer opens each entry by `names()` followed by `entry()`. */
import {expect,test} from "bun:test";
const source=await Bun.file(new URL("../../🟦️.ts",import.meta.url)).text();

const body=(name:string)=>{const start=source.indexOf(`function ${name}`);if(start<0)throw Error(`${name} not found`);const end=source.indexOf("\n/** ",start+1);return source.slice(start,end<0?source.length:end);};

test("the workspace edge walk of a produced generation scans each directory once and opens no entry by name",()=>{
 const walk=body("publishJavascriptDependencyExportsDescriptorV1");
 expect(walk).toContain("await directory.scan()");
 expect(walk).not.toContain("directory.names()");
 expect(walk).not.toMatch(/directory\.entry\(/u);
 expect(walk).toContain("entry.dev!==device");
});
