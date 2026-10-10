//! 🖋️ Native chart text output and cancellable LaTeX serialization.
use crate::ChartSnapshot;
use crate::inferences::{catalog,validate_chart,paint,ChartDiagnostic,ChartInference};
use semio_framework_value::DslValue;
use semio_framework_value_derive::{ToValue,FromValue};
semio_framework_value_derive::value_codec!{struct ChartDiagnostic{pub code:String,pub path:String,pub message:String}}
semio_framework_value_derive::value_codec!{
 struct ChartInference{
  #[value(default,skip_serializing_if="Option::is_none")]
  pub chart:Option<ChartSnapshot>,pub diagnostics:Vec<ChartDiagnostic>,pub complete:bool
 }
}
#[derive(Clone,Debug,PartialEq,ToValue,FromValue)]
pub struct ChartTextOutput {pub tikz:String,pub diagnostics:Vec<ChartDiagnostic>,pub complete:bool}
impl Default for ChartTextOutput{fn default()->Self{Self::render(&ChartSnapshot::default()).expect("chart text output describes admission")}}
impl ChartTextOutput {
    /// 📝️ Renders a chart through its physical native grammar owner.
    pub fn render(snapshot:&ChartSnapshot)->Result<Self,semio_framework_value::ValueError>{Ok(match render_chart_controlled(snapshot,&mut |_|Ok(())){Ok(tikz)=>Self{tikz,diagnostics:Vec::new(),complete:true},Err(message)=>Self{tikz:String::new(),diagnostics:vec![ChartDiagnostic{code:"print.chart.inference".into(),path:"chart".into(),message}],complete:false}})}
}
fn token(value: &str) -> Result<String, String> {
    if value.is_empty() || !value.chars().all(|character| character.is_ascii_alphanumeric() || ['-', '_', '.', ':', '/'].contains(&character)) {
        return Err(format!("invalid grammar identifier {value:?}"));
    }
    Ok(value.into())
}

fn text(value: &str) -> String {
    value.chars().map(|character| match character {
        '\\' => "\\textbackslash{}".into(), '{' => "\\{".into(), '}' => "\\}".into(), '%' => "\\%".into(), '#' => "\\#".into(), '$' => "\\$".into(), '&' => "\\&".into(), '_' => "\\_".into(), '^' => "\\textasciicircum{}".into(), '~' => "\\textasciitilde{}".into(), character => character.to_string(),
    }).collect()
}

fn scalar(value: &DslValue) -> Result<String, String> {
    match value {
        DslValue::String(value) => Ok(text(value)),
        DslValue::Number(number) if number.as_f64().is_finite() => Ok(number.as_f64().to_string()),
        DslValue::Bool(value) => Ok(value.to_string()),
        DslValue::Null => Ok(String::new()),
        DslValue::Array(items) => items.iter().map(scalar).collect::<Result<Vec<_>, _>>().map(|items| items.join(",")),
        _ => Err("grammar options must contain finite scalar values or scalar lists".into()),
    }
}

fn row_scalar(value:&DslValue)->Result<String,String>{
    match value{
        DslValue::Null=>Ok("\\SemioVizNull{}".into()),
        DslValue::String(value)=>Ok(format!("\\SemioVizString{{{}}}",text(value))),
        DslValue::Bool(value)=>Ok(format!("\\SemioVizBoolean{{{value}}}")),
        _=>scalar(value),
    }
}
fn row_cell(value:Option<&DslValue>)->Result<String,String>{
    value.map(row_scalar).unwrap_or_else(||Ok("\\SemioVizUndefined{}".into()))
}
fn paint_scalar(value:&DslValue)->Result<String,String>{
    match value{
        DslValue::String(value)=>paint::parse(value).map(|paint|paint.name()),
        DslValue::Array(values)=>values.iter().map(paint_scalar).collect::<Result<Vec<_>,_>>().map(|values|values.join(",")),
        _=>scalar(value),
    }
}



fn unique_name(preferred:String,names:&mut std::collections::BTreeSet<String>)->String{
    let mut value=preferred.clone();let mut suffix=0;
    while names.contains(&value){suffix+=1;value=format!("{preferred}-{suffix}");}
    names.insert(value.clone());value
}

fn colors(chart:&DslValue)->String{
    let mut values=std::collections::BTreeMap::new();
    let mut stack=vec![chart];
    while let Some(value)=stack.pop(){match value{
        DslValue::String(value)=>{if let Ok(paint)=paint::parse(value){values.insert(paint.name(),paint);}},
        DslValue::Array(items)=>stack.extend(items),
        DslValue::Object(items)=>stack.extend(items.iter().map(|(_,value)|value)),
        _=>{}
    }}
    values.values().map(paint::Paint::declaration).collect()
}

fn annotation_options(value:Option<&DslValue>,label:bool)->Result<String,String>{
    let Some(value)=value else{return Ok(String::new());};
    let mut entries=value.as_object().ok_or_else(||"annotation options must be an object".to_string())?.iter().collect::<Vec<_>>();
    entries.sort_by(|(a,_),(b,_)|a.cmp(b));
    let opacity=value.get("opacity").and_then(DslValue::as_f64).unwrap_or(1.0);
    let alpha=|key|->Result<f64,String>{match value.get(key).and_then(DslValue::as_str){Some(value)=>{native_paint(value)?;Ok(paint::parse(value).map_or(1.0,|paint|paint.alpha)*opacity)},None=>Ok(opacity)}};
    let fill_alpha=alpha("fill")?;let stroke_alpha=alpha("stroke")?;
    let mut result=entries.into_iter().filter(|(key,_)|key.as_str()!="opacity").map(|(key,value)|{
        let value=native_option(value,option_descriptor("annotation",key,false)?,false)?;
        Ok(match key.as_str(){
            "fill"=>format!("{}={{{value}}}",if label{"text"}else{"fill"}),
            "stroke"=>format!("draw={{{value}}}"),
            "strokeWidth"=>format!("line width={value}mm"),
            "rotation"=>format!("rotate={value}"),
            "cap"=>format!("line cap={value}"),
            "join"=>format!("line join={value}"),
            "size"|"fontSize"=>format!("font={{\\fontsize{{{value}}}{{{value}}}\\selectfont}}"),
            "anchor"=>format!("anchor={}",match value.as_str(){"start"=>"west","middle"=>"center","end"=>"east",_=>value.as_str()}),
            "dash"=>format!("dash pattern={}",value.split(|character:char|character==','||character.is_whitespace()).filter(|length|!length.is_empty()).enumerate().map(|(index,length)|format!("{} {length}mm",if index%2==0{"on"}else{"off"})).collect::<Vec<_>>().join(" ")),
            _=>format!("{}={{{value}}}",token(key)?)
        })
    }).collect::<Result<Vec<_>,String>>()?;
    result.push(format!("fill opacity={fill_alpha},draw opacity={stroke_alpha},text opacity={fill_alpha}"));
    Ok(result.join(","))
}

