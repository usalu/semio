//! 🧰️ Joins authored mesh assets by stable keys and checks compatible duplicates.

use super::*;
use semio_framework_value::retirement::RetireOwned;

const MAX_TEXTURE_BYTES:usize=16_000_000;
const MAX_COMPARE_DEPTH:usize=128;

struct Equality {
    path:[usize;MAX_COMPARE_DEPTH],
    depth:usize,
    offset:usize,
    key_equal:bool,
    returning:bool,
}

impl RetireOwned for Equality {
    type RetirementCursor=<() as RetireOwned>::RetirementCursor;
    fn retirement_seed(self)->Self::RetirementCursor {().retirement_seed()}
    fn controlled_retirement_supported()->bool {true}
}

impl Equality {
    fn new()->Self {Self {path:[0;MAX_COMPARE_DEPTH],depth:0,offset:0,key_equal:false,returning:false}}
    fn chunk(&mut self,a:&[u8],b:&[u8])->Option<bool>{
        if a.len()!=b.len(){return Some(false);}
        let end=(self.offset+256).min(a.len());
        if a[self.offset..end]!=b[self.offset..end] {return Some(false);}
        self.offset=end;
        (end==a.len()).then_some(true)
    }

    fn node<'a>(&self,mut value:&'a DslValue,depth:usize)->Option<(&'a DslValue,Option<&'a str>)> {
        let mut key=None;
        for index in &self.path[..depth] {
            match value {
                DslValue::Array(values)=>{value=values.get(*index)?;key=None;}
                DslValue::Object(values)=>{let (name,row)=values.get(*index)?;value=row;key=Some(name.as_str());}
                _=>return None,
            }
        }
        Some((value,key))
    }

    fn material(&mut self,left:&DslValue,right:&DslValue)->Result<Option<bool>,ValueError>{
        if self.returning {
            if self.depth==0 {return Ok(Some(true));}
            let parent=self.depth-1;
            let count=match self.node(left,parent).map(|(row,_)|row) {
                Some(DslValue::Array(values))=>values.len(),
                Some(DslValue::Object(values))=>values.len(),
                _=>return Err(ValueError::invariant("Combine asset comparison lost a parent")),
            };
            if self.path[parent]+1<count{
                self.path[parent]+=1;self.returning=false;self.key_equal=false;self.offset=0;
            }else{self.depth=parent;}
            return Ok(None);
        }
        let ((a,ak),(b,bk))=self.node(left,self.depth).zip(self.node(right,self.depth)).ok_or_else(||ValueError::invariant("Combine asset comparison lost a row"))?;
        if let (Some(a),Some(b))=(ak,bk) {
            if !self.key_equal {
                match self.chunk(a.as_bytes(),b.as_bytes()) {
                    Some(true)=>{self.key_equal=true;self.offset=0;}
                    Some(false)=>return Ok(Some(false)),
                    None=>{},
                }
                return Ok(None);
            }
        }
        let leaf=match (a,b) {
            (DslValue::Null,DslValue::Null)=>Some(true),
            (DslValue::Bool(a),DslValue::Bool(b))=>Some(a==b),
            (DslValue::Number(a),DslValue::Number(b))=>Some(a==b),
            (DslValue::String(a),DslValue::String(b))=>self.chunk(a.as_bytes(),b.as_bytes()),
            (DslValue::Bytes(a),DslValue::Bytes(b))=>self.chunk(a,b),
            (DslValue::Array(a),DslValue::Array(b)) if a.len()==b.len()&&!a.is_empty()=>return self.descend(),
            (DslValue::Object(a),DslValue::Object(b)) if a.len()==b.len()&&!a.is_empty()=>return self.descend(),
            (DslValue::Array(a),DslValue::Array(b))=>Some(a.is_empty()&&b.is_empty()),
            (DslValue::Object(a),DslValue::Object(b))=>Some(a.is_empty()&&b.is_empty()),
            _=>Some(false),
        };
        match leaf {
            Some(true)=>{self.returning=true;Ok(None)}
            verdict=>Ok(verdict),
        }
    }

    fn descend(&mut self)->Result<Option<bool>,ValueError> {
        let next=self.path.get_mut(self.depth).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"Combine duplicate asset comparison exceeds its depth limit"))?;
        *next=0;self.depth+=1;self.key_equal=false;self.offset=0;
        Ok(None)
    }
}

trait Asset:RetireOwned {
    const COUNT_LIMIT:usize;
    fn bytes(&self)->usize;
    fn compare(left:&Self,right:&Self,state:&mut Equality)->Result<Option<bool>,ValueError>;
}

impl Asset for MeshTexture {
    const COUNT_LIMIT:usize=256;
    fn bytes(&self)->usize {self.bytes.len()}
    fn compare(left:&Self,right:&Self,state:&mut Equality)->Result<Option<bool>,ValueError> {
        if !state.key_equal {
            match state.chunk(left.mime.as_bytes(),right.mime.as_bytes()){
                Some(true)=>{state.key_equal=true;state.offset=0;}
                Some(false)=>return Ok(Some(false)),
                None=>{},
            }
            return Ok(None);
        }
        Ok(state.chunk(&left.bytes,&right.bytes))
    }
}

