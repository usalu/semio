//! 📄️ DWG document metadata, classes, application history and indexed preview entities.
use super::super::*;
use super::reader::{boolean,byte,document,full_unsigned,high,low,ordinal,unsigned,word,Reader};
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell;
use super::number::Projection;
use Cell::{Integer as I,Text as T};

pub(super) struct DocumentState {
    pub schema:String,pub version:String,pub maintenance_version:u8,pub codepage:u16,
    pub classes:Vec<DwgClass>,pub dependencies:Vec<DwgDependency>,pub summary:DwgSummaryInfo,
    pub application:DwgApplicationInfo,pub template:DwgTemplate,pub auxiliary_header:DwgAuxiliaryHeader,
    pub revision_history:DwgRevisionHistory,pub preview:DwgIndexedPreview,pub application_history:DwgApplicationHistory,
}
pub(super) fn project(projection:&mut Projection<'_,'_>,snapshot:&DwgSnapshot)->Result<(),String>{
    projection.insert("dwg_document",&[T(&snapshot.schema),T(&snapshot.version),I(i64::from(snapshot.maintenance_version)),I(i64::from(snapshot.codepage))])?;
    let summary=&snapshot.summary;
    projection.insert("dwg_summary",&[I(1),T(&summary.title),T(&summary.subject),T(&summary.author),T(&summary.keywords),T(&summary.comments),T(&summary.last_saved_by),T(&summary.revision_number),T(&summary.hyperlink_base),I(high(summary.total_editing_time)),I(low(summary.total_editing_time)),I(i64::from(summary.created_at.days)),I(i64::from(summary.created_at.milliseconds)),I(i64::from(summary.modified_at.days)),I(i64::from(summary.modified_at.milliseconds))])?;
    for(index,value)in summary.custom_properties.iter().enumerate(){projection.insert("dwg_custom_property",&[I(1),I(ordinal(index)?),T(&value.key),T(&value.value)])?;}
    let application=&snapshot.application;
    projection.insert("dwg_application_info",&[I(1),T(&application.name),T(&application.version_checksum),T(&application.version),T(&application.comment_checksum),T(&application.comment),T(&application.product_checksum),T(&application.product),T(&application.application_version)])?;
    let template=&snapshot.template;
    let measurement=match template.measurement{DwgMeasurement::English=>"english",DwgMeasurement::Metric=>"metric"};
    projection.insert("dwg_template",&[I(1),T(&template.description),T(measurement)])?;
    let header=&snapshot.auxiliary_header;
    let compatibility=match header.compatibility_profile{DwgCompatibilityProfile::Autocad2009=>"autocad2009"};
    projection.insert("dwg_auxiliary_header",&[I(1),I(i64::from(header.total_saves)),I(i64::from(header.save_partition_one)),I(i64::from(header.save_partition_two)),I(i64::from(header.save_generation)),I(i64::from(header.legacy_stamp_one.version)),I(i64::from(header.legacy_stamp_one.maintenance)),I(i64::from(header.legacy_stamp_two.version)),I(i64::from(header.legacy_stamp_two.maintenance)),T(compatibility),I(i64::from(header.created_at.days)),I(i64::from(header.created_at.milliseconds)),I(i64::from(header.updated_at.days)),I(i64::from(header.updated_at.milliseconds)),I(high(header.handle_seed)),I(low(header.handle_seed)),I(i64::from(header.terminal_save_generation))])?;
    let history=&snapshot.revision_history;
    projection.insert("dwg_revision_history",&[I(1),I(i64::from(history.format_major)),I(i64::from(history.format_minor))])?;
    for(index,value)in history.revisions.iter().enumerate(){projection.insert("dwg_revision",&[I(1),I(ordinal(index)?),I(i64::from(*value))])?;}
    let preview=&snapshot.preview;
    let origin=match preview.origin{DwgPreviewOrigin::BottomUp=>"bottom_up"};
    projection.insert("dwg_preview",&[I(1),I(i64::from(preview.width)),I(i64::from(preview.height)),T(origin),I(i64::from(preview.background_palette_index))])?;
    for(index,value)in preview.palette.iter().enumerate(){projection.insert("dwg_preview_palette_entry",&[I(1),I(ordinal(index)?),I(i64::from(value.red)),I(i64::from(value.green)),I(i64::from(value.blue)),I(i64::from(value.alpha))])?;}
    for(index,value)in preview.pixel_indices.iter().enumerate(){projection.insert("dwg_preview_pixel_index",&[I(1),I(ordinal(index)?),I(i64::from(*value))])?;}
    let history=&snapshot.application_history;
    projection.insert("dwg_application_history",&[I(1),T(&history.history_identifier_one),T(&history.history_identifier_two),I(i64::from(history.class_version)),T(&history.application_version_digest),T(&history.application_version),T(&history.trust_comment_digest),T(&history.trust_comment),T(&history.property_set_digest),T(&history.property_format_identifier),T(&history.product_digest)])?;
    for(index,value)in history.properties.iter().enumerate(){
        let kind=match value.kind{DwgApplicationPropertyKind::String=>"string",DwgApplicationPropertyKind::DateTime=>"date_time"};
        projection.insert("dwg_application_property",&[I(1),I(ordinal(index)?),I(i64::from(value.id)),T(kind),T(&value.value)])?;
    }
    let product=&history.product;
    projection.insert("dwg_product_information",&[I(1),T(&product.name),T(&product.build_version),T(&product.registry_version),T(&product.install_id),T(&product.locale_id)])?;
    for(index,value)in snapshot.classes.iter().enumerate(){
        let id=projection.insert("dwg_class",&[I(1),I(ordinal(index)?),I(i64::from(value.number)),I(i64::from(value.proxy_flags)),T(&value.application_name),T(&value.cpp_class_name),T(&value.dxf_name),I(i64::from(value.was_zombie)),I(i64::from(value.item_class_id)),I(i64::from(value.object_count)),I(i64::from(value.dwg_version)),I(i64::from(value.maintenance_version))])?;
        for(index,value)in value.reserved_values.iter().enumerate(){projection.insert("dwg_class_reserved_value",&[I(id),I(ordinal(index)?),I(i64::from(*value))])?;}
    }
    for(index,value)in snapshot.dependencies.iter().enumerate(){projection.insert("dwg_dependency",&[I(1),I(ordinal(index)?),T(&value.feature),T(&value.full_path),T(&value.relative_path),T(&value.fingerprint),T(&value.version),I(i64::from(value.timestamp)),I(i64::from(value.file_size)),I(i64::from(value.affects_graphics)),I(i64::from(value.reference_count))])?;}
    Ok(())
}
pub(super) fn reconstruct(reader:&mut Reader<'_,'_,'_>)->Result<DocumentState,String>{
    let row=reader.one("dwg_document")?;
    let schema=row.text(1)?.into();let version=row.text(2)?.into();let maintenance_version=byte(row,3)?;let codepage=word(row,4)?;
    let row=reader.one("dwg_summary")?;document(row)?;
    let mut summary=DwgSummaryInfo{title:row.text(2)?.into(),subject:row.text(3)?.into(),author:row.text(4)?.into(),keywords:row.text(5)?.into(),comments:row.text(6)?.into(),last_saved_by:row.text(7)?.into(),revision_number:row.text(8)?.into(),hyperlink_base:row.text(9)?.into(),total_editing_time:full_unsigned(row,10,11)?,created_at:DwgJulianDate{days:unsigned(row,12)?,milliseconds:unsigned(row,13)?},modified_at:DwgJulianDate{days:unsigned(row,14)?,milliseconds:unsigned(row,15)?},custom_properties:Vec::new()};
    for row in reader.list("dwg_custom_property",1,1,2)?{summary.custom_properties.push(DwgCustomProperty{key:row.text(3)?.into(),value:row.text(4)?.into()});}
    let row=reader.one("dwg_application_info")?;document(row)?;
    let application=DwgApplicationInfo{name:row.text(2)?.into(),version_checksum:row.text(3)?.into(),version:row.text(4)?.into(),comment_checksum:row.text(5)?.into(),comment:row.text(6)?.into(),product_checksum:row.text(7)?.into(),product:row.text(8)?.into(),application_version:row.text(9)?.into()};
    let row=reader.one("dwg_template")?;document(row)?;
    let template=DwgTemplate{description:row.text(2)?.into(),measurement:match row.text(3)?{"english"=>DwgMeasurement::English,"metric"=>DwgMeasurement::Metric,_=>return Err("DWG template measurement is unknown".into())}};
    let row=reader.one("dwg_auxiliary_header")?;document(row)?;
    let auxiliary_header=DwgAuxiliaryHeader{total_saves:unsigned(row,2)?,save_partition_one:word(row,3)?,save_partition_two:word(row,4)?,save_generation:unsigned(row,5)?,legacy_stamp_one:DwgVersionStamp{version:word(row,6)?,maintenance:word(row,7)?},legacy_stamp_two:DwgVersionStamp{version:word(row,8)?,maintenance:word(row,9)?},compatibility_profile:match row.text(10)?{"autocad2009"=>DwgCompatibilityProfile::Autocad2009,_=>return Err("DWG compatibility profile is unknown".into())},created_at:DwgJulianDate{days:unsigned(row,11)?,milliseconds:unsigned(row,12)?},updated_at:DwgJulianDate{days:unsigned(row,13)?,milliseconds:unsigned(row,14)?},handle_seed:full_unsigned(row,15,16)?,terminal_save_generation:word(row,17)?};
    let row=reader.one("dwg_revision_history")?;document(row)?;
    let mut revision_history=DwgRevisionHistory{format_major:unsigned(row,2)?,format_minor:unsigned(row,3)?,revisions:Vec::new()};
    for row in reader.list("dwg_revision",1,1,2)?{revision_history.revisions.push(unsigned(row,3)?);}
    let row=reader.one("dwg_preview")?;document(row)?;
    let mut preview=DwgIndexedPreview{width:unsigned(row,2)?,height:unsigned(row,3)?,origin:match row.text(4)?{"bottom_up"=>DwgPreviewOrigin::BottomUp,_=>return Err("DWG preview origin is unknown".into())},palette:Vec::new(),pixel_indices:Vec::new(),background_palette_index:byte(row,5)?};
    for row in reader.list("dwg_preview_palette_entry",1,1,2)?{preview.palette.push(DwgRgba{red:byte(row,3)?,green:byte(row,4)?,blue:byte(row,5)?,alpha:byte(row,6)?});}
    for row in reader.list("dwg_preview_pixel_index",1,1,2)?{preview.pixel_indices.push(byte(row,3)?);}
    let row=reader.one("dwg_application_history")?;document(row)?;
    let history_identifier_one=row.text(2)?.into();let history_identifier_two=row.text(3)?.into();let class_version=unsigned(row,4)?;
    let application_version_digest=row.text(5)?.into();let application_version=row.text(6)?.into();let trust_comment_digest=row.text(7)?.into();let trust_comment=row.text(8)?.into();let property_set_digest=row.text(9)?.into();let property_format_identifier=row.text(10)?.into();let product_digest=row.text(11)?.into();
    let mut properties=Vec::new();
    for row in reader.list("dwg_application_property",1,1,2)?{properties.push(DwgApplicationProperty{id:unsigned(row,3)?,kind:match row.text(4)?{"string"=>DwgApplicationPropertyKind::String,"date_time"=>DwgApplicationPropertyKind::DateTime,_=>return Err("DWG application property kind is unknown".into())},value:row.text(5)?.into()});}
    let row=reader.one("dwg_product_information")?;document(row)?;
    let product=DwgProductInformation{name:row.text(2)?.into(),build_version:row.text(3)?.into(),registry_version:row.text(4)?.into(),install_id:row.text(5)?.into(),locale_id:row.text(6)?.into()};
    let application_history=DwgApplicationHistory{history_identifier_one,history_identifier_two,class_version,application_version_digest,application_version,trust_comment_digest,trust_comment,property_set_digest,property_format_identifier,properties,product_digest,product};
    let mut classes=Vec::new();
    for row in reader.list("dwg_class",1,1,2)?{
        let mut value=DwgClass{number:word(row,3)?,proxy_flags:unsigned(row,4)?,application_name:row.text(5)?.into(),cpp_class_name:row.text(6)?.into(),dxf_name:row.text(7)?.into(),was_zombie:boolean(row,8)?,item_class_id:word(row,9)?,object_count:unsigned(row,10)?,dwg_version:unsigned(row,11)?,maintenance_version:unsigned(row,12)?,reserved_values:Vec::new()};
        for child in reader.list("dwg_class_reserved_value",1,row.rowid,2)?{value.reserved_values.push(unsigned(child,3)?);}
        classes.push(value);
    }
    let mut dependencies=Vec::new();
    for row in reader.list("dwg_dependency",1,1,2)?{dependencies.push(DwgDependency{feature:row.text(3)?.into(),full_path:row.text(4)?.into(),relative_path:row.text(5)?.into(),fingerprint:row.text(6)?.into(),version:row.text(7)?.into(),timestamp:unsigned(row,8)?,file_size:unsigned(row,9)?,affects_graphics:boolean(row,10)?,reference_count:unsigned(row,11)?});}
    Ok(DocumentState{schema,version,maintenance_version,codepage,classes,dependencies,summary,application,template,auxiliary_header,revision_history,preview,application_history})
}
