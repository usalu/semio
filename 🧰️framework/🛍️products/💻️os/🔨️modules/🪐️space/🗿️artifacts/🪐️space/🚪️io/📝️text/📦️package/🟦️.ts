import {admitSpacePackageDeclaration,type SpaceArtifactPackage,type SpacePackageDeclaration} from "./../../../🧬️schema/📦️package/🟦️.ts";
import {decodeJsonSyntax,type JsonSyntaxControl,type JsonSyntaxNode} from "./../../../../../../../../../🔨️modules/🎒️pack/🔤️json/📥️decode/🟦️.ts";
import {JsonMemberPolicy} from "./../../../../../../../../../🔨️modules/🎒️pack/🔤️json/🧩️members/🟦️.ts";
/** 📥️ Decodes exact package fields with caller-owned limits and cancellation. */
export async function decodeSpacePackageJson(source:string,control:JsonSyntaxControl):Promise<SpaceArtifactPackage>{
 const node=await decodeJsonSyntax(source,JsonMemberPolicy.Reject,control);
 const keys=["definition_version", "id", "artifact", "directory", "rust_package", "nx_project", "dependencies"];
 if(node.kind!=="object" || node.members.length!==keys.length || node.members.some(member=>!keys.includes(member.name))) throw Error("Invalid package fields");
 const values=new Map(node.members.map(member=>[member.name,member.value]));
 const text=(key:string):string=>{const value=values.get(key);if(value?.kind!=="string")throw Error("Expected package text");return value.value;};
 const version=values.get("definition_version"),dependencies=values.get("dependencies");
 if(version?.kind!=="number" || version.text!=="1" || dependencies?.kind!=="array" || dependencies.items.some(item=>item.kind!=="string"))throw Error("Invalid package declaration types");
 const declaration:SpacePackageDeclaration={definition_version:1,id:text("id"),artifact:text("artifact"),directory:text("directory"),rust_package:text("rust_package"),nx_project:text("nx_project"),dependencies:dependencies.items.map(item=>(item as Extract<JsonSyntaxNode,{kind:"string"}>).value)};
 return admitSpacePackageDeclaration(declaration);
}
