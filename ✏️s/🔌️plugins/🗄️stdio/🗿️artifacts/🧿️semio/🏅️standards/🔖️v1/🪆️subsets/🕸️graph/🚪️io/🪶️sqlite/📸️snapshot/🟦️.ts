/** 🕸️ Native graph node, port, typed property and directed edge relationships. */
import type {SemioGraphSnapshot, SemioGraphNode, SemioGraphEdge, SemioGraphPort, SemioGraphPortKind} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import {projectSemioValueTree,reconstructSemioValueForest,type ValueSqliteTables} from "../../../../🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteDocument,artifactSqliteDocumentReference,artifactSqliteInteger,artifactSqliteText,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {encodeIeee754Cells,readBinary64,type Ieee754Column} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import type {SqliteDatabase,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export const SEMIO_GRAPH_SQLITE_SCHEMA="CREATE TABLE semio_graph_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL);\nCREATE TABLE semio_graph_node (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_graph_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), native_id TEXT NOT NULL, kind TEXT NOT NULL, label TEXT NOT NULL, position_x REAL, position_y REAL, width REAL, height REAL, position_x_ieee754_bits INTEGER, position_x_numeric_class TEXT, position_y_ieee754_bits INTEGER, position_y_numeric_class TEXT, width_ieee754_bits INTEGER, width_numeric_class TEXT, height_ieee754_bits INTEGER, height_numeric_class TEXT);\nCREATE TABLE semio_graph_port (id INTEGER PRIMARY KEY, node_id INTEGER NOT NULL REFERENCES semio_graph_node(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), name TEXT NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('in','out','in_out')), category TEXT NOT NULL);\nCREATE TABLE semio_graph_property (id INTEGER PRIMARY KEY, node_id INTEGER NOT NULL REFERENCES semio_graph_node(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), property_key TEXT NOT NULL, value_id INTEGER NOT NULL REFERENCES semio_graph_value(id));\nCREATE TABLE semio_graph_edge (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_graph_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), native_id TEXT NOT NULL, source_node_id INTEGER NOT NULL REFERENCES semio_graph_node(id), target_node_id INTEGER NOT NULL REFERENCES semio_graph_node(id), kind TEXT NOT NULL, label TEXT NOT NULL, source_port TEXT, target_port TEXT);\nCREATE TABLE semio_graph_value (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK (kind IN ('null','bool','int','float','str','bytes','list','map','ref')), boolean_value INTEGER CHECK (boolean_value IN (0,1)), integer_lexeme TEXT, float_lexeme TEXT, string_value TEXT, bytes_value BLOB, reference_native_id TEXT);\nCREATE TABLE semio_graph_list_element (id INTEGER PRIMARY KEY, parent_value_id INTEGER NOT NULL REFERENCES semio_graph_value(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), value_id INTEGER NOT NULL REFERENCES semio_graph_value(id));\nCREATE TABLE semio_graph_map_entry (id INTEGER PRIMARY KEY, parent_value_id INTEGER NOT NULL REFERENCES semio_graph_value(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), member_key TEXT NOT NULL, value_id INTEGER NOT NULL REFERENCES semio_graph_value(id));\nCREATE TABLE semio_graph_port_property (id INTEGER PRIMARY KEY, port_id INTEGER NOT NULL REFERENCES semio_graph_port(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), property_key TEXT NOT NULL, value_id INTEGER NOT NULL REFERENCES semio_graph_value(id));\nCREATE TABLE semio_graph_edge_property (id INTEGER PRIMARY KEY, edge_id INTEGER NOT NULL REFERENCES semio_graph_edge(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), property_key TEXT NOT NULL, value_id INTEGER NOT NULL REFERENCES semio_graph_value(id));\n";
const POSITION:readonly Ieee754Column[]=[{index:6,width:64},{index:7,width:64},{index:8,width:64},{index:9,width:64}],VALUES:ValueSqliteTables={value:"semio_graph_value",listElement:"semio_graph_list_element",mapEntry:"semio_graph_map_entry"};
function identity(row:SqliteRow,columns:number):void{if(row.rowid<=0n||row.values.length!==columns||artifactSqliteInteger(row,0)!==row.rowid)throw Error("invalid Semio graph entity identity or columns");}
function port(value:string):SemioGraphPortKind{switch(value){case "in":case "out":return value;case "in_out":return "inOut";default:throw Error("unknown Semio graph port kind");}}

