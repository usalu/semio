/** 🔢️ Exact native binary64 fields of each handwritten PDF resource entity. */
import { pdfFontNumberColumns } from "../../🔤️font/🟦️.ts";
import type { Ieee754Column } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";

/** 🖼️ Resource companions preserve every actual IEEE scalar and optional tuple component. */
export function pdfResourceNumberColumns(table:string):readonly Ieee754Column[]{
  switch(table){
    case "pdf_image_real":case "pdf_g_state_real":return [{index:4,width:64}];
    case "pdf_form_xobject":return [{index:2,width:64},{index:3,width:64},{index:4,width:64},{index:5,width:64},{index:6,width:64},{index:7,width:64},{index:8,width:64},{index:9,width:64},{index:10,width:64},{index:11,width:64}];
    case "pdf_ext_g_state":return [{index:2,width:64},{index:5,width:64},{index:6,width:64},{index:12,width:64},{index:18,width:64},{index:19,width:64},{index:22,width:64},{index:23,width:64}];
    case "pdf_shading":return [{index:5,width:64},{index:6,width:64},{index:7,width:64},{index:8,width:64}];
    case "pdf_shading_background":case "pdf_mesh_decode":return [{index:3,width:64}];
    case "pdf_function_shading":return [{index:1,width:64},{index:2,width:64},{index:3,width:64},{index:4,width:64},{index:5,width:64},{index:6,width:64},{index:7,width:64},{index:8,width:64},{index:9,width:64},{index:10,width:64}];
    case "pdf_axial_shading":return [{index:1,width:64},{index:2,width:64},{index:3,width:64},{index:4,width:64},{index:5,width:64},{index:6,width:64}];
    case "pdf_radial_shading":return [{index:1,width:64},{index:2,width:64},{index:3,width:64},{index:4,width:64},{index:5,width:64},{index:6,width:64},{index:7,width:64},{index:8,width:64}];
    case "pdf_pattern":return [{index:3,width:64},{index:4,width:64},{index:5,width:64},{index:6,width:64},{index:7,width:64},{index:8,width:64}];
    case "pdf_tiling_pattern":return [{index:3,width:64},{index:4,width:64},{index:5,width:64},{index:6,width:64},{index:7,width:64},{index:8,width:64}];
    default:return pdfFontNumberColumns(table);
  }
}
