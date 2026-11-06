//! 🚦️ Controlled existing JSON text and raw value-Pack carriers of space history.
use super::*;
use semio_framework_dsl_record::native_encoding::EncodedRecord;
use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::NativeSchemaControl;
use semio_framework_dsl_record::RecordLayout;
use semio_framework_dsl_record::RecordSpec;
use semio_framework_dsl_record::Shape;
use semio_framework_value::{DslValue, FromValue, NativeEncodeControl, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
fn spec<C: NativeSchemaControl>(c: &mut C) -> Result<RecordSpec, ValueError> {
    let mut fields = c.allocate_vec(1)?;
    fields.push(semio_framework_dsl_record::producer::field(1, "value", semio_framework_dsl_record::Shape::Value, c)?);
    semio_framework_dsl_record::producer::record(None, semio_framework_dsl_record::RecordLayout::Lines, fields, c)
}
fn add(left: usize, right: usize) -> Result<usize, ValueError> {
    left.checked_add(right).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "space row count overflow"))
}
fn object<const N: usize>(fields: [(&str, DslValue); N], c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    let mut values = c.allocate_vec(N)?;
    for (key, value) in fields {
        values.push((c.copy_text(key)?, value));
    }
    Ok(DslValue::Object(values))
}
fn text(value: &str, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    Ok(DslValue::String(c.copy_text(value)?))
}
fn project(value: &SpaceHistorySnapshot, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    c.begin_stage(0)?;
    let mut checkpoints = c.allocate_vec(value.checkpoints.len())?;
    for row in &value.checkpoints {
        let mut authors = c.allocate_vec(row.authors.len())?;
        for author in &row.authors {
            let mut fields = c.allocate_vec(if author.avatar.is_some() { 3 } else { 2 })?;
            fields.push((c.copy_text("id")?, text(&author.id, c)?));
            fields.push((c.copy_text("name")?, text(&author.name, c)?));
            if let Some(avatar) = &author.avatar {
                fields.push((c.copy_text("avatar")?, text(avatar, c)?));
            }
            authors.push(DslValue::Object(fields));
            c.step()?;
        }
        let timestamp = object([("actor", DslValue::uint(row.timestamp.actor)), ("physical_ms", DslValue::uint(row.timestamp.physical_ms)), ("logical", DslValue::uint(row.timestamp.logical))], c)?;
        let mut members = c.allocate_vec(row.members.len())?;
        for pin in &row.members {
            members.push(object([("documentId", text(&pin.document_id, c)?), ("checkpointId", text(&pin.checkpoint_id, c)?), ("alternativeId", text(&pin.alternative_id, c)?)], c)?);
            c.step()?;
        }
        let mut fields = c.allocate_vec(if row.parent_id.is_some() { 6 } else { 5 })?;
        fields.push((c.copy_text("id")?, text(&row.id, c)?));
        if let Some(parent) = &row.parent_id {
            fields.push((c.copy_text("parentId")?, text(parent, c)?));
        }
        fields.push((c.copy_text("message")?, text(&row.message, c)?));
        fields.push((c.copy_text("authors")?, DslValue::Array(authors)));
        fields.push((c.copy_text("timestamp")?, timestamp));
        fields.push((c.copy_text("members")?, DslValue::Array(members)));
        checkpoints.push(DslValue::Object(fields));
        c.step()?;
    }
    let mut alternatives = c.allocate_vec(value.alternatives.len())?;
    for row in &value.alternatives {
        let mut pins = c.allocate_vec(row.checkpoint_ids.len())?;
        for pin in &row.checkpoint_ids {
            pins.push(text(pin, c)?);
            c.step()?;
        }
        alternatives.push(object([("id", text(&row.id, c)?), ("name", text(&row.name, c)?), ("checkpointIds", DslValue::Array(pins))], c)?);
        c.step()?;
    }
    let mut fields = c.allocate_vec(if value.active_alternative_id.is_some() { 3 } else { 2 })?;
    fields.push((c.copy_text("checkpoints")?, DslValue::Array(checkpoints)));
    fields.push((c.copy_text("alternatives")?, DslValue::Array(alternatives)));
    if let Some(active) = &value.active_alternative_id {
        fields.push((c.copy_text("activeAlternativeId")?, text(active, c)?));
    }
    Ok(DslValue::Object(fields))
}
fn retire_projection(value: DslValue) {
    let mut retirement = semio_framework_value::IntrinsicRetirement::new(value);
    while !retirement.terminal_is_empty() { retirement.close_step(256); }
}
fn retire_record(record: semio_framework_dsl_record::RecordValue) {
    for value in record.fields.into_values() {
        match value {
            FieldValue::Value(value) => retire_projection(value),
            _ => unreachable!("History producer emits only the declared intrinsic field"),
        }
    }
}

