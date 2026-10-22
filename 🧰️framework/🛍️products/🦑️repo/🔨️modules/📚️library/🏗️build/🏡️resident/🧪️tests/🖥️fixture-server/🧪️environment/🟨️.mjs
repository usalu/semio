/** 🧪 Where the benchmarks keep their files and find the native tools, on every platform: the process temporary directory, the directory named by the caller and no path of any machine. */
import {mkdtempSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";

/** 🗂 A fresh temporary directory; where the platform's own temporary path is too long for a unix socket beneath it, the POSIX root `/tmp` is used instead. */
export function benchTemporaryDirectoryV1(prefix){
 const base=process.platform==="win32"||tmpdir().length<=40?tmpdir():"/tmp";
 return mkdtempSync(join(base,prefix));
}

/** 🧰 The directory with the native tools: `--bin`, else `SEMIO_REPO_NATIVE_DIR`; refuses when neither names one. */
export function nativeDirectoryV1(flagged){
 const directory=flagged??process.env.SEMIO_REPO_NATIVE_DIR;
 if(!directory)throw new Error("Name the directory with the native tools: --bin <directory> or SEMIO_REPO_NATIVE_DIR.\nNenne das Verzeichnis mit den nativen Werkzeugen: --bin <Verzeichnis> oder SEMIO_REPO_NATIVE_DIR.");
 return directory;
}

/** 🔧 The path of one native tool inside that directory, with the platform's executable suffix. */
export function nativeToolV1(directory,name){return join(directory,name+(process.platform==="win32"?".exe":""));}

/** 🌍 The languages a benchmark runs in; there is no default, an unnamed language is refused. */
export function benchLanguagesV1(value){
 const languages=(value??"").split(",").filter(Boolean);
 if(languages.length===0)throw new Error("Name the languages: --languages en,de.\nNenne die Sprachen: --languages en,de.");
 return languages;
}
