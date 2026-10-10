//! 🏛️ Original schema tokens and scalar declarations retained through caller-funded close.
use super::{Result,SqliteDatabase,SqliteTable,SqliteSnapshotControl,SqliteSnapshotPhase,ascii_compare,heap_sort};
use semio_framework_value::{ValueError,ValueRefusalKind,list::PagedList};

fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn overflow()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"SQLite schema storage limit")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(overflow)}
fn reserve<T>(values:&mut Vec<T>,count:usize,control:&mut SqliteSnapshotControl<'_>)->Result<()>{if values.capacity()!=0{return Err(overflow())}let bytes=count.checked_mul(size_of::<T>()).filter(|v|*v<=isize::MAX as usize).ok_or_else(overflow)?;control.admit_allocation_bytes(bytes)?;values.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"SQLite schema storage allocation"))}

#[derive(Debug,semio_framework_value::RetireOwned)]
struct Token{text:String,quoted:bool,literal:bool,start:usize,end:usize}
impl Token{fn keyword(&self,value:&str)->bool{!self.quoted&&!self.literal&&self.text.eq_ignore_ascii_case(value)}}
#[derive(Debug,semio_framework_value::RetireOwned)]
struct Column{name:usize,integer:bool}
#[derive(Debug,semio_framework_value::RetireOwned)]
struct Table{name:usize,start:usize,end:usize,columns_start:usize,columns_end:usize}
#[derive(Debug,semio_framework_value::RetireOwned)]
struct Unit{tokens:Vec<Token>,tables:Vec<Table>,columns:Vec<Column>,groups:Vec<(usize,usize)>,names:Vec<usize>,identifiers:Vec<u8>,table_order:Vec<usize>}
impl Unit{fn empty()->Self{Self{tokens:Vec::new(),tables:Vec::new(),columns:Vec::new(),groups:Vec::new(),names:Vec::new(),identifiers:Vec::new(),table_order:Vec::new()}}}
/// 🫙️ Holds every original schema-validation allocation across success, refusal and cancellation.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct SchemaValidationStorage{units:PagedList<Unit,{usize::MAX}>}
impl SchemaValidationStorage{
 pub fn empty()->Self{Self{units:PagedList::new()}}
 fn parse(&mut self,sql:&str,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<usize>{
  if sql.len()>control.limits().max_schema_bytes{return Err(overflow())}
  while !self.units.has_reserved_slot(){let quote=self.units.next_allocation_bytes().map_err(|_|overflow())?;let available=control.allocation_remaining_bytes();control.admit_allocation_bytes(quote)?;let result=self.units.reserve_one(available).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"SQLite schema owner page"))?;if result.allocated_bytes!=quote{return Err(invalid("SQLite schema owner extent"))}}
  let index=self.units.len();self.units.push_reserved(Unit::empty()).map_err(|_|invalid("SQLite schema owner slot"))?;let unit=&mut self.units[index];lex_into(unit,sql,phase,control)?;parse_into(unit,phase,control)?;Ok(index)
 }
}

