import {dirname,resolve} from "node:path";
import {createRequire} from "node:module";
import {readFileSync} from "node:fs";
const schema=JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json",import.meta.url),"utf8"));
const key="SEMIO_OWNER_ARGUMENTS";

/** 🛡️ Admits literal process arguments against their owned envelope authority. */
function admit(value){
 if(!value||typeof value!=="object"||Array.isArray(value)||Object.keys(value).length!==schema.required.length||value.version!==schema.properties.version.const||!Array.isArray(value.arguments)||value.arguments.length>schema.properties.arguments.maxItems||value.arguments.some(argument=>typeof argument!=="string"||argument.length>schema.properties.arguments.items.maxLength||argument.includes("\u0000")))throw Error("Invalid owner argument envelope");
 return [...value.arguments];
}

/** 📤️ Encodes raw argv as environment data independently of platform shell syntax. */
export function encodeOwnerArgumentsV1(arguments_){
 const value={version:1,arguments:arguments_};admit(value);return JSON.stringify(value);
}

/** 📥️ Removes the private envelope before handing unchanged arguments to the actual child owner. */
export function consumeOwnerArgumentsV1(environment){
 const next={...environment},encoded=next[key];delete next[key];
 if(encoded===undefined)return {arguments:[],environment:next};
 return {arguments:admit(JSON.parse(encoded)),environment:next};
}

/** 🎯️ Keeps Nx process handling while its static owner command receives no dynamic shell interpolation. */
export async function executeOwnerArgumentsV1(options,context,delegate){
 if(typeof options.command!=="string"||options.commands!==undefined||options.env?.[key]!==undefined||/\{args[.}]/u.test(options.command))throw Error("Owner executor requires one declared command and a private argv envelope");
 if(options.args!==undefined&&!Array.isArray(options.args))throw Error("Owner executor requires literal argument arrays");
 if(options.forwardAllArgs!==false&&!Array.isArray(options.__unparsed__))throw Error("Nx did not admit the original forwarded arguments");
 const arguments_=options.forwardAllArgs===false?[]:[...options.args??[],...options.__unparsed__??[]];
 const plan={...options,args:undefined,__unparsed__:[],forwardAllArgs:false,env:{...options.env,[key]:encodeOwnerArgumentsV1(arguments_)}};
 return delegate(plan,context);
}

/** 🚀️ Delegates the registered native/process owner route through the current Nx lifecycle. */
export default async function executor(options,context){
 const require=createRequire(import.meta.url),registryPath=require.resolve("nx/executors.json"),registry=JSON.parse(readFileSync(registryPath,"utf8")),implementation=registry.executors?.["run-commands"]?.implementation;
 if(typeof implementation!=="string")throw Error("Nx has no registered process executor");
 const delegate=require(resolve(dirname(registryPath),implementation)).default;
 return executeOwnerArgumentsV1(options,context,delegate);
}
