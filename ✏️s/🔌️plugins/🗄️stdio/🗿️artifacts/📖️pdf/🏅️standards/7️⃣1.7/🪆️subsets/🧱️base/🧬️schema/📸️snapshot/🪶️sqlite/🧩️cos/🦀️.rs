//! 🧩️ PDF COS values, ordered dictionary members and explicitly typed stream filters.
use super::*;

pub(super) fn write_filters(out: &mut Projection<'_, '_>, filters: &[PdfStreamFilter]) -> Result<i64, String> {
    let chain = out.insert("pdf_filter_chain", &[])?;
    for (ordinal, filter) in filters.iter().enumerate() {
        let mut fields = [C::Null; 4];
        let (kind, predictor) = match filter {
            PdfStreamFilter::Flate { predictor } => ("flate", predictor.as_ref()),
            PdfStreamFilter::Lzw { predictor, early_change } => { fields[0] = C::Integer(i64::from(*early_change)); ("lzw", predictor.as_ref()) },
            PdfStreamFilter::AsciiHex => ("asciiHex", None), PdfStreamFilter::Ascii85 => ("ascii85", None), PdfStreamFilter::RunLength => ("runLength", None),
            PdfStreamFilter::Dct { color_transform } => { fields[1] = color_transform.map_or(C::Null, |value| C::Integer(i64::from(value))); ("dct", None) },
            PdfStreamFilter::Jpx => ("jpx", None), PdfStreamFilter::Ccitt { .. } => ("ccitt", None),
            PdfStreamFilter::Jbig2 { globals } => { fields[2] = globals.as_deref().map_or(C::Null, C::Blob); ("jbig2", None) },
            PdfStreamFilter::Crypt { name } => { fields[3] = name.as_deref().map_or(C::Null, C::Text); ("crypt", None) },
        };
        let mut cells = vec![C::Integer(chain), C::Integer(ordinal as i64), C::Text(kind)]; cells.extend(fields);
        let key = out.insert("pdf_stream_filter", &cells)?;
        if let Some(predictor) = predictor { out.insert_key("pdf_filter_predictor", key, &[C::Integer(i64::from(predictor.predictor)), C::Integer(i64::from(predictor.colors)), C::Integer(i64::from(predictor.bits_per_component)), C::Integer(i64::from(predictor.columns))])?; }
        if let PdfStreamFilter::Ccitt { parameters } = filter { out.insert_key("pdf_filter_ccitt", key, &[C::Integer(i64::from(parameters.k)), C::Integer(i64::from(parameters.columns)), C::Integer(i64::from(parameters.rows)), C::Integer(i64::from(parameters.black_is_1)), C::Integer(i64::from(parameters.encoded_byte_align)), C::Integer(i64::from(parameters.end_of_line)), C::Integer(i64::from(parameters.end_of_block)), C::Integer(i64::from(parameters.damaged_rows_before_error))])?; }
    }
    Ok(chain)
}

pub(super) fn read_filters(reader: &mut Reader<'_, '_, '_>, chain: i64) -> Result<Vec<PdfStreamFilter>, String> {
    reader.take("pdf_filter_chain", chain, 1)?;
    let mut filters = Vec::new();
    for row in reader.children("pdf_stream_filter", 1, 2, chain)? {
        let row = reader.take("pdf_stream_filter", row.rowid, 8)?;
        let predictor = if reader.has("pdf_filter_predictor", row.rowid) { let value = reader.take("pdf_filter_predictor", row.rowid, 5)?; Some(PdfPredictor { predictor: integer(value, 1)?, colors: integer(value, 2)?, bits_per_component: integer(value, 3)?, columns: integer(value, 4)? }) } else { None };
        let kind = row.text(3)?;
        if predictor.is_some() && !matches!(kind, "flate" | "lzw") { return Err("PDF predictor belongs only to Flate or LZW filters".into()); }
        let present: &[usize] = match kind { "lzw" => &[4], "dct" => &[5], "jbig2" => &[6], "crypt" => &[7], _ => &[] };
        null_except(row, 4..8, present)?;
        filters.push(match kind {
            "flate" => PdfStreamFilter::Flate { predictor }, "lzw" => PdfStreamFilter::Lzw { predictor, early_change: boolean(row, 4)? },
            "asciiHex" => PdfStreamFilter::AsciiHex, "ascii85" => PdfStreamFilter::Ascii85, "runLength" => PdfStreamFilter::RunLength,
            "dct" => PdfStreamFilter::Dct { color_transform: optional_integer(row, 5)?.map(|value| u32::try_from(value).map_err(|_| "PDF color transform exceeds u32".to_string())).transpose()? },
            "jpx" => PdfStreamFilter::Jpx,
            "ccitt" => { let value = reader.take("pdf_filter_ccitt", row.rowid, 9)?; PdfStreamFilter::Ccitt { parameters: PdfCcittParameters { k: integer(value, 1)?, columns: integer(value, 2)?, rows: integer(value, 3)?, black_is_1: boolean(value, 4)?, encoded_byte_align: boolean(value, 5)?, end_of_line: boolean(value, 6)?, end_of_block: boolean(value, 7)?, damaged_rows_before_error: integer(value, 8)? } } },
            "jbig2" => PdfStreamFilter::Jbig2 { globals: if row.values[6] == V::Null { None } else { Some(reader.blob(row,6)?) } },
            "crypt" => PdfStreamFilter::Crypt { name: reader.optional_text(row,7)? },
            _ => return Err("unknown PDF stream filter kind".into()),
        });
    }
    Ok(filters)
}

