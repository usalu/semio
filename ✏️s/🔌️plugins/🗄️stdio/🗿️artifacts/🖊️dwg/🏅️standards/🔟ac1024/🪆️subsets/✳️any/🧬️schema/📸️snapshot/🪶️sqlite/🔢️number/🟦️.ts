/** 🔢️ Literal DWG numeric fields retain exact binary64 words and native SQL classes. */
import { parseBinary64,binary64Value,binary64,type Binary64 } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import { sqliteValueByteLength,type SqliteValue,type SqliteRow } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { artifactSqliteValueBudget,ArtifactSqliteProjection,artifactSqliteCheckpoint,type ArtifactSqliteOptions } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";

export type DwgCell=SqliteValue|Binary64;
export function dwgNumericPositions(table:string):readonly number[]{
  switch(table){
    case "dwg_annotation_scale":return[2,3];
    case "dwg_constraint_work_plane_coordinate":return[3];case "dwg_constraint_coordinate":return[5];
    case "dwg_mleader_style":return[5,6,8,11];case "dwg_mleader_landing":case "dwg_mleader_dogleg":return[2];case "dwg_mleader_arrow":return[3];case "dwg_mleader_text_style":return[8,11];case "dwg_mleader_block_style":return[4];case "dwg_mleader_block_scale_coordinate":return[3];
    case "dwg_layout":return[9,10,12,18];case "dwg_layout_coordinate":return[5];
    case "dwg_cell_content_format":return[6,7,11];case "dwg_cell_margins":return[1,2,3,4,5,6];case "dwg_cell_border":return[10];
    case "dwg_material":return[3,4,5,6,7,8];case "dwg_material_color":case "dwg_material_map":return[4];case "dwg_material_map_transform_coordinate":return[3];
    case "dwg_mline_style":return[11,12];case "dwg_mline_style_element":return[3];
    case "dwg_visual_style_face":return[9,11];case "dwg_visual_style_edge":return[11,16];case "dwg_visual_style_display":return[3];
    case "dwg_block_move_action":case "dwg_block_stretch_action":return[5,6];
    case "dwg_block_action_display_coordinate":case "dwg_stretch_point_coordinate":case "dwg_block_action_offset_coordinate":case "dwg_block_action_base_point_coordinate":return[3];
    case "dwg_block_linear_parameter":return[3];case "dwg_block_linear_constraint_parameter":return[7];
    case "dwg_block_grip_location_coordinate":case "dwg_block_two_point_definition_base_coordinate":case "dwg_block_two_point_definition_end_coordinate":case "dwg_block_one_point_definition_coordinate":case "dwg_block_linear_allowed_value":case "dwg_block_linear_grip_orientation_coordinate":case "dwg_block_flip_grip_orientation_coordinate":case "dwg_block_alignment_grip_orientation_coordinate":case "dwg_block_base_point_coordinate":case "dwg_block_base_point_base_coordinate":case "dwg_block_linear_constraint_allowed_value":case "dwg_block_flip_definition_base_coordinate":case "dwg_block_flip_definition_end_coordinate":case "dwg_block_flip_label_point_coordinate":case "dwg_block_visibility_definition_coordinate":return[3];
    case "dwg_evaluation_expression":return[6];case "dwg_evaluation_point_group_10_coordinate":case "dwg_evaluation_point_group_11_coordinate":return[3];
    case "dwg_header_units":return[2,3,4,5];
    case "dwg_header_scalars":return[2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,24];
    case "dwg_header_space":return[3];
    case "dwg_header_dimension_settings":return[2,3,4,5,6,7,8,9,10,11,12,25,26,27,28,29,30,31,32,33,64,65];
    case "dwg_header_drawing_policy":return[25,26,27,28,29,32,33,34,35,36,37,40,41,42,51];
    case "dwg_header_space_insertion_base":case "dwg_header_space_extents_minimum":case "dwg_header_space_extents_maximum":case "dwg_header_space_limits_minimum":case "dwg_header_space_limits_maximum":case "dwg_header_space_ucs_origin":case "dwg_header_space_ucs_x_axis":case "dwg_header_space_ucs_y_axis":case "dwg_header_space_ucs_origin_top":case "dwg_header_space_ucs_origin_bottom":case "dwg_header_space_ucs_origin_left":case "dwg_header_space_ucs_origin_right":case "dwg_header_space_ucs_origin_front":case "dwg_header_space_ucs_origin_back":case "dwg_drawing_extmin":case "dwg_drawing_extmax":return[3];
    case "dwg_xrecord_value":return[7,9,10,11];
    case "dwg_text_style_record":return[3,4,5,7];case "dwg_linetype_record":return[2];case "dwg_linetype_dash":return[3,7,8,9,10];case "dwg_block_header_record":return[6,7,8];
    case "dwg_viewport_record":return[1,2,3,4,5,6,7,8,9,10,11,12,13,14,22,23,24,25,26,27,33,34,38,39,40,41,42,45,46,47,48,49,50,51,52,53,54];
    case "dwg_dimension_style_geometry":return[1,2,3,4,5,6,7,8,9,10,11];case "dwg_dimension_style_text":return[1,2,3,4,5,6,7,8,9];case "dwg_dimension_style_r2010":return[3,5];
    case "dwg_entity_common":return[2];case "dwg_line_entity":return[1];case "dwg_arc_entity":return[1,2,3,4];case "dwg_lwpolyline_entity":return[2,3,4];case "dwg_lwpolyline_vertex":return[3,5,6];case "dwg_insert_entity":return[1];case "dwg_point_entity":return[1,2];case "dwg_circle_entity":return[1,2];case "dwg_ellipse_entity":return[1,2,3];case "dwg_text_entity":return[1,3,4,5,6,7];case "dwg_spline_entity":return[5,6];
    case "dwg_lwpolyline_vertex_coordinate":case "dwg_entity_line_start_coordinate":case "dwg_entity_line_end_coordinate":case "dwg_entity_extrusion_coordinate":case "dwg_entity_center_coordinate":case "dwg_entity_insert_insertion_coordinate":case "dwg_entity_insert_scale_coordinate":case "dwg_entity_point_coordinate":case "dwg_entity_ellipse_major_axis_coordinate":case "dwg_entity_text_insertion_coordinate":case "dwg_entity_text_alignment_coordinate":case "dwg_entity_spline_knot":case "dwg_entity_spline_control_coordinate":case "dwg_entity_spline_weight":case "dwg_entity_face3d_corner_coordinate":case "dwg_entity_vertex_coordinate":return[3];
    case "dwg_linear_dimension_entity":return[1,5,6,7,10,11,18,19];
    case "dwg_dimension_text_midpoint_coordinate":case "dwg_dimension_insertion_scale_coordinate":case "dwg_dimension_clone_insertion_coordinate":case "dwg_dimension_extension_line_1_coordinate":case "dwg_dimension_extension_line_2_coordinate":case "dwg_dimension_definition_coordinate":return[3];
    case "dwg_viewport_entity":return[1,2,3,4,5,6,7,8,15,20,21];
    case "dwg_viewport_view_target_coordinate":case "dwg_viewport_view_direction_coordinate":case "dwg_viewport_view_center_coordinate":case "dwg_viewport_snap_base_coordinate":case "dwg_viewport_snap_unit_coordinate":case "dwg_viewport_grid_unit_coordinate":case "dwg_viewport_ucs_origin_coordinate":case "dwg_viewport_ucs_x_axis_coordinate":case "dwg_viewport_ucs_y_axis_coordinate":return[3];
    default:return[];
  }
}
function numericClass(bits:bigint):string{
  if(bits===0x8000000000000000n)return"negative_zero";
  if(((bits>>52n)&2047n)!==2047n)return"finite";
  if((bits&0xfffffffffffffn)!==0n)return"nan";
  return(bits>>63n)===0n?"positive_infinity":"negative_infinity";
}
export function encodeDwgNumberCells(table:string,cells:readonly DwgCell[],options:ArtifactSqliteOptions={},bytesBefore=0):SqliteValue[]{
  const positions=dwgNumericPositions(table);
  if(cells.length+positions.length*2+1>(options.maxColumns??1024)||positions.some(index=>index>cells.length))throw new Error("DWG numeric column limit");
  let bytes=bytesBefore+8;
  for(let index=0;index<cells.length;index++){
    const value=cells[index]!;
    if(positions.includes(index+1)){
      if(value!==null){const bits=parseBinary64(value).bits;const kind=numericClass(bits);bytes+=kind.length+8+(kind==="finite"?8:0);}
    }else{
      if(typeof value==="number"||(value!==null&&typeof value==="object"&&!(value instanceof Uint8Array)))throw new Error("DWG scalar has no authored numeric column");
      bytes+=sqliteValueByteLength(value as SqliteValue);
    }
    artifactSqliteValueBudget(bytes,options);
  }
  artifactSqliteValueBudget(bytes,options);
  const result:SqliteValue[]=[];
  for(let index=0;index<cells.length;index++){
    const value=cells[index]!;
    if(!positions.includes(index+1)){result.push(value as SqliteValue);continue;}
    if(value===null){result.push(null,null,null);continue;}
    const bits=parseBinary64(value).bits;const kind=numericClass(bits);
    result.push(kind,BigInt.asIntN(64,bits),kind==="finite"?binary64Value({bits}):null);
  }
  return result;
}