/** 📤️ Preserve literal graph entities and every ordered property owner. */
export async function semioGraphSnapshotToSqliteDatabase(snapshot:SemioGraphSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
  await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);
  const names=new Map<string,bigint>(),p=await ArtifactSqliteProjection.create(SEMIO_GRAPH_SQLITE_SCHEMA,options);
  await p.insert("semio_graph_document",[snapshot.schema],1n);
  async function properties(table:string,owner:bigint,entries:SemioGraphNode["properties"]):Promise<void>{
    for(let i=0;i<entries.length;i++){const entry=entries[i]!,value=await projectSemioValueTree(entry.value,VALUES,null,p,options);await p.insert(table,[owner,BigInt(i),entry.key,value]);}
  }
  for(let i=0;i<snapshot.nodes.length;i++){
    const node=snapshot.nodes[i]!,id=BigInt(i+1);if(names.has(node.id.value))throw Error("duplicate Semio graph native node");names.set(node.id.value,id);
    const cells=encodeIeee754Cells([id,1n,BigInt(i),node.id.value,node.kind,node.label,node.position.x,node.position.y,node.width,node.height],POSITION,options.maxColumns);
    await p.insert("semio_graph_node",cells.slice(1),id);
    for(let ordinal=0;ordinal<node.ports.length;ordinal++){const value=node.ports[ordinal]!,kind=value.kind==="inOut"?"in_out":value.kind;port(kind);const owner=await p.insert("semio_graph_port",[id,BigInt(ordinal),value.name,kind,value.category]);await properties("semio_graph_port_property",owner,value.properties);}
    await properties("semio_graph_property",id,node.properties);
  }
  const seen=new Set<string>();
  for(let i=0;i<snapshot.edges.length;i++){
    const edge=snapshot.edges[i]!,source=names.get(edge.source.value),target=names.get(edge.target.value);if(source===undefined||target===undefined||seen.has(edge.id.value))throw Error("invalid Semio graph native edge");seen.add(edge.id.value);
    const owner=await p.insert("semio_graph_edge",[1n,BigInt(i),edge.id.value,source,target,edge.kind,edge.label,edge.sourcePort??null,edge.targetPort??null]);
    await properties("semio_graph_edge_property",owner,edge.properties);
  }
  return p.finish();
}