fn field<'a>(value: &'a DslValue, key: &str) -> Result<&'a DslValue, ValueError> {
    let DslValue::Object(fields) = value else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "space history native object is required")) };
    fields.iter().find(|(name, _)| name == key).map(|(_, value)| value).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, format!("space history native field {key} is required")))
}
fn list(value: &DslValue) -> Result<&[DslValue], ValueError> {
    match value {
        DslValue::Array(values) => Ok(values),
        _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "space history native list is required")),
    }
}
fn text_into(value:&DslValue,output:&mut String,c:&mut NativeDecodeControl<'_>,body:&mut semio_framework_io_control::NativeSnapshotBodyWallet)->Result<(),ValueError>{
    let DslValue::String(text)=value else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"space history native text is required"))};
    body.copy_text_into(c,text,output,7)
}
fn string_into(value:&DslValue,key:&str,output:&mut String,c:&mut NativeDecodeControl<'_>,body:&mut semio_framework_io_control::NativeSnapshotBodyWallet)->Result<(),ValueError>{text_into(field(value,key)?,output,c,body)}
fn inline(body:&mut semio_framework_io_control::NativeSnapshotBodyWallet,bytes:usize,depth:usize,operation:impl FnOnce())->Result<(),ValueError>{body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:depth})?;operation();body.record_progress(semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})}
fn push<T>(output:&mut Vec<T>,body:&mut semio_framework_io_control::NativeSnapshotBodyWallet,depth:usize,value:impl FnOnce()->T)->Result<(),ValueError>{if output.len()==output.capacity()&&size_of::<T>()>0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"space history child slot has no admitted backing"))}inline(body,size_of::<T>(),depth,||output.push(value()))}
fn optional_into(value:&DslValue,key:&str,output:&mut Option<String>,c:&mut NativeDecodeControl<'_>,body:&mut semio_framework_io_control::NativeSnapshotBodyWallet)->Result<(),ValueError>{
    let DslValue::Object(fields)=value else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"space history native object is required"))};
    match fields.iter().find(|(name,_)|name==key).map(|(_,value)|value){
        None|Some(DslValue::Null)=>Ok(()),
        Some(value)=>{inline(body,size_of::<Option<String>>(),6,||*output=Some(String::new()))?;text_into(value,output.as_mut().unwrap(),c,body)}
    }
}
fn construct(value:&DslValue,output:&mut Option<SpaceHistorySnapshot>,c:&mut NativeDecodeControl<'_>,body:&mut semio_framework_io_control::NativeSnapshotBodyWallet)->Result<(),ValueError>{
    c.begin_stage(0)?;
    inline(body,size_of::<SpaceHistorySnapshot>(),2,||*output=Some(SpaceHistorySnapshot::default()))?;
    let output=output.as_mut().unwrap();
    body.allocate_vec_into(c,list(field(value,"checkpoints")?)?.len(),&mut output.checkpoints,3)?;
    for row in list(field(value,"checkpoints")?)?{
        push(&mut output.checkpoints,body,4,||SpaceCheckpoint{id:String::new(),parent_id:None,message:String::new(),authors:Vec::new(),timestamp:HybridLogicalTimestamp{actor:0,physical_ms:0,logical:0},members:Vec::new()})?;
        let checkpoint=output.checkpoints.last_mut().unwrap();
        string_into(row,"id",&mut checkpoint.id,c,body)?;
        optional_into(row,"parentId",&mut checkpoint.parent_id,c,body)?;
        string_into(row,"message",&mut checkpoint.message,c,body)?;
        body.allocate_vec_into(c,list(field(row,"authors")?)?.len(),&mut checkpoint.authors,5)?;
        for author in list(field(row,"authors")?)?{
            push(&mut checkpoint.authors,body,6,||Author{id:String::new(),name:String::new(),avatar:None})?;
            let output=checkpoint.authors.last_mut().unwrap();
            string_into(author,"id",&mut output.id,c,body)?;
            string_into(author,"name",&mut output.name,c,body)?;
            optional_into(author,"avatar",&mut output.avatar,c,body)?;
            c.step()?;
        }
        let clock=field(row,"timestamp")?;
        for(key,output)in[("actor",&mut checkpoint.timestamp.actor),("physical_ms",&mut checkpoint.timestamp.physical_ms),("logical",&mut checkpoint.timestamp.logical)]{let value=u64::from_value_controlled(field(clock,key)?,c)?;inline(body,size_of::<u64>(),5,||*output=value)?;}
        body.allocate_vec_into(c,list(field(row,"members")?)?.len(),&mut checkpoint.members,5)?;
        for pin in list(field(row,"members")?)?{
            push(&mut checkpoint.members,body,6,||SpaceMemberPin{document_id:String::new(),checkpoint_id:String::new(),alternative_id:String::new()})?;
            let output=checkpoint.members.last_mut().unwrap();
            string_into(pin,"documentId",&mut output.document_id,c,body)?;
            string_into(pin,"checkpointId",&mut output.checkpoint_id,c,body)?;
            if field(pin,"alternativeId").is_ok(){string_into(pin,"alternativeId",&mut output.alternative_id,c,body)?;}
            c.step()?;
        }
        c.step()?;
    }
    body.allocate_vec_into(c,list(field(value,"alternatives")?)?.len(),&mut output.alternatives,3)?;
    for row in list(field(value,"alternatives")?)?{
        push(&mut output.alternatives,body,4,||SpaceAlternative{id:String::new(),name:String::new(),checkpoint_ids:Vec::new()})?;
        let alternative=output.alternatives.last_mut().unwrap();
        string_into(row,"id",&mut alternative.id,c,body)?;
        string_into(row,"name",&mut alternative.name,c,body)?;
        body.allocate_vec_into(c,list(field(row,"checkpointIds")?)?.len(),&mut alternative.checkpoint_ids,5)?;
        for pin in list(field(row,"checkpointIds")?)?{
            push(&mut alternative.checkpoint_ids,body,6,String::new)?;
            text_into(pin,alternative.checkpoint_ids.last_mut().unwrap(),c,body)?;
            c.step()?;
        }
        c.step()?;
    }
    optional_into(value,"activeAlternativeId",&mut output.active_alternative_id,c,body)
}
fn string_size(text: &str, c: &mut NativeEncodeControl<'_>) -> Result<usize, ValueError> {
    c.scoped_stage(|c| {
        c.begin_stage(text.len())?;
        let mut bytes = 2usize;
        for character in text.chars() {
            let size = match character {
                '\\' | '"' | '\n' | '\r' | '\t' | '\u{0008}' | '\u{000c}' => 2,
                character if character < ' ' => 6,
                _ => character.len_utf8(),
            };
            bytes = add(bytes, size)?;
            c.advance(character.len_utf8())?;
        }
        Ok(bytes)
    })
}
fn json_size(value: &DslValue, c: &mut NativeEncodeControl<'_>) -> Result<usize, ValueError> {
    c.step()?;
    match value {
        DslValue::String(text) => string_size(text, c),
        DslValue::Number(semio_framework_value::Number::UInt(value)) => {
            let mut count = 1;
            let mut value = *value;
            while value >= 10 {
                value /= 10;
                count += 1;
            }
            Ok(count)
        }
        DslValue::Array(values) => {
            let mut count = add(2, values.len().saturating_sub(1))?;
            for value in values {
                count = add(count, json_size(value, c)?)?;
            }
            Ok(count)
        }
        DslValue::Object(values) => {
            let mut count = add(2, values.len().saturating_sub(1))?;
            for (key, value) in values {
                count = add(count, add(string_size(key, c)?, add(1, json_size(value, c)?)?)?)?;
            }
            Ok(count)
        }
        _ => Err(ValueError::new(ValueRefusalKind::InvariantViolated, "space history projector emitted an undeclared native primitive")),
    }
}
fn encoded_effect(native:&mut NativeEncodeControl<'_>,body:&mut semio_framework_io_control::NativeSnapshotBodyWallet,marker:&mut semio_framework_io_control::NativeSnapshotEffectProgress,unwind:&mut Option<semio_framework_io_control::NativeSnapshotFailure>,bytes:usize,depth:usize,effect:impl FnOnce())->Result<(),semio_framework_io_control::NativeSnapshotFailure>{
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
    let quote=semio_framework_io_control::NativeSnapshotEncodeEffectQuote::encode(body,native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:depth,..Default::default()})?;let mut effect=Some(effect);
    let turn=quote.run(marker,unwind,&mut |performed,physical|{effect.take().unwrap()();*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});turn.result?;marker.reset_completed()?;Ok(())
}
fn encoded_vector<T>(native:&mut NativeEncodeControl<'_>,body:&mut semio_framework_io_control::NativeSnapshotBodyWallet,marker:&mut semio_framework_io_control::NativeSnapshotEffectProgress,unwind:&mut Option<semio_framework_io_control::NativeSnapshotFailure>,allocation:&mut semio_framework_io_control::NativeSnapshotEncodeAllocation,output:&mut Vec<T>,count:usize,depth:usize)->Result<(),semio_framework_io_control::NativeSnapshotFailure>{
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
    if !output.is_empty()||(size_of::<T>()!=0&&output.capacity()!=0){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"history projection vector slot must be empty").into())}
    if allocation.is_unsettled(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"history projection allocation state is unsettled").into())}
    let layout=std::alloc::Layout::array::<T>(count).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"history projection vector extent overflow"))?;let bytes=layout.size();
    let quote=semio_framework_io_control::NativeSnapshotEncodeEffectQuote::encode(body,native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:bytes,maximum_depth:depth,..Default::default()})?;
    if bytes==0{let turn=quote.run(marker,unwind,&mut |performed,physical|{*output=Vec::new();*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<T>>(),..Default::default()};Ok(())});turn.result?;marker.reset_completed()?;return Ok(())}let turn=quote.run_native(marker,unwind,&mut |native,performed,physical|{let outcome=native.allocate_effect(bytes,&mut allocation.actual,&mut allocation.unwind,&mut |actual|{let pointer=unsafe{std::alloc::alloc(layout)}.cast::<T>();if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"history projection vector allocation failed"))}*output=unsafe{Vec::from_raw_parts(pointer,0,count)};*actual=Some(bytes);*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<T>>(),retained_capacity_bytes:bytes,released_bytes:0};Ok(())});match allocation.capture(outcome,bytes){Ok(true)=>Ok(()),Ok(false)=>Ok(()),Err(outcome)=>{allocation.collision=Some(outcome);Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"history projection allocation custody is occupied"))}}});allocation.finish(turn.result).map_err(|error|error.with_retained_progress(body.progress()))?;marker.reset_completed()?;Ok(())
}
type ProjectError=semio_framework_io_control::NativeSnapshotFailure;
type ProjectWallet=semio_framework_io_control::NativeSnapshotBodyWallet;
fn object_into(value:&mut DslValue,count:usize,native:&mut NativeEncodeControl<'_>,body:&mut ProjectWallet,marker:&mut semio_framework_io_control::NativeSnapshotEffectProgress,unwind:&mut Option<ProjectError>,allocation:&mut semio_framework_io_control::NativeSnapshotEncodeAllocation,depth:usize)->Result<(),ProjectError>{
 encoded_effect(native,body,marker,unwind,size_of::<DslValue>(),depth,||*value=DslValue::Object(Vec::new()))?;
 let DslValue::Object(fields)=value else{unreachable!()};encoded_vector(native,body,marker,unwind,allocation,fields,count,depth+1)
}
fn array_into(value:&mut DslValue,count:usize,native:&mut NativeEncodeControl<'_>,body:&mut ProjectWallet,marker:&mut semio_framework_io_control::NativeSnapshotEffectProgress,unwind:&mut Option<ProjectError>,allocation:&mut semio_framework_io_control::NativeSnapshotEncodeAllocation,depth:usize)->Result<(),ProjectError>{
 encoded_effect(native,body,marker,unwind,size_of::<DslValue>(),depth,||*value=DslValue::Array(Vec::new()))?;
 let DslValue::Array(items)=value else{unreachable!()};encoded_vector(native,body,marker,unwind,allocation,items,count,depth+1)
}
fn member_into<'a>(value:&'a mut DslValue,key:&str,native:&mut NativeEncodeControl<'_>,body:&mut ProjectWallet,marker:&mut semio_framework_io_control::NativeSnapshotEffectProgress,unwind:&mut Option<ProjectError>,allocation:&mut semio_framework_io_control::NativeSnapshotEncodeAllocation,depth:usize)->Result<&'a mut DslValue,ProjectError>{
 let DslValue::Object(fields)=value else{unreachable!()};if fields.len()==fields.capacity(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"history projection object has no admitted slot").into())}
 encoded_effect(native,body,marker,unwind,size_of::<(String,DslValue)>(),depth,||fields.push((String::new(),DslValue::Null)))?;
 let (name,slot)=fields.last_mut().unwrap();body.copy_text_encoded(native,key,name,marker,unwind,allocation,depth+1)?;Ok(slot)
}
fn item_into<'a>(value:&'a mut DslValue,native:&mut NativeEncodeControl<'_>,body:&mut ProjectWallet,marker:&mut semio_framework_io_control::NativeSnapshotEffectProgress,unwind:&mut Option<ProjectError>,_allocation:&mut semio_framework_io_control::NativeSnapshotEncodeAllocation,depth:usize)->Result<&'a mut DslValue,ProjectError>{
 let DslValue::Array(items)=value else{unreachable!()};if items.len()==items.capacity(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"history projection array has no admitted slot").into())}
 encoded_effect(native,body,marker,unwind,size_of::<DslValue>(),depth,||items.push(DslValue::Null))?;Ok(items.last_mut().unwrap())
}
fn text_into_encoded(value:&mut DslValue,text:&str,native:&mut NativeEncodeControl<'_>,body:&mut ProjectWallet,marker:&mut semio_framework_io_control::NativeSnapshotEffectProgress,unwind:&mut Option<ProjectError>,allocation:&mut semio_framework_io_control::NativeSnapshotEncodeAllocation,depth:usize)->Result<(),ProjectError>{
 encoded_effect(native,body,marker,unwind,size_of::<DslValue>(),depth,||*value=DslValue::String(String::new()))?;let DslValue::String(output)=value else{unreachable!()};body.copy_text_encoded(native,text,output,marker,unwind,allocation,depth+1)
}
fn text_member(value:&mut DslValue,key:&str,text:&str,native:&mut NativeEncodeControl<'_>,body:&mut ProjectWallet,marker:&mut semio_framework_io_control::NativeSnapshotEffectProgress,unwind:&mut Option<ProjectError>,allocation:&mut semio_framework_io_control::NativeSnapshotEncodeAllocation,depth:usize)->Result<(),ProjectError>{let slot=member_into(value,key,native,body,marker,unwind,allocation,depth)?;text_into_encoded(slot,text,native,body,marker,unwind,allocation,depth+1)}
fn uint_member(value:&mut DslValue,key:&str,number:u64,native:&mut NativeEncodeControl<'_>,body:&mut ProjectWallet,marker:&mut semio_framework_io_control::NativeSnapshotEffectProgress,unwind:&mut Option<ProjectError>,allocation:&mut semio_framework_io_control::NativeSnapshotEncodeAllocation,depth:usize)->Result<(),ProjectError>{let slot=member_into(value,key,native,body,marker,unwind,allocation,depth)?;encoded_effect(native,body,marker,unwind,size_of::<DslValue>(),depth+1,||*slot=DslValue::uint(number))}
#[derive(semio_framework_value::RetireOwned)]
struct ProjectFrame{value:Option<DslValue>,allocation:semio_framework_io_control::NativeSnapshotEncodeAllocation}
fn project_into(value:&SpaceHistorySnapshot,frame:&mut ProjectFrame,native:&mut NativeEncodeControl<'_>,body:&mut ProjectWallet,marker:&mut semio_framework_io_control::NativeSnapshotEffectProgress,unwind:&mut Option<ProjectError>)->Result<(),ProjectError>{
 encoded_effect(native,body,marker,unwind,size_of::<Option<DslValue>>(),1,||frame.value=Some(DslValue::Null))?;let allocation=&mut frame.allocation;let root=frame.value.as_mut().unwrap();object_into(root,if value.active_alternative_id.is_some(){3}else{2},native,body,marker,unwind,allocation,2)?;
 let checkpoints=member_into(root,"checkpoints",native,body,marker,unwind,allocation,3)?;array_into(checkpoints,value.checkpoints.len(),native,body,marker,unwind,allocation,4)?;
 for row in &value.checkpoints{let entry=item_into(checkpoints,native,body,marker,unwind,allocation,5)?;object_into(entry,if row.parent_id.is_some(){6}else{5},native,body,marker,unwind,allocation,6)?;text_member(entry,"id",&row.id,native,body,marker,unwind,allocation,7)?;if let Some(parent)=&row.parent_id{text_member(entry,"parentId",parent,native,body,marker,unwind,allocation,7)?}text_member(entry,"message",&row.message,native,body,marker,unwind,allocation,7)?;
  let authors=member_into(entry,"authors",native,body,marker,unwind,allocation,7)?;array_into(authors,row.authors.len(),native,body,marker,unwind,allocation,8)?;for author in &row.authors{let item=item_into(authors,native,body,marker,unwind,allocation,9)?;object_into(item,if author.avatar.is_some(){3}else{2},native,body,marker,unwind,allocation,10)?;text_member(item,"id",&author.id,native,body,marker,unwind,allocation,11)?;text_member(item,"name",&author.name,native,body,marker,unwind,allocation,11)?;if let Some(avatar)=&author.avatar{text_member(item,"avatar",avatar,native,body,marker,unwind,allocation,11)?}}
  let clock=member_into(entry,"timestamp",native,body,marker,unwind,allocation,7)?;object_into(clock,3,native,body,marker,unwind,allocation,8)?;uint_member(clock,"actor",row.timestamp.actor,native,body,marker,unwind,allocation,9)?;uint_member(clock,"physical_ms",row.timestamp.physical_ms,native,body,marker,unwind,allocation,9)?;uint_member(clock,"logical",row.timestamp.logical,native,body,marker,unwind,allocation,9)?;
  let members=member_into(entry,"members",native,body,marker,unwind,allocation,7)?;array_into(members,row.members.len(),native,body,marker,unwind,allocation,8)?;for pin in &row.members{let item=item_into(members,native,body,marker,unwind,allocation,9)?;object_into(item,3,native,body,marker,unwind,allocation,10)?;text_member(item,"documentId",&pin.document_id,native,body,marker,unwind,allocation,11)?;text_member(item,"checkpointId",&pin.checkpoint_id,native,body,marker,unwind,allocation,11)?;text_member(item,"alternativeId",&pin.alternative_id,native,body,marker,unwind,allocation,11)?}
 }
 let alternatives=member_into(root,"alternatives",native,body,marker,unwind,allocation,3)?;array_into(alternatives,value.alternatives.len(),native,body,marker,unwind,allocation,4)?;for row in &value.alternatives{let entry=item_into(alternatives,native,body,marker,unwind,allocation,5)?;object_into(entry,3,native,body,marker,unwind,allocation,6)?;text_member(entry,"id",&row.id,native,body,marker,unwind,allocation,7)?;text_member(entry,"name",&row.name,native,body,marker,unwind,allocation,7)?;let pins=member_into(entry,"checkpointIds",native,body,marker,unwind,allocation,7)?;array_into(pins,row.checkpoint_ids.len(),native,body,marker,unwind,allocation,8)?;for pin in &row.checkpoint_ids{let item=item_into(pins,native,body,marker,unwind,allocation,9)?;text_into_encoded(item,pin,native,body,marker,unwind,allocation,10)?}}
 if let Some(active)=&value.active_alternative_id{text_member(root,"activeAlternativeId",active,native,body,marker,unwind,allocation,3)?}Ok(())
}
#[derive(semio_framework_value::RetireOwned)]
struct EncodeFrame {
    value:Option<DslValue>,
    spec:Option<RecordSpec>,
    record:Option<semio_framework_dsl_record::RecordValue>,
}
struct BorrowedJson<'a>(&'a DslValue);
impl semio_framework_pack_json::JsonWriteSource for BorrowedJson<'_> {
    fn node_at_path(&self, path: &[usize]) -> Result<semio_framework_pack_json::JsonWriteNode<'_>, ValueError> {
        semio_framework_pack_json::JsonWriteSource::node_at_path(self.0, path)
    }
    fn object_key_at_path(&self, path: &[usize], index: usize) -> Result<&str, ValueError> {
        semio_framework_pack_json::JsonWriteSource::object_key_at_path(self.0, path, index)
    }
}
#[derive(semio_framework_value::RetireOwned)]
struct JsonEncodeFrame{
 writer:semio_framework_pack_json::JsonBorrowedWriteCursor,
 settlement:Option<semio_framework_io_control::NativeSnapshotEncodeSettlement>,
 receipt:Option<semio_framework_value::native_encoding::NativeEncodeEffectReceipt>,
 cancellation:Option<semio_framework_value::native_encoding::NativeEncodeEffectCancellation>,
 text:Option<String>,
 failure:Option<ValueError>,
 receipt_failure:Option<ValueError>,
}
fn write_json(value:&DslValue,owner:&mut semio_framework_io_control::NativeSnapshotEncodeOwner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<semio_framework_io_schema::IoPayload>>)->semio_framework_value::RetainedWorkOutcome<(),semio_framework_io_control::NativeSnapshotFailure>{
 use semio_framework_io_control::{NativeSnapshotEncodeSettlement,NativeSnapshotFailure};use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome};
 let mut cursor=Some(JsonEncodeFrame{writer:semio_framework_pack_json::JsonBorrowedWriteCursor::new(),settlement:None,receipt:None,cancellation:None,text:None,failure:None,receipt_failure:None});
 owner.drive_cursor_receipted(&mut cursor,output,&mut |frame,output,_marker,_failure,owner|{
  let mut work=0;let result=(||->Result<(),NativeSnapshotFailure>{
   if frame.settlement.is_some()||frame.receipt.is_some()||frame.cancellation.is_some()||frame.receipt_failure.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON original settlement requires recovery").into())}
   if frame.text.is_some(){
    if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON output slot is occupied").into())}
    let bytes=size_of::<Option<String>>()+size_of::<Option<crate::lifted_serialization::SnapshotReceivingOutput<semio_framework_io_schema::IoPayload>>>();owner.begin_original_settlement(&mut frame.settlement,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;
    *output=Some(Ok(semio_framework_io_schema::IoPayload::Text(frame.text.take().unwrap())));work=1;if let Err(error)=frame.settlement.as_mut().unwrap().record_effect(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}){frame.receipt_failure=Some(error)}owner.settle_original_effect(&mut frame.settlement)?;let _=NativeSnapshotEncodeSettlement::take_receipt(&mut frame.settlement)?;return Ok(())
   }
   let available=owner.remaining_grant();let demand=frame.writer.normal_step_demands_with_copy(&BorrowedJson(value),available.maximum_copy_bytes)?;let output_handoff=frame.writer.phase==2&&frame.writer.frames.is_empty();let extra=RetainedCloneProgress{copied_bytes:if output_handoff{size_of::<Option<String>>()}else{0},..Default::default()};let copy_bytes=demand.copy_bytes.checked_add(extra.copied_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"JSON output handoff receipt overflow"))?;owner.begin_original_settlement(&mut frame.settlement,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth})?;frame.writer.bind_original_turn(frame.settlement.as_ref().unwrap().original().unwrap(),extra)?;
   let step=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||frame.writer.step(&BorrowedJson(value),1,owner.native(),available)));
   match step{Ok(Ok(Some(text)))=>frame.text=Some(text),Ok(Ok(None))=>{},Ok(Err(error))=>frame.failure=Some(error),Err(payload)=>{if let Err(payload)=frame.settlement.as_mut().unwrap().retain_operation_unwind(payload){std::panic::resume_unwind(payload)}}}
   work=frame.writer.last_turn_work();if work==0{frame.cancellation=Some(owner.cancel_original_settlement(&mut frame.settlement)?);let token=frame.cancellation.take().unwrap();if let Err((error,token))=frame.writer.acknowledge_canceled_turn(token){frame.cancellation=Some(token);return Err(error.into())}}else{let progress=frame.writer.normal_step_progress().checked_add(extra)?;if let Err(error)=frame.settlement.as_mut().unwrap().record_effect(progress){frame.receipt_failure=Some(error)}owner.settle_original_effect(&mut frame.settlement)?;frame.receipt=Some(NativeSnapshotEncodeSettlement::take_receipt(&mut frame.settlement)?);let token=frame.receipt.take().unwrap();if let Err((error,token))=frame.writer.acknowledge_posted_turn(token){frame.receipt=Some(token);return Err(error.into())}}
   if frame.settlement.as_ref().is_some_and(NativeSnapshotEncodeSettlement::has_operation_unwind){NativeSnapshotEncodeSettlement::resume_operation_unwind(&mut frame.settlement)?}if let Some(error)=frame.failure.take(){return Err(error.into())}Ok(())
  })();RetainedWorkOutcome{result,work}
 })
}