/** 🫳️ Read caller-authored logical positions without inferring an object layout. */
export class DwgNumberRow{
  readonly rowid:bigint;
  private readonly positions:readonly number[];
  constructor(table:string,readonly raw:SqliteRow){this.positions=dwgNumericPositions(table);this.rowid=raw.rowid;}
  value(column:number):DwgCell{
    const position=column+2*this.positions.filter(value=>value<column).length;
    if(!this.positions.includes(column)){
      const value=this.raw.values[position];
      if(value===undefined||typeof value==="number")throw new Error("DWG column is missing or REAL has no authored class");
      return value;
    }
    const kind=this.raw.values[position],bits=this.raw.values[position+1],query=this.raw.values[position+2];
    if(kind===null&&bits===null&&query===null)return null;
    if(typeof bits!=="bigint"||bits< -9223372036854775808n||bits>9223372036854775807n)throw new Error("DWG numeric identity requires a signed INTEGER64 word");
    const word=BigInt.asUintN(64,bits);if(kind!==numericClass(word))throw new Error("DWG numeric class disagrees with IEEE-754 identity");
    if(kind==="finite"){
      if((typeof query!=="number"&&typeof query!=="bigint")||binary64(Number(query)).bits!==word)throw new Error("DWG query REAL disagrees with IEEE-754 identity");
    }else if(query!==null)throw new Error("DWG special numeric class requires a NULL REAL");
    return{bits:word};
  }
  integer(column:number):bigint{const value=this.value(column);if(typeof value!=="bigint"||value< -9223372036854775808n||value>9223372036854775807n)throw new Error("DWG column requires INTEGER64");return value;}
  text(column:number):string{const value=this.value(column);if(typeof value!=="string")throw new Error("DWG column requires TEXT");return value;}
  real(column:number):Binary64{return parseBinary64(this.value(column));}
}
/** 🔗️ Split complete unsigned64 identity into queryable unsigned32 words. */
export function dwgUnsignedWords(value:bigint):readonly[bigint,bigint]{
  if(typeof value!=="bigint"||value<0n||value>18446744073709551615n)throw new Error("DWG requires a complete unsigned64 bigint");
  return[value>>32n,value&4294967295n];
}

