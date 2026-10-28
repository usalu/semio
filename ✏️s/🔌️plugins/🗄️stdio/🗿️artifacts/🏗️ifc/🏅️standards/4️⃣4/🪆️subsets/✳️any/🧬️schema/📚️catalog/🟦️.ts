import data from "./🔣️.json" with {type:"json"};
import {ifcDeclaration,ifcAttributes,ifcSubtype,ifcAttribute,ifcResolveType,ifcEnumMembers,ifcSelectMembers,ifcSelectAccepts,type IfcCatalog,type IfcTypeExpression} from "../../../../../../📚️catalog/🟦️.ts";
export type {IfcCatalog,IfcDeclaration,IfcEntityDeclaration,IfcAttributeDeclaration,IfcTypeDeclaration,IfcTypeExpression} from "../../../../../../📚️catalog/🟦️.ts";
/** 🏗️ Complete normative IFC4 ADD2 TC1 declarations with exact retained source identity. */
export const ifc4Catalog=data as IfcCatalog;
export const ifc4Declaration=(name:string)=>ifcDeclaration(ifc4Catalog,name);
export const ifc4AttributeDeclarations=(name:string)=>ifcAttributes(ifc4Catalog,name);
export const ifc4IsSubtype=(actual:string,expected:string):boolean=>ifcSubtype(ifc4Catalog,actual,expected);
export const ifc4Attribute=(entity:string,index:number)=>ifcAttribute(ifc4Catalog,entity,index);
export const ifc4ResolveType=(expression:IfcTypeExpression)=>ifcResolveType(ifc4Catalog,expression);
export const ifc4EnumMembers=(name:string)=>ifcEnumMembers(ifc4Catalog,name);
export const ifc4SelectMembers=(name:string)=>ifcSelectMembers(ifc4Catalog,name);
export const ifc4SelectAccepts=(select:string,actual:string):boolean=>ifcSelectAccepts(ifc4Catalog,select,actual);
