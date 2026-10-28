/** 🪞️ A test that loads a copy of the revision reader needs the modules it imports at the same relative places: this lays the churn policy out under a scratch root exactly as the repository does, and returns the directory the copy of the reader belongs in. */
import {copyFileSync,mkdirSync} from "node:fs";
import {dirname,join,relative} from "node:path";
import {fileURLToPath} from "node:url";

const framework=fileURLToPath(new URL("../../../../../../../../../",import.meta.url));
const reader=fileURLToPath(new URL("../../",import.meta.url));
const churn=join(framework,"🔨️modules","📁️filesystem","🧾️observation","🔁️churn");

/** 📂️ The directory inside `root` that mirrors the reader's own place in the repository; the churn policy, the race table it consults and their schemas are copied beside it first. */
export function mirrorRevisionReaderDirectoryV1(root:string):string{
 for(const path of [join(churn,"🟦️.ts"),join(churn,"🧬️schema","🔣️.json"),join(churn,"🌊️races","🟦️.ts"),join(churn,"🧬️schema","🌊️races.json")]){const copy=join(root,relative(framework,path));mkdirSync(dirname(copy),{recursive:true});copyFileSync(path,copy);}
 const directory=join(root,relative(framework,reader));mkdirSync(directory,{recursive:true});return directory;
}