enum WriteChildren<'a>{Array{key:i64,values:std::iter::Enumerate<std::slice::Iter<'a,PdfObject>>},Dictionary{key:i64,entries:std::iter::Enumerate<std::slice::Iter<'a,PdfDictEntry>>}}
impl<'a> WriteChildren<'a>{
    fn next(&mut self)->Option<(i64,usize,Option<&'a str>,&'a PdfObject)>{match self{Self::Array{key,values}=>values.next().map(|(ordinal,value)|(*key,ordinal,None,value)),Self::Dictionary{key,entries}=>entries.next().map(|(ordinal,entry)|(*key,ordinal,Some(entry.key.as_str()),&entry.value))}}
}
fn write_children(out:&mut Projection<'_,'_>,root:WriteChildren<'_>)->Result<(),String>{
    let mut pending=vec![root];
    while let Some(frame)=pending.last_mut(){
        if let Some((parent,ordinal,name,value))=frame.next(){
            let key=write_shallow(out,value)?;let ordinal=i64::try_from(ordinal).map_err(|_|"PDF member ordinal exceeds i64")?;
            if let Some(name)=name{out.insert("pdf_cos_dictionary_entry",&[C::Integer(parent),C::Integer(ordinal),C::Text(name),C::Integer(key)])?;}else{out.insert("pdf_cos_array_element",&[C::Integer(parent),C::Integer(ordinal),C::Integer(key)])?;}
            if let Some(children)=object_children(value,key){pending.push(children);}
        }else{pending.pop();}
    }
    Ok(())
}
fn object_children(value:&PdfObject,key:i64)->Option<WriteChildren<'_>>{match value{PdfObject::Array(values)=>Some(WriteChildren::Array{key,values:values.iter().enumerate()}),PdfObject::Dict(entries)|PdfObject::Stream{dict:entries,..}=>Some(WriteChildren::Dictionary{key,entries:entries.iter().enumerate()}),_=>None}}
pub(super) fn write_dictionary(out:&mut Projection<'_,'_>,entries:&[PdfDictEntry])->Result<i64,String>{
    let key=out.insert("pdf_cos_value",&[C::Text("dictionary"),C::Null,C::Null,C::Null,C::Null,C::Null,C::Null,C::Null,C::Null,C::Null,C::Null,C::Null])?;
    write_children(out,WriteChildren::Dictionary{key,entries:entries.iter().enumerate()})?;Ok(key)
}
pub(super) fn write_object(out:&mut Projection<'_,'_>,value:&PdfObject)->Result<i64,String>{let key=write_shallow(out,value)?;if let Some(children)=object_children(value,key){write_children(out,children)?;}Ok(key)}
fn write_shallow(out:&mut Projection<'_,'_>,value:&PdfObject)->Result<i64,String>{
    let mut fields = [C::Null; 11];
    let kind = match value {
        PdfObject::Null => "null", PdfObject::Bool(value) => { fields[0] = C::Integer(i64::from(*value)); "boolean" },
        PdfObject::Int(value) => { fields[1] = C::Integer(*value); "integer" },
        PdfObject::Real(value) => { fields[2] = C::Integer(i64::from(value.negative)); fields[3] = C::Text(&value.coefficient); fields[4] = C::Integer(i64::from(value.scale)); "decimal" },
        PdfObject::Str(value) => { fields[5] = C::Blob(value); "string" }, PdfObject::Name(value) => { fields[6] = C::Text(value); "name" },
        PdfObject::Array(_) => "array", PdfObject::Dict(_) => "dictionary",
        PdfObject::Ref(value) => { fields[7] = C::Integer(i64::from(value.num)); fields[8] = C::Integer(i64::from(value.gen)); "reference" },
        PdfObject::Stream { data, filters, .. } => { fields[9] = C::Blob(data); fields[10] = C::Integer(write_filters(out, filters)?); "stream" },
    };
    let mut cells = vec![C::Text(kind)]; cells.extend(fields);
    out.insert("pdf_cos_value", &cells)
}
pub(super) fn read_dictionary(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<Vec<PdfDictEntry>,String>{match read_object(reader,key)?{PdfObject::Dict(entries)=>Ok(entries),_=>Err("PDF relationship requires a dictionary value".into())}}
fn read_children<'a>(reader:&mut Reader<'a,'_,'_>,key:i64,value:&PdfObject)->Result<std::vec::IntoIter<&'a RawRow>,String>{match value{PdfObject::Array(_)=>reader.children("pdf_cos_array_element",1,2,key),PdfObject::Dict(_)|PdfObject::Stream{..}=>reader.children("pdf_cos_dictionary_entry",1,2,key),_=>Ok(Vec::new())}.map(Vec::into_iter)}
pub(super) fn read_object(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfObject,String>{
    let value=read_shallow(reader,key)?;let children=read_children(reader,key,&value)?;let mut pending=vec![(value,children,None::<String>)];
    loop{
        let child=pending.last_mut().and_then(|(value,children,_)|children.next().map(|row|(matches!(value,PdfObject::Array(_)),row)));
        if let Some((array,row))=child{
            let row=reader.take(if array{"pdf_cos_array_element"}else{"pdf_cos_dictionary_entry"},row.rowid,if array{4}else{5})?;
            let name=if array{None}else{Some(reader.text(row,3)?)};let key=row.integer(if array{3}else{4})?;let value=read_shallow(reader,key)?;let children=read_children(reader,key,&value)?;pending.push((value,children,name));
        }else{
            let(value,_,name)=pending.pop().ok_or("PDF COS reconstruction stack is empty")?;
            if let Some((parent,_,_))=pending.last_mut(){match parent{PdfObject::Array(values)=>{if name.is_some(){return Err("PDF array member has a dictionary key".into());}values.push(value);},PdfObject::Dict(entries)|PdfObject::Stream{dict:entries,..}=>entries.push(PdfDictEntry{key:name.ok_or("PDF dictionary member has no key")?,value}),_=>return Err("PDF COS scalar cannot own child values".into())}}else{return Ok(value);}
        }
    }
}
fn read_shallow(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfObject,String>{
    reader.control.checkpoint(Phase::ReconstructSnapshot, reader.used.len(), reader.total)?;
    let row = reader.take("pdf_cos_value", key, 13)?;
    let kind = row.text(1)?;
    let present: &[usize] = match kind { "null" | "array" | "dictionary" => &[], "boolean" => &[2], "integer" => &[3], "decimal" => &[4, 5, 6], "string" => &[7], "name" => &[8], "reference" => &[9, 10], "stream" => &[11, 12], _ => return Err("unknown PDF COS value kind".into()) };
    null_except(row, 2..13, present)?;
    Ok(match kind {
        "null" => PdfObject::Null, "boolean" => PdfObject::Bool(boolean(row, 2)?), "integer" => PdfObject::Int(row.integer(3)?),
        "decimal" => PdfObject::Real(PdfDecimal { negative: boolean(row, 4)?, coefficient: reader.text(row,5)?, scale: integer(row, 6)? }),
        "string" => PdfObject::Str(reader.blob(row,7)?), "name" => PdfObject::Name(reader.text(row,8)?),
        "reference" => PdfObject::Ref(ObjRef { num: integer(row, 9)?, gen: integer(row, 10)? }),
        "array" => PdfObject::Array(Vec::new()),
        "dictionary" => PdfObject::Dict(Vec::new()),
        "stream" => PdfObject::Stream { dict: Vec::new(), data: reader.blob(row,11)?, filters: read_filters(reader, row.integer(12)?)? },
        _ => unreachable!(),
    })
}
