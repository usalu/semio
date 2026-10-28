/** 📐 An EXPRESS named type or nested aggregate declaration. */
export interface IfcTypeExpression {
  readonly kind: "named" | "aggregate";
  readonly name?: string;
  readonly width?: number | null;
  readonly fixed?: boolean;
  readonly aggregate?: "LIST" | "SET" | "BAG" | "ARRAY";
  readonly lower?: number | null;
  readonly upper?: number | null;
  readonly lowerExpression?: string;
  readonly upperExpression?: string;
  readonly unique?: boolean;
  readonly optional?: boolean;
  readonly item?: IfcTypeExpression;
}
/** 🏷️ A positional explicit attribute, including inherited derived redeclarations. */
export interface IfcAttributeDeclaration {
  readonly name: string;
  readonly typeExpr: IfcTypeExpression;
  readonly optional: boolean;
  readonly derived: boolean;
  readonly declaredBy: string;
}
/** 🧩 An EXPRESS entity declaration and its serialized attribute order. */
export interface IfcEntityDeclaration {
  readonly name: string;
  readonly abstractEntity: boolean;
  readonly supertype: string | null;
  readonly attributes: readonly IfcAttributeDeclaration[];
  readonly allAttributes: readonly IfcAttributeDeclaration[];
  readonly derived: readonly Readonly<{name:string;typeExpr:IfcTypeExpression;redeclares:string|null;expression:string}>[];
  readonly inverse: readonly string[];
  readonly whereRules: readonly string[];
  readonly uniqueRules: readonly string[];
  readonly supertypeExpression: string;
}
/** 🔀 An EXPRESS defined type, enumeration, or select. */
export interface IfcTypeDeclaration {
  readonly name: string;
  readonly kind: "defined" | "enum" | "select";
  readonly members: readonly string[];
  readonly typeExpr?: IfcTypeExpression;
  readonly whereRules: readonly string[];
}
/** 📚 The complete retained EXPRESS declaration catalog. */
export interface IfcCatalog {
  readonly schema: string;
  readonly version: string;
  readonly source: Readonly<{url:string;mirror:string;sha256:string}>;
  readonly validation: Readonly<{structural:boolean;whereRules:boolean;uniqueRules:boolean;inverseRules:boolean;contextualBounds:boolean;functions:boolean;globalRules:boolean;complexInstances:boolean;binaryValues:boolean}>;
  readonly entities: Readonly<Record<string,IfcEntityDeclaration>>;
  readonly types: Readonly<Record<string,IfcTypeDeclaration>>;
  readonly functions: Readonly<Record<string,string>>;
  readonly rules: Readonly<Record<string,string>>;
}

export type IfcDeclaration={kind:"entity";declaration:IfcEntityDeclaration}|{kind:"type";declaration:IfcTypeDeclaration};
/** 🔎 Read a named declaration using its schema's own namespace. */
export function ifcDeclaration(catalog:IfcCatalog,name:string):IfcDeclaration|undefined{const key=name.toUpperCase(),entity=catalog.entities[key],type=catalog.types[key];return entity?{kind:"entity",declaration:entity}:type?{kind:"type",declaration:type}:undefined;}
/** 🧬 Borrow inherited attributes in their original Part-21 positions. */
export function ifcAttributes(catalog:IfcCatalog,name:string):readonly IfcAttributeDeclaration[]|undefined{return catalog.entities[name.toUpperCase()]?.allAttributes;}
/** 🌳 Resolve declared entity inheritance with finite cycle protection. */
export function ifcSubtype(catalog:IfcCatalog,actual:string,expected:string):boolean{return catalogResult(ifcSubtypeSteps(catalog,actual,expected));}
/** 🧷 Read one serialization attribute by its stable zero-based position. */
export function ifcAttribute(catalog:IfcCatalog,entity:string,index:number):IfcAttributeDeclaration|undefined{return Number.isSafeInteger(index)&&index>=0?ifcAttributes(catalog,entity)?.[index]:undefined;}
/** 🪢 Follow defined aliases while retaining aggregate, enum and select domains. */
export function ifcResolveType(catalog:IfcCatalog,expression:IfcTypeExpression):IfcTypeExpression|undefined{return catalogResult(ifcResolveTypeSteps(catalog,expression));}
/** 🔤 Read the closed enumeration domain of a named type or alias. */
export function ifcEnumMembers(catalog:IfcCatalog,name:string):readonly string[]|undefined{const type=ifcResolveType(catalog,{kind:"named",name:name.toUpperCase()}),declaration=type?.name?catalog.types[type.name]:undefined;return declaration?.kind==="enum"?declaration.members:undefined;}
/** 🔀 Read the declared select members without flattening their semantic types. */
export function ifcSelectMembers(catalog:IfcCatalog,name:string):readonly string[]|undefined{const type=ifcResolveType(catalog,{kind:"named",name:name.toUpperCase()}),declaration=type?.name?catalog.types[type.name]:undefined;return declaration?.kind==="select"?declaration.members:undefined;}
/** 🎯 Resolve nested selects and entity subtype membership without a runtime EXPRESS parser. */
export function ifcSelectAccepts(catalog:IfcCatalog,select:string,actual:string):boolean{return catalogResult(ifcSelectAcceptsSteps(catalog,select,actual));}

