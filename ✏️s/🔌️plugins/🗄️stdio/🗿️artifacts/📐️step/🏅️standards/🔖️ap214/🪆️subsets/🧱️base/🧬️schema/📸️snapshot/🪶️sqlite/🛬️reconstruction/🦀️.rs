//! 🛬️ Paid STEP relational ownership follows domain rows and ordered edges.
use super::{identity,null,unsigned,sort_paid,REAL};
use super::{StepSnapshot,StepHeader,StepFileDescription,StepFileName,StepFileSchema,StepEntity,StepComplexType,StepValue};
use super::super::native;
use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind,artifact::{validate_ieee754_row,ieee754_is_null,read_binary64}};
use semio_framework_value::NativeDecodeControl;
use semio_framework_dsl_record::__rt::DecodedFieldOwner;
use std::ops::Range;

const TABLES:[(&str,usize);12]=[("step_document",3),("step_header",7),("step_description",4),("step_author",4),("step_organization",4),("step_schema_identifier",4),("step_entity",5),("step_complex_type",4),("step_value",10),("step_argument",5),("step_aggregate_element",4),("step_typed_value",4)];
fn invalid(message:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
#[derive(Clone, Copy)]
struct Relation{parent:(u8,i64),ordinal:i64,id:i64}
struct Table<'a>{rows:Vec<&'a SqliteRow>,seen:Vec<bool>,relations:Vec<Relation>}
impl Table<'_>{
    fn index(&self,id:i64)->Result<usize,ValueError>{self.rows.binary_search_by_key(&id,|row|row.rowid).map_err(|_|invalid("dangling STEP relational ownership"))}
    fn row(&self,id:i64)->Result<&SqliteRow,ValueError>{Ok(self.rows[self.index(id)?])}
    fn range(&self,parent:(u8,i64))->Range<usize>{self.relations.partition_point(|edge|edge.parent<parent)..self.relations.partition_point(|edge|edge.parent<=parent)}
}
struct Catalog<'a>{tables:[Table<'a>;12]}
impl<'a> Catalog<'a>{
    fn new(database:&'a SqliteDatabase,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
        let mut tables=std::array::from_fn(|_|Table{rows:Vec::new(),seen:Vec::new(),relations:Vec::new()});
        control.begin_stage(0)?;
        for(index,(name,width))in TABLES.iter().copied().enumerate(){
            let source=&database.table(name)?.rows;let table=&mut tables[index];
            table.rows=control.allocate_vec(source.len())?;table.seen=control.allocate_vec(source.len())?;table.seen.resize(source.len(),false);
            let relational=matches!(index,2..=7|9..=11);if relational{table.relations=control.allocate_vec(source.len())?;}
            for row in source{
                identity(row,width)?;table.rows.push(row);
                if relational{
                    let(parent,ordinal)=match index{
                        9=>{let parent=match(&row.values[1],&row.values[2]){(SqliteValue::Integer(id),SqliteValue::Null)=>(0,*id),(SqliteValue::Null,SqliteValue::Integer(id))=>(1,*id),_=>return Err(invalid("STEP argument must have exactly one owner"))};(parent,row.integer(3)?)},
                        11=>((0,row.integer(1)?),0),
                        _=>((0,row.integer(1)?),row.integer(2)?),
                    };
                    if parent.1<=0||ordinal<0{return Err(invalid("invalid STEP relationship owner or ordinal"));}
                    table.relations.push(Relation{parent,ordinal,id:row.rowid});
                }
                control.step()?;
            }
            sort_paid(&mut table.rows,|left,right|{control.step()?;Ok(left.rowid.cmp(&right.rowid))})?;
            for pair in table.rows.windows(2){if pair[0].rowid==pair[1].rowid{return Err(invalid("duplicate STEP relational row identity"));}control.step()?;}
            sort_paid(&mut table.relations,|left,right|{control.step()?;Ok((left.parent,left.ordinal).cmp(&(right.parent,right.ordinal)))})?;
            let mut parent=None;let mut ordinal=0i64;
            for edge in &table.relations{
                if parent!=Some(edge.parent){parent=Some(edge.parent);ordinal=0;}
                if edge.ordinal!=ordinal{return Err(invalid("STEP relationship ordinals must be contiguous and unique"));}
                ordinal=ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"STEP ordinal overflow"))?;control.step()?;
            }
        }
        Ok(Self{tables})
    }
    fn take(&mut self,table:usize,id:i64)->Result<&'a SqliteRow,ValueError>{
        let index=self.tables[table].index(id)?;
        if std::mem::replace(&mut self.tables[table].seen[index],true){return Err(invalid("multiply owned STEP relational row"));}
        Ok(self.tables[table].rows[index])
    }
    fn edge(&self,table:usize,index:usize)->Relation{self.tables[table].relations[index]}
    fn list(&mut self,table:usize,header:i64,control:&mut NativeDecodeControl<'_>)->Result<Vec<String>,ValueError>{
        let range=self.tables[table].range((0,header));let mut output=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(range.len())?,native::close::<Vec<String>>);
        for index in range{let row=self.take(table,self.edge(table,index).id)?;output.as_mut().push(control.copy_text(row.text(3)?)?);control.step()?;}
        Ok(output.take())
    }
    fn ensure_owned(&self,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{for table in &self.tables{for seen in &table.seen{if !seen{return Err(invalid("unowned STEP relational row"));}control.step()?;}}Ok(())}
}
struct Forest{
    state:Vec<u8>,
    values:DecodedFieldOwner<Vec<Option<StepValue>>>,
    pending:Vec<(i64,bool)>,
}
impl Forest{
    fn new(count:usize,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
        let mut state=control.allocate_vec(count)?;state.resize(count,0);
        let mut values=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(count)?,native::close::<Vec<Option<StepValue>>>);values.as_mut().resize_with(count,||None);
        let size=count.checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"STEP value frontier size overflow"))?;
        Ok(Self{state,values,pending:control.allocate_vec(size)?})
    }
    fn push(&mut self,item:(i64,bool))->Result<(),ValueError>{if self.pending.len()==self.pending.capacity(){return Err(invalid("STEP value frontier exceeds its unique ownership census"));}self.pending.push(item);Ok(())}
    fn pop(&mut self,catalog:&Catalog<'_>,id:i64)->Result<StepValue,ValueError>{self.values.as_mut()[catalog.tables[8].index(id)?].take().ok_or_else(||invalid("missing or multiply owned STEP value"))}
    fn value(&mut self,catalog:&mut Catalog<'_>,id:i64,entities:&[(i64,u64)],control:&mut NativeDecodeControl<'_>)->Result<StepValue,ValueError>{
        if !self.pending.is_empty(){return Err(invalid("unfinished STEP value frontier"));}
        self.push((id,false))?;
        while let Some((id,exit))=self.pending.pop(){
            control.step()?;let index=catalog.tables[8].index(id)?;
            if !exit{
                if self.state[index]!=0{return Err(invalid("cyclic or multiply owned STEP value"));}
                self.state[index]=1;let row=catalog.take(8,id)?;let kind=row.text(1)?;
                let aggregate=catalog.tables[10].range((0,id));let typed=catalog.tables[11].range((0,id));
                if(kind!="aggregate"&&!aggregate.is_empty())||(kind!="typedValue"&&!typed.is_empty())||(kind=="typedValue"&&typed.len()!=1){return Err(invalid("STEP value relationship differs from its variant"));}
                self.push((id,true))?;
                for edge in aggregate.rev(){let row=catalog.take(10,catalog.edge(10,edge).id)?;self.push((row.integer(3)?,false))?;control.step()?;}
                for edge in typed{let row=catalog.take(11,catalog.edge(11,edge).id)?;self.push((row.integer(3)?,false))?;control.step()?;}
                continue;
            }
            let row=catalog.tables[8].rows[index];validate_ieee754_row(row,8,REAL)?;let kind=row.text(1)?;
            let active=match kind{"integer"=>Some(2),"real"=>Some(3),"string"=>Some(4),"enum"=>Some(5),"reference"=>Some(6),"unset"|"derived"|"aggregate"|"typedValue"=>None,_=>return Err(invalid("unknown STEP value kind"))};
            for column in 2..8{if Some(column)!=active&&!(kind=="reference"&&column==7){null(row,column)?;}}
            if kind!="real"&&!ieee754_is_null(row,3,REAL)?{return Err(invalid("inactive STEP real companions"));}
            let value=match kind{
                "unset"=>StepValue::Unset,"derived"=>StepValue::Derived,"integer"=>StepValue::Integer(row.integer(2)?),"real"=>StepValue::Real(read_binary64(row,3,REAL)?),
                "string"=>StepValue::String(control.copy_text(row.text(4)?)?),"enum"=>StepValue::Enum(control.copy_text(row.text(5)?)?),
                "reference"=>{let word=unsigned(row.text(6)?)?;let entity=row.integer(7)?;let actual=entities.binary_search_by_key(&entity,|item|item.0).ok().map(|index|entities[index].1);if actual!=Some(word){return Err(invalid("STEP reference word differs from related entity identity"));}StepValue::Reference(word)},
                "aggregate"=>{let range=catalog.tables[10].range((0,id));let mut output=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(range.len())?,native::close::<Vec<StepValue>>);for edge in range{let row=catalog.tables[10].row(catalog.edge(10,edge).id)?;output.as_mut().push(self.pop(catalog,row.integer(3)?)?);control.step()?;}StepValue::Aggregate(output.take())},
                "typedValue"=>{let edge=catalog.tables[11].range((0,id)).start;let row=catalog.tables[11].row(catalog.edge(11,edge).id)?;let name=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.copy_text(row.text(2)?)?,native::close::<String>);let mut slot=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(1)?,native::close::<Vec<StepValue>>);slot.as_mut().push(self.pop(catalog,row.integer(3)?)?);StepValue::TypedValue{type_name:name.take(),value:native::box_slot(slot.take())?}},
                _=>unreachable!(),
            };
            self.state[index]=2;self.values.as_mut()[index]=Some(value);
        }
        self.pop(catalog,id)
    }
    fn arguments(&mut self,catalog:&mut Catalog<'_>,parent:(u8,i64),entities:&[(i64,u64)],control:&mut NativeDecodeControl<'_>)->Result<Vec<StepValue>,ValueError>{
        let range=catalog.tables[9].range(parent);let mut output=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(range.len())?,native::close::<Vec<StepValue>>);
        for edge in range{let row=catalog.take(9,catalog.edge(9,edge).id)?;output.as_mut().push(self.value(catalog,row.integer(4)?,entities,control)?);control.step()?;}
        Ok(output.take())
    }
}
fn reconstruct(database:&SqliteDatabase,control:&mut NativeDecodeControl<'_>)->Result<StepSnapshot,ValueError>{
    let mut catalog=Catalog::new(database,control)?;
    if catalog.tables[0].rows.len()!=1||catalog.tables[1].rows.len()!=1{return Err(invalid("STEP must own one document and one header"));}
    let document=catalog.take(0,catalog.tables[0].rows[0].rowid)?;let header=catalog.take(1,document.integer(2)?)?;
    let mut result=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(StepSnapshot{schema:String::new(),header:StepHeader{file_description:StepFileDescription{description:Vec::new(),implementation_level:String::new()},file_name:StepFileName{name:String::new(),timestamp:String::new(),author:Vec::new(),organization:Vec::new(),preprocessor_version:String::new(),originating_system:String::new(),authorization:String::new()},file_schema:StepFileSchema{schemas:Vec::new()}},entities:Vec::new()},native::close::<StepSnapshot>);
    let out=result.as_mut();out.schema=control.copy_text(document.text(1)?)?;
    out.header.file_description.implementation_level=control.copy_text(header.text(1)?)?;
    out.header.file_description.description=catalog.list(2,header.rowid,control)?;
    out.header.file_name.name=control.copy_text(header.text(2)?)?;out.header.file_name.timestamp=control.copy_text(header.text(3)?)?;
    out.header.file_name.author=catalog.list(3,header.rowid,control)?;out.header.file_name.organization=catalog.list(4,header.rowid,control)?;
    out.header.file_name.preprocessor_version=control.copy_text(header.text(4)?)?;out.header.file_name.originating_system=control.copy_text(header.text(5)?)?;out.header.file_name.authorization=control.copy_text(header.text(6)?)?;
    out.header.file_schema.schemas=catalog.list(5,header.rowid,control)?;
    let mut entities=control.allocate_vec(catalog.tables[6].rows.len())?;let mut words=control.allocate_vec(catalog.tables[6].rows.len())?;
    for row in &catalog.tables[6].rows{let word=unsigned(row.text(3)?)?;entities.push((row.rowid,word));words.push(word);control.step()?;}
    sort_paid(&mut words,|left,right|{control.step()?;Ok(left.cmp(right))})?;
    for pair in words.windows(2){if pair[0]==pair[1]{return Err(invalid("duplicate STEP instance identifier"));}control.step()?;}
    let mut forest=Forest::new(catalog.tables[8].rows.len(),control)?;
    let range=catalog.tables[6].range((0,document.rowid));out.entities=control.allocate_vec(range.len())?;
    for edge in range{
        let row=catalog.take(6,catalog.edge(6,edge).id)?;let word=unsigned(row.text(3)?)?;
        let mut entity=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(StepEntity{id:word,name:control.copy_text(row.text(4)?)?,args:Vec::new(),complex:Vec::new()},native::close::<StepEntity>);
        entity.as_mut().args=forest.arguments(&mut catalog,(0,row.rowid),&entities,control)?;
        let range=catalog.tables[7].range((0,row.rowid));entity.as_mut().complex=control.allocate_vec(range.len())?;
        for edge in range{
            let row=catalog.take(7,catalog.edge(7,edge).id)?;let mut complex=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(StepComplexType{name:control.copy_text(row.text(3)?)?,args:Vec::new()},native::close::<StepComplexType>);
            complex.as_mut().args=forest.arguments(&mut catalog,(1,row.rowid),&entities,control)?;
            entity.as_mut().complex.push(complex.take());control.step()?;
        }
        out.entities.push(entity.take());control.step()?;
    }
    catalog.ensure_owned(control)?;
    if forest.values.as_mut().iter().any(Option::is_some){return Err(invalid("unowned STEP typed value"));}
    control.checkpoint()?;Ok(result.take())
}
pub(super) fn read(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<StepSnapshot,ValueError>{
    control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,progress|{
        let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|progress(event.completed,event.total);
        let mut native=NativeDecodeControl::new(remaining,&mut callback);let result=reconstruct(database,&mut native);(result,native.owned_bytes())
    })?
}

