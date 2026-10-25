import fs from "node:fs";
import {syncBuiltinESMExports} from "node:module";
import {fileURLToPath} from "node:url";
const readFile=fs.readFileSync,openFile=fs.openSync,statFile=fs.lstatSync,named=input=>input instanceof URL?fileURLToPath(input):String(input);
fs.readFileSync=(input,...options)=>{
 const bytes=readFile(input,...options),target=globalThis.semioSourceMutationV1;
 if(target===undefined||named(input)!==target.path)return bytes;
 return Buffer.isBuffer(bytes)?Buffer.concat([bytes,Buffer.from(target.mutation)]):bytes+target.mutation;
};
fs.openSync=(input,...options)=>{
 const target=globalThis.semioSourceMutationV1;
 return openFile(target!==undefined&&named(input)===target.path?target.copy:input,...options);
};
fs.lstatSync=(input,...options)=>{
 const target=globalThis.semioSourceMutationV1;
 return statFile(target!==undefined&&typeof input!=="number"&&named(input)===target.path?target.copy:input,...options);
};
syncBuiltinESMExports();
