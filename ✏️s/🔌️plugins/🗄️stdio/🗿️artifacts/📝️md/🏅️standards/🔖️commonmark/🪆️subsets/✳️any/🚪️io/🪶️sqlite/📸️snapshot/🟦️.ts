/** 📝️ Explicit CommonMark domain and ownership tables. */
import type {MdSnapshot,MdBlock,MdInline} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import type {SqliteDatabase,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type { ArtifactDialect } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteCheckpoint,artifactSqliteTables,artifactSqliteInteger,artifactSqliteText,artifactSqliteBoolean,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
/** 🏛️ The same individually authored CommonMark SQL declaration as the native owner. */
export const MD_SQLITE_SCHEMA=String.raw`CREATE TABLE md_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL);
CREATE TABLE md_block (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK (kind IN ('heading','paragraph','list','codeBlock','blockQuote','thematicBreak','htmlBlock')));
CREATE TABLE md_document_block (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES md_document(id), ordinal INTEGER NOT NULL CHECK (ordinal>=0), block_id INTEGER NOT NULL REFERENCES md_block(id));
CREATE TABLE md_heading (id INTEGER PRIMARY KEY REFERENCES md_block(id), level INTEGER NOT NULL CHECK (level BETWEEN 0 AND 255));
CREATE TABLE md_paragraph (id INTEGER PRIMARY KEY REFERENCES md_block(id));
CREATE TABLE md_list (id INTEGER PRIMARY KEY REFERENCES md_block(id), ordered INTEGER NOT NULL CHECK (ordered IN (0,1)), start INTEGER CHECK (start BETWEEN 0 AND 4294967295), tight INTEGER NOT NULL CHECK (tight IN (0,1)));
CREATE TABLE md_list_item (id INTEGER PRIMARY KEY, list_id INTEGER NOT NULL REFERENCES md_list(id), ordinal INTEGER NOT NULL CHECK (ordinal>=0));
CREATE TABLE md_list_item_block (id INTEGER PRIMARY KEY, item_id INTEGER NOT NULL REFERENCES md_list_item(id), ordinal INTEGER NOT NULL CHECK (ordinal>=0), block_id INTEGER NOT NULL REFERENCES md_block(id));
CREATE TABLE md_block_quote (id INTEGER PRIMARY KEY REFERENCES md_block(id));
CREATE TABLE md_quote_block (id INTEGER PRIMARY KEY, quote_id INTEGER NOT NULL REFERENCES md_block_quote(id), ordinal INTEGER NOT NULL CHECK (ordinal>=0), block_id INTEGER NOT NULL REFERENCES md_block(id));
CREATE TABLE md_code_block (id INTEGER PRIMARY KEY REFERENCES md_block(id), info TEXT, literal TEXT NOT NULL);
CREATE TABLE md_thematic_break (id INTEGER PRIMARY KEY REFERENCES md_block(id));
CREATE TABLE md_html_block (id INTEGER PRIMARY KEY REFERENCES md_block(id), raw TEXT NOT NULL);
CREATE TABLE md_inline (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK (kind IN ('text','emphasis','strong','code','link','image','softBreak','hardBreak','htmlInline')));
CREATE TABLE md_block_inline (id INTEGER PRIMARY KEY, block_id INTEGER NOT NULL REFERENCES md_block(id), ordinal INTEGER NOT NULL CHECK (ordinal>=0), inline_id INTEGER NOT NULL REFERENCES md_inline(id));
CREATE TABLE md_text (id INTEGER PRIMARY KEY REFERENCES md_inline(id), text TEXT NOT NULL);
CREATE TABLE md_emphasis (id INTEGER PRIMARY KEY REFERENCES md_inline(id));
CREATE TABLE md_emphasis_inline (id INTEGER PRIMARY KEY, emphasis_id INTEGER NOT NULL REFERENCES md_emphasis(id), ordinal INTEGER NOT NULL CHECK (ordinal>=0), inline_id INTEGER NOT NULL REFERENCES md_inline(id));
CREATE TABLE md_strong (id INTEGER PRIMARY KEY REFERENCES md_inline(id));
CREATE TABLE md_strong_inline (id INTEGER PRIMARY KEY, strong_id INTEGER NOT NULL REFERENCES md_strong(id), ordinal INTEGER NOT NULL CHECK (ordinal>=0), inline_id INTEGER NOT NULL REFERENCES md_inline(id));
CREATE TABLE md_code_span (id INTEGER PRIMARY KEY REFERENCES md_inline(id), literal TEXT NOT NULL);
CREATE TABLE md_link (id INTEGER PRIMARY KEY REFERENCES md_inline(id), url TEXT NOT NULL, title TEXT);
CREATE TABLE md_link_text (id INTEGER PRIMARY KEY, link_id INTEGER NOT NULL REFERENCES md_link(id), ordinal INTEGER NOT NULL CHECK (ordinal>=0), inline_id INTEGER NOT NULL REFERENCES md_inline(id));
CREATE TABLE md_image (id INTEGER PRIMARY KEY REFERENCES md_inline(id), alt TEXT NOT NULL, url TEXT NOT NULL, title TEXT);
CREATE TABLE md_soft_break (id INTEGER PRIMARY KEY REFERENCES md_inline(id));
CREATE TABLE md_hard_break (id INTEGER PRIMARY KEY REFERENCES md_inline(id));
CREATE TABLE md_html_inline (id INTEGER PRIMARY KEY REFERENCES md_inline(id), raw TEXT NOT NULL);
`;
/** 📤️ Publishes the owned block/inline model as semantic entities. */
export async function mdSnapshotToSqliteDatabase(snapshot:MdSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const out=await ArtifactSqliteProjection.create(MD_SQLITE_SCHEMA,options);await out.insert("md_document",[snapshot.schema]);const frames:WriteFrame[]=[{kind:"blocks",values:snapshot.blocks,index:0,owner:{table:"md_document_block",id:1n}}];let steps=0;
 while(frames.length){const frame=frames.pop()!;if(steps%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",steps,0);steps++;if(frames.length>(options.maxRows??1_000_000))throw Error("CommonMark traversal exceeds row limit");if(frame.kind==="items"){if(frame.index%256===0||frame.index===frame.values.length)await artifactSqliteCheckpoint(options,"projectSnapshot",frame.index,frame.values.length);const blocks=frame.values[frame.index];if(!blocks)continue;frames.push({...frame,index:frame.index+1});const id=await out.insert("md_list_item",[frame.list,BigInt(frame.index)]);frames.push({kind:"blocks",values:blocks,index:0,owner:{table:"md_list_item_block",id}});continue;}
  if(frame.kind==="blocks"){const block=frame.values[frame.index];if(!block)continue;frames.push({...frame,index:frame.index+1});const id=await out.insert("md_block",[block.kind]);await out.insert(frame.owner.table,[frame.owner.id,BigInt(frame.index),id]);switch(block.kind){
   case "heading":await out.insert("md_heading",[word(block.level,255)],id);frames.push({kind:"inlines",values:block.inlines,index:0,owner:{table:"md_block_inline",id}});break;
   case "paragraph":await out.insert("md_paragraph",[],id);frames.push({kind:"inlines",values:block.inlines,index:0,owner:{table:"md_block_inline",id}});break;
   case "list":await out.insert("md_list",[flag(block.ordered),block.start===undefined?null:word(block.start,4294967295),flag(block.tight)],id);frames.push({kind:"items",values:block.items,index:0,list:id});break;
   case "codeBlock":await out.insert("md_code_block",[block.info??null,block.literal],id);break;
   case "blockQuote":await out.insert("md_block_quote",[],id);frames.push({kind:"blocks",values:block.blocks,index:0,owner:{table:"md_quote_block",id}});break;
   case "thematicBreak":await out.insert("md_thematic_break",[],id);break;
   case "htmlBlock":await out.insert("md_html_block",[block.raw],id);break;
   default:throw Error("CommonMark block kind is unknown");
  }continue;}
  const inline=frame.values[frame.index];if(!inline)continue;frames.push({...frame,index:frame.index+1});const id=await out.insert("md_inline",[inline.kind]);await out.insert(frame.owner.table,[frame.owner.id,BigInt(frame.index),id]);switch(inline.kind){
   case "text":await out.insert("md_text",[inline.text],id);break;
   case "emphasis":await out.insert("md_emphasis",[],id);frames.push({kind:"inlines",values:inline.inlines,index:0,owner:{table:"md_emphasis_inline",id}});break;
   case "strong":await out.insert("md_strong",[],id);frames.push({kind:"inlines",values:inline.inlines,index:0,owner:{table:"md_strong_inline",id}});break;
   case "code":await out.insert("md_code_span",[inline.literal],id);break;
   case "link":await out.insert("md_link",[inline.url,inline.title??null],id);frames.push({kind:"inlines",values:inline.text,index:0,owner:{table:"md_link_text",id}});break;
   case "image":await out.insert("md_image",[inline.alt,inline.url,inline.title??null],id);break;
   case "softBreak":await out.insert("md_soft_break",[],id);break;
   case "hardBreak":await out.insert("md_hard_break",[],id);break;
   case "htmlInline":await out.insert("md_html_inline",[inline.raw],id);break;
   default:throw Error("CommonMark inline kind is unknown");
  }
 }return out.finish();
}

type WriteFrame={kind:"blocks";values:MdBlock[];index:number;owner:{table:"md_document_block"|"md_quote_block"|"md_list_item_block";id:bigint}}|{kind:"inlines";values:MdInline[];index:number;owner:{table:"md_block_inline"|"md_emphasis_inline"|"md_strong_inline"|"md_link_text";id:bigint}}|{kind:"items";values:MdBlock[][];index:number;list:bigint};
function word(value:number,max:number):bigint{if(!Number.isSafeInteger(value)||value<0||value>max)throw Error("CommonMark scalar exceeds its native width");return BigInt(value);}
function flag(value:boolean):bigint{if(typeof value!=="boolean")throw Error("CommonMark flag must be boolean");return value?1n:0n;}
function number(row:SqliteRow,index:number,max:number):number{const value=artifactSqliteInteger(row,index);if(value<0n||value>BigInt(max))throw Error("CommonMark scalar exceeds its native width");return Number(value);}
async function checkpoint(options:ArtifactSqliteOptions,position:number,total:number):Promise<void>{if(position>0&&position%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",position,total);}
type Entities=Map<bigint,SqliteRow>;
type Children=Map<bigint,bigint[]>;
async function entities(rows:readonly SqliteRow[],columns:number,options:ArtifactSqliteOptions):Promise<Entities>{const keys:Entities=new Map();for(const[position,row]of rows.entries()){await checkpoint(options,position,rows.length);if(row.values.length!==columns||row.rowid<=0n||artifactSqliteInteger(row,0)!==row.rowid||keys.has(row.rowid))throw Error("CommonMark entities require positive unique aliased identities and exact columns");keys.set(row.rowid,row);}return keys;}
async function selectKind(owners:Entities,kind:string,options:ArtifactSqliteOptions):Promise<Entities>{const selected:Entities=new Map();let position=0;for(const[id,row]of owners){await checkpoint(options,position++,owners.size);if(artifactSqliteText(row,1)===kind)selected.set(id,row);}return selected;}
async function subtypes(tables:Map<string,readonly SqliteRow[]>,owners:Entities,types:readonly (readonly[string,string,number])[],options:ArtifactSqliteOptions):Promise<Entities>{const bodies:Entities=new Map();for(const[table,kind,columns]of types){let position=0;for(const[id,row]of await entities(tables.get(table)!,columns,options)){await checkpoint(options,position++,owners.size);const owner=owners.get(id);if(!owner||artifactSqliteText(owner,1)!==kind||bodies.has(id))throw Error("CommonMark entity requires exactly its declared subtype");bodies.set(id,row);}}if(bodies.size!==owners.size)throw Error("CommonMark entity is missing its subtype");return bodies;}
async function relationships(rows:readonly SqliteRow[],parents:Entities,targets:Entities|undefined,owned:Set<bigint>,options:ArtifactSqliteOptions):Promise<Children>{const groups=new Map<bigint,SqliteRow[]>();const keys=await entities(rows,targets?4:3,options);let position=0;for(const row of keys.values()){await checkpoint(options,position++,keys.size);const parent=artifactSqliteInteger(row,1),target=targets?artifactSqliteInteger(row,3):row.rowid;if(!parents.has(parent)||owned.has(target)||(targets&&!targets.has(target)))throw Error("CommonMark entity requires exactly one known owner");owned.add(target);const group=groups.get(parent)??[];group.push(row);groups.set(parent,group);}const children:Children=new Map();let groupPosition=0;for(const[parent,group]of groups){await checkpoint(options,groupPosition++,groups.size);const slots=new Array<bigint|undefined>(group.length);for(const[position,row]of group.entries()){await checkpoint(options,position,group.length);const ordinal=artifactSqliteInteger(row,2);if(ordinal<0n||ordinal>=BigInt(slots.length)||slots[Number(ordinal)]!==undefined)throw Error("CommonMark child ordinals must be unique and dense");slots[Number(ordinal)]=targets?artifactSqliteInteger(row,3):row.rowid;}const result:bigint[]=[];for(const[position,id]of slots.entries()){await checkpoint(options,position,slots.length);if(id===undefined)throw Error("CommonMark child ordinals must be dense");result.push(id);}children.set(parent,result);}return children;}

interface Reader{document:SqliteRow;blocks:Entities;inlines:Entities;blockBodies:Entities;inlineBodies:Entities;roots:bigint[];quotes:Children;lists:Children;items:Children;blockInlines:Children;emphasis:Children;strong:Children;links:Children}
async function reader(database:SqliteDatabase,options:ArtifactSqliteOptions):Promise<Reader>{
 await artifactSqliteTables(database,MD_SQLITE_SCHEMA,options);const tables=new Map(database.tables.map(table=>[table.name,table.rows]));const documents=await entities(tables.get("md_document")!,2,options);if(documents.size!==1)throw Error("CommonMark requires exactly one document");const document=documents.values().next().value!,blocks=await entities(tables.get("md_block")!,2,options),inlines=await entities(tables.get("md_inline")!,2,options);
 const blockBodies=await subtypes(tables,blocks,[["md_heading","heading",2],["md_paragraph","paragraph",1],["md_list","list",4],["md_code_block","codeBlock",3],["md_block_quote","blockQuote",1],["md_thematic_break","thematicBreak",1],["md_html_block","htmlBlock",2]],options);
 const inlineBodies=await subtypes(tables,inlines,[["md_text","text",2],["md_emphasis","emphasis",1],["md_strong","strong",1],["md_code_span","code",2],["md_link","link",3],["md_image","image",4],["md_soft_break","softBreak",1],["md_hard_break","hardBreak",1],["md_html_inline","htmlInline",2]],options);
 const lists=await selectKind(blocks,"list",options),quotes=await selectKind(blocks,"blockQuote",options),items=await entities(tables.get("md_list_item")!,3,options),ownedBlocks=new Set<bigint>(),ownedItems=new Set<bigint>();const roots=(await relationships(tables.get("md_document_block")!,documents,blocks,ownedBlocks,options)).get(document.rowid)??[],quoteBlocks=await relationships(tables.get("md_quote_block")!,quotes,blocks,ownedBlocks,options),listItems=await relationships(tables.get("md_list_item")!,lists,undefined,ownedItems,options),itemBlocks=await relationships(tables.get("md_list_item_block")!,items,blocks,ownedBlocks,options);if(ownedBlocks.size!==blocks.size)throw Error("CommonMark block is missing its unique owner");
 const inlineBlocks=await selectKind(blocks,"heading",options);let position=0;for(const[id,row]of await selectKind(blocks,"paragraph",options)){await checkpoint(options,position++,blocks.size);inlineBlocks.set(id,row);}const emphasis=await selectKind(inlines,"emphasis",options),strong=await selectKind(inlines,"strong",options),links=await selectKind(inlines,"link",options),ownedInlines=new Set<bigint>();const blockInlines=await relationships(tables.get("md_block_inline")!,inlineBlocks,inlines,ownedInlines,options),emphasisInlines=await relationships(tables.get("md_emphasis_inline")!,emphasis,inlines,ownedInlines,options),strongInlines=await relationships(tables.get("md_strong_inline")!,strong,inlines,ownedInlines,options),linkText=await relationships(tables.get("md_link_text")!,links,inlines,ownedInlines,options);if(ownedInlines.size!==inlines.size)throw Error("CommonMark inline is missing its unique owner");return{document,blocks,inlines,blockBodies,inlineBodies,roots,quotes:quoteBlocks,lists:listItems,items:itemBlocks,blockInlines,emphasis:emphasisInlines,strong:strongInlines,links:linkText};
}
function children(groups:Children,id:bigint):bigint[]{return groups.get(id)??[];}
function optionalText<K extends string>(row:SqliteRow,index:number,key:K):Partial<Record<K,string>>{return row.values[index]===null?{}:{[key]:artifactSqliteText(row,index)} as Record<K,string>;}
async function takeTail<T>(values:T[],start:number,options:ArtifactSqliteOptions):Promise<T[]>{if(start<0||start>values.length)throw Error("CommonMark reconstruction frame is invalid");const total=values.length-start,result:T[]=[];if(total>256)await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,total);for(let index=start;index<values.length;index++){if(total>256)await checkpoint(options,index-start,total);result.push(values[index]!);}values.length=start;return result;}
type ReadTask={kind:"blocks"|"inlines"|"items";values:bigint[];index:number}|{kind:"block"|"inline";id:bigint}|{kind:"finishBlock"|"finishInline";id:bigint;start:number}|{kind:"finishItem";start:number};

/** 📥️ Reconstructs each owned tree iteratively and refuses stranded or shared entities. */
export async function mdSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<MdSnapshot>{
 await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);const source=await reader(database,options),tasks:ReadTask[]=[{kind:"blocks",values:source.roots,index:0}],blocks:MdBlock[]=[],inlines:MdInline[]=[],items:MdBlock[][]=[],seenBlocks=new Set<bigint>(),seenInlines=new Set<bigint>();let steps=0;
 while(tasks.length){const task=tasks.pop()!;await checkpoint(options,steps++,0);if(task.kind==="blocks"||task.kind==="inlines"||task.kind==="items"){const id=task.values[task.index];if(id===undefined)continue;tasks.push({...task,index:task.index+1});if(task.kind==="items"){tasks.push({kind:"finishItem",start:blocks.length});tasks.push({kind:"blocks",values:children(source.items,id),index:0});}else tasks.push({kind:task.kind==="blocks"?"block":"inline",id});continue;}
  if(task.kind==="finishItem"){items.push(await takeTail(blocks,task.start,options));continue;}
  if(task.kind==="block"){if(seenBlocks.has(task.id))throw Error("CommonMark block graph has a cycle or repeated entity");seenBlocks.add(task.id);const row=source.blockBodies.get(task.id)!,kind=artifactSqliteText(source.blocks.get(task.id)!,1);switch(kind){
   case "heading":case "paragraph":tasks.push({kind:"finishBlock",id:task.id,start:inlines.length});tasks.push({kind:"inlines",values:children(source.blockInlines,task.id),index:0});break;
   case "list":tasks.push({kind:"finishBlock",id:task.id,start:items.length});tasks.push({kind:"items",values:children(source.lists,task.id),index:0});break;
   case "blockQuote":tasks.push({kind:"finishBlock",id:task.id,start:blocks.length});tasks.push({kind:"blocks",values:children(source.quotes,task.id),index:0});break;
   case "codeBlock":blocks.push({kind,literal:artifactSqliteText(row,2),...optionalText(row,1,"info")});break;
   case "thematicBreak":blocks.push({kind});break;
   case "htmlBlock":blocks.push({kind,raw:artifactSqliteText(row,1)});break;
   default:throw Error("CommonMark block kind is unknown");
  }continue;}
  if(task.kind==="finishBlock"){const row=source.blockBodies.get(task.id)!,kind=artifactSqliteText(source.blocks.get(task.id)!,1);switch(kind){
   case "heading":blocks.push({kind,level:number(row,1,255),inlines:await takeTail(inlines,task.start,options)});break;
   case "paragraph":blocks.push({kind,inlines:await takeTail(inlines,task.start,options)});break;
   case "list":blocks.push({kind,ordered:artifactSqliteBoolean(row,1),tight:artifactSqliteBoolean(row,3),...(row.values[2]===null?{}:{start:number(row,2,4294967295)}),items:await takeTail(items,task.start,options)});break;
   case "blockQuote":blocks.push({kind,blocks:await takeTail(blocks,task.start,options)});break;
   default:throw Error("CommonMark block completion kind is invalid");
  }continue;}
  if(task.kind==="inline"){if(seenInlines.has(task.id))throw Error("CommonMark inline graph has a cycle or repeated entity");seenInlines.add(task.id);const row=source.inlineBodies.get(task.id)!,kind=artifactSqliteText(source.inlines.get(task.id)!,1);switch(kind){
   case "emphasis":case "strong":case "link":tasks.push({kind:"finishInline",id:task.id,start:inlines.length});tasks.push({kind:"inlines",values:children(kind==="emphasis"?source.emphasis:kind==="strong"?source.strong:source.links,task.id),index:0});break;
   case "text":inlines.push({kind,text:artifactSqliteText(row,1)});break;
   case "code":inlines.push({kind,literal:artifactSqliteText(row,1)});break;
   case "image":inlines.push({kind,alt:artifactSqliteText(row,1),url:artifactSqliteText(row,2),...optionalText(row,3,"title")});break;
   case "softBreak":case "hardBreak":inlines.push({kind});break;
   case "htmlInline":inlines.push({kind,raw:artifactSqliteText(row,1)});break;
   default:throw Error("CommonMark inline kind is unknown");
  }continue;}
  if(task.kind==="finishInline"){const row=source.inlineBodies.get(task.id)!,kind=artifactSqliteText(source.inlines.get(task.id)!,1),values=await takeTail(inlines,task.start,options);switch(kind){case "emphasis":case "strong":inlines.push({kind,inlines:values});break;case "link":inlines.push({kind,text:values,url:artifactSqliteText(row,1),...optionalText(row,2,"title")});break;default:throw Error("CommonMark inline completion kind is invalid");}}
 }
 if(seenBlocks.size!==source.blocks.size||seenInlines.size!==source.inlines.size||inlines.length||items.length)throw Error("CommonMark tree contains unreachable entities");await artifactSqliteCheckpoint(options,"reconstructSnapshot",steps,steps);return{schema:artifactSqliteText(source.document,1),blocks};
}

/** 🧭️ Borrowed CommonMark admission checks its exact declaration and authored document identity. */
export async function validateMdSnapshotSqliteDialect(snapshot:MdSnapshot,dialect:ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<readonly never[]>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0,false);if(dialect.artifactKind!=="s.stdio.md"||dialect.standard!=="commonmark"||dialect.subset!=="*")throw Error("CommonMark does not own this semantic subset");
 const candidate=await mdSnapshotToSqliteDatabase(await mdSnapshotFromSqliteDatabase(database,options),options),expected=await mdSnapshotToSqliteDatabase(snapshot,options);let completed=0;const total=expected.tables.reduce((count,table)=>count+table.rows.length,0);
 for(let index=0;index<expected.tables.length;index++){const wanted=expected.tables[index]!,actual=candidate.tables[index];if(!actual||actual.name!==wanted.name||actual.rows.length!==wanted.rows.length)throw Error("CommonMark document identity disagrees with its snapshot");for(let position=0;position<wanted.rows.length;position++){await artifactSqliteCheckpoint(options,"projectSnapshot",completed++,total);const row=wanted.rows[position]!,other=actual.rows[position]!;if(row.rowid!==other.rowid||row.values.length!==other.values.length||row.values.some((value,column)=>value!==other.values[column]))throw Error("CommonMark document identity disagrees with its snapshot");}}
 await artifactSqliteCheckpoint(options,"projectSnapshot",total,total);return[];
}