export type IfcCatalogSteps<T> = Generator<void,T,void>;
function catalogResult<T>(steps:IfcCatalogSteps<T>):T{for(;;){const next=steps.next();if(next.done)return next.value;}}
/** 🔠️ Read bounded schema identifiers without copying arbitrary source text in one turn. */
export function* ifcNameSteps(input:string):IfcCatalogSteps<string|undefined>{if(typeof input!=="string"||!input.length||input.length>256)return undefined;for(let index=0;index<input.length;index++){yield;const code=input.charCodeAt(index);if(!(code>=65&&code<=90||code>=97&&code<=122||code===95||index>0&&code>=48&&code<=57))return undefined;}return input.toUpperCase();}
/** 🪜️ Suspend at every inherited declaration visited. */
export function* ifcSubtypeSteps(catalog:IfcCatalog,actual:string,expected:string):IfcCatalogSteps<boolean>{let current:string|null|undefined=yield* ifcNameSteps(actual);const target=yield* ifcNameSteps(expected),seen=new Set<string>();if(!target)return false;while(current){yield;if(seen.has(current)||!catalog.entities[current])return false;seen.add(current);if(current===target)return true;current=catalog.entities[current].supertype;}return false;}
/** 🧶️ Resolve aliases with a retained metadata frontier. */
export function* ifcResolveTypeSteps(catalog:IfcCatalog,expression:IfcTypeExpression):IfcCatalogSteps<IfcTypeExpression|undefined>{const seen=new Set<string>();let current=expression;while(current.kind==="named"){const name=yield* ifcNameSteps(current.name??"");yield;if(!name||seen.has(name))return undefined;seen.add(name);const declaration=catalog.types[name];if(!declaration||declaration.kind!=="defined")return current;if(!declaration.typeExpr)return undefined;current=declaration.typeExpr;}return current;}
/** 🔀️ Traverse nested SELECTs through the same declared membership rules. */
export function* ifcSelectAcceptsSteps(catalog:IfcCatalog,select:string,actual:string):IfcCatalogSteps<boolean>{const target=yield* ifcNameSteps(actual),seen=new Set<string>();if(!target)return false;const acceptedName:string=target;function* members(name:string):IfcCatalogSteps<readonly string[]|undefined>{const type=yield* ifcResolveTypeSteps(catalog,{kind:"named",name}),declaration=type?.name?catalog.types[type.name.toUpperCase()]:undefined;return declaration?.kind==="select"?declaration.members:undefined;}function* accepts(name:string,depth:number):IfcCatalogSteps<boolean>{yield;if(depth>128||seen.has(name))return false;seen.add(name);if(name===acceptedName)return Boolean(catalog.entities[name]||catalog.types[name]);if(yield* ifcSubtypeSteps(catalog,acceptedName,name))return true;for(const member of (yield* members(name))??[])if(yield* accepts(member,depth+1))return true;return false;}for(const member of (yield* members(select))??[])if(yield* accepts(member,0))return true;return false;}