pub(super) fn encode(value: &SpaceHistorySnapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>, native_owner:&mut semio_framework_io_control::NativeSnapshotEncodeOwner<'_, '_>) -> Result<semio_framework_io_schema::IoPayload, ValueError> {
    let limits = control.limits();
    let result=control.allocation_stage_native(SqliteSnapshotPhase::EncodeNative, |remaining, checkpoint| {
        let mut progress = |event: semio_framework_value::native_encoding::NativeEncodeProgress| checkpoint(event.completed, event.total);
        let before = native_owner.native().owned_bytes();
        let maximum = before.checked_add(remaining).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit,"space history native allocation allowance overflow"));
        let result = maximum.and_then(|maximum| native_owner.scoped_native(maximum, &mut progress, |owner| {
            let value = semio_framework_value::DecodedValue::new(project(value, owner.native())?, retire_projection);
            match encoding {
                SnapshotEncoding::Text => {
                    owner.native().begin_stage(0)?;
                    if json_size(value.get(), owner.native())? > limits.max_file_bytes {
                        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "space history native text exceeds file limit"));
                    }
                    write_json(value.get(), owner).map(semio_framework_io_schema::IoPayload::Text)
                }
                SnapshotEncoding::Binary => {
                    let c = owner.native();
                    let spec = spec(c)?;
                    let mut record = EncodedRecord::new(1, c)?;
                    record.insert(1, FieldValue::Value(value.take()))?;
                    let record = semio_framework_value::DecodedValue::new(record.take(), retire_record);
                    let mut options = pack::record::EncodeOptions::default();
                    options.limits.max_file_len = limits.max_file_bytes as u64;
                    let bytes = pack::record::encode_document_controlled(&spec, record.get(), &options, c).map_err(semio_os_pack::PackRefusal::into_value_error)?;
                    drop(record);
                    Ok(semio_framework_io_schema::IoPayload::Binary(bytes))
                }
            }
        }));
        (result, native_owner.native().owned_bytes() - before)
    });
    result?
}