impl Asset for DslValue {
    const COUNT_LIMIT:usize=10_000;
    fn bytes(&self)->usize {0}
    fn compare(left:&Self,right:&Self,state:&mut Equality)->Result<Option<bool>,ValueError> {state.material(left,right)}
}

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct AssetCursor<T:Asset> {
    pending:Option<(String,T)>,
    comparison:Equality,
    slot:usize,
    bytes:usize,
}

impl<T:Asset> AssetCursor<T> {
    fn new()->Self {Self {pending:None,comparison:Equality::new(),slot:0,bytes:0}}

    fn demand(&self,source:&HistoryFoldIndex<String,T>,out:&HistoryFoldIndex<String,T>,copy:usize)->Result<RetirementDemand,ValueError>{
        let mut demand=RetirementDemand{depth:1,..Default::default()};
        if let Some((key,_))=&self.pending {
            demand.capacity_bytes=out.next_insert_capacity_byte_demand(key,copy)?;
            demand.depth=out.next_insert_depth_demand(key)?;
        }else if self.slot<source.slot_count() {
            demand.depth=source.next_slot_depth_demand(self.slot)?.max(1);
            if let Some((key,_))=source.slot_entry(self.slot) {
                demand.depth=demand.depth.max(out.next_contains_depth_demand(key)?);
                if out.contains_key(key){demand.depth=demand.depth.max(self.comparison.depth+1);}
                else{demand.depth=demand.depth.max(source.next_extract_slot_depth_demand(self.slot)?);}
            }
        }
        Ok(demand)
    }

    fn step(&mut self,source:&mut HistoryFoldIndex<String,T>,out:&mut HistoryFoldIndex<String,T>,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<bool,ValueError>{
        let grant=effect.grant();
        if let Some((key,value))=&self.pending {
            if out.len()>=T::COUNT_LIMIT || self.bytes.saturating_add(value.bytes())>MAX_TEXTURE_BYTES {
                return Err(invalid("combined mesh exceeds asset limits"));
            }
            if out.next_insert_capacity_byte_demand(key,grant.maximum_copy_bytes)?!=0 {
                *receipt=match out.reserve_insert_step(key,grant) {
                    Ok(progress)=>progress,
                    Err((error,progress))=>{*receipt=progress;return Err(error.with_retained_progress(progress));}
                };
            }else{
                let (key,value)=self.pending.take().unwrap();
                let bytes=value.bytes();
                match out.insert_reserved(key,value,grant) {
                    Ok((old,progress))=>{assert!(old.is_none());*receipt=progress;self.bytes+=bytes;}
                    Err((error,key,value))=>{self.pending=Some((key,value));return Err(error);}
                }
            }
            return Ok(false);
        }
        if self.slot==source.slot_count(){return Ok(true);}
        if let Some((key,right))=source.slot_entry(self.slot){
            if let Some(left)=out.get(key) {
                match T::compare(left,right,&mut self.comparison)? {
                    Some(true)=>{self.comparison=Equality::new();self.slot+=1;}
                    Some(false)=>return Err(invalid("combined meshes have incompatible assets with the same key")),
                    None=>{},
                }
                return Ok(false);
            }
            self.pending=source.extract_slot_if(self.slot,|_,_|false);
        }
        self.slot+=1;
        Ok(false)
    }
}

/// 🎲️ Keeps both original asset maps and every staged payload available to the enclosing owner.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Assets {
    textures:AssetCursor<MeshTexture>,
    materials:AssetCursor<DslValue>,
    phase:u8,
}

impl Assets {
    pub(super) fn new()->Self {Self{textures:AssetCursor::new(),materials:AssetCursor::new(),phase:0}}

    pub(super) fn demand(&self,right:&PolygonMeshSource,out:&PolygonMeshSource,copy:usize)->Result<RetirementDemand,ValueError>{
        match self.phase {
            2 if self.textures.slot<out.textures.slot_count()=>Ok(RetirementDemand {depth:out.textures.next_slot_depth_demand(self.textures.slot)?.max(1),..Default::default()}),
            3=>self.textures.demand(&right.textures,&out.textures,copy),
            4=>self.materials.demand(&right.materials,&out.materials,copy),
            _=>Ok(RetirementDemand {depth:1,..Default::default()}),
        }
    }

    pub(super) fn step(&mut self,left:&mut PolygonMeshSource,right:&mut PolygonMeshSource,out:&mut PolygonMeshSource,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<bool,ValueError>{
        match self.phase {
            0=>{out.textures=std::mem::take(&mut left.textures);self.phase=1;}
            1=>{out.materials=std::mem::take(&mut left.materials);self.phase=2;}
            2=>{
                if self.textures.slot==out.textures.slot_count(){
                    if self.textures.bytes>MAX_TEXTURE_BYTES || out.textures.len()>MeshTexture::COUNT_LIMIT || out.materials.len()>DslValue::COUNT_LIMIT {return Err(invalid("combined mesh exceeds asset limits"));}
                    self.textures.slot=0;self.phase=3;
                }
                else {if let Some((_,value))=out.textures.slot_entry(self.textures.slot){self.textures.bytes+=value.bytes.len();}self.textures.slot+=1;}
            },
            3=>if self.textures.step(&mut right.textures,&mut out.textures,effect,receipt)?{self.phase=4;},
            4=>return self.materials.step(&mut right.materials,&mut out.materials,effect,receipt),
            _=>unreachable!(),
        }
        Ok(false)
    }
}
