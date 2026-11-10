import {mkdtempSync,readFileSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";

/** 🧱️ The line between two joined modules: a function that ends its module is delimited by it. */
export const GRAPH_SOURCE_BOUNDARY_V1="/** 🧱️ module boundary */";

/** 📖️ The graph entry and the cohesive modules it re-exports, in one text, for tests that assert on the source of a graph function wherever the split put it. */
export const GRAPH_SOURCE_MODULES_V1=Object.freeze(["🔐️authority","📜️contract","🛑️custody","🕊️release","📁️workspace","🧭️discovery","📍️pin","🏷️retention","🕰️host","🤝️prefetch","📥️load","🎯️resolve"]);

/** 📖️ Reads the whole graph source: the entry followed by every split module. */
export function graphSourceV1():string{
  const graph=fileURLToPath(new URL("../../",import.meta.url));
  return [graph+"🟦️.ts",...GRAPH_SOURCE_MODULES_V1.map(name=>graph+name+"/🟦️.ts")].map(path=>readFileSync(path,"utf8")).join("\n"+GRAPH_SOURCE_BOUNDARY_V1+"\n");
}

/** 📖️ Writes the whole graph source to one file under the system temporary directory, for independent oracles that read the source by path. */
export function graphSourcePathV1():string{
  const path=join(mkdtempSync(join(tmpdir(),"graph-source-")),"graph.ts");
  writeFileSync(path,graphSourceV1());
  return path;
}