pub(super) fn decode(payload:&semio_framework_io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_io_control::NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<SpaceHistorySnapshot>>)->crate::lifted_serialization::SnapshotReceivingOutcome{
    let limits=control.limits();
    decode_with(payload,control,native_owner,output,|value,output,native,body|{admission::intrinsic(value,native,limits)?;construct(value,output,native,body)})
}

pub fn decode_with<T:semio_framework_value::retirement::RetireOwned>(payload:&semio_framework_io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_io_control::NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<T>>,bind:impl FnOnce(&DslValue,&mut Option<T>,&mut NativeDecodeControl<'_>,&mut semio_framework_io_control::NativeSnapshotBodyWallet)->Result<(),ValueError>)->crate::lifted_serialization::SnapshotReceivingOutcome{
    use semio_framework_io_control::NativeSnapshotFailure;
    use semio_framework_io_sqlite_snapshot::SqliteStageFailure as Stage;
    use semio_framework_value::RetainedWorkOutcome;
    let limits=control.limits();
    let preflight=(||->Result<usize,ValueError>{
        control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;
        let length=match payload{semio_framework_io_schema::IoPayload::Text(text)=>text.len(),semio_framework_io_schema::IoPayload::Binary(bytes)=>bytes.len()};
        if length>limits.max_file_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"space history native input exceeds file limit"))}
        native_owner.native().owned_bytes().checked_add(control.reconstruction_remaining_bytes()?.min(control.allocation_remaining_bytes())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"space history native reconstruction allowance overflow"))
    })();
    let maximum=match preflight{Ok(maximum)=>maximum,Err(error)=>return RetainedWorkOutcome{result:Err(Stage::NotStarted(error)),work:0}};
    let before=native_owner.native().owned_bytes();
    let settlement=super::settlement::TypedFileSettlement::new(control,before,SqliteSnapshotPhase::DecodeNative);
    let mut observer=|event:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
    let mut bind=Some(bind);
    let turn=native_owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<(Option<RecordSpec>,Option<semio_framework_dsl_record::RecordValue>,Option<DslValue>,Option<T>),crate::lifted_serialization::SnapshotReceivingOutput<T>,NativeSnapshotFailure>(output,&mut |intermediate,published,native,body|{
        let before_work=native.actual_work();
        *intermediate=Some((None,None,None,None));
        let (schema,record,value,output)=intermediate.as_mut().unwrap();
        let operation=(||->Result<T,ValueError>{
            match payload{
                semio_framework_io_schema::IoPayload::Text(text)=>{
                    *value=Some(semio_framework_pack_json::from_json_str_controlled::<DslValue>(text,semio_framework_pack_json::JsonMemberPolicy::Reject,native)?);
                    bind.take().unwrap()(value.as_ref().unwrap(),output,native,body)?;
                }
                semio_framework_io_schema::IoPayload::Binary(bytes)=>{
                    *schema=Some(spec(native)?);
                    *record=Some(pack::record::decode_document_controlled(bytes,schema.as_ref().unwrap(),&pack::record::DecodeOptions::default(),native).map_err(semio_os_pack::PackRefusal::into_value_error)?.0);
                    let Some(FieldValue::Value(value))=record.as_ref().unwrap().get(1)else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"space history native value field is missing"))};
                    bind.take().unwrap()(value,output,native,body)?;
                }
            }
            output.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"space history native constructor did not retain output"))
        })().map_err(NativeSnapshotFailure::from);
        let owned=native.owned_bytes();
        let settled=settlement.finish_body(owned).and_then(|()|owned.checked_sub(before).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"space history native owned counter regressed"))).and_then(|actual|settlement.control.borrow_mut().admit_reconstruction_bytes(actual));
        *published=Some(match settled{Ok(())=>operation.map_err(Stage::Operation),Err(refusal)=>Err(Stage::Settlement{operation,refusal,receipt:None})});
        RetainedWorkOutcome{result:Ok(()),work:native.actual_work().checked_sub(before_work).expect("original native work is monotone")}
    }));
    let result=match settlement.cause.borrow_mut().take(){Some(refusal)=>Err(Stage::Settlement{operation:turn.result,refusal,receipt:None}),None=>turn.result.map_err(Stage::Operation)};
    RetainedWorkOutcome{result,work:turn.work}
}

