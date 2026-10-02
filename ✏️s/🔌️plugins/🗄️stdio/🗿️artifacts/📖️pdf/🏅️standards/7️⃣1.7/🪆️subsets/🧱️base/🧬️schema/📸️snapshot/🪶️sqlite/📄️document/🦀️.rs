//! 📄️ Complete PDF document ownership and ordered top-level entity collections.
use super::*;
fn write_page(out:&mut Projection<'_,'_>,value:&PdfPage)->Result<i64,String>{let content=content::write_ops(out,&value.content)?;let group=value.group.as_ref().map(|value|resource::write_group(out,value)).transpose()?;let transition=value.transition.as_ref().map(|value|cos::write_dictionary(out,value)).transpose()?;let actions=cos::write_dictionary(out,&value.additional_actions)?;let extra=cos::write_dictionary(out,&value.extra)?;let mut cells=value.media_box.map(C::Real).to_vec();for array in [&value.crop_box,&value.bleed_box,&value.trim_box,&value.art_box]{cells.extend(array_cells(array));}cells.extend([C::Integer(i64::from(value.rotate)),real_cell(value.user_unit),C::Integer(content),group.map_or(C::Null,C::Integer),text_cell(&value.thumbnail),integer_cell(value.struct_parents),transition.map_or(C::Null,C::Integer),real_cell(value.duration),text_cell(&value.metadata),C::Integer(actions),C::Integer(extra)]);let key=out.insert("pdf_page",&cells)?;for(ordinal,value)in value.annotations.iter().enumerate(){let annotation=annotation::write_annotation(out,value)?;out.insert("pdf_page_annotation",&[C::Integer(key),C::Integer(ordinal as i64),C::Integer(annotation)])?;}Ok(key)}
fn read_page(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfPage,String>{let row=reader.take("pdf_page",key,32)?;let mut annotations=Vec::new();for child in reader.children("pdf_page_annotation",1,2,key)?{let child=reader.take("pdf_page_annotation",child.rowid,4)?;annotations.push(annotation::read_annotation(reader,child.integer(3)?)?);}Ok(PdfPage{media_box:[row.real(1)?,row.real(2)?,row.real(3)?,row.real(4)?],crop_box:optional_array(row,5)?,bleed_box:optional_array(row,9)?,trim_box:optional_array(row,13)?,art_box:optional_array(row,17)?,rotate:integer(row,21)?,user_unit:optional_real(row,22)?,content:content::read_ops(reader,row.integer(23)?)?,annotations,group:optional_integer(row,24)?.map(|key|resource::read_group(reader,key)).transpose()?,thumbnail:reader.optional_text(row,25)?,struct_parents:optional_typed_integer(row,26)?,transition:optional_integer(row,27)?.map(|key|cos::read_dictionary(reader,key)).transpose()?,duration:optional_real(row,28)?,metadata:reader.optional_text(row,29)?,additional_actions:cos::read_dictionary(reader,row.integer(30)?)?,extra:cos::read_dictionary(reader,row.integer(31)?)?})}
fn write_colors(out:&mut Projection<'_,'_>,value:&PdfNamedColorSpace)->Result<i64,String>{let color=color::write_color(out,&value.color_space)?;out.insert("pdf_named_color",&[C::Text(&value.name),C::Integer(color)])}
fn read_colors(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfNamedColorSpace,String>{let row=reader.take("pdf_named_color",key,3)?;Ok(PdfNamedColorSpace{name:reader.text(row,1)?,color_space:color::read_color(reader,row.integer(2)?)?})}
fn write_properties(out:&mut Projection<'_,'_>,value:&PdfNamedProperties)->Result<i64,String>{let entries=cos::write_dictionary(out,&value.entries)?;out.insert("pdf_named_properties",&[C::Text(&value.name),C::Integer(entries)])}
fn read_properties(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfNamedProperties,String>{let row=reader.take("pdf_named_properties",key,3)?;Ok(PdfNamedProperties{name:reader.text(row,1)?,entries:cos::read_dictionary(reader,row.integer(2)?)?})}
fn write_indirect(out:&mut Projection<'_,'_>,value:&PdfIndirectObject)->Result<i64,String>{let object=cos::write_object(out,&value.value)?;out.insert("pdf_indirect_object",&[C::Integer(i64::from(value.id.num)),C::Integer(i64::from(value.id.gen)),C::Integer(object)])}
fn read_indirect(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfIndirectObject,String>{let row=reader.take("pdf_indirect_object",key,4)?;Ok(PdfIndirectObject{id:ObjRef{num:integer(row,1)?,gen:integer(row,2)?},value:cos::read_object(reader,row.integer(3)?)?})}
fn write_collection<T>(out:&mut Projection<'_,'_>,table:&str,values:&[T],write:impl Fn(&mut Projection<'_,'_>,&T)->Result<i64,String>)->Result<(),String>{for(ordinal,value)in values.iter().enumerate(){let key=write(out,value)?;out.insert(table,&[C::Integer(1),C::Integer(ordinal as i64),C::Integer(key)])?;}Ok(())}
fn read_collection<T>(reader:&mut Reader<'_,'_,'_>,table:&'static str,read:impl Fn(&mut Reader<'_,'_,'_>,i64)->Result<T,String>)->Result<Vec<T>,String>{let mut values=Vec::new();for child in reader.children(table,1,2,1)?{let child=reader.take(table,child.rowid,4)?;values.push(read(reader,child.integer(3)?)?);}Ok(values)}

impl store::ArtifactSqliteSnapshot for PdfSnapshot{
    fn encode_sqlite_snapshot_native(&self,encoding:sqlite_snapshot::SnapshotEncoding,control:&mut Control<'_>)->Result<store::io_schema::IoPayload,String>{control.checkpoint(Phase::EncodeNative,0,0)?;let mut forecast=Projection::forecast(control)?;self.write_sqlite_rows(&mut forecast)?;forecast.finish_forecast()?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),snapshot_text::spec_producer(),|native|snapshot_text::to_record_controlled(self,native),control)}
    fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut Control<'_>)->Result<Self,String>{store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),snapshot_text::spec_producer(),snapshot_text::from_record_controlled,control)}
    fn retire_sqlite_snapshot(self){<Self as pack::value::FromValue>::retire_decoded(self)}
    const SQLITE_SCHEMA:&'static str=include_str!("../🗄️.sql");
    fn preflight_sqlite_snapshot_encoding(&self,_encoding:sqlite_snapshot::SnapshotEncoding,control:&mut Control<'_>)->Result<(),String>{
        control.checkpoint(Phase::EncodeNative,0,0)?;
        let database=self.to_sqlite_database(control)?;
        let mut bound=sqlite_snapshot::artifact::NativeEncodingBound::new(control)?;
        bound.add(16_384)?;
        for table in &database.tables{for row in &table.rows{
            bound.add(8_192)?;
            for value in &row.values{match value{
                V::Text(text)=>{bound.repeated(text.len(),16)?;bound.add(256)?;},
                V::Blob(bytes)=>{bound.repeated(bytes.len(),16)?;bound.add(256)?;},
                V::Integer(_)|V::Real(_)|V::Null=>bound.add(2_048)?,
            }}
        }}
        bound.finish()
    }
    fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,database:&Db,control:&mut Control<'_>)->store::io_schema::IoResult<()>{
        use crate::standards::v1_7::subsets;
        if dialect.artifact_kind!="s.stdio.pdf"||dialect.standard!="1.7"{return Err("PDF1.7 snapshot does not own this dialect".to_string().into());}
        control.checkpoint(Phase::ProjectSnapshot,0,0)?;
        let mut diagnostics=match dialect.subset.as_str(){
            "*"=>return Ok(store::io_schema::IoOutcome::clean(())),
            "a"=>subsets::a::schema::check_pdf_a_conformance(self),
            "x"=>subsets::x::schema::check_x_conformance(self),
            "e"=>subsets::e::schema::check_e_conformance(self),
            "ua"=>subsets::ua::schema::check_ua_conformance(self),
            "vt"=>subsets::vt::schema::check_vt_conformance(self),
            "h"=>subsets::h::schema::check_h_conformance(self),
            _=>return Err("PDF1.7 snapshot subset has no semantic validator".to_string().into()),
        };
        let action_severity=match dialect.subset.as_str(){"a"|"e"=>Some(dsl::Severity::Error),"x"|"vt"|"h"=>Some(dsl::Severity::Warning),_=>None};
        let mut report=|code:&str,message:String,severity:dsl::Severity|{diagnostics.push(dsl::Diagnostic{code:dsl::FaultCode::new(code),severity,span:dsl::TextSpan::at(1,1),message,expected:None,scope:dsl::FaultScope::default()});};
        if self.encryption.is_some()&&matches!(dialect.subset.as_str(),"a"|"x"|"e"|"vt"){report("stdio.pdf.sqlite.encryption-forbidden","typed document encryption is forbidden by this PDF profile".into(),dsl::Severity::Error);}
        if let Some(severity)=action_severity{
            for(index,row)in database.table("pdf_action")?.rows.iter().enumerate(){
                if index%256==0{control.checkpoint(Phase::ProjectSnapshot,index,database.table("pdf_action")?.rows.len())?;}
                let kind=row.text(1)?;
                if matches!(kind,"javaScript"|"launch")||(kind=="unknown"&&matches!(row.optional_text(22)?,Some("JavaScript"|"Launch"))){report("stdio.pdf.sqlite.forbidden-action",format!("semantic action {} has forbidden kind {kind}",row.rowid),severity);}
            }
            let mut names=std::collections::BTreeMap::new();
            for(index,row)in database.table("pdf_cos_value")?.rows.iter().enumerate(){
                if index%256==0{control.checkpoint(Phase::ProjectSnapshot,index,database.table("pdf_cos_value")?.rows.len())?;}
                if row.text(1)?=="name"{names.insert(row.rowid,row.text(8)?);}
            }
            for(index,row)in database.table("pdf_cos_dictionary_entry")?.rows.iter().enumerate(){
                if index%256==0{control.checkpoint(Phase::ProjectSnapshot,index,database.table("pdf_cos_dictionary_entry")?.rows.len())?;}
                if row.text(3)?=="JS"||(row.text(3)?=="S"&&matches!(names.get(&row.integer(4)?),Some(&"JavaScript"|&"Launch"))){report("stdio.pdf.sqlite.forbidden-dictionary-action",format!("semantic dictionary {} contains a forbidden action entry",row.integer(1)?),severity);}
            }
        }
        if matches!(dialect.subset.as_str(),"x"|"vt"){
            for(index,page)in self.pages.iter().enumerate(){if index%256==0{control.checkpoint(Phase::ProjectSnapshot,index,self.pages.len())?;}if page.trim_box.is_none()&&page.art_box.is_none(){report("stdio.pdf.sqlite.page-box-required",format!("typed page {index} has neither TrimBox nor ArtBox"),dsl::Severity::Error);}}
        }
        control.check_value_bytes(diagnostics.iter().try_fold(0usize,|sum,value|sum.checked_add(value.message.len()).ok_or_else(||"PDF diagnostic byte count overflow".to_string()))?)?;
        control.checkpoint(Phase::ProjectSnapshot,1,1)?;
        Ok(store::io_schema::IoOutcome{value:(),diagnostics})
    }
    fn to_sqlite_database(&self,control:&mut Control<'_>)->Result<Db,String>{
        let mut out=Projection::new(Self::SQLITE_SCHEMA,control)?;
        self.write_sqlite_rows(&mut out)?;
        out.finish()
    }
    fn from_sqlite_database(db:&Db,control:&mut Control<'_>)->Result<Self,String>{
        sqlite_snapshot::validate_sqlite_database_schema(db,Self::SQLITE_SCHEMA,control.limits()).map_err(|error|error.to_string())?;let mut reader=Reader::new(db,control)?;let row=reader.take("pdf_document",1,18)?;
        let document_id=match(&row.values[12],&row.values[13]){(V::Null,V::Null)=>None,(V::Blob(first),V::Blob(second))=>Some([reader.copy_blob(first)?,reader.copy_blob(second)?]),_=>return Err("PDF document identifier requires both intrinsic byte strings".into())};
        let value=Self{schema:reader.text(row,1)?,declared_version:reader.text(row,2)?,pages:read_collection(&mut reader,"pdf_document_page",read_page)?,fonts:read_collection(&mut reader,"pdf_document_font",font::read_font)?,images:read_collection(&mut reader,"pdf_document_image",resource::read_image)?,forms:read_collection(&mut reader,"pdf_document_form",resource::read_form)?,ext_g_states:read_collection(&mut reader,"pdf_document_state",resource::read_state)?,shadings:read_collection(&mut reader,"pdf_document_shading",resource::read_shading)?,patterns:read_collection(&mut reader,"pdf_document_pattern",resource::read_pattern)?,color_spaces:read_collection(&mut reader,"pdf_document_color",read_colors)?,properties:read_collection(&mut reader,"pdf_document_properties",read_properties)?,outlines:read_collection(&mut reader,"pdf_document_outline",|reader,key|navigation::read_outline(reader,key))?,named_destinations:read_collection(&mut reader,"pdf_document_destination",navigation::read_named_destination)?,page_labels:read_collection(&mut reader,"pdf_document_label",navigation::read_label)?,embedded_files:read_collection(&mut reader,"pdf_document_file",metadata::read_file)?,output_intents:read_collection(&mut reader,"pdf_document_intent",metadata::read_intent)?,acro_form:optional_integer(row,3)?.map(|key|form::read_acro(&mut reader,key)).transpose()?,optional_content:optional_integer(row,4)?.map(|key|form::read_optional(&mut reader,key)).transpose()?,page_layout:row.optional_text(5)?.map(metadata::read_page_layout).transpose()?,page_mode:row.optional_text(6)?.map(metadata::read_page_mode).transpose()?,viewer_preferences:optional_integer(row,7)?.map(|key|metadata::read_preferences(&mut reader,key)).transpose()?,open_action:optional_integer(row,8)?.map(|key|navigation::read_open_action(&mut reader,key)).transpose()?,language:reader.optional_text(row,9)?,mark_info:optional_integer(row,10)?.map(|key|metadata::read_mark(&mut reader,key)).transpose()?,metadata:reader.optional_text(row,11)?,document_id,encryption:optional_integer(row,14)?.map(|key|metadata::read_encryption(&mut reader,key)).transpose()?,info:metadata::read_info(&mut reader,row.integer(15)?)?,catalog_extra:cos::read_dictionary(&mut reader,row.integer(16)?)?,objects:read_collection(&mut reader,"pdf_document_object",read_indirect)?,trailer:cos::read_dictionary(&mut reader,row.integer(17)?)?};reader.finish()?;Ok(value)
    }
}

impl PdfSnapshot{
    fn write_sqlite_rows(&self,out:&mut Projection<'_,'_>)->Result<(),String>{
        let acro=self.acro_form.as_ref().map(|value|form::write_acro(out,value)).transpose()?;let optional=self.optional_content.as_ref().map(|value|form::write_optional(out,value)).transpose()?;let preferences=self.viewer_preferences.as_ref().map(|value|metadata::write_preferences(out,value)).transpose()?;let open=self.open_action.as_ref().map(|value|navigation::write_open_action(out,value)).transpose()?;let mark=self.mark_info.as_ref().map(|value|metadata::write_mark(out,value)).transpose()?;let encryption=self.encryption.as_ref().map(|value|metadata::write_encryption(out,value)).transpose()?;let info=metadata::write_info(out,&self.info)?;let catalog=cos::write_dictionary(out,&self.catalog_extra)?;let trailer=cos::write_dictionary(out,&self.trailer)?;
        let ids=self.document_id.as_ref().map_or([C::Null;2],|value|[C::Blob(&value[0]),C::Blob(&value[1])]);
        out.insert_key("pdf_document",1,&[C::Text(&self.schema),C::Text(&self.declared_version),acro.map_or(C::Null,C::Integer),optional.map_or(C::Null,C::Integer),self.page_layout.map_or(C::Null,|value|C::Text(metadata::page_layout(value))),self.page_mode.map_or(C::Null,|value|C::Text(metadata::page_mode(value))),preferences.map_or(C::Null,C::Integer),open.map_or(C::Null,C::Integer),text_cell(&self.language),mark.map_or(C::Null,C::Integer),text_cell(&self.metadata),ids[0],ids[1],encryption.map_or(C::Null,C::Integer),C::Integer(info),C::Integer(catalog),C::Integer(trailer)])?;
        write_collection(out,"pdf_document_page",&self.pages,write_page)?;
        write_collection(out,"pdf_document_font",&self.fonts,font::write_font)?;
        write_collection(out,"pdf_document_image",&self.images,resource::write_image)?;
        write_collection(out,"pdf_document_form",&self.forms,resource::write_form)?;
        write_collection(out,"pdf_document_state",&self.ext_g_states,resource::write_state)?;
        write_collection(out,"pdf_document_shading",&self.shadings,resource::write_shading)?;
        write_collection(out,"pdf_document_pattern",&self.patterns,resource::write_pattern)?;
        write_collection(out,"pdf_document_color",&self.color_spaces,write_colors)?;
        write_collection(out,"pdf_document_properties",&self.properties,write_properties)?;
        write_collection(out,"pdf_document_outline",&self.outlines,|out,value|navigation::write_outline(out,value))?;
        write_collection(out,"pdf_document_destination",&self.named_destinations,navigation::write_named_destination)?;
        write_collection(out,"pdf_document_label",&self.page_labels,navigation::write_label)?;
        write_collection(out,"pdf_document_file",&self.embedded_files,metadata::write_file)?;
        write_collection(out,"pdf_document_intent",&self.output_intents,metadata::write_intent)?;
        write_collection(out,"pdf_document_object",&self.objects,write_indirect)?;
        Ok(())
    }
}