/** 🏗️ Bound exact DWG cells before numeric expansion and owned row allocation. */
export class DwgProjection{
  private bytes=0;
  private rows=0;
  private constructor(private readonly physical:ArtifactSqliteProjection,readonly options:ArtifactSqliteOptions){}
  static async create(sql:string,options:ArtifactSqliteOptions={}):Promise<DwgProjection>{return new DwgProjection(await ArtifactSqliteProjection.create(sql,options),options);}
  async insert(table:string,cells:readonly DwgCell[],identity?:bigint):Promise<bigint>{
    if(this.rows+1>(this.options.maxRows??1_000_000))throw new Error("DWG row limit");
    const encoded=encodeDwgNumberCells(table,cells,this.options,this.bytes);
    const id=await this.physical.insert(table,encoded,identity);
    this.bytes+=8;for(const cell of encoded)this.bytes+=sqliteValueByteLength(cell);
    this.rows++;return id;
  }
  async checkpoint():Promise<void>{await artifactSqliteCheckpoint(this.options,"projectSnapshot",this.rows,0);}
  async finish(){return this.physical.finish();}
}

/** 🛂️ Preserve declared native integer widths before creating SQLite cells. */
export function dwgInteger(value:number,minimum:number,maximum:number):bigint{
  if(typeof value!=="number"||!Number.isSafeInteger(value)||value<minimum||value>maximum)throw new Error("DWG integer exceeds its native width");return BigInt(value);
}
export function dwgBoolean(value:boolean):bigint{if(typeof value!=="boolean")throw new Error("DWG boolean requires a boolean");return value?1n:0n;}
export function dwgOptionalWords(value:bigint|undefined):readonly[bigint|null,bigint|null]{return value===undefined?[null,null]:dwgUnsignedWords(value);}
