/** 🗂️ Mandatory IFC STEP header field admission is shared by import and export. */
import type {IfcSnapshot,IfcValue} from "../🧬️schema/📸️snapshot/🟦️.ts";
import {ifcCheckpoint,ifcIoLimit,IfcStepError,type IfcIoControl} from "./control.ts";

/** 📋️ Admit structural header fields without claiming complete Part 21 physical conformance. */
export async function validateIfcStepHeader(header:IfcSnapshot["header"],control:IfcIoControl,offset=0):Promise<void>{
 const refuse=(message:string):never=>{throw new IfcStepError("step.header",message,offset);},batch=ifcIoLimit(control.batchSize,256,"batchSize");
 const list=async(value:IfcValue|undefined):Promise<boolean>=>{if(value?.kind!=="aggregate"||!value.value.length)return false;for(let i=0;i<value.value.length;i++){if(value.value[i]?.kind!=="string")return false;if((i+1)%batch===0)await ifcCheckpoint(control,"header",i+1,value.value.length);}return true;};
 if(header.fileDescription.length!==2||header.fileDescription[1]?.kind!=="string"||!await list(header.fileDescription[0]))refuse("FILE_DESCRIPTION requires a nonempty string list and implementation level");
 const name=header.fileName;if(name.length!==7||[0,1,4,5,6].some(index=>name[index]?.kind!=="string")||!await list(name[2])||!await list(name[3]))refuse("FILE_NAME requires seven correctly typed attributes and nonempty author and organisation lists");
 const schema=header.fileSchema;if(schema.length!==1||schema[0]?.kind!=="aggregate"||schema[0].value.length!==1||schema[0].value[0]?.kind!=="string"||schema[0].value[0].value!=="IFC4X3_ADD2")throw new IfcStepError("step.schema","FILE_SCHEMA must contain exactly IFC4X3_ADD2",offset);
}
