/** 🧪 Platform-neutral benchmark storage, explicit fixture tools and caller-selected languages. */
import {mkdtempSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";

/** 🗂 A fresh temporary directory; where the platform's own temporary path is too long for a unix socket beneath it, the POSIX root `/tmp` is used instead. */
export function benchTemporaryDirectoryV1(prefix){
 const base=process.platform==="win32"||tmpdir().length<=40?tmpdir():"/tmp";
 return mkdtempSync(join(base,prefix));
}

/** 🌍 The languages a benchmark runs in; there is no default, an unnamed language is refused. */
export function benchLanguagesV1(value){
 const languages=(value??"").split(",").filter(Boolean);
 if(languages.length===0)throw new Error("Name the languages: --languages en,de.\nNenne die Sprachen: --languages en,de.");
 return languages;
}

export {readNativeFixtureV1} from "../../../🔌️ports/🧰️native-tools/🟨️.mjs";
