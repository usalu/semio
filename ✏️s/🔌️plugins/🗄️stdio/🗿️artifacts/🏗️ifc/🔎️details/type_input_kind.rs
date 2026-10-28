//! 🧭️ Borrowed input kinds resolve one released alias node per original turn.
use super::*;
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub(super) enum Kind {
    #[default]
    Reference,
    String,
    Integer,
    Enum,
    Real,
    Number,
    Binary,
    List,
    Set,
    Array,
    Select,
}
impl Kind {
    pub(super) fn tag(self) -> &'static str {
        match self {
            Self::Reference => "reference",
            Self::String => "string",
            Self::Integer => "integer",
            Self::Enum => "enum",
            Self::Real => "real",
            Self::Number => "number",
            Self::Binary => "binary",
            Self::List | Self::Set | Self::Array => "aggregate",
            Self::Select => "typedValue",
        }
    }
}
#[derive(Clone, Copy, Default)]
pub(super) struct Cursor {
    node: Option<&'static dyn IfcDetailsTypeMetadata>,
    depth: usize,
    domain: Option<&'static str>,
    list: Option<(usize,usize)>,
    select: Option<(&'static str,usize)>,
    leaves: super::type_input_select::Cursor,
    select_next: usize,
    select_pending: bool,
    select_references: usize,
    reference_member: Option<usize>,
    select_scalar: bool,
    first_leaf:Option<&'static str>,
    selected_member: bool,
    set: bool,
    array:bool,
}
impl Cursor {
    pub(super) fn domain(&self) -> Option<&'static str> {
        self.domain
    }
    pub(super) fn reference_member(&self)->Option<usize>{self.reference_member.filter(|_|self.select_scalar&&self.select_references==1)}
    pub(super) fn list_bounds(&self)->Option<(usize,usize)>{self.list}
    pub(super) fn first_leaf(&self)->Option<&'static str>{self.first_leaf}
    pub(super) fn leaf_count(&self)->Option<usize>{self.select.map(|(_,count)|count).filter(|count|*count>0)}
    pub(super) fn member<C: IfcDetailsProfile>(domain: &str, index: usize) -> Option<&'static str> {
        match domain {
            "BOOLEAN" => ["T", "F"].get(index).copied(),
            "LOGICAL" => ["T", "F", "U"].get(index).copied(),
            _ => C::enumeration_member(domain, index),
        }
    }
    pub(super) fn count<C: IfcDetailsProfile>(domain: &str) -> Option<usize> {
        match domain {
            "BOOLEAN" => Some(2),
            "LOGICAL" => Some(3),
            _ => C::enumeration_count(domain),
        }
    }
    pub(super) fn clear(&mut self) {
        *self = Self::default();
    }
    fn resolved(&mut self,kind:Kind)->Result<Option<Kind>,semio_framework_value::ValueError>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"IFC required input kind unavailable");
        let kind=if self.set{if kind!=Kind::Reference{return Err(invalid());}self.set=false;Kind::Set}else{kind};
        let kind=if self.array{if kind!=Kind::Real{return Err(invalid());}self.array=false;Kind::Array}else{kind};
        let kind=if self.list.is_some(){if kind!=Kind::Real&&!(kind==Kind::Integer&&(self.selected_member||self.select.is_some())){return Err(invalid());}Kind::List}else{kind};
        if self.select.is_some(){if !matches!(kind,Kind::Reference|Kind::String|Kind::Integer|Kind::Enum|Kind::Real|Kind::Number|Kind::Binary|Kind::List|Kind::Set|Kind::Array){return Err(invalid());}if kind==Kind::Reference{self.reference_member=Some(self.select_next);}self.select_references+=usize::from(kind==Kind::Reference);self.select_scalar|=kind!=Kind::Reference;self.select_next+=1;self.node=None;self.depth=0;self.list=None;self.select_pending=true;return Ok(None);}
        Ok(Some(kind))
    }
    pub(super) fn advance<C: IfcDetailsProfile>(&mut self, expression: &'static dyn IfcDetailsTypeMetadata) -> Result<Option<Kind>, semio_framework_value::ValueError> {
        self.advance_mode::<C>(expression,false)
    }
    pub(super) fn advance_selected<C:IfcDetailsProfile>(&mut self,expression:&'static dyn IfcDetailsTypeMetadata)->Result<Option<Kind>,semio_framework_value::ValueError>{self.advance_mode::<C>(expression,true)}
    fn advance_mode<C:IfcDetailsProfile>(&mut self,expression:&'static dyn IfcDetailsTypeMetadata,selected:bool)->Result<Option<Kind>,semio_framework_value::ValueError>{
        use semio_framework_value::{ValueError, ValueRefusalKind};
        let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "IFC required input kind unavailable");
        if self.selected_member!=selected&&(self.depth!=0||self.node.is_some()||self.select.is_some()){return Err(invalid());}self.selected_member=selected;
        if self.depth >= 64 {
            return Err(invalid());
        }
        if self.select_pending{let(domain,_)=self.select.ok_or_else(invalid)?;match self.leaves.advance::<C>(domain)?{super::type_input_select::Step::Pending=>return Ok(None),super::type_input_select::Step::Complete{count}=>{if count==0||self.select_scalar&&self.select_references>1{return Err(invalid());}self.select=Some((domain,count));self.select_pending=false;return Ok(Some(if self.select_scalar{Kind::Select}else{Kind::Reference}));},super::type_input_select::Step::Leaf{index,name}=>{if index!=self.select_next{return Err(invalid());}if index==0{self.first_leaf=Some(name);}if C::direct_attribute_count(name).is_some(){return self.resolved(Kind::Reference);}self.node=Some(C::defined_type_expression(name).ok_or_else(invalid)?);self.select_pending=false;return Ok(None);}}}
        let node = self.node.unwrap_or(expression).projection();
        if node.kind=="aggregate"{
            if node.aggregate==Some("ARRAY"){
                let lower=node.lower.ok_or_else(invalid)?;let upper=node.upper.ok_or_else(invalid)?;let cardinality=upper.checked_sub(lower).and_then(|value|value.checked_add(1)).and_then(|value|usize::try_from(value).ok()).filter(|value|*value>0).ok_or_else(invalid)?;
                if self.array||self.set||self.list.is_some()||self.select.is_none()&&!self.selected_member||node.optional{return Err(invalid());}let _=cardinality;self.array=true;self.node=Some(node.item.ok_or_else(invalid)?);self.depth+=1;return Ok(None);
            }
            let lower=node.lower.and_then(|value|usize::try_from(value).ok()).ok_or_else(invalid)?;
            if node.aggregate==Some("SET"){
                if self.set||self.array||self.list.is_some()||self.select.is_none()&&!self.selected_member||!node.unique||node.upper.is_some_and(|upper|usize::try_from(upper).map_or(true,|upper|upper<lower)){return Err(invalid());}
                self.set=true;self.node=Some(node.item.ok_or_else(invalid)?);self.depth+=1;return Ok(None);
            }
            let upper=node.upper.and_then(|value|usize::try_from(value).ok()).ok_or_else(invalid)?;
            if self.list.is_some()||node.aggregate!=Some("LIST")||upper<lower||upper>=semio_framework_plugin::action_staging::STAGED_FORM_BODY_LIMITS.content{return Err(invalid());}
            let item=node.item.ok_or_else(invalid)?;self.list=Some((lower,upper));self.node=Some(item);self.depth+=1;return Ok(None);
        }
        if node.kind != "named" {
            return Err(invalid());
        }
        let name = node.name.ok_or_else(invalid)?;
        if let Some(count)=C::select_count(name){
            if self.list.is_some()||self.select.is_some()||count==0{return Err(invalid());}
            self.select=Some((name,0));self.domain=Some(name);self.select_next=0;self.depth=0;self.node=None;self.select_pending=true;
            return Ok(None);
        }
        let kind = match name {
            "STRING" => Some(Kind::String),
            "INTEGER" => Some(Kind::Integer),
            "REAL" => Some(Kind::Real),
            "NUMBER" => Some(Kind::Number),
            "BINARY" => Some(Kind::Binary),
            "BOOLEAN" | "LOGICAL" => Some(Kind::Enum),
            _ if C::enumeration_count(name).is_some() => Some(Kind::Enum),
            _ if C::direct_attribute_count(name).is_some() => Some(Kind::Reference),
            _ => None,
        };
        if let Some(kind) = kind {
            if kind == Kind::Enum && self.select.is_none() && self.list.is_none() {
                self.domain = Some(name);
            }
            return self.resolved(kind);
        }
        let node=C::defined_type_expression(name).ok_or_else(invalid)?;self.node=Some(node);self.depth+=1;
        Ok(None)
    }
}