/// 🧿️ Source-owned lexical contracts for canonical native option projection.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ChartOptionSyntax { Text,Identifier,Expression,Records,KeyList,Style,Paint,NumberList,Scalar,Value }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ChartOptionItems { Text,Identifier }
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct ChartOptionDescriptor { pub syntax:ChartOptionSyntax,pub items:Option<ChartOptionItems>,pub item_count:Option<usize>,contract:&'static DslValue }
fn option_schema()->Result<&'static DslValue,String>{
 static SCHEMA:std::sync::OnceLock<DslValue>=std::sync::OnceLock::new();
 Ok(SCHEMA.get_or_init(||semio_framework_value_derive::owned_json_file!("../../🧬️schema/🔣️.json")))
}
fn option_descriptor(owner:&str,key:&str,family:bool)->Result<ChartOptionDescriptor,String>{
 let schema=option_schema()?;
 let descriptors=if family{schema.get("x-semio-family-options").and_then(|value|value.get(owner)).and_then(|value|value.get("options"))}else{schema.get("x-semio-option-syntax").and_then(|value|value.get(owner))};
 let descriptor=descriptors.and_then(|value|value.get(key)).ok_or_else(||format!("undeclared native option {owner}/{key}"))?;
 let syntax=match descriptor.get("syntax").and_then(DslValue::as_str){Some("text")=>ChartOptionSyntax::Text,Some("identifier")=>ChartOptionSyntax::Identifier,Some("expression")=>ChartOptionSyntax::Expression,Some("records")=>ChartOptionSyntax::Records,Some("key-list")=>ChartOptionSyntax::KeyList,Some("style")=>ChartOptionSyntax::Style,Some("paint")=>ChartOptionSyntax::Paint,Some("number-list")=>ChartOptionSyntax::NumberList,Some("scalar")=>ChartOptionSyntax::Scalar,Some("value")=>ChartOptionSyntax::Value,_=>return Err(format!("invalid source-owned option syntax {owner}/{key}"))};
 let items=match descriptor.get("items"){None=>None,Some(value)=>match value.as_str(){Some("text")=>Some(ChartOptionItems::Text),Some("identifier")=>Some(ChartOptionItems::Identifier),_=>return Err(format!("invalid record item syntax {owner}/{key}"))}};
 if items.is_some()&&syntax!=ChartOptionSyntax::Records{return Err(format!("record item syntax on non-record {owner}/{key}"));}
 let item_count=match descriptor.get("itemCount"){None=>None,Some(value)=>Some(value.as_u64().and_then(|value|usize::try_from(value).ok()).ok_or_else(||format!("invalid numeric list item count {owner}/{key}"))?)};if item_count.is_some()&&syntax!=ChartOptionSyntax::NumberList{return Err(format!("item count on nonnumeric list {owner}/{key}"));}
 Ok(ChartOptionDescriptor{syntax,items,item_count,contract:descriptor})
}
fn raw_option(value:&DslValue)->Result<String,String>{match value{DslValue::String(value)=>Ok(value.clone()),DslValue::Array(values)=>values.iter().map(raw_option).collect::<Result<Vec<_>,_>>().map(|values|values.join(",")),DslValue::Bool(_)|DslValue::Number(_)=>scalar(value),_=>Err("native grammar requires strings, finite scalar values or scalar lists".into())}}
fn record_parts(value:&str,separator:char)->Result<Vec<String>,String>{
 let mut result=Vec::new();let(mut start,mut depth,mut escaped)=(0,0_i64,false);
 for(index,character)in value.char_indices(){if escaped{escaped=false;continue;}if character=='\\'{escaped=true;continue;}match character{'{'=>depth+=1,'}'=>{depth-=1;if depth<0{return Err("unbalanced native records".into());}},character if character==separator&&depth==0=>{result.push(value[start..index].trim().to_string());start=index+1;},_=>{}}}
 if depth!=0{return Err("unbalanced native records".into());}result.push(value[start..].trim().to_string());
 Ok(result.into_iter().map(|item|if item.starts_with('{')&&item.ends_with('}'){item[1..item.len()-1].to_string()}else{item}).collect())
}
fn record_items(value:&str)->Result<Vec<String>,String>{record_parts(value,',')}
fn record_rows(value:&DslValue,descriptor:ChartOptionDescriptor)->Result<Vec<Vec<String>>,String>{
 let Some(fields)=descriptor.contract.get("recordFields").and_then(DslValue::as_array)else{return Ok(Vec::new());};let raw=raw_option(value)?;if raw.trim().is_empty(){return Ok(Vec::new());}let mut unique=(0..fields.len()).map(|_|std::collections::HashSet::<String>::new()).collect::<Vec<_>>();
 record_items(&raw)?.into_iter().map(|record|{let mut row=record_parts(&record,'/')?;if row.len()!=fields.len(){return Err(format!("native record requires {} fields",fields.len()));}for(index,field)in fields.iter().enumerate(){let value=row[index].trim().to_string();if field.get("nonempty").and_then(DslValue::as_bool)==Some(true)&&value.is_empty(){return Err("native record field must not be empty".into());}if field.get("enum").and_then(DslValue::as_array).is_some_and(|choices|!choices.iter().any(|choice|choice.as_str()==Some(&value))){return Err(format!("unknown native record vocabulary {value}"));}match field.get("syntax").and_then(DslValue::as_str){Some("expression")=>{if value.is_empty()||value.parse::<f64>().is_ok_and(|number|!number.is_finite()){return Err("native record coordinates require finite literals or nonempty expressions".into());}},_=>{}}if field.get("unique").and_then(DslValue::as_bool)==Some(true)&&!unique[index].insert(value.clone()){return Err(format!("duplicate native record ID {value}"));}row[index]=value;}Ok(row)}).collect()
}
fn validate_family_records(owner:&str,authored:Option<&DslValue>,stock:Option<&DslValue>)->Result<(),String>{
 let schema=option_schema()?;let descriptors=schema.get("x-semio-family-options").and_then(|families|families.get(owner)).and_then(|family|family.get("options")).and_then(DslValue::as_object).ok_or_else(||format!("unknown native family {owner}"))?;if !descriptors.iter().any(|(_,descriptor)|descriptor.get("recordFields").is_some()){return Ok(());}
 let selected=|key:&str|authored.and_then(|options|options.get(key)).or_else(||stock.and_then(|options|options.get(key)));let mode=selected("mode").or_else(||descriptors.iter().find(|(key,_)|key=="mode").and_then(|(_,descriptor)|descriptor.get("default"))).and_then(DslValue::as_str).unwrap_or("");let empty=DslValue::String(String::new());let mut resolved=std::collections::HashMap::<String,Vec<Vec<String>>>::new();
 if descriptors.iter().any(|(key,_)|key=="mode"){validate_option_value(&DslValue::String(mode.into()),option_descriptor(owner,"mode",true)?)?;}
 for(key,contract)in descriptors{if contract.get("recordFields").is_none(){continue;}let value=selected(key).or_else(||contract.get("defaultsByMode").and_then(|defaults|defaults.get(mode))).or_else(||contract.get("default")).unwrap_or(&empty);let descriptor=option_descriptor(owner,key,true)?;validate_option_value(value,descriptor)?;resolved.insert(key.clone(),record_rows(value,descriptor)?);}
 for(key,rows)in &resolved{let contract=descriptors.iter().find(|(name,_)|name==key).unwrap().1.get("recordFields").and_then(DslValue::as_array).unwrap();for(index,field)in contract.iter().enumerate(){let Some(reference)=field.get("reference")else{continue;};let option=reference.get("option").and_then(DslValue::as_str).ok_or_else(||"record reference option missing".to_string())?;let target=reference.get("field").and_then(DslValue::as_u64).ok_or_else(||"record reference field missing".to_string())?as usize;let ids=resolved.get(option).into_iter().flatten().filter_map(|row|row.get(target)).collect::<std::collections::HashSet<_>>();if rows.iter().any(|row|!ids.contains(&row[index])){return Err(format!("dangling native record endpoint {owner}/{key}"));}}}Ok(())
}
/// 🪢️ Directed neural incidence references ordered layer rows and visible unit slots.
fn validate_neural_links(chart:&DslValue,preset:&DslValue,entry:&DslValue)->Result<(),String>{
 if preset.get("options").and_then(|options|options.get("links")).is_none()&&(preset.get("data").is_some()||["data","layerColumn","unitsColumn","maxUnits","connections"].iter().any(|key|preset.get("options").and_then(|options|options.get(*key)).is_some())){return Ok(());}
 let selected=|key:&str|preset.get("options").and_then(|options|options.get(key)).or_else(||entry.get("options").and_then(|options|options.get(key)));let Some(links)=selected("links")else{return Ok(());};let rows=record_rows(links,option_descriptor("neural-network","links",true)?)?;if rows.is_empty(){return Ok(());}
 let data=preset.get("data").and_then(DslValue::as_str).or_else(||selected("data").and_then(DslValue::as_str)).or_else(||entry.get("data").and_then(DslValue::as_str)).ok_or_else(||"neural layer table missing".to_string())?;let data=if data=="demo"{entry.get("data").and_then(DslValue::as_str).unwrap_or(data)}else{data};let schema=option_schema()?;let table=list(chart,"tables")?.iter().find(|table|table.get("name").and_then(DslValue::as_str)==Some(data)).or_else(||schema.get("x-semio-demo-tables").and_then(DslValue::as_array).into_iter().flatten().find(|table|table.get("name").and_then(DslValue::as_str)==Some(data))).ok_or_else(||"neural links require a declared layer table".to_string())?;
 let layer=selected("layerColumn").and_then(DslValue::as_str).unwrap_or("layer");let units=selected("unitsColumn").and_then(DslValue::as_str).unwrap_or("units");let numeric=|value:&DslValue|value.as_f64().or_else(||value.as_str().and_then(|value|value.trim().parse::<f64>().ok()));let maximum=match selected("maxUnits"){Some(value)=>numeric(value).ok_or_else(||"neural visible unit limit requires a positive integer literal".to_string())?,None=>10.0};if !maximum.is_finite()||maximum.fract()!=0.0||!(1.0..=2147483647.0).contains(&maximum){return Err("neural visible unit limit requires a positive integer literal".into());}
 let mut ordered=list(table,"rows")?.iter().enumerate().map(|(index,row)|{let layer=row.get(layer).and_then(numeric).filter(|value|value.is_finite()).ok_or_else(||"neural layer ranks require finite indices".to_string())?;let count=row.get(units).and_then(numeric).filter(|value|value.is_finite()&&value.fract()==0.0&&(0.0..=2147483647.0).contains(value)).ok_or_else(||"neural unit counts require positive integers".to_string())?;Ok((layer,index,count.min(maximum)as usize))}).collect::<Result<Vec<_>,String>>()?;ordered.sort_by(|a,b|a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
 for row in rows{let endpoints=row.iter().map(|value|{if value.starts_with('0')||!value.chars().all(|character|character.is_ascii_digit()){return Err("neural links require positive integer endpoints".to_string());}value.parse::<usize>().ok().filter(|value|*value>0&&(*value as f64)<=9007199254740991.0).ok_or_else(||"neural links require positive integer endpoints".to_string())}).collect::<Result<Vec<_>,_>>()?;if endpoints[0]>ordered.len()||endpoints[2]>ordered.len()||endpoints[1]>ordered[endpoints[0]-1].2||endpoints[3]>ordered[endpoints[2]-1].2{return Err("neural link endpoint exceeds visible layer/unit bounds".into());}}Ok(())
}
fn native_paint(value:&str)->Result<String,String>{
 if let Ok(paint)=paint::parse(value){return Ok(paint.name());}
 let named=|name:&str|name=="."||name.chars().next().is_some_and(|character|character.is_ascii_alphabetic())&&name.chars().all(|character|character.is_ascii_alphanumeric()||['_','-'].contains(&character));
 let parts=value.split('!').collect::<Vec<_>>();
 if !named(parts[0].trim())||parts.iter().skip(1).enumerate().any(|(index,part)|if index%2==0{part.trim().parse::<f64>().ok().is_none_or(|value|!value.is_finite()||!(0.0..=100.0).contains(&value))}else{!named(part.trim())}){return Err(format!("invalid native or CSS paint {value:?}"));}
 Ok(value.into())
}
/// 🧴️ The owned schema validator checks declared carriers without evaluating native arithmetic grammar.
fn validate_option_value(value:&DslValue,descriptor:ChartOptionDescriptor)->Result<(),String>{
 let contract=descriptor.contract.as_object().ok_or_else(||"native option contract must be an object".to_string())?.iter().filter(|(key,_)|["type","enum","minimum","exclusiveMinimum","maximum"].contains(&key.as_str())).cloned().collect::<Vec<_>>();if contract.is_empty(){return Ok(());}
 let types=descriptor.contract.get("type");let has_type=|name:&str|types.is_some_and(|types|types.as_str()==Some(name)||types.as_array().is_some_and(|types|types.iter().any(|value|value.as_str()==Some(name))));let(mut contract,mut candidate)=(DslValue::Object(contract),value.clone());
 if descriptor.syntax==ChartOptionSyntax::Expression&&(has_type("number")||has_type("integer")){if let Some(value)=value.as_str(){let literal=value.trim();let numeric=(!literal.is_empty()&&literal.chars().all(|character|character.is_ascii_digit()||['.','+','-','e','E'].contains(&character))).then(||literal.parse::<f64>()).and_then(Result::ok);if let Some(number)=numeric{if !number.is_finite(){return Err("native option numbers must be finite".into());}candidate=DslValue::Number(number.into());}else if !has_type("string")&&!literal.is_empty(){if let DslValue::Object(entries)=&mut contract{for(key,value)in entries{if key=="type"{*value=DslValue::String("string".into());}}}}}}
 let validator=semio_framework_schema_validator::OwnedJsonSchemaValidator::compile_intrinsic_with_documents(&contract,&[]).map_err(|error|error.to_string())?;
 validator.validate_intrinsic(&candidate).map(|_|()).map_err(|error|format!("invalid declared native option value: {error}"))
}
fn native_option(value:&DslValue,descriptor:ChartOptionDescriptor,paint_result:bool)->Result<String,String>{
 validate_option_value(value,descriptor)?;
 match descriptor.syntax{
  ChartOptionSyntax::Text=>value.as_str().map(text).ok_or_else(||"literal option text must be a string".into()),
  ChartOptionSyntax::Identifier|ChartOptionSyntax::Expression|ChartOptionSyntax::KeyList|ChartOptionSyntax::Style=>raw_option(value),
  ChartOptionSyntax::NumberList=>{let result=raw_option(value)?;let items=if result.trim().is_empty(){Vec::new()}else{record_items(&result)?};if descriptor.item_count.is_some_and(|count|count!=items.len()){return Err(format!("native numeric list requires {} items",descriptor.item_count.unwrap()));}for item in items{let literal=item.trim();if literal.is_empty(){return Err("native numeric list items must not be empty".into());}if ["inf","+inf","-inf","infinity","+infinity","-infinity","nan","+nan","-nan"].iter().any(|value|literal.eq_ignore_ascii_case(value))||literal.parse::<f64>().is_ok_and(|value|!value.is_finite()){return Err("native numeric list literals must be finite".into());}}Ok(result)},
  ChartOptionSyntax::Records=>{if descriptor.contract.get("recordFields").is_some(){record_rows(value,descriptor)?;}if let Some(items)=descriptor.items{record_items(&raw_option(value)?)?.iter().map(|value|Ok(format!("{{{}}}",match items{ChartOptionItems::Text=>text(value),ChartOptionItems::Identifier=>token(value)?}))).collect::<Result<Vec<_>,String>>().map(|values|values.join(","))}else{raw_option(value)}},
  ChartOptionSyntax::Scalar=>match value{DslValue::Bool(_)|DslValue::Number(_)=>scalar(value),_=>Err("scalar options require a number or boolean".into())},
  ChartOptionSyntax::Value=>match value{DslValue::Array(values)=>values.iter().map(row_scalar).collect::<Result<Vec<_>,_>>().map(|values|values.join(",")),DslValue::String(value)if paint_result=>native_paint(value),_=>row_scalar(value)},
  ChartOptionSyntax::Paint=>match value{DslValue::String(value)=>if let Ok(paint)=paint::parse(value){Ok(paint.name())}else{record_items(value)?.iter().map(|value|native_paint(value)).collect::<Result<Vec<_>,_>>().map(|values|values.join(","))},DslValue::Array(values)=>values.iter().map(|value|value.as_str().ok_or_else(||"paint lists require strings".into()).and_then(native_paint)).collect::<Result<Vec<_>,String>>().map(|values|values.join(",")),_=>Err("paint options require strings or paint lists".into())},
 }
}
fn options(owner:&str,value:Option<&DslValue>,family:bool,paint_result:bool)->Result<String,String>{
 let Some(value)=value else{return Ok(String::new());};let mut entries=value.as_object().ok_or_else(||"grammar options must be an object".to_string())?.iter().collect::<Vec<_>>();entries.sort_by(|(left,_),(right,_)|left.cmp(right));
 entries.into_iter().map(|(key,value)|Ok(format!("{}={{{}}}",token(key)?,native_option(value,option_descriptor(owner,key,family)?,paint_result)?))).collect::<Result<Vec<_>,String>>().map(|values|values.join(","))
}


fn name(value: &DslValue, key: &str) -> Result<String, String> {
    token(value.get(key).and_then(DslValue::as_str).ok_or_else(|| format!("missing {key}"))?)
}

fn list<'a>(value: &'a DslValue, key: &str) -> Result<&'a [DslValue], String> {
    match value.get(key) {
        None => Ok(&[]),
        Some(value) => value.as_array().ok_or_else(|| format!("{key} must be an array")),
    }
}

