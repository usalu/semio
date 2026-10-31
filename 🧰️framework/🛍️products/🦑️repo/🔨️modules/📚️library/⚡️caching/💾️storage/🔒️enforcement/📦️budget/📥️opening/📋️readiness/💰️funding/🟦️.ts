import{lstatSync}from"node:fs";
import{join}from"node:path";
import schema from"../../📤️publication/⚙️setup/🧬️schema.json"with{type:"json"};
const fundingProtocol=schema.definitions.MarkerPublicationProtocol.const;
/** 🚫️ Keeps ordinary dependency readers closed until canonical funding metadata publication has fully settled. */
export function requireRepositoryDependencyFundingSettledV1(directory:string,check:()=>void):void{check();const path=join(directory,fundingProtocol.name);if(path.length>256)throw Error("Bounded dependency funding metadata path required");try{lstatSync(path);}catch(error){check();if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}check();throw Error("Pending Dependencies funding publication requires canonical setup all recovery");}