#[test]
fn sqlite_snapshot_framework_space_history_native_temporary_retirement_requests_no_backing_for_every_intrinsic_kind() {
    fn literal(value: &serde_json::Value) -> DslValue {
        match value["kind"].as_str().unwrap() {
            "null" => DslValue::Null,
            "bool" => DslValue::Bool(value["value"].as_bool().unwrap()),
            "uint" => DslValue::uint(value["value"].as_str().unwrap().parse().unwrap()),
            "int" => DslValue::int(value["value"].as_str().unwrap().parse().unwrap()),
            "float" => DslValue::float(f64::from_bits(u64::from_str_radix(value["value"].as_str().unwrap(), 16).unwrap())),
            "string" => DslValue::String(value["value"].as_str().unwrap().into()),
            "bytes" => { let text = value["value"].as_str().unwrap(); DslValue::Bytes((0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).unwrap()).collect()) },
            "array" => DslValue::Array(value["value"].as_array().unwrap().iter().map(literal).collect()),
            "object" => DslValue::Object(value["value"].as_array().unwrap().iter().map(|member| (member["key"].as_str().unwrap().into(), literal(&member["value"]))).collect()),
            _ => panic!("closed intrinsic fixture kind"),
        }
    }
    let contract: serde_json::Value = serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🧮️ownership/🔣️.json")).unwrap();
    let contract = &contract["output"]["retirement"];
    assert_eq!(contract["releasedBytes"], "completeBorrowedCapacityCensus");
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🎒️pack/🌱️value/🧪️tests/🧫️fixtures/🎞️intrinsic-media/🔣️.json")).unwrap();
    assert_eq!(corpus["contract"], contract["sourceCorpus"]);
    let long = contract["longBranch"]["unit"].as_str().unwrap().repeat(usize::try_from(contract["longBranch"]["repeat"].as_u64().unwrap()).unwrap());
    assert_eq!(long.len(), usize::try_from(contract["longBranch"]["utf8Bytes"].as_u64().unwrap()).unwrap());
    let mut cases = vec![
        ("null", DslValue::Null), ("bool", DslValue::Bool(true)), ("uint", DslValue::uint(u64::MAX)),
        ("int", DslValue::int(i64::MIN)), ("float", DslValue::float(f64::from_bits(0xfff800000000002a))),
        ("string", DslValue::String("\0literal 語".into())), ("bytes", DslValue::Bytes(vec![0, 255, 128, 127])),
        ("empty array", DslValue::Array(Vec::new())), ("empty object", DslValue::Object(Vec::new())),
        ("complete ordered intrinsic owner", literal(&corpus["value"])),
    ];
    let depth = usize::try_from(contract["depth"].as_u64().unwrap()).unwrap();
    let mut array = DslValue::String(long.clone());
    let mut object = DslValue::String(long.clone());
    for _ in 0..depth { array = DslValue::Array(vec![array]); object = DslValue::Object(vec![("duplicate\0語".into(), object)]); }
    cases.push(("deep array", array)); cases.push(("deep object", object));
    cases.push(("long ordered duplicate branches", DslValue::Object(vec![("same".into(), DslValue::String(long)), ("same".into(), literal(&corpus["value"]))])));
    for (label, owner) in cases {
        let mut reject = |_| false;
        let mut canceled = NativeEncodeControl::new(0, &mut reject);
        assert_eq!(canceled.begin_stage(1).unwrap_err().kind, ValueRefusalKind::Canceled);
        let mut source = vec![&owner];
        let mut expected_release = 0usize;
        while let Some(value) = source.pop() {
            match value {
                DslValue::String(value) => expected_release += value.capacity(),
                DslValue::Bytes(value) => expected_release += value.capacity(),
                DslValue::Array(values) => { expected_release += values.capacity() * std::mem::size_of::<DslValue>(); source.extend(values.iter()); },
                DslValue::Object(values) => { expected_release += values.capacity() * std::mem::size_of::<(String, DslValue)>(); for (key, value) in values { expected_release += key.capacity(); source.push(value); } },
                DslValue::Null | DslValue::Bool(_) | DslValue::Number(_) => {},
            }
        }
        drop(source);
        let ((), requests, released) = semio_framework_trace_memory_testing::observe_backing(|| retire_projection(owner));
        assert_eq!(requests, usize::try_from(contract["requestBytes"].as_u64().unwrap()).unwrap(), "{label} mandatory retirement requested unadmitted backing after cancellation");
        assert_eq!(released, expected_release, "{label} complete source-owned backing must be deallocated");
    }
}