/// 🧭️ Axis merge comparisons and writes retain paid cancellation throughout sorting.
fn spatial_axis(axis:&mut Vec<f64>,checkpoint:&mut dyn FnMut()->Result<(),String>)->Result<(),String>{
 let mut buffer=vec![0.0;axis.len()];let mut width=1;
 while width<axis.len(){let mut start=0;while start<axis.len(){let middle=(start+width).min(axis.len());let end=middle.saturating_add(width).min(axis.len());let(mut left,mut right)=(start,middle);for output in &mut buffer[start..end]{checkpoint()?;if right==end||left<middle&&axis[left]<=axis[right]{*output=axis[left];left+=1;}else{*output=axis[right];right+=1;}}start=end;}std::mem::swap(axis,&mut buffer);width=width.saturating_mul(2);}
 Ok(())
}
/// 🗺️ Declared spatial rows lower into the existing native registries with cancellable work.
fn spatial_table(table:&DslValue,checkpoint:&mut dyn FnMut()->Result<(),String>)->Result<String,String>{
 let Some(structure)=table.get("structure")else{return Ok(String::new());};
 let table_name=name(table,"name")?;let columns=list(table,"columns")?;let rows=list(table,"rows")?;
 if rows.is_empty(){return Err("spatial table requires rows".into());}
 let number=|row:&DslValue,column:&str,tick:&mut dyn FnMut()->Result<(),String>|->Result<f64,String>{tick()?;if !columns.iter().any(|value|value.as_str()==Some(column)){return Err(format!("unknown spatial column {column}"));}row.get(column).and_then(DslValue::as_f64).filter(|value|value.is_finite()).map(|value|if value==0.0{0.0}else{value}).ok_or_else(||format!("spatial column {column} requires finite numbers"))};
 let pair=|row:&DslValue,x:&str,y:&str,tick:&mut dyn FnMut()->Result<(),String>|->Result<[f64;2],String>{Ok([number(row,x,tick)?,number(row,y,tick)?])};
 match name(structure,"kind")?.as_str(){
  "points"=>{let(x,y)=(name(structure,"x")?,name(structure,"y")?);let mut points=Vec::new();for row in rows{let point=pair(row,&x,&y,checkpoint)?;points.push(format!("{},{}",point[0],point[1]));}Ok(format!("\\SemioVizPointSet{{{table_name}}}{{{}}}\n",points.join(";")))},
  "geometry"=>{
   let(x,y,shape)=(name(structure,"longitude")?,name(structure,"latitude")?,name(structure,"shape")?);
   let group_column=structure.get("group").and_then(DslValue::as_str);let value_column=structure.get("value").and_then(DslValue::as_str);
   let mut groups:Vec<(String,DslValue,f64,Vec<[f64;2]>)>=Vec::new();let mut indices:std::collections::BTreeMap<String,usize>=std::collections::BTreeMap::new();
   for row in rows{checkpoint()?;let identity=if let Some(column)=group_column{if !columns.iter().any(|value|value.as_str()==Some(column)){return Err("unknown geometry group column".into());}row.get(column).ok_or_else(||"missing geometry group".to_string())?.clone()}else{DslValue::String(table_name.clone())};
    let id=match &identity{DslValue::String(value)=>value.clone(),DslValue::Bool(value)=>value.to_string(),DslValue::Number(value)if value.as_f64().is_finite()=>if value.as_f64()==0.0{"0".into()}else{value.as_f64().to_string()},_=>return Err("invalid geometry group".into())};
    let value=if let Some(column)=value_column{number(row,column,checkpoint)?}else{1.0};let point=pair(row,&x,&y,checkpoint)?;
    if let Some(index)=indices.get(&id){let group=&mut groups[*index];if group.1!=identity||group.2!=value{return Err("geometry group requires one identity and value".into());}group.3.push(point);}else{indices.insert(id.clone(),groups.len());groups.push((id,identity,value,vec![point]));}
   }
   let command=match shape.as_str(){"polygon"=>"SemioVizGeoPolygon","line"=>"SemioVizGeoLine","point"=>"SemioVizGeoPointPart",_=>return Err("unknown geometry shape".into())};let mut output=format!("\\SemioVizGeoCollection{{{table_name}}}\n");
   for(id,_,value,mut points)in groups{checkpoint()?;if shape=="polygon"&&points.len()>1&&points.first()==points.last(){points.pop();}if if shape=="point"{points.len()!=1}else{points.len()<if shape=="polygon"{3}else{2}}{return Err("invalid geometry vertex count".into());}let coordinates=points.iter().map(|point|format!("{},{}",point[0],point[1])).collect::<Vec<_>>().join(";");output.push_str(&format!("\\{command}{{{table_name}}}[id={{{}}},value={value}]{{{coordinates}}}\n",text(&id)));}Ok(output)
  },
  "grid"=>{
   let(x,y,value)=(name(structure,"column")?,name(structure,"row")?,name(structure,"value")?);let mut samples=Vec::new();let(mut width,mut height)=(0_usize,0_usize);
   for row in rows{let point=pair(row,&x,&y,checkpoint)?;if point.iter().any(|value|*value<0.0||value.fract()!=0.0){return Err("grid coordinates require nonnegative integers".into());}if point.iter().any(|value|*value>=rows.len()as f64){return Err("grid must be a complete rectangle".into());}let(x,y)=(point[0]as usize,point[1]as usize);width=width.max(x+1);height=height.max(y+1);samples.push((x,y,number(row,&value,checkpoint)?));}
   if width.checked_mul(height)!=Some(rows.len()){return Err("grid must be a complete rectangle".into());}let mut cells=vec![None;rows.len()];for(x,y,value)in samples{checkpoint()?;if cells[y*width+x].replace(value).is_some(){return Err("duplicate grid cell".into());}}
   let mut values=Vec::new();for cell in cells{checkpoint()?;values.push(cell.ok_or_else(||"missing grid cell".to_string())?.to_string());}Ok(format!("\\SemioVizValueGrid{{{table_name}}}{{{width}}}{{{height}}}{{{}}}\n",values.join(",")))
  },
  "vector-field"=>{
   let(x,y,u,v)=(name(structure,"x")?,name(structure,"y")?,name(structure,"u")?,name(structure,"v")?);let mut samples=Vec::new();let(mut xs,mut ys)=(Vec::new(),Vec::new());
   for row in rows{let point=pair(row,&x,&y,checkpoint)?;let vector=pair(row,&u,&v,checkpoint)?;xs.push(point[0]);ys.push(point[1]);samples.push((point,vector));}
   for axis in [&mut xs,&mut ys]{spatial_axis(axis,checkpoint)?;axis.dedup();if axis.len()<2{return Err("vector field requires two samples per axis".into());}let step=axis[1]-axis[0];for(index,value)in axis.iter().enumerate(){checkpoint()?;if (*value-axis[0]-index as f64*step).abs()>1e-10*step.abs().max(1.0){return Err("vector field requires uniform axes".into());}}}
   if xs.len().checked_mul(ys.len())!=Some(samples.len()){return Err("vector field must be a complete rectangle".into());}let mut cells=vec![None;samples.len()];for(point,vector)in samples{checkpoint()?;let xi=xs.binary_search_by(|value|value.total_cmp(&point[0])).map_err(|_|"missing vector x")?;let yi=ys.binary_search_by(|value|value.total_cmp(&point[1])).map_err(|_|"missing vector y")?;if cells[yi*xs.len()+xi].replace(vector).is_some(){return Err("duplicate vector field cell".into());}}
   let(mut us,mut vs)=(Vec::new(),Vec::new());for cell in cells{checkpoint()?;let vector=cell.ok_or_else(||"missing vector field cell".to_string())?;us.push(vector[0].to_string());vs.push(vector[1].to_string());}Ok(format!("\\SemioVizVectorField{{{table_name}}}{{{}}}{{{}}}{{{},{},{},{}}}{{{}}}{{{}}}\n",xs.len(),ys.len(),xs[0],xs[xs.len()-1],ys[0],ys[ys.len()-1],us.join(","),vs.join(",")))
  },
  _=>Err("unknown spatial structure".into())
 }
}

