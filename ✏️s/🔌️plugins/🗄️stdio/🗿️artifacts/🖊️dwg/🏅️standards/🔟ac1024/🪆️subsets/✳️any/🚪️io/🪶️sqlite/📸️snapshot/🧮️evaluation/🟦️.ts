/** 🗄️ Exact handwritten DWG domain DDL, embedded for filesystem-free runtimes. */
import type { DwgEvaluationExpression,DwgBlockGripLocationComponent,DwgDynamicBlockProxyNode } from "../../../../🧬️schema/🟦️.ts";
import type {Binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import { DwgProjection,dwgInteger,dwgUnsignedWords as H,type DwgCell } from "../🔢️number/🟦️.ts";
import { DwgReader,dwgSignedInteger,dwgSignedWord,dwgUnsigned,dwgFullUnsigned } from "../🫳️reader/🟦️.ts";
import { dwgRequireNull } from "../🏷️xrecord/🟦️.ts";
const I=(v:number)=>dwgInteger(v,-2147483648,2147483647),U=(v:number)=>dwgInteger(v,0,4294967295);
export async function dwgProjectCoordinates(p:DwgProjection,table:string,id:bigint,values:readonly Binary64[]):Promise<void>{for(const[index,value]of values.entries())await p.insert(table,[id,BigInt(index),value]);}
export async function dwgReconstructCoordinates(r:DwgReader,table:string,id:bigint):Promise<Binary64[]>{return(await r.list(table,1,id,2)).map(row=>row.real(3));}
export async function dwgProjectExpression(p:DwgProjection,id:bigint,v:DwgEvaluationExpression):Promise<void>{
  let kind:string,double:DwgCell=null,text:DwgCell=null,integer:DwgCell=null,short:DwgCell=null,handle:readonly DwgCell[]=[null,null];
  switch(v.value.kind){
    case"empty":kind="empty";break;case"double":kind="double";double=v.value.value;break;
    case"pointGroup10":kind="point_group_10";break;case"pointGroup11":kind="point_group_11";break;
    case"string":kind="string";text=v.value.value;break;case"integer32":kind="integer32";integer=I(v.value.value);break;
    case"objectReference":kind="object_reference";handle=H(v.value.value);break;
    case"integer16":kind="integer16";short=dwgInteger(v.value.value,-32768,32767);break;
    default:throw new Error("DWG evaluation value kind is unknown");
  }
  await p.insert("dwg_evaluation_expression",[I(v.parentId),U(v.majorVersion),U(v.minorVersion),U(v.nodeId),kind,double,text,integer,...handle,short],id);
  if(v.value.kind==="pointGroup10")await dwgProjectCoordinates(p,"dwg_evaluation_point_group_10_coordinate",id,v.value.value);
  if(v.value.kind==="pointGroup11")await dwgProjectCoordinates(p,"dwg_evaluation_point_group_11_coordinate",id,v.value.value);
}
export async function dwgReconstructExpression(r:DwgReader,id:bigint):Promise<DwgEvaluationExpression>{
  const row=await r.component("dwg_evaluation_expression",id),group10=await dwgReconstructCoordinates(r,"dwg_evaluation_point_group_10_coordinate",id),group11=await dwgReconstructCoordinates(r,"dwg_evaluation_point_group_11_coordinate",id),kind=row.text(5);
  if((kind!=="point_group_10"&&group10.length!==0)||(kind!=="point_group_11"&&group11.length!==0))throw new Error("DWG evaluation coordinates have the wrong value kind");
  let value:DwgEvaluationExpression["value"];
  switch(kind){
    case"empty":dwgRequireNull(row,[6,7,8,9,10,11]);value={kind:"empty"};break;
    case"double":dwgRequireNull(row,[7,8,9,10,11]);value={kind:"double",value:row.real(6)};break;
    case"point_group_10":dwgRequireNull(row,[6,7,8,9,10,11]);value={kind:"pointGroup10",value:group10};break;
    case"point_group_11":dwgRequireNull(row,[6,7,8,9,10,11]);value={kind:"pointGroup11",value:group11};break;
    case"string":dwgRequireNull(row,[6,8,9,10,11]);value={kind:"string",value:row.text(7)};break;
    case"integer32":dwgRequireNull(row,[6,7,9,10,11]);value={kind:"integer32",value:dwgSignedInteger(row,8)};break;
    case"object_reference":dwgRequireNull(row,[6,7,8,11]);value={kind:"objectReference",value:dwgFullUnsigned(row,9,10)};break;
    case"integer16":dwgRequireNull(row,[6,7,8,9,10]);value={kind:"integer16",value:dwgSignedWord(row,11)};break;
    default:throw new Error("DWG evaluation value kind is unknown");
  }
  return{parentId:dwgSignedInteger(row,1),majorVersion:dwgUnsigned(row,2),minorVersion:dwgUnsigned(row,3),nodeId:dwgUnsigned(row,4),value};
}
export async function dwgProjectGripLocation(p:DwgProjection,id:bigint,v:DwgBlockGripLocationComponent):Promise<void>{await dwgProjectExpression(p,id,v.evaluationExpression);await p.insert("dwg_block_grip_location_component",[U(v.gripType),v.gripExpression],id);}
export async function dwgReconstructGripLocation(r:DwgReader,id:bigint):Promise<DwgBlockGripLocationComponent>{const row=await r.component("dwg_block_grip_location_component",id);return{evaluationExpression:await dwgReconstructExpression(r,id),gripType:dwgUnsigned(row,1),gripExpression:row.text(2)};}
export async function dwgProjectProxy(p:DwgProjection,id:bigint,v:DwgDynamicBlockProxyNode):Promise<void>{await dwgProjectExpression(p,id,v.evaluationExpression);await p.insert("dwg_dynamic_block_proxy_node",[],id);}
export async function dwgReconstructProxy(r:DwgReader,id:bigint):Promise<DwgDynamicBlockProxyNode>{await r.component("dwg_dynamic_block_proxy_node",id);return{evaluationExpression:await dwgReconstructExpression(r,id)};}
export const DWG_EVALUATION_SQL="CREATE TABLE dwg_evaluation_expression (\n id INTEGER PRIMARY KEY REFERENCES dwg_object(id), parent_identifier INTEGER NOT NULL CHECK(parent_identifier BETWEEN -2147483648 AND 2147483647),\n major_version INTEGER NOT NULL CHECK(major_version BETWEEN 0 AND 4294967295), minor_version INTEGER NOT NULL CHECK(minor_version BETWEEN 0 AND 4294967295), node_identifier INTEGER NOT NULL CHECK(node_identifier BETWEEN 0 AND 4294967295),\n value_kind TEXT NOT NULL CHECK(value_kind IN ('empty','double','point_group_10','point_group_11','string','integer32','object_reference','integer16')),\n double_class TEXT CHECK(double_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), double_value_ieee754_bits INTEGER, double_value REAL, string_value TEXT,\n integer32_value INTEGER CHECK(integer32_value BETWEEN -2147483648 AND 2147483647),\n object_reference_high INTEGER CHECK(object_reference_high BETWEEN 0 AND 4294967295), object_reference_low INTEGER CHECK(object_reference_low BETWEEN 0 AND 4294967295),\n integer16_value INTEGER CHECK(integer16_value BETWEEN -32768 AND 32767),\n CHECK((double_class IS NULL AND double_value IS NULL) OR (double_class='finite' AND double_value IS NOT NULL AND abs(double_value)<=1.7976931348623157e308) OR (double_class!='finite' AND double_value IS NULL)),\n CHECK((value_kind='double')=(double_class IS NOT NULL)), CHECK((value_kind='string')=(string_value IS NOT NULL)), CHECK((value_kind='integer32')=(integer32_value IS NOT NULL)),\n CHECK((value_kind='object_reference')=(object_reference_high IS NOT NULL)), CHECK((object_reference_high IS NULL)=(object_reference_low IS NULL)), CHECK((value_kind='integer16')=(integer16_value IS NOT NULL))\n);\nCREATE TABLE dwg_evaluation_point_group_10_coordinate (\n id INTEGER PRIMARY KEY, evaluation_expression_id INTEGER NOT NULL REFERENCES dwg_evaluation_expression(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),\n coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL,\n CHECK((coordinate_class='finite' AND coordinate IS NOT NULL AND abs(coordinate)<=1.7976931348623157e308) OR (coordinate_class!='finite' AND coordinate IS NULL))\n);\nCREATE TABLE dwg_evaluation_point_group_11_coordinate (\n id INTEGER PRIMARY KEY, evaluation_expression_id INTEGER NOT NULL REFERENCES dwg_evaluation_expression(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),\n coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL,\n CHECK((coordinate_class='finite' AND coordinate IS NOT NULL AND abs(coordinate)<=1.7976931348623157e308) OR (coordinate_class!='finite' AND coordinate IS NULL))\n);\nCREATE TABLE dwg_block_grip_location_component (\n id INTEGER PRIMARY KEY REFERENCES dwg_evaluation_expression(id), grip_type INTEGER NOT NULL CHECK(grip_type BETWEEN 0 AND 4294967295), grip_expression TEXT NOT NULL\n);\nCREATE TABLE dwg_dynamic_block_proxy_node (id INTEGER PRIMARY KEY REFERENCES dwg_evaluation_expression(id));\n";