#[derive(Clone,Copy)]
struct Span{start:usize,end:usize,content:usize,decoded:usize,quoted:bool,literal:bool,close:u8}
fn tick(at:usize,total:usize,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<()>{if at%65536==0{control.checkpoint(phase,at,total)?}Ok(())}
fn next(sql:&str,at:&mut usize,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<Option<Span>>{
 let bytes=sql.as_bytes();while *at<bytes.len(){tick(*at,bytes.len(),phase,control)?;if bytes[*at].is_ascii_whitespace(){*at+=1;continue}if bytes[*at..].starts_with(b"--"){*at+=2;while *at<bytes.len()&&bytes[*at]!=b'\n'{tick(*at,bytes.len(),phase,control)?;*at+=1;}continue}if bytes[*at..].starts_with(b"/*"){*at+=2;while *at+1<bytes.len()&&!bytes[*at..].starts_with(b"*/"){tick(*at,bytes.len(),phase,control)?;*at+=1;}if *at+1>=bytes.len(){return Err(invalid("unterminated SQL comment"))}*at+=2;continue}break}
 if *at==bytes.len(){return Ok(None)}let start=*at;let quoted=matches!(bytes[*at],b'"'|b'`'|b'[');let literal=bytes[*at]==b'\'';let mut decoded=0;let mut content=start;let mut close=0;
 if quoted||literal{close=if bytes[*at]==b'['{b']'}else{bytes[*at]};*at+=1;content=*at;loop{if *at>=bytes.len(){return Err(invalid("unterminated SQL quote"))}tick(*at,bytes.len(),phase,control)?;if bytes[*at]==close{*at+=1;if close!=b']'&&bytes.get(*at)==Some(&close){*at+=1;decoded=add(decoded,1)?;}else{break}}else{decoded=add(decoded,1)?;*at+=1;}}}else{*at+=1;if bytes[start].is_ascii_alphanumeric()||bytes[start]==b'_'||bytes[start]>=128{while *at<bytes.len()&&(bytes[*at].is_ascii_alphanumeric()||bytes[*at]==b'_'||bytes[*at]>=128){tick(*at,bytes.len(),phase,control)?;*at+=1;}}decoded=*at-start;}
 Ok(Some(Span{start,end:*at,content,decoded,quoted,literal,close}))
}
fn text_into(text:&mut String,sql:&str,span:Span,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<()>{
 control.admit_allocation_bytes(span.decoded)?;text.try_reserve_exact(span.decoded).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"SQLite schema token allocation"))?;
 let end=span.end-usize::from(span.quoted||span.literal);let mut at=span.content;let bytes=sql.as_bytes();while at<end{let mut stop=at.saturating_add(65536).min(end);while !sql.is_char_boundary(stop){stop+=1;}if span.quoted||span.literal{let mut segment=at;while at<stop{if span.close!=b']'&&bytes[at]==span.close&&bytes.get(at+1)==Some(&span.close){text.push_str(&sql[segment..at]);text.push(span.close as char);at+=2;segment=at;stop=stop.max(at);}else{at+=1;}}text.push_str(&sql[segment..at]);}else{text.push_str(&sql[at..stop]);at=stop;}control.checkpoint(phase,at,sql.len())?;}Ok(())
}
fn lex_into(unit:&mut Unit,sql:&str,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<()>{
 let mut at=0;let mut count=0;while next(sql,&mut at,phase,control)?.is_some(){count=add(count,1)?;}reserve(&mut unit.tokens,count,control)?;reserve(&mut unit.tables,count,control)?;reserve(&mut unit.columns,count,control)?;reserve(&mut unit.groups,count,control)?;reserve(&mut unit.names,count,control)?;reserve(&mut unit.identifiers,count,control)?;unit.identifiers.resize(count,0);at=0;
 while let Some(span)=next(sql,&mut at,phase,control)?{unit.tokens.push(Token{text:String::new(),quoted:span.quoted,literal:span.literal,start:span.start,end:span.end});text_into(&mut unit.tokens.last_mut().unwrap().text,sql,span,phase,control)?;}control.checkpoint(phase,sql.len(),sql.len())
}
fn parse_into(unit:&mut Unit,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<()>{
 let mut start=0;for end in 0..=unit.tokens.len(){if end!=unit.tokens.len()&&!unit.tokens[end].keyword(";"){continue}if start<end{parse_table_into(unit,start,end,phase,control)?;}start=end+1;}
 unit.names.clear();for table in &unit.tables{unit.names.push(table.name);}heap_sort(&mut unit.names,phase,control,|a,b,c|ascii_compare(&unit.tokens[*a].text,&unit.tokens[*b].text,phase,c))?;for pair in unit.names.windows(2){if ascii_compare(&unit.tokens[pair[0]].text,&unit.tokens[pair[1]].text,phase,control)?.is_eq(){return Err(invalid("SQLite duplicate declared table"))}}Ok(())
}
fn parse_table_into(unit:&mut Unit,start:usize,end:usize,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<()>{
 let tokens=&unit.tokens;let slice=&tokens[start..end];if slice.len()<5||!slice[0].keyword("CREATE")||!slice[1].keyword("TABLE")||slice[2].literal||slice[2].text.is_empty()||slice[3].text!="("{return Err(invalid("expected CREATE TABLE"))}if slice.iter().any(|token|["UNIQUE","AUTOINCREMENT","WITHOUT","DESC","GENERATED"].iter().any(|word|token.keyword(word))){return Err(invalid("unsupported implicit index or table feature"))}if slice[2].text.get(..7).is_some_and(|v|v.eq_ignore_ascii_case("sqlite_")){return Err(invalid("reserved table name"))}
 let groups_start=unit.groups.len();let mut depth=1usize;let mut group_start=start+4;let mut close=None;for at in start+4..end{let token=&tokens[at];if !token.quoted&&!token.literal{match token.text.as_str(){"("=>depth=add(depth,1)?,")"=>{depth=depth.checked_sub(1).ok_or_else(||invalid("unbalanced table definition"))?;if depth==0{if group_start<at{unit.groups.push((group_start,at));}close=Some(at);break}},"," if depth==1=>{if group_start==at{return Err(invalid("empty column definition"))}unit.groups.push((group_start,at));group_start=at+1},_=>{}}}if at%256==0{control.checkpoint(phase,at,tokens.len())?;}}
 let close=close.ok_or_else(||invalid("unbalanced table definition"))?;if close+1!=end||groups_start==unit.groups.len(){return Err(invalid("unsupported table suffix or empty table"))}let columns_start=unit.columns.len();let mut primary=None;
 for &(mut from,to) in &unit.groups[groups_start..]{let mut group=&tokens[from..to];if group[0].keyword("CONSTRAINT"){if group.len()<3{return Err(invalid("incomplete named constraint"))}from+=2;group=&tokens[from..to];}if group[0].keyword("PRIMARY"){if group.len()<5||!group[1].keyword("KEY")||group[2].text!="("||group.last().is_none_or(|t|t.text!=")")||(group.len()!=5&&!(group.len()==6&&group[4].keyword("ASC")))||primary.is_some(){return Err(invalid("unsupported table primary key"))}primary=Some(from+3);continue}if group[0].keyword("FOREIGN")||group[0].keyword("CHECK"){continue}if group[0].literal||group[0].text.is_empty(){return Err(invalid("invalid column"))}
  let constraints=["PRIMARY","NOT","NULL","DEFAULT","CHECK","REFERENCES","COLLATE","CONSTRAINT"];let type_end=(1..group.len()).find(|i|constraints.iter().any(|word|group[*i].keyword(word))).unwrap_or(group.len());let mut depth=0usize;for(index,token)in group.iter().enumerate(){if !token.quoted&&!token.literal{if token.text=="("{depth=add(depth,1)?;}else if token.text==")"{depth=depth.checked_sub(1).ok_or_else(||invalid("unbalanced column definition"))?;}}if index%256==0{control.checkpoint(phase,from+index,tokens.len())?;}}
  if group.windows(2).any(|v|v[0].keyword("PRIMARY")&&v[1].keyword("KEY")){if primary.is_some(){return Err(invalid("multiple primary keys"))}primary=Some(from);}unit.columns.push(Column{name:from,integer:type_end==2&&group[1].text.eq_ignore_ascii_case("INTEGER")});
 }
 let columns_end=unit.columns.len();if columns_start==columns_end{return Err(invalid("table has no columns"))}if columns_end-columns_start>control.limits().max_columns{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"SQLite columns"))}unit.names.clear();let names_start=0;for column in &unit.columns[columns_start..]{unit.names.push(column.name);}heap_sort(&mut unit.names[names_start..],phase,control,|a,b,c|ascii_compare(&tokens[*a].text,&tokens[*b].text,phase,c))?;for pair in unit.names[names_start..].windows(2){if ascii_compare(&tokens[pair[0]].text,&tokens[pair[1]].text,phase,control)?.is_eq(){return Err(invalid("duplicate column"))}}
 if let Some(primary)=primary{let mut column=None;for candidate in &unit.columns[columns_start..]{if ascii_compare(&tokens[candidate.name].text,&tokens[primary].text,phase,control)?.is_eq(){column=Some(candidate);break}}if !column.ok_or_else(||invalid("unknown primary key column"))?.integer{return Err(invalid("non-INTEGER primary key creates implicit index"))}}
 unit.tables.push(Table{name:start+2,start,end,columns_start,columns_end});if unit.tables.len()>control.limits().max_tables{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"SQLite tables"))}Ok(())
}

