//! 🖋️ Borrowed operator operands account for their literal relational cells.
use super::*;
#[derive(Clone,Copy)]
enum O{LineWidth,LineCap,LineJoin,MiterLimit,DashPhase,RenderingIntent,Flatness,ExtGState,X1,Y1,X2,Y2,X3,Y3,Width,Height,CharSpacing,WordSpacing,HorizontalScale,Leading,FontName,FontSize,TextRenderingMode,TextRise,Tx,Ty,TextKind,TextValue,TextCodeCount,GlyphWx,GlyphWy,BboxLlx,BboxLly,BboxUrx,BboxUry,ColorSpaceName,PatternName,Gray,Red,Green,Blue,Cyan,Magenta,Yellow,Black,ShadingName,XobjectName,MarkedTag,PropertyKind,PropertyName,PropertyDictionary,InlineImage,UnknownOperator}
const WIDTH:usize=O::UnknownOperator as usize+1;
fn logical_codes(c:&mut Census<'_ ,'_>,table:&str,value:&D)->Result<(),ValueError> {for code in list(value)? {if !code.as_u64().is_some_and(|code|code<=u32::MAX as u64) {return Err(invalid());}c.row(table,&[Int,Int,Int])?;}Ok(())}
fn operand_text<'a>(c:&mut Census<'_,'_>,f:&mut[Cell<'a>;WIDTH],v:&'a D)->Result<(),ValueError>{let tag=kind(v)?;f[O::TextKind as usize]=Text(tag);match tag{"text"=>f[O::TextValue as usize]=Text(text(field(v,"text")?)?),"codes"=>{logical_codes(c,"pdf_operation_code",field(v,"codes")?)?;f[O::TextCodeCount as usize]=Int;},_=>return Err(invalid())}Ok(())}
fn inline(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{let color=field(v,"colorSpace")?;if !matches!(color,D::Null){colors::color(c,color)?;}objects::dictionary(c,field(v,"extra")?)?;let body=field(v,"body")?;let tag=kind(body)?;let reference=match tag {"artifact"=>{artifact_reference(c,field(body,"reference")?)?;Int},"samples"=>{for value in list(field(body,"values")?)? {c.row("pdf_inline_sample",&[Int,Int,integer(value)?])?;}Null},_=>return Err(invalid())};c.row("pdf_inline_image",&[Int,Int,Int,if matches!(color,D::Null){Null}else{Int},Int,Int,Text(tag),reference,Int])?;colors::sequence(c,"pdf_inline_decode",field(v,"decode")?)}
pub(super) fn ops(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 c.row("pdf_content",&[])?;
 for v in list(v)?{let tag=text(field(v,"op")?)?;let mut f=[Null;WIDTH];
 let named:&[(O,&str)]=match tag{
 "setLineWidth"=>&[(O::LineWidth,"width")],"setLineCap"=>&[(O::LineCap,"cap")],"setLineJoin"=>&[(O::LineJoin,"join")],"setMiterLimit"=>&[(O::MiterLimit,"limit")],"setDash"=>&[(O::DashPhase,"phase")],"setRenderingIntent"=>&[(O::RenderingIntent,"intent")],"setFlatness"=>&[(O::Flatness,"flatness")],"setExtGState"=>&[(O::ExtGState,"name")],
 "moveTo"|"lineTo"=>&[(O::X1,"x"),(O::Y1,"y")],"curveTo"=>&[(O::X1,"x1"),(O::Y1,"y1"),(O::X2,"x2"),(O::Y2,"y2"),(O::X3,"x3"),(O::Y3,"y3")],"curveToInitial"=>&[(O::X2,"x2"),(O::Y2,"y2"),(O::X3,"x3"),(O::Y3,"y3")],"curveToFinal"=>&[(O::X1,"x1"),(O::Y1,"y1"),(O::X3,"x3"),(O::Y3,"y3")],"rectangle"=>&[(O::X1,"x"),(O::Y1,"y"),(O::Width,"width"),(O::Height,"height")],
 "setCharSpacing"=>&[(O::CharSpacing,"spacing")],"setWordSpacing"=>&[(O::WordSpacing,"spacing")],"setHorizontalScale"=>&[(O::HorizontalScale,"scale")],"setLeading"=>&[(O::Leading,"leading")],"setFont"=>&[(O::FontName,"name"),(O::FontSize,"size")],"setTextRenderingMode"=>&[(O::TextRenderingMode,"mode")],"setTextRise"=>&[(O::TextRise,"rise")],"moveText"|"moveTextSetLeading"=>&[(O::Tx,"tx"),(O::Ty,"ty")],"nextLineShowTextSpaced"=>&[(O::WordSpacing,"wordSpacing"),(O::CharSpacing,"charSpacing")],
 "setGlyphWidth"=>&[(O::GlyphWx,"wx"),(O::GlyphWy,"wy")],"setGlyphWidthAndBox"=>&[(O::GlyphWx,"wx"),(O::GlyphWy,"wy"),(O::BboxLlx,"llx"),(O::BboxLly,"lly"),(O::BboxUrx,"urx"),(O::BboxUry,"ury")],
 "setStrokeColorSpace"|"setFillColorSpace"=>&[(O::ColorSpaceName,"name")],"setStrokeColorN"|"setFillColorN"=>&[(O::PatternName,"pattern")],"setStrokeGray"|"setFillGray"=>&[(O::Gray,"gray")],"setStrokeRgb"|"setFillRgb"=>&[(O::Red,"r"),(O::Green,"g"),(O::Blue,"b")],"setStrokeCmyk"|"setFillCmyk"=>&[(O::Cyan,"c"),(O::Magenta,"m"),(O::Yellow,"y"),(O::Black,"k")],
 "paintShading"=>&[(O::ShadingName,"name")],"paintXObject"=>&[(O::XobjectName,"name")],"markedContentPoint"|"beginMarkedContent"|"markedContentPointWithProperties"|"beginMarkedContentWithProperties"=>&[(O::MarkedTag,"tag")],"unknown"=>&[(O::UnknownOperator,"operator")],_=>&[]};
 for (column,key)in named{let value=field(v,key)?;f[*column as usize]=match column{O::LineCap|O::LineJoin|O::RenderingIntent|O::ExtGState|O::FontName|O::ColorSpaceName|O::PatternName|O::ShadingName|O::XobjectName|O::MarkedTag|O::UnknownOperator=>optional_text(value)?,O::TextRenderingMode=>integer(value)?,_=>real(value)?};}
 if matches!(tag,"showText"|"nextLineShowText"|"nextLineShowTextSpaced"){operand_text(c,&mut f,field(v,"text")?)?;}
 if tag=="inlineImage"{inline(c,field(v,"image")?)?;f[O::InlineImage as usize]=Int;}
 if matches!(tag,"markedContentPointWithProperties"|"beginMarkedContentWithProperties"){let p=field(v,"properties")?;let k=kind(p)?;f[O::PropertyKind as usize]=Text(k);match k{"named"=>f[O::PropertyName as usize]=Text(text(field(p,"name")?)?),"inline"=>{objects::dictionary(c,field(p,"entries")?)?;f[O::PropertyDictionary as usize]=Int;},_=>return Err(invalid())}}
 let mut cells=Cells::from(&[Int,Int,Text(tag)]);cells.extend(f);c.row("pdf_operation",&cells)?;
 match tag{
 "transform"|"setTextMatrix"=>{let a=list(field(v,"matrix")?)?;let cells=resources::rect(field(v,"matrix")?,6)?;c.row("pdf_operation_matrix",&cells)?;},
 "setDash"|"setStrokeColor"|"setFillColor"|"setStrokeColorN"|"setFillColorN"=>colors::sequence(c,"pdf_operation_component",field(v,if tag=="setDash"{"array"}else{"components"})?)?,
 "showTextArray"=>{for item in list(field(v,"items")?)?{let k=kind(item)?;let (t,b,a)=match k{"text"=>(Text(text(field(item,"text")?)?),Null,Null),"codes"=>{logical_codes(c,"pdf_array_item_code",field(item,"codes")?)?;(Null,Int,Null)},"adjust"=>(Null,Null,real(field(item,"amount")?)?),_=>return Err(invalid())};c.row("pdf_text_array_item",&[Int,Int,Text(k),t,b,a])?;}},
 "unknown"=>{for operand in list(field(v,"operands")?)?{objects::object(c,operand)?;c.relation("pdf_unknown_operand")?;}},_=>{}
 }
 }Ok(())
}
