import {isAbsolute,join,normalize,sep} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🏠️ The authored protocol of the machine state root. @see ./🧬️schema/🔣️.json */
export const MACHINE_STATE_PROTOCOL_V1=schema.definitions.Protocol.const;
const P=MACHINE_STATE_PROTOCOL_V1;

/** 🗂️ The directory a machine keeps its own state in when it declares one (`SEMIO_MACHINE_STATE_DIRECTORY`: an absolute path that stands for `<checkout>/.cache`), else `undefined`: the state stays in the checkout. A relative, empty or overlong declaration is no declaration. */
export function machineStateDirectoryV1(environment:Readonly<Record<string,string|undefined>>=process.env):string|undefined{
 const named=environment[P.environment];
 return named!==undefined&&named!==""&&isAbsolute(named)&&named.length<=P.maximumDirectoryCharacters?normalize(named):undefined;
}

/** 🧭️ `join(root,...segments)` with the machine state moved (a root inside the declared directory, such as a private test repository, keeps its own state): a path below `<root>/.cache` (or `<root>/.nx`) goes below the declared state directory instead, every other path stays below the root. It is the only place that knows where a machine keeps what it writes; without a declaration it is exactly `join`. @see https://nodejs.org/api/path.html#pathjoinpaths */
export function machinePathV1(root:string,...segments:string[]):string{
 const directory=machineStateDirectoryV1();
 if(directory===undefined||normalize(root)===directory.replace(/[\\/]+$/,"")||normalize(root).startsWith(directory.replace(/[\\/]+$/,"")+sep))return join(root,...segments);
 const parts=join(".",...segments).split(sep),moved=P.moved.find(row=>row.prefix===parts[0]);
 return moved===undefined?join(root,...segments):join(directory,...moved.to,...parts.slice(1));
}