#[test]
fn sqlite_snapshot_framework_space_history_native_temporary_retirement_partial_grants_and_drop_release_every_owned_capacity() {
    fn literal(value: &serde_json::Value) -> DslValue {
        match value["kind"].as_str().unwrap() {
            "null" => DslValue::Null,
            "bool" => DslValue::Bool(value["value"].as_bool().unwrap()),
            "uint" => DslValue::uint(value["value"].as_str().unwrap().parse().unwrap()),
            "int" => DslValue::int(value["value"].as_str().unwrap().parse().unwrap()),
            "float" => DslValue::float(f64::from_bits(u64::from_str_radix(value["value"].as_str().unwrap(), 16).unwrap())),
            "string" => DslValue::String(value["value"].as_str().unwrap().into()),
            "bytes" => { let text = value["value"].as_str().unwrap(); DslValue::Bytes((0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).unwrap()).collect()) },
            "array" => DslValue::Array(value["value"].as_array().unwrap().iter().map(literal).collect()),
            "object" => DslValue::Object(value["value"].as_array().unwrap().iter().map(|member| (member["key"].as_str().unwrap().into(), literal(&member["value"]))).collect()),
            _ => panic!("closed intrinsic fixture kind"),
        }
    }
    let contract: serde_json::Value = serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🧮️ownership/🔣️.json")).unwrap();
    let contract = &contract["output"]["retirement"];
    assert_eq!(contract["releasedBytes"], "completeBorrowedCapacityCensus");
    let partial = &contract["partial"];
    assert_eq!(partial["schema"], "history.native.partial-retirement/v1");
    assert_eq!(partial["drop"], "mandatoryCompleteReleaseAfterCancellation");
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🎒️pack/🌱️value/🧪️tests/🧫️fixtures/🎞️intrinsic-media/🔣️.json")).unwrap();
    assert_eq!(corpus["contract"], contract["sourceCorpus"]);
    let long = contract["longBranch"]["unit"].as_str().unwrap().repeat(usize::try_from(contract["longBranch"]["repeat"].as_u64().unwrap()).unwrap());
    assert_eq!(long.len(), usize::try_from(contract["longBranch"]["utf8Bytes"].as_u64().unwrap()).unwrap());
    let mut cases = vec![
        ("null", DslValue::Null), ("bool", DslValue::Bool(true)), ("uint", DslValue::uint(u64::MAX)),
        ("int", DslValue::int(i64::MIN)), ("float", DslValue::float(f64::from_bits(0xfff800000000002a))),
        ("string", DslValue::String("\0literal 語".into())), ("bytes", DslValue::Bytes(vec![0, 255, 128, 127])),
        ("empty array", DslValue::Array(Vec::new())), ("empty object", DslValue::Object(Vec::new())),
        ("complete ordered intrinsic owner", literal(&corpus["value"])),
    ];
    let depth = usize::try_from(contract["depth"].as_u64().unwrap()).unwrap();
    let mut array = DslValue::String(long.clone());
    let mut object = DslValue::String(long.clone());
    for _ in 0..depth { array = DslValue::Array(vec![array]); object = DslValue::Object(vec![("duplicate\0語".into(), object)]); }
    cases.push(("deep array", array)); cases.push(("deep object", object));
    cases.push(("long ordered duplicate branches", DslValue::Object(vec![("same".into(), DslValue::String(long)), ("same".into(), literal(&corpus["value"]))])));
    for (label, owner) in cases {
        let mut reject = |_| false;
        let mut canceled = NativeEncodeControl::new(0, &mut reject);
        assert_eq!(canceled.begin_stage(1).unwrap_err().kind, ValueRefusalKind::Canceled);
        let mut source = vec![&owner];
        let mut expected_release = 0usize;
        let mut expected_transitions = 0usize;
        while let Some(value) = source.pop() {
            expected_transitions += 1;
            match value {
                DslValue::String(value) => expected_release += value.capacity(),
                DslValue::Bytes(value) => expected_release += value.capacity(),
                DslValue::Array(values) => { expected_transitions += values.len(); expected_release += values.capacity() * std::mem::size_of::<DslValue>(); source.extend(values.iter()); },
                DslValue::Object(values) => { expected_transitions += values.len(); expected_release += values.capacity() * std::mem::size_of::<(String, DslValue)>(); for (key, value) in values { expected_release += key.capacity(); source.push(value); } },
                DslValue::Null | DslValue::Bool(_) | DslValue::Number(_) => {},
            }
        }
        drop(source);
        for grant in partial["grants"].as_array().unwrap() {
            let grant = if grant.as_str() == Some("complete") { usize::MAX } else { usize::try_from(grant.as_u64().unwrap()).unwrap() };
            let owner = owner.clone();
            let mut expected_release = 0usize;
            let mut source = vec![&owner];
            while let Some(value) = source.pop() {
                match value {
                    DslValue::String(value) => expected_release += value.capacity(),
                    DslValue::Bytes(value) => expected_release += value.capacity(),
                    DslValue::Array(values) => { expected_release += values.capacity() * std::mem::size_of::<DslValue>(); source.extend(values.iter()); },
                    DslValue::Object(values) => { expected_release += values.capacity() * std::mem::size_of::<(String, DslValue)>(); for (key, value) in values { expected_release += key.capacity(); source.push(value); } },
                    DslValue::Null | DslValue::Bool(_) | DslValue::Number(_) => {},
                }
            }
            drop(source);
            let ((steps, pending, step_requests, step_released), requests, released) = semio_framework_trace_memory_testing::observe_backing(|| {
                let mut retirement = semio_framework_value::IntrinsicRetirement::new(owner);
                let (steps, step_requests, step_released) = semio_framework_trace_memory_testing::observe_backing(|| retirement.close_step(grant));
                let pending = !retirement.terminal_is_empty();
                drop(retirement);
                (steps, pending, step_requests, step_released)
            });
            assert_eq!(steps, grant.min(expected_transitions), "{label} explicit transition grant");
            assert_eq!(pending, grant < expected_transitions, "{label} retained ownership before mandatory Drop");
            assert_eq!(step_requests, 0, "{label} partial close requests backing");
            assert!(step_released <= expected_release, "{label} partial close releases only actual owned backing");
            if grant == 0 { assert_eq!(step_released, 0, "{label} zero grant releases nothing"); }
            assert_eq!(requests, 0, "{label} close and mandatory Drop request no backing");
            assert_eq!(released, expected_release, "{label} partial close plus Drop releases complete source capacity");
        }
        retire_projection(owner);
    }
}

#[test]
fn sqlite_snapshot_framework_space_history_native_codec_body_has_explicit_io_owner() {
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🚪️ownership/🔣️.json")).unwrap();
    assert!(module_path!().contains(corpus["owner"].as_str().unwrap()));
    assert_eq!(corpus["schemaCodecMounts"],0);
}

#[cfg(test)]
#[path="🧪️tests/🪢️partial-projection/🦀️.rs"]
mod partial_projection_tests;

#[cfg(test)]
#[path="🧪️tests/🧾️json-recovery/🦀️.rs"]
mod json_recovery_tests;
