/** 🪡 Preserve rectilinear silhouette semantics through subpixel snapping. */
import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import path from "node:path";
import {pathToFileURL} from "node:url";

const root=process.cwd(), c=JSON.parse(await readFile(new URL("./🔣️.json",import.meta.url),"utf8"));
const silhouette=await import(pathToFileURL(path.join(root,"🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowSilhouette/shared-border.ts")).href);
const border=await import(pathToFileURL(path.join(root,"🧰️framework/🔨️modules/🖱️ui/🔨️modules/📏️border-presentation/🟦️.ts")).href);
const transform={left:0,top:0,scaleX:1,scaleY:1};
const lines=silhouette.sharedOutlineSegments("source",c.outline,c.outline,"normal",transform);
assert.deepEqual([lines[0].from.x,lines[0].from.y],c.expected.start);
assert.deepEqual([lines[0].to.x,lines[0].to.y],c.expected.end);
assert.doesNotThrow(()=>border.resolveBorderSegments(lines));
const invalid=silhouette.sharedOutlineSegments("invalid",c.diagonal,c.diagonal,"normal",transform);
assert.throws(()=>border.resolveBorderSegments(invalid),/axis-aligned/);
console.log("PASS axis-aligned snapping");
