//! 🛬️ Paid IFC2x3 ownership follows independently authored semantic rows and ordered edges.
use super::{identity,null,word,sort_paid,Ifc2x3Snapshot,Ifc2x3EdmPreamble,Part21Document,Part21Header,Part21Instance,Part21Value,Part21Decimal};
use crate::standards::v2x3::subsets::base::io::sqlite::snapshot::native;
use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};
use semio_framework_value::NativeDecodeControl;
use semio_framework_dsl_record::__rt::DecodedFieldOwner;
use std::ops::Range;

const TABLES:[(&str,usize);12]=[("ifc2x3_document",4),("ifc2x3_header",1),("ifc2x3_file_description_argument",4),("ifc2x3_file_name_argument",4),("ifc2x3_file_schema_argument",4),("ifc2x3_instance",4),("ifc2x3_entity_type",4),("ifc2x3_entity_argument",4),("ifc2x3_value",12),("ifc2x3_list_element",4),("ifc2x3_typed_argument",4),("ifc2x3_edm_preamble",17)];
fn invalid(message:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
#[derive(Clone, Copy)]
struct Relation{parent:i64,ordinal:i64,id:i64}
struct Table<'a>{rows:Vec<&'a SqliteRow>,seen:Vec<bool>,relations:Vec<Relation>}
impl Table<'_>{
    fn index(&self,id:i64)->Result<usize,ValueError>{self.rows.binary_search_by_key(&id,|row|row.rowid).map_err(|_|invalid("dangling IFC2x3 relational ownership"))}
    fn row(&self,id:i64)->Result<&SqliteRow,ValueError>{Ok(self.rows[self.index(id)?])}
    fn range(&self,parent:i64)->Range<usize>{self.relations.partition_point(|edge|edge.parent<parent)..self.relations.partition_point(|edge|edge.parent<=parent)}
}
struct Catalog<'a>{tables:[Table<'a>;12]}
impl<'a> Catalog<'a>{
    fn new(database:&'a SqliteDatabase,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
        let mut tables=std::array::from_fn(|_|Table{rows:Vec::new(),seen:Vec::new(),relations:Vec::new()});
        control.begin_stage(0)?;
        for(index,(name,width))in TABLES.iter().copied().enumerate(){
            let source=&database.table(name)?.rows;let table=&mut tables[index];
            table.rows=control.allocate_vec(source.len())?;table.seen=control.allocate_vec(source.len())?;table.seen.resize(source.len(),false);
            let relational=matches!(index,2..=7|9..=10);if relational{table.relations=control.allocate_vec(source.len())?;}
            for row in source{
                identity(row,width)?;table.rows.push(row);
                if relational{let parent=row.integer(1)?;let ordinal=row.integer(2)?;if parent<=0||ordinal<0{return Err(invalid("invalid IFC2x3 relationship owner or ordinal"));}table.relations.push(Relation{parent,ordinal,id:row.rowid});}
                control.step()?;
            }
            sort_paid(&mut table.rows,|left,right|{control.step()?;Ok(left.rowid.cmp(&right.rowid))})?;
            for pair in table.rows.windows(2){if pair[0].rowid==pair[1].rowid{return Err(invalid("duplicate IFC2x3 relational row identity"));}control.step()?;}
            sort_paid(&mut table.relations,|left,right|{control.step()?;Ok((left.parent,left.ordinal).cmp(&(right.parent,right.ordinal)))})?;
            let mut parent=None;let mut ordinal=0i64;
            for edge in &table.relations{
                if parent!=Some(edge.parent){parent=Some(edge.parent);ordinal=0;}
                if edge.ordinal!=ordinal{return Err(invalid("IFC2x3 relationship ordinals must be contiguous and unique"));}
                ordinal=ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC2x3 ordinal overflow"))?;control.step()?;
            }
        }
        Ok(Self{tables})
    }
    fn take(&mut self,table:usize,id:i64)->Result<&'a SqliteRow,ValueError>{
        let index=self.tables[table].index(id)?;
        if std::mem::replace(&mut self.tables[table].seen[index],true){return Err(invalid("multiply owned IFC2x3 relational row"));}
        Ok(self.tables[table].rows[index])
    }
    fn edge(&self,table:usize,index:usize)->Relation{self.tables[table].relations[index]}
    fn ensure_owned(&self,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{for table in &self.tables{for seen in &table.seen{if !seen{return Err(invalid("unowned IFC2x3 relational row"));}control.step()?;}}Ok(())}
}
struct Forest{state:Vec<u8>,values:DecodedFieldOwner<Vec<Option<Part21Value>>>,pending:Vec<(i64,bool)>}
impl Forest{
    fn new(count:usize,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
        let mut state=control.allocate_vec(count)?;state.resize(count,0);
        let mut values=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(count)?,native::close::<Vec<Option<Part21Value>>>);values.as_mut().resize_with(count,||None);
        let size=count.checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"IFC2x3 value frontier size overflow"))?;
        Ok(Self{state,values,pending:control.allocate_vec(size)?})
    }
    fn push(&mut self,item:(i64,bool))->Result<(),ValueError>{if self.pending.len()==self.pending.capacity(){return Err(invalid("IFC2x3 value frontier exceeds its unique ownership census"));}self.pending.push(item);Ok(())}
    fn pop(&mut self,catalog:&Catalog<'_>,id:i64)->Result<Part21Value,ValueError>{self.values.as_mut()[catalog.tables[8].index(id)?].take().ok_or_else(||invalid("missing or multiply owned IFC2x3 value"))}
    fn value(&mut self,catalog:&mut Catalog<'_>,id:i64,instances:&[(i64,u64)],words:&[u64],control:&mut NativeDecodeControl<'_>)->Result<Part21Value,ValueError>{
        if !self.pending.is_empty(){return Err(invalid("unfinished IFC2x3 value frontier"));}
        self.push((id,false))?;
        while let Some((id,exit))=self.pending.pop(){
            control.step()?;let index=catalog.tables[8].index(id)?;
            if !exit{
                if self.state[index]!=0{return Err(invalid("cyclic or multiply owned IFC2x3 value"));}
                self.state[index]=1;let row=catalog.take(8,id)?;let kind=row.text(1)?;
                let list=catalog.tables[9].range(id);let typed=catalog.tables[10].range(id);
                if(kind!="list"&&!list.is_empty())||(kind!="typed"&&!typed.is_empty()){return Err(invalid("IFC2x3 value relationship differs from its variant"));}
                self.push((id,true))?;
                for(table,range)in[(9,list),(10,typed)]{for edge in range.rev(){let row=catalog.take(table,catalog.edge(table,edge).id)?;self.push((row.integer(3)?,false))?;control.step()?;}}
                continue;
            }
            let row=catalog.tables[8].rows[index];let kind=row.text(1)?;
            for column in 2..12{let active=matches!((kind,column),("int",2)|("real",3..=6)|("str",7)|("enum",8)|("ref",9..=10)|("typed",11));if !active{null(row,column)?;}}
            let value=match kind{
                "unset"=>Part21Value::Unset,"derived"=>Part21Value::Derived,"int"=>Part21Value::Int(row.integer(2)?),
                "real"=>{
                    let negative=match row.integer(3)?{0=>false,1=>true,_=>return Err(invalid("invalid IFC2x3 decimal sign"))};
                    let scale=u32::try_from(row.integer(5)?).map_err(|_|invalid("IFC2x3 decimal scale exceeds unsigned32"))?;
                    let exponent=match &row.values[6]{SqliteValue::Null=>None,SqliteValue::Integer(value)=>Some(i32::try_from(*value).map_err(|_|invalid("IFC2x3 decimal exponent exceeds signed32"))?),_=>return Err(invalid("invalid IFC2x3 decimal exponent storage"))};
                    Part21Value::Real(Part21Decimal{negative,coefficient:control.copy_text(row.text(4)?)?,scale,exponent})
                },
                "str"=>Part21Value::Str(control.copy_text(row.text(7)?)?),"enum"=>Part21Value::Enum(control.copy_text(row.text(8)?)?),
                "ref"=>{let word=word(row.text(9)?)?;match &row.values[10]{SqliteValue::Null=>{if words.binary_search(&word).is_ok(){return Err(invalid("resolved IFC2x3 reference lacks its relationship"));}},SqliteValue::Integer(instance)=>{let actual=instances.binary_search_by_key(instance,|item|item.0).ok().map(|index|instances[index].1);if actual!=Some(word){return Err(invalid("IFC2x3 reference word differs from related instance identity"));}},_=>return Err(invalid("invalid IFC2x3 resolved reference storage"))}Part21Value::Ref(word)},
                "list"|"typed"=>{
                    let table=if kind=="list"{9}else{10};let range=catalog.tables[table].range(id);
                    let mut items=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(range.len())?,native::close::<Vec<Part21Value>>);
                    for edge in range{let row=catalog.tables[table].row(catalog.edge(table,edge).id)?;items.as_mut().push(self.pop(catalog,row.integer(3)?)?);control.step()?;}
                    if kind=="list"{Part21Value::List(items.take())}else{let name=control.copy_text(row.text(11)?)?;Part21Value::Typed{name,items:items.take()}}
                },
                _=>return Err(invalid("unknown IFC2x3 value kind")),
            };
            self.state[index]=2;self.values.as_mut()[index]=Some(value);
        }
        self.pop(catalog,id)
    }
    fn arguments(&mut self,catalog:&mut Catalog<'_>,table:usize,parent:i64,instances:&[(i64,u64)],words:&[u64],control:&mut NativeDecodeControl<'_>)->Result<Vec<Part21Value>,ValueError>{
        let range=catalog.tables[table].range(parent);let mut output=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(range.len())?,native::close::<Vec<Part21Value>>);
        for edge in range{let row=catalog.take(table,catalog.edge(table,edge).id)?;output.as_mut().push(self.value(catalog,row.integer(3)?,instances,words,control)?);control.step()?;}
        Ok(output.take())
    }
}
fn reconstruct(database:&SqliteDatabase,control:&mut NativeDecodeControl<'_>)->Result<Ifc2x3Snapshot,ValueError>{
    let mut catalog=Catalog::new(database,control)?;
    if catalog.tables[0].rows.len()!=1||catalog.tables[1].rows.len()!=1{return Err(invalid("IFC2x3 must own one document and one header"));}
    let document=catalog.take(0,catalog.tables[0].rows[0].rowid)?;let header=catalog.take(1,document.integer(2)?)?;
    let mut result=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Ifc2x3Snapshot{schema:String::new(),document:Part21Document{header:Part21Header{file_description:Vec::new(),file_name:Vec::new(),file_schema:Vec::new()},instances:Vec::new()},edm_preamble:None},native::close::<Ifc2x3Snapshot>);
    let out=result.as_mut();out.schema=control.copy_text(document.text(1)?)?;
    match &document.values[3]{
        SqliteValue::Null=>{},
        SqliteValue::Integer(id)=>{
            let row=catalog.take(11,*id)?;out.edm_preamble=Some(Ifc2x3EdmPreamble::default());let edm=out.edm_preamble.as_mut().unwrap();
            edm.producer=control.copy_text(row.text(1)?)?;edm.module=control.copy_text(row.text(2)?)?;edm.creation_date=control.copy_text(row.text(3)?)?;edm.host=control.copy_text(row.text(4)?)?;edm.database=control.copy_text(row.text(5)?)?;edm.database_version=control.copy_text(row.text(6)?)?;edm.database_creation_date=control.copy_text(row.text(7)?)?;edm.schema=control.copy_text(row.text(8)?)?;edm.model=control.copy_text(row.text(9)?)?;edm.model_creation_date=control.copy_text(row.text(10)?)?;edm.header_model=control.copy_text(row.text(11)?)?;edm.header_model_creation_date=control.copy_text(row.text(12)?)?;edm.user=control.copy_text(row.text(13)?)?;edm.group=control.copy_text(row.text(14)?)?;edm.license=control.copy_text(row.text(15)?)?;edm.options=control.copy_text(row.text(16)?)?;
        },
        _=>return Err(invalid("invalid IFC2x3 optional EDM relationship")),
    }
    let mut instances=control.allocate_vec(catalog.tables[5].rows.len())?;let mut words=control.allocate_vec(catalog.tables[5].rows.len())?;
    for row in &catalog.tables[5].rows{let word=word(row.text(3)?)?;instances.push((row.rowid,word));words.push(word);control.step()?;}
    sort_paid(&mut words,|left,right|{control.step()?;Ok(left.cmp(right))})?;
    for pair in words.windows(2){if pair[0]==pair[1]{return Err(invalid("duplicate IFC2x3 instance identifier"));}control.step()?;}
    let mut forest=Forest::new(catalog.tables[8].rows.len(),control)?;
    out.document.header.file_description=forest.arguments(&mut catalog,2,header.rowid,&instances,&words,control)?;
    out.document.header.file_name=forest.arguments(&mut catalog,3,header.rowid,&instances,&words,control)?;
    out.document.header.file_schema=forest.arguments(&mut catalog,4,header.rowid,&instances,&words,control)?;
    let range=catalog.tables[5].range(document.rowid);out.document.instances=control.allocate_vec(range.len())?;
    for edge in range{
        let row=catalog.take(5,catalog.edge(5,edge).id)?;let word=word(row.text(3)?)?;
        let range=catalog.tables[6].range(row.rowid);
        let mut instance=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Part21Instance{id:word,entities:control.allocate_vec(range.len())?},native::close::<Part21Instance>);
        for edge in range{
            let row=catalog.take(6,catalog.edge(6,edge).id)?;let name=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.copy_text(row.text(3)?)?,native::close::<String>);
            let arguments=forest.arguments(&mut catalog,7,row.rowid,&instances,&words,control)?;
            instance.as_mut().entities.push((name.take(),arguments));control.step()?;
        }
        out.document.instances.push(instance.take());control.step()?;
    }
    catalog.ensure_owned(control)?;
    if forest.values.as_mut().iter().any(Option::is_some){return Err(invalid("unowned IFC2x3 typed value"));}
    control.checkpoint()?;Ok(result.take())
}
pub(super) fn read(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Ifc2x3Snapshot,ValueError>{
    control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,progress,allocation|{
        let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|progress(event.completed,event.total);
        let mut native_allocation=|request:semio_framework_value::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=NativeDecodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);let result=reconstruct(database,&mut native);(result,native.owned_bytes())
    })?
}