/// ⏱️ Each table row, scale, transform, layer and guide has an explicit cancellable checkpoint.
pub fn render_chart_controlled(snapshot: &ChartSnapshot, checkpoint: &mut dyn FnMut(u64) -> Result<(), String>) -> Result<String, String> {
    checkpoint(0)?;
    validate_chart(snapshot)?;
    let chart = &snapshot.chart;
    let language = name(chart, "language")?;
    if !["en", "de"].contains(&language.as_str()) { return Err("chart language must be explicitly en or de".into()); }
    let dimension = |key| chart.get(key).and_then(DslValue::as_f64).filter(|value| value.is_finite() && *value > 0.0).ok_or_else(|| format!("{key} must be positive and finite"));
    let width = dimension("width")?;
    let height = dimension("height")?;
    if let Some(margin)=chart.get("margin"){
        let side=|key|margin.get(key).and_then(DslValue::as_f64).unwrap_or(0.0);
        if side("left")+side("right")>width||side("top")+side("bottom")>height{return Err("chart margins exceed dimensions".into());}
    }
    let mut work = 0;
    checkpoint(work)?;
    let title = chart.get("title").and_then(|title| title.get(&language)).and_then(DslValue::as_str).unwrap_or("");
    let mut output = format!("\\begingroup\n\\ExplSyntaxOn\\tl_set:Nn\\l_semio_language_tl{{{language}}}\\ExplSyntaxOff\n\\begin{{VizFigure}}[width={width},height={height},title={{{}}}]\n", text(title));
    output.push_str(&colors(chart));
    if let Some(theme) = chart.get("theme") {
        let theme_name = theme.get("name").and_then(DslValue::as_str).unwrap_or("default");
        let theme_name = if theme_name == "semio" { "default" } else { theme_name };
        let appearance = theme.get("appearance").and_then(DslValue::as_str).unwrap_or("light");
        output.push_str(&format!("\\SemioVizTheme{{{}}}[appearance={appearance}]\n", token(theme_name)?));
        if let Some(palette) = theme.get("palette").and_then(DslValue::as_array) {
            let palette = palette.iter().map(paint_scalar).collect::<Result<Vec<_>, _>>()?;
            output.push_str(&format!("\\SemioVizThemeSet[colors={{{}}}]\n", palette.join(",")));
        }
    }
    let mut tables = std::collections::BTreeSet::new();
    for table in list(chart, "tables")? {
        let table_name = name(table, "name")?;
        if !tables.insert(table_name.clone()) { return Err(format!("duplicate table {table_name}")); }
        let columns = list(table, "columns")?.iter().map(|column| token(column.as_str().ok_or_else(|| "column must be a string".to_string())?)).collect::<Result<Vec<_>, _>>()?;
        output.push_str(&format!("\\SemioVizTable{{{table_name}}}{{{}}}\n", columns.join(",")));
        for row in list(table, "rows")? {
            let values = columns.iter().map(|column| row_cell(row.get(column)).map(|value| format!("{{{value}}}"))).collect::<Result<Vec<_>, _>>()?;
            output.push_str(&format!("\\SemioVizRow{{{table_name}}}{{{}}}\n", values.join(",")));
            work += 1; checkpoint(work)?;
        }
        output.push_str(&spatial_table(table,&mut ||{work+=1;checkpoint(work)})?);
    }
    let mut scales = std::collections::BTreeSet::new();
    let paint_scales=list(chart,"layers")?.iter().flat_map(|layer|["fill","stroke"].into_iter().filter_map(move|channel|layer.get("encodings").and_then(|encodings|encodings.get(channel)).and_then(|encoding|encoding.get("scale")).and_then(DslValue::as_str))).collect::<std::collections::BTreeSet<_>>();
    for scale in list(chart, "scales")? {
        let scale_name = name(scale, "name")?;
        if !scales.insert(scale_name.clone()) { return Err(format!("duplicate scale {scale_name}")); }
        let range=scale.get("range").ok_or_else(||"missing scale range".to_string())?;
        output.push_str(&format!("\\SemioVizScale{{{scale_name}}}{{{}}}{{{}}}{{{}}}[{}]\n", name(scale, "kind")?, if ["ordinal","band","point"].contains(&name(scale,"kind")?.as_str()){list(scale,"domain")?.iter().map(row_scalar).collect::<Result<Vec<_>,_>>()?.join(",")}else{scalar(scale.get("domain").ok_or_else(|| "missing scale domain".to_string())?)?}, if paint_scales.contains(scale_name.as_str()){paint_scalar(range)?}else{scalar(range)?}, options("scale",scale.get("options"),false,paint_scales.contains(scale_name.as_str()))?));
        work += 1; checkpoint(work)?;
    }
    if let Some(coordinate) = chart.get("coordinate") {
        output.push_str(&format!("\\SemioVizCoordinate{{{}}}[{}]\n", name(coordinate, "kind")?, options("coordinate",coordinate.get("options"),false,false)?));
    }
    for preset in list(chart, "presets")? {
        let kind = name(preset, "kind")?;
        let entry = list(catalog()?, "kinds")?.iter().find(|entry| entry.get("slug").and_then(DslValue::as_str) == Some(&kind)).ok_or_else(|| format!("unknown catalogue kind {kind}"))?;
        let family=entry.get("family").and_then(DslValue::as_str).ok_or_else(||"catalogue family missing".to_string())?;
        validate_family_records(family,preset.get("options"),entry.get("options"))?;
        if family=="neural-network"{validate_neural_links(chart,preset,entry)?;}
        let mut settings = options(family,preset.get("options"),true,false)?;
        if let Some(data)=preset.get("data").and_then(DslValue::as_str){
            if data!="demo"&&entry.get("data").and_then(DslValue::as_str)!=Some(data)&&!tables.contains(data){return Err(format!("unknown preset table {data}"));}
            if !settings.is_empty(){settings.push(',');}settings.push_str(&format!("data={}",token(data)?));
        }
        output.push_str(&format!("\\SemioVizChart{{{kind}}}[{settings}]\n"));
        work += 1; checkpoint(work)?;
    }
    let mut table_names=tables.clone();
    for (index, layer) in list(chart, "layers")?.iter().enumerate() {
        let data = match layer.get("data") { Some(value) => token(value.as_str().ok_or_else(|| "layer data must be a table name".to_string())?)?, None => list(chart,"tables")?.first().map(|table| name(table,"name")).transpose()?.ok_or_else(|| "layer needs a declared table".to_string())? };
        if !tables.contains(&data) { return Err(format!("unknown table {data}")); }
        let prepared = unique_name(format!("semio-print-layer-{index}"),&mut table_names);
        let table = list(chart,"tables")?.iter().find(|table| table.get("name").and_then(DslValue::as_str) == Some(&data)).ok_or_else(|| format!("missing table {data}"))?;
        let mut columns = list(table,"columns")?.iter().map(|column| token(column.as_str().ok_or_else(|| "column must be a string".to_string())?)).collect::<Result<Vec<_>, _>>()?;
        let mut constants = layer.get("encodings").and_then(DslValue::as_object).unwrap_or(&[]).iter().filter_map(|(channel, encoding)| encoding.get("value").map(|value| (channel, value))).collect::<Vec<_>>();
        let mut implicit_paints=std::collections::BTreeSet::new();
        for (channel,_) in option_schema()?.get("x-semio-option-syntax").and_then(|value|value.get("layer")).and_then(DslValue::as_object).ok_or_else(||"layer option descriptors missing".to_string())? {
            if columns.contains(channel)&&layer.get("encodings").and_then(|encodings|encodings.get(channel)).is_none()&&option_descriptor("layer",channel,false)?.syntax==ChartOptionSyntax::Paint {implicit_paints.insert(channel.clone());}
        }
        let mut option_paints=std::collections::BTreeMap::new();
        let mut layer_settings=Vec::new();
        if let Some(value)=layer.get("options") {
            let mut entries=value.as_object().ok_or_else(||"layer options must be an object".to_string())?.iter().collect::<Vec<_>>();entries.sort_by(|(left,_),(right,_)|left.cmp(right));
            for (channel,value) in entries {
                let descriptor=option_descriptor("layer",channel,false)?;
                if descriptor.syntax==ChartOptionSyntax::Paint {
                    let paint=native_paint(value.as_str().ok_or_else(||"literal layer paints must be strings".to_string())?)?;
                    if layer.get("encodings").and_then(|encodings|encodings.get(channel)).is_none() {option_paints.insert(channel.clone(),paint);if !implicit_paints.contains(channel){constants.push((channel,value));}}
                } else {layer_settings.push(format!("{}={{{}}}",token(channel)?,native_option(value,descriptor,false)?));}
            }
        }
        constants.sort_by(|(a,_),(b,_)|a.cmp(b));
        let mut paints=layer.get("encodings").and_then(DslValue::as_object).unwrap_or(&[]).iter().filter_map(|(channel,encoding)|(["fill","stroke"].contains(&channel.as_str())&&encoding.get("scale").is_none()).then(||encoding.get("column").and_then(DslValue::as_str).map(|column|(channel,column))).flatten()).collect::<Vec<_>>();
        paints.extend(implicit_paints.iter().map(|channel|(channel,channel.as_str())));
        paints.sort_by(|(a,_),(b,_)|a.cmp(b));
        let mut column_names=columns.iter().cloned().collect::<std::collections::BTreeSet<_>>();
        let constant_names=constants.iter().map(|(channel,_)|((*channel).clone(),unique_name(format!("semio-constant-{channel}"),&mut column_names))).collect::<std::collections::BTreeMap<_,_>>();
        let paint_names=paints.iter().map(|(channel,_)|((*channel).clone(),unique_name(format!("semio-paint-{channel}"),&mut column_names))).collect::<std::collections::BTreeMap<_,_>>();
        columns.extend(constant_names.values().cloned());
        columns.extend(paint_names.values().cloned());
        output.push_str(&format!("\\SemioVizTable{{{prepared}}}{{{}}}\n", columns.join(",")));
        for row in list(table,"rows")? {
            let values = columns.iter().map(|column|{
                let constant=constants.iter().find(|(channel,_)|constant_names.get(*channel)==Some(column));
                let paint=paints.iter().find(|(channel,_)|paint_names.get(*channel)==Some(column));
                let value=constant.map(|(_,value)|*value).or_else(||paint.and_then(|(_,column)|row.get(column))).or_else(||row.get(column));
                if let Some((channel,_))=paint.filter(|(channel,_)|implicit_paints.contains(channel.as_str())){match value{Some(DslValue::Null)=>Ok("none".into()),Some(DslValue::String(value))=>native_paint(value),Some(_)=>Err("implicit paint columns require text or null".into()),None=>Ok(option_paints.get(channel.as_str()).cloned().unwrap_or_default())}}else if let Some(paint)=constant.and_then(|(channel,_)|option_paints.get(channel.as_str())){Ok(paint.clone())}else if paint.is_some()||constant.is_some_and(|(channel,_)|["fill","stroke"].contains(&channel.as_str())){value.map(paint_scalar).unwrap_or_else(||Ok("none".into()))}else{row_cell(value)}.map(|value|format!("{{{value}}}"))
            }).collect::<Result<Vec<_>,_>>()?;
            output.push_str(&format!("\\SemioVizRow{{{prepared}}}{{{}}}\n",values.join(",")));
            work += 1; checkpoint(work)?;
        }
        for transform in list(layer, "transform")? {
            let settings = options(&format!("transform/{}",name(transform,"kind")?),transform.get("options"),false,false)?;
            let kind = name(transform, "kind")?;
            output.push_str(&format!("\\SemioVizTransform{{{prepared}}}{{{prepared}}}[kind={kind}{}{settings}]\n",if settings.is_empty(){""}else{","}));
            work += 1; checkpoint(work)?;
        }
        if let Some(layout) = layer.get("layout") {
            let side=|key,fallback|chart.get("margin").and_then(|margin|margin.get(key)).and_then(DslValue::as_f64).unwrap_or(fallback);
            let settings=options(&format!("layout/{}",name(layout,"algorithm")?),layout.get("options"),false,false)?;
            output.push_str(&format!("\\SemioVizLayout{{{}}}{{{prepared}}}{{{prepared}}}[width={},height={}{}{}]\n",name(layout,"algorithm")?,width-side("left",16.0)-side("right",8.0),height-side("top",8.0)-side("bottom",14.0),if settings.is_empty(){""}else{","},settings));
        }
        let mark = name(layer, "mark")?;
        let mut bindings = vec![format!("data={prepared}"), format!("mark={mark}"),format!("colorIndex={index}")];
        for channel in option_paints.keys().filter(|channel|!implicit_paints.contains(*channel)){bindings.push(format!("{channel}={{column={}}}",constant_names.get(channel).ok_or_else(||"literal paint was not prepared".to_string())?));}
        for channel in &implicit_paints{bindings.push(format!("{channel}={{column={}}}",paint_names.get(channel).ok_or_else(||"implicit paint was not prepared".to_string())?));}
        if let Some(encodings) = layer.get("encodings").and_then(DslValue::as_object) {
            let mut encodings=encodings.iter().collect::<Vec<_>>();encodings.sort_by(|(a,_),(b,_)|a.cmp(b));
            for (channel, encoding) in encodings {
                let channel = token(channel)?;
                let column = if encoding.get("value").is_some() { constant_names.get(&channel).cloned().ok_or_else(||"constant channel was not prepared".to_string())? } else if ["fill","stroke"].contains(&channel.as_str())&&encoding.get("scale").is_none(){paint_names.get(&channel).cloned().ok_or_else(||"paint channel was not prepared".to_string())?}else{ name(encoding, "column")? };
                let mut binding = format!("column={column}");
                if let Some(scale) = encoding.get("scale") {
                    let scale = token(scale.as_str().ok_or_else(|| "encoding scale must be a string".to_string())?)?;
                    if !scales.contains(&scale) { return Err(format!("unknown scale {scale}")); }
                    binding.push_str(&format!(",scale={scale}"));
                }
                bindings.push(format!("{channel}={{{binding}}}"));
            }
        }
        let settings = layer_settings.join(",");
        if !settings.is_empty() { bindings.push(settings); }
        if let Some(layout)=layer.get("layout"){
            bindings.push("layoutCoordinates=true".into());
            if layout.get("algorithm").and_then(DslValue::as_str)==Some("hexbin")&&layer.get("encodings").and_then(|encodings|encodings.get("shape")).is_none()&&layer.get("options").and_then(|options|options.get("shape")).is_none(){bindings.push("shape=hexagon".into());}
        }
        if let Some(coordinate) = chart.get("coordinate") {
            bindings.push(format!("coordinate={}", name(coordinate,"kind")?));
            let settings = options("coordinate",coordinate.get("options"),false,false)?;
            if !settings.is_empty() { bindings.push(format!("coordinateOptions={{{settings}}}")); }
        }
        let side=|key,fallback|chart.get("margin").and_then(|margin|margin.get(key)).and_then(DslValue::as_f64).unwrap_or(fallback);
        bindings.extend([format!("x0={}",side("left",16.0)),format!("y0={}",side("top",8.0)),format!("x1={}",width-side("right",8.0)),format!("y1={}",height-side("bottom",14.0))]);
        output.push_str(&format!("\\SemioVizPlot[{}]\n", bindings.join(",")));
        work += 1; checkpoint(work)?;
    }
    for guide in list(chart, "guides")? {
        let kind = name(guide, "kind")?;
        let mut settings = Vec::new();
        let mut fields=guide.as_object().ok_or_else(|| "guide must be an object".to_string())?.iter().collect::<Vec<_>>();fields.sort_by(|(a,_),(b,_)|a.cmp(b));
        for (key, value) in fields {
            if key == "kind" || key == "options" { continue; }
            let value = if key == "title" { value.get(&language).ok_or_else(|| "guide title missing selected language".to_string())? } else { value };
            settings.push(format!("{}={{{}}}", token(key)?, native_option(value,option_descriptor(&format!("guide/{kind}"),key,false)?,false)?));
        }
        if kind == "grid" { settings.extend(["grid=true".into(),"domainLine=false".into(),"labels=false".into(),"tickSize=0".into()]); }
        let additional = options(&format!("guide/{kind}"),guide.get("options"),false,false)?;
        if !additional.is_empty() { settings.push(additional); }
        let command = if kind == "legend" { "SemioVizLegend" } else if ["axis", "grid"].contains(&kind.as_str()) { "SemioVizAxis" } else { return Err(format!("unknown guide {kind}")); };
        output.push_str(&format!("\\{command}[{}]\n", settings.join(",")));
        work += 1; checkpoint(work)?;
    }
    for annotation in list(chart,"annotations")? {
        let kind = name(annotation,"kind")?;
        let x = scalar(annotation.get("x").ok_or_else(||"annotation x is absent".to_string())?)?;
        let y = scalar(annotation.get("y").ok_or_else(||"annotation y is absent".to_string())?)?;
        let settings = annotation_options(annotation.get("options"),["text","label"].contains(&kind.as_str()))?;
        let number = |key, fallback| annotation.get(key).and_then(DslValue::as_f64).unwrap_or(fallback);
        match kind.as_str() {
            "text"|"label" => { let content=annotation.get("text").and_then(|value|value.get(&language)).and_then(DslValue::as_str).unwrap_or(""); output.push_str(&format!("\\node[{settings}] at ({x},{y}) {{{}}};\n",text(content))); },
            "line"|"rule" => output.push_str(&format!("\\draw[{settings}] ({x},{y})--({},{});\n",number("x2",width),number("y2",number("y",0.0)))),
            "rect" => output.push_str(&format!("\\path[{settings}] ({x},{y}) rectangle ({},{});\n",number("x2",number("x",0.0)+number("width",0.0)),number("y2",number("y",0.0)+number("height",0.0)))),
            "circle"|"point" => output.push_str(&format!("\\path[{settings}] ({x},{y}) circle ({});\n",number("radius",1.0))),
            _ => return Err(format!("unknown annotation {kind}")),
        }
        work += 1; checkpoint(work)?;
    }
    output.push_str("\\end{VizFigure}\n\\endgroup\n");
    Ok(output)
}

impl paint::Paint {
    pub fn name(&self)->String{if self.alpha>=1.0{format!("semio-print-color-{}",self.hex)}else{format!("semio-print-color-{}-A{}",self.hex,self.alpha.to_string().replace('.',"p"))}}
    pub fn declaration(&self)->String{let name=self.name();format!("\\definecolor{{{name}}}{{HTML}}{{{}}}\n{}",self.hex,if self.alpha<1.0{format!("\\SemioVizPaintAlpha{{{name}}}{{{}}}\n",self.alpha)}else{String::new()})}
}
