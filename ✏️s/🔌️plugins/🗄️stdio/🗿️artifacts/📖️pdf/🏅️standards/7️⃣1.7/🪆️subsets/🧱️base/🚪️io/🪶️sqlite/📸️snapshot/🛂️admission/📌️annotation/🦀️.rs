//! 📌️ Borrowed annotation variants preserve literal optional and geometric cells.
use super::*;
#[derive(Clone,Copy)]
#[repr(usize)]
enum O{State=2,StateModel,Action,Destination,Highlight,DefaultAppearance,Quadding,LineEnding,RichText,EndingStart,EndingEnd,LeaderLength,Caption,Icon,Symbol,ParentHigh,ParentLow,Open,File,Sound,Title,Movie,Activation,Field,Characteristics,AdditionalActions,MarkStyle,Colorants,Entries,FixedPrint,OverlayText,Repeat,Subtype,InteriorPresent,CalloutPresent,Point0,Point1,Point2,Point3}
const WIDTH:usize=O::Point3 as usize+1;
fn put<'a>(f:&mut[Cell<'a>],o:O,v:Cell<'a>){f[o as usize-2]=v}
fn reference(c:&mut Census<'_,'_>,v:&D,visit:fn(&mut Census<'_,'_>,&D)->Result<(),ValueError>)->Result<Cell<'static>,ValueError>{if matches!(v,D::Null){Ok(Null)}else{visit(c,v)?;Ok(Int)}}
fn wide(v:&D)->[Cell<'static>;2]{if matches!(v,D::Null){[Null;2]}else{[Int;2]}}
fn entry(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{let tag=kind(v)?;c.row("pdf_appearance_entry",&[Text(tag),if tag=="single"{Text(text(field(v,"form")?)?)}else{Null}])?;for state in list(field(v,"states")?)?{c.row("pdf_appearance_state",&[Int,Int,Text(text(field(state,"state")?)?),Text(text(field(state,"form")?)?)])?;}Ok(())}
fn appearance(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{entry(c,field(v,"normal")?)?;let r=reference(c,field(v,"rollover")?,entry)?;let d=reference(c,field(v,"down")?,entry)?;c.row("pdf_appearance",&[Int,r,d])}
fn border(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{let mut cells=Cells::from(&[real(field(v,"width")?)?,optional_text(field(v,"style")?)?,Int]);cells.extend(resources::rect(field(v,"radii")?,2)?);c.row("pdf_annotation_border",&cells)?;colors::sequence(c,"pdf_border_dash",field(v,"dash")?)}
fn markup(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{let date=reference(c,field(v,"creationDate")?,meta::date)?;let mut cells=Cells::from(&[optional_text(field(v,"title")?)?]);cells.extend(wide(field(v,"popup")?));cells.extend([real(field(v,"opacity")?)?,optional_text(field(v,"richContents")?)?,date]);cells.extend(wide(field(v,"inReplyTo")?));cells.extend([optional_text(field(v,"subject")?)?,optional_text(field(v,"replyType")?)?,optional_text(field(v,"intent")?)?]);c.row("pdf_annotation_markup",&cells)}
fn detail(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 let tag=kind(v)?;let mut f=[Null;WIDTH-2];
 let names:&[(O,&str)]=match tag{
 "text"=>&[(O::Open,"open"),(O::Icon,"icon"),(O::State,"state"),(O::StateModel,"stateModel")],
 "link"=>&[(O::Action,"action"),(O::Destination,"destination"),(O::Highlight,"highlight")],
 "freeText"=>&[(O::DefaultAppearance,"defaultAppearance"),(O::Quadding,"quadding"),(O::CalloutPresent,"callout"),(O::LineEnding,"lineEnding"),(O::RichText,"richText")],
 "line"=>&[(O::InteriorPresent,"interiorColor"),(O::LeaderLength,"leaderLength"),(O::Caption,"caption")],
 "square"|"circle"|"polygon"|"polyLine"=>&[(O::InteriorPresent,"interiorColor")],
 "stamp"=>&[(O::Icon,"icon")],"caret"=>&[(O::Symbol,"symbol")],"popup"=>&[(O::Open,"open")],
 "fileAttachment"=>&[(O::File,"file"),(O::Icon,"icon")],"sound"=>&[(O::Sound,"sound"),(O::Icon,"icon")],
 "movie"=>&[(O::Title,"title"),(O::Movie,"movie"),(O::Activation,"activation")],
 "widget"=>&[(O::Field,"field"),(O::Highlight,"highlight"),(O::Characteristics,"characteristics"),(O::Action,"action"),(O::AdditionalActions,"additionalActions")],
 "screen"=>&[(O::Title,"title"),(O::Characteristics,"characteristics"),(O::Action,"action"),(O::AdditionalActions,"additionalActions")],
 "printerMark"=>&[(O::MarkStyle,"markStyle"),(O::Colorants,"colorants")],
 "trapNet"|"threeD"=>&[(O::Entries,"entries")],"watermark"=>&[(O::FixedPrint,"fixedPrint")],
 "redact"=>&[(O::InteriorPresent,"interiorColor"),(O::OverlayText,"overlayText"),(O::Repeat,"repeat"),(O::DefaultAppearance,"defaultAppearance"),(O::Quadding,"quadding")],
 "unknown"=>&[(O::Subtype,"subtype"),(O::Entries,"entries")],_=>&[]};
 for (o,key)in names{let value=field(v,key)?;let cell=match o{
 O::Action=>reference(c,value,nav::action)?,O::Destination=>reference(c,value,nav::destination)?,O::File=>reference(c,value,nav::file)?,
 O::Sound|O::Movie|O::Activation|O::Characteristics|O::AdditionalActions|O::Colorants|O::Entries|O::FixedPrint=>reference(c,value,objects::dictionary)?,
 O::Quadding|O::Caption|O::Open|O::Repeat=>if matches!(value,D::Null){Int}else{integer(value)?},
 O::InteriorPresent|O::CalloutPresent=>Int,O::LeaderLength=>real(value)?,_=>optional_text(value)?};put(&mut f,*o,cell);}
 let coordinates=match tag{"line"=>field(v,"points")?,"square"|"circle"|"caret"=>field(v,"rectDifferences")?,_=>&D::Null};
 if !matches!(coordinates,D::Null){for(i,value)in list(coordinates)?.iter().enumerate(){if i>=4{return Err(invalid())}f[O::Point0 as usize-2+i]=real(value)?;}}
 if matches!(tag,"line"|"polyLine"){let endings=list(field(v,"lineEndings")?)?;if !endings.is_empty(){put(&mut f,O::EndingStart,Text(text(&endings[0])?));put(&mut f,O::EndingEnd,Text(text(&endings[1])?));}}
 if tag=="popup"{let words=wide(field(v,"parent")?);put(&mut f,O::ParentHigh,words[0]);put(&mut f,O::ParentLow,words[1]);}
 let mut cells=Cells::from(&[Text(tag)]);cells.extend(f);c.row("pdf_annotation_detail",&cells)?;
 for (role,key)in match tag{"link"|"highlight"|"underline"|"squiggly"|"strikeOut"|"redact"=>&[("quad","quadPoints")][..],"polygon"|"polyLine"=>&[("vertices","vertices")],"freeText"=>&[("callout","callout")],_=>&[]}{kind_reals(c,role,field(v,key)?)?;}
 if matches!(tag,"line"|"square"|"circle"|"polygon"|"polyLine"|"redact"){kind_reals(c,"interior",field(v,"interiorColor")?)?;}
 if tag=="ink"{for path in list(field(v,"paths")?)?{c.row("pdf_annotation_ink_path",&[Int,Int])?;colors::sequence(c,"pdf_annotation_ink_coordinate",path)?;}}Ok(())
}
fn kind_reals(c:&mut Census<'_,'_>,role:&str,v:&D)->Result<(),ValueError>{for value in list(v)?{c.row("pdf_annotation_kind_real",&[Int,Int,Text(role),real(value)?])?;}Ok(())}
pub(super) fn annotation(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 let border=reference(c,field(v,"border")?,border)?;let appearance=reference(c,field(v,"appearance")?,appearance)?;let markup=reference(c,field(v,"markup")?,markup)?;objects::dictionary(c,field(v,"extra")?)?;detail(c,field(v,"kind")?)?;
 let mut cells=resources::rect(field(v,"rect")?,4)?;cells.extend([optional_text(field(v,"contents")?)?,optional_text(field(v,"name")?)?,optional_text(field(v,"modified")?)?,Int,border,appearance,optional_text(field(v,"appearanceState")?)?,markup,optional_text(field(v,"optionalContent")?)?,integer(field(v,"structParent")?)?,Int,Int]);c.row("pdf_annotation",&cells)?;colors::sequence(c,"pdf_annotation_real",field(v,"color")?)
}
