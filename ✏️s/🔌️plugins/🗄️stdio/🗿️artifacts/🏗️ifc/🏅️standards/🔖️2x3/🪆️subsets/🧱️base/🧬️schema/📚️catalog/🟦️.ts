import data from "./🔣️.json" with {type:"json"};
import {ifcDeclaration,ifcAttributes,ifcSubtype,ifcAttribute,ifcResolveType,ifcEnumMembers,ifcSelectMembers,ifcSelectAccepts,type IfcCatalog,type IfcTypeExpression} from "../../../../../../📚️catalog/🟦️.ts";
export type {IfcCatalog,IfcDeclaration,IfcEntityDeclaration,IfcAttributeDeclaration,IfcTypeDeclaration,IfcTypeExpression} from "../../../../../../📚️catalog/🟦️.ts";
/** 🏗️ Complete normative IFC2x3 TC1 declarations with exact retained source identity. */
export const ifc2x3Catalog=data as IfcCatalog;
export const ifc2x3Declaration=(name:string)=>ifcDeclaration(ifc2x3Catalog,name);
export const ifc2x3AttributeDeclarations=(name:string)=>ifcAttributes(ifc2x3Catalog,name);
export const ifc2x3IsSubtype=(actual:string,expected:string):boolean=>ifcSubtype(ifc2x3Catalog,actual,expected);
export const ifc2x3Attribute=(entity:string,index:number)=>ifcAttribute(ifc2x3Catalog,entity,index);
export const ifc2x3ResolveType=(expression:IfcTypeExpression)=>ifcResolveType(ifc2x3Catalog,expression);
export const ifc2x3EnumMembers=(name:string)=>ifcEnumMembers(ifc2x3Catalog,name);
export const ifc2x3SelectMembers=(name:string)=>ifcSelectMembers(ifc2x3Catalog,name);
export const ifc2x3SelectAccepts=(select:string,actual:string):boolean=>ifcSelectAccepts(ifc2x3Catalog,select,actual);