/** 📥️ Consume each graph property root exactly once and retain literal optional ports. */
export async function semioGraphSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<SemioGraphSnapshot>{
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);
  const[documents,nodes,ports,nodeProperties,edges,,,,portProperties,edgeProperties]=await artifactSqliteTables(database,SEMIO_GRAPH_SQLITE_SCHEMA,options);
  const document=artifactSqliteDocument(documents!);identity(document,2);
  const ordered=await artifactSqliteOrderedRowsControlled(nodes!,2,options),orderedEdges=await artifactSqliteOrderedRowsControlled(edges!,2,options),names=new Map<bigint,string>(),nativeIds=new Set<string>();
  const portRows=new Map<bigint,SqliteRow[]>(),nodeRows=new Map<bigint,SqliteRow[]>(),portOwners=new Map<bigint,SqliteRow[]>(),edgeRows=new Map<bigint,SqliteRow[]>();
  for(const row of ordered){identity(row,18);artifactSqliteDocumentReference(row,1);const name=artifactSqliteText(row,3);if(names.has(row.rowid)||nativeIds.has(name))throw Error("duplicate Semio graph native node");names.set(row.rowid,name);nativeIds.add(name);portRows.set(row.rowid,[]);nodeRows.set(row.rowid,[]);}
  const portIds=new Set<bigint>();for(const row of ports!){identity(row,6);const group=portRows.get(artifactSqliteInteger(row,1));if(!group||portIds.has(row.rowid))throw Error("invalid Semio graph port owner");portIds.add(row.rowid);group.push(row);portOwners.set(row.rowid,[]);}
  const edgeIds=new Set<bigint>();nativeIds.clear();for(const row of orderedEdges){identity(row,10);artifactSqliteDocumentReference(row,1);const name=artifactSqliteText(row,3);if(edgeIds.has(row.rowid)||nativeIds.has(name)||!names.has(artifactSqliteInteger(row,4))||!names.has(artifactSqliteInteger(row,5)))throw Error("invalid Semio graph edge ownership");edgeIds.add(row.rowid);nativeIds.add(name);edgeRows.set(row.rowid,[]);}
  for(const[rows,groups]of [[nodeProperties!,nodeRows],[portProperties!,portOwners],[edgeProperties!,edgeRows]]as const){const seen=new Set<bigint>();for(const row of rows){identity(row,5);const group=groups.get(artifactSqliteInteger(row,1));if(!group||seen.has(row.rowid))throw Error("invalid Semio graph property owner");seen.add(row.rowid);group.push(row);}}
  const propertyGroups:SqliteRow[][]=[],roots:bigint[]=[];for(const groups of [nodeRows,portOwners,edgeRows])for(const rows of groups.values())propertyGroups.push(rows);
  for(const group of propertyGroups){const ordered=await artifactSqliteOrderedRowsControlled(group,2,options);group.length=0;for(const row of ordered)group.push(row);for(const row of group)roots.push(artifactSqliteInteger(row,4));}
  const values=await reconstructSemioValueForest(database,VALUES,roots,null,options),decoded=new Map<SqliteRow,SemioGraphNode["properties"][number]>();let position=0;
  for(const group of propertyGroups)for(const row of group)decoded.set(row,{key:artifactSqliteText(row,3),value:values[position++]!});
  const properties=(rows:SqliteRow[]):SemioGraphNode["properties"]=>rows.map(row=>decoded.get(row)!);
  const out:SemioGraphNode[]=[];
  for(const row of ordered){const ports:SemioGraphPort[]=[];for(const p of await artifactSqliteOrderedRowsControlled(portRows.get(row.rowid)!,2,options))ports.push({name:artifactSqliteText(p,3),kind:port(artifactSqliteText(p,4)),category:artifactSqliteText(p,5),properties:properties(portOwners.get(p.rowid)!)});out.push({id:{value:names.get(row.rowid)!},kind:artifactSqliteText(row,4),label:artifactSqliteText(row,5),position:{x:readBinary64(row,6,POSITION),y:readBinary64(row,7,POSITION)},width:readBinary64(row,8,POSITION),height:readBinary64(row,9,POSITION),ports,properties:properties(nodeRows.get(row.rowid)!)});}
  const optional=(row:SqliteRow,column:number):string|undefined=>row.values[column]===null?undefined:artifactSqliteText(row,column);
  const links:SemioGraphEdge[]=orderedEdges.map(row=>{const sourcePort=optional(row,8),targetPort=optional(row,9);return{id:{value:artifactSqliteText(row,3)},source:{value:names.get(artifactSqliteInteger(row,4))!},target:{value:names.get(artifactSqliteInteger(row,5))!},kind:artifactSqliteText(row,6),label:artifactSqliteText(row,7),...(sourcePort===undefined?{}:{sourcePort}),...(targetPort===undefined?{}:{targetPort}),properties:properties(edgeRows.get(row.rowid)!)};});
  return{schema:artifactSqliteText(document,1),nodes:out,edges:links};
}