fn match_table(actual:&Unit,a:&Table,actual_sql:&str,expected:&mut Unit,b_index:usize,expected_sql:&str,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<bool>{
 let b=&expected.tables[b_index];if a.end-a.start!=b.end-b.start{return Ok(false)}let tokens=&expected.tokens;let marks=&mut expected.identifiers;marks[b.name]=1;let mut depth=0usize;let mut start=false;let syntax=["NOT","NULL","IS","IN","BETWEEN","AND","OR","LIKE","GLOB","MATCH","REGEXP","ESCAPE","CASE","WHEN","THEN","ELSE","END","COLLATE","AS"];
 for position in b.start+3..b.end{let token=&tokens[position];if token.keyword("("){depth=add(depth,1)?;if depth==1{start=true}continue}if token.keyword(")"){depth=depth.saturating_sub(1);continue}if depth==1&&token.keyword(","){start=true;continue}if start{if token.keyword("CONSTRAINT"){if position+1<b.end{marks[position+1]=1;}}else if !["PRIMARY","FOREIGN","CHECK"].iter().any(|word|token.keyword(word)){marks[position]=1;}start=false;}
  if token.keyword("REFERENCES"){if position+1<b.end{marks[position+1]=1;}if tokens.get(position+2).is_some_and(|t|t.keyword("(")){let mut at=position+3;while at<b.end&&!tokens[at].keyword(")"){if !tokens[at].keyword(","){marks[at]=1;}control.checkpoint(phase,at,tokens.len())?;at+=1;}}}if token.keyword("COLLATE")&&position+1<b.end{marks[position+1]=1;}if !token.literal&&(token.quoted||!syntax.iter().any(|word|token.keyword(word))){for column in &expected.columns[b.columns_start..b.columns_end]{if ascii_compare(&tokens[column.name].text,&token.text,phase,control)?.is_eq(){marks[position]=1;break}}}if position%256==0{control.checkpoint(phase,position,tokens.len())?;}
 }
 for (offset,(left,right))in actual.tokens[a.start..a.end].iter().zip(&tokens[b.start..b.end]).enumerate(){if left.literal||right.literal{if left.literal!=right.literal||!super::compare_text(&actual_sql[left.start..left.end],&expected_sql[right.start..right.end],phase,control)?.is_eq(){return Ok(false)}}else if !ascii_compare(&left.text,&right.text,phase,control)?.is_eq()||marks[b.start+offset]==0&&left.quoted!=right.quoted{return Ok(false)}if offset%256==0{control.checkpoint(phase,offset,a.end-a.start)?;}}Ok(true)
}

fn validate(storage:&mut SchemaValidationStorage,database:&SqliteDatabase,sql:&str,exact:bool,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<()>{
 let expected=storage.parse(sql,phase,control)?;if database.tables.len()>control.limits().max_tables{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"SQLite tables"))}if exact&&database.tables.len()!=storage.units[expected].tables.len(){return Err(invalid("SQLite artifact table count"))}reserve(&mut storage.units[expected].table_order,database.tables.len(),control)?;let mut schema_bytes=0usize;
 for(index,table)in database.tables.iter().enumerate(){schema_bytes=add(add(schema_bytes,table.sql.len())?,table.name.len())?;if schema_bytes>control.limits().max_schema_bytes{return Err(overflow())}storage.units[expected].table_order.push(index);}heap_sort(&mut storage.units[expected].table_order,phase,control,|a,b,c|ascii_compare(&database.tables[*a].name,&database.tables[*b].name,phase,c))?;for pair in storage.units[expected].table_order.windows(2){if ascii_compare(&database.tables[pair[0]].name,&database.tables[pair[1]].name,phase,control)?.is_eq(){return Err(invalid("SQLite artifact duplicate table name"))}}
 for declared in 0..storage.units[expected].tables.len(){let name_index=storage.units[expected].tables[declared].name;let name=&storage.units[expected].tokens[name_index].text;let mut found=None;let(mut low,mut high)=(0,storage.units[expected].table_order.len());while low<high{let middle=low+(high-low)/2;let table=&database.tables[storage.units[expected].table_order[middle]];match ascii_compare(&table.name,name,phase,control)?{std::cmp::Ordering::Less=>low=middle+1,std::cmp::Ordering::Greater=>high=middle,std::cmp::Ordering::Equal=>{found=Some(storage.units[expected].table_order[middle]);break}}}let table=&database.tables[found.ok_or_else(||invalid("SQLite missing declared table"))?];let actual=storage.parse(&table.sql,phase,control)?;if storage.units[actual].tables.len()!=1{return Err(invalid("SQLite artifact table schema"))}let actual_unit=&storage.units[actual];if !ascii_compare(&actual_unit.tokens[actual_unit.tables[0].name].text,&table.name,phase,control)?.is_eq(){return Err(invalid("SQLite artifact table name"))}let mut units=storage.units.iter_mut();let expected_unit=units.nth(expected).unwrap();let actual_unit=units.nth(actual-expected-1).unwrap();if !match_table(actual_unit,&actual_unit.tables[0],&table.sql,expected_unit,declared,sql,phase,control)?{return Err(invalid("SQLite artifact table schema"))}
 }Ok(())
}
/// 🏗️ Validates exactly the declared tables while retaining every original scratch owner.
pub fn validate_database_into(storage:&mut SchemaValidationStorage,database:&SqliteDatabase,sql:&str,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<()>{validate(storage,database,sql,true,phase,control)}
/// 🧩️ Validates all declared component tables in a composed database without constructing a schema database.
pub fn validate_component_into(storage:&mut SchemaValidationStorage,database:&SqliteDatabase,sql:&str,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<()>{validate(storage,database,sql,false,phase,control)}
/// 🪧️ Validates one original metadata table with retained lexical and declaration storage.
pub fn validate_table_into(storage:&mut SchemaValidationStorage,table:&SqliteTable,sql:&str,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<()>{let expected=storage.parse(sql,phase,control)?;let actual=storage.parse(&table.sql,phase,control)?;if storage.units[actual].tables.len()!=1||storage.units[expected].tables.len()!=1{return Err(invalid("SQLite artifact table schema"))}let actual_unit=&storage.units[actual];let actual_table=&actual_unit.tables[0];if !ascii_compare(&table.name,&actual_unit.tokens[actual_table.name].text,phase,control)?.is_eq()||!ascii_compare(&table.name,&storage.units[expected].tokens[storage.units[expected].tables[0].name].text,phase,control)?.is_eq(){return Err(invalid("SQLite artifact table name"))}let mut units=storage.units.iter_mut();let expected_unit=units.nth(expected).unwrap();let actual_unit=units.nth(actual-expected-1).unwrap();if !match_table(actual_unit,&actual_unit.tables[0],&table.sql,expected_unit,0,sql,phase,control)?{return Err(invalid("SQLite artifact table schema"))}Ok(())}

/// 🏛️ Creates exact authored tables inside the original output before each fallible field copy.
pub fn construct_database_into(storage:&mut SchemaValidationStorage,database:&mut SqliteDatabase,sql:&str,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<()>{
 if database.tables.capacity()!=0{return Err(invalid("schema output already owns table backing"))}
 let index=storage.parse(sql,phase,control)?;let unit=&storage.units[index];
 reserve(&mut database.tables,unit.tables.len(),control)?;
 let mut bytes=0;
 for table in &unit.tables{
  let name=&unit.tokens[table.name].text;
  let declaration=sql[unit.tokens[table.start].start..unit.tokens[table.end-1].end].trim();
  bytes=add(add(bytes,name.len())?,declaration.len())?;
  if bytes>control.limits().max_schema_bytes{return Err(overflow())}
  database.tables.push(SqliteTable{name:String::new(),sql:String::new(),rows:Vec::new()});
  let output=database.tables.last_mut().unwrap();
  crate::artifact::projection::copy_text_into(&mut output.name,name,phase,control)?;
  crate::artifact::projection::copy_text_into(&mut output.sql,declaration,phase,control)?;
 }
 control.checkpoint(phase,sql.len(),sql.len())
}
