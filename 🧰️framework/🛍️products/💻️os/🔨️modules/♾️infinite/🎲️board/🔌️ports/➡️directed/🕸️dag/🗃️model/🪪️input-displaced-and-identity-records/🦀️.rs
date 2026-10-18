//! 🪪️ Input displaced and identity records of the DAG model: the displaced input, the identity cursor and its step. See [`super`].

use super::*;

pub struct DagInputDisplaced{pub selection:Option<(Selection,Selection,Selection)>,pub chrome:Option<(Option<NodeId>,NumericSet<NodeId>,NumericIndex<NodeId,DagNodeEvalStatusKind>,NumericSet<(NodeId,usize)>)>,pub progress:Option<(Option<NodeId>,NumericSet<NodeId>)>}

impl DagInputDisplaced{pub fn empty()->Self{Self{selection:None,chrome:None,progress:None}}}

pub struct Identity<'a>{pub first:&'a str,pub second:Option<&'a str>}

impl Identity<'_>{pub fn len(&self)->usize{self.first.len()+self.second.map_or(0,|text|text.len()+1)}pub fn byte(&self,index:usize)->Option<u8>{if index<self.first.len(){self.first.as_bytes().get(index).copied()}else if index==self.first.len()&&self.second.is_some(){Some(b'@')}else{self.second?.as_bytes().get(index-self.first.len()-1).copied()}}}

pub enum IdentityStep{Progress,Resolved(u64),Missing}

semio_framework_value::artifact_retire_struct!(DagInputDisplaced{selection,chrome,progress});
