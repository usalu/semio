/** 🔗️ Relative import closure: reads the files a bootstrap entry reaches through relative specifiers and the workspace package specifiers among them, so the entry can be held to boot without `node_modules`.
 * @see ../🏗️construction/🟨️.cjs — the `no-cross-package-relative` rules that make a relative import legal only on an authored export target
 */
import {existsSync,readFileSync} from "node:fs";
import {dirname,relative,resolve} from "node:path";

export type RelativeClosureV1=Readonly<{files:readonly string[];workspaceSpecifiers:readonly string[]}>;

const specifier=/(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s*|\brequire\s*\(\s*)(["'])([^"'\n]+)\1/gu;
const sources=/\.(?:[cm]?[jt]s|[jt]sx)$/u;
const candidates=(base:string):string[]=>[base,`${base}.ts`,`${base}.tsx`,`${base}/index.ts`,base.replace(/\.js$/u,".ts")];

/** 🔎️ Returns every source file reachable from `entry` through relative specifiers, and every `@semio-tech/` specifier those files name. */
export function readRelativeClosureV1(root:string,entry:string):RelativeClosureV1{
  const seen=new Set<string>(),bare=new Set<string>(),queue=[resolve(root,entry)];
  while(queue.length){
    const file=queue.pop()!;
    if(seen.has(file)||!existsSync(file))continue;
    seen.add(file);
    for(const match of readFileSync(file,"utf8").matchAll(specifier)){
      const text=match[2]!;
      if(text.startsWith("@semio-tech/"))bare.add(`${relative(root,file)} -> ${text}`);
      if(!text.startsWith("."))continue;
      const target=candidates(resolve(dirname(file),text.split("?")[0]!)).find(path=>sources.test(path)&&existsSync(path));
      if(target)queue.push(target);
    }
  }
  return {files:[...seen].map(path=>relative(root,path)).sort(),workspaceSpecifiers:[...bare].sort()};
}
