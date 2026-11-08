//! 🌳️ Defining persistent AVL update engine expands beside its original private metadata types.
#[macro_export]
macro_rules! ordered_update_engine{()=>{
impl<V> OrderedMap<V>{
 pub fn owned_lookup_cursor_demand(&self)->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:size_of::<Option<LookupCursor<V>>>()+size_of::<Option<SharedOwner<String>>>()+2*usize::from(!self.is_empty())*SharedOwner::<Node<V>>::duplicate_copy_bytes(),depth:1,..Default::default()}}
 pub fn owned_lookup_cursor_work(&self)->u64{1+2*SharedOwner::<Node<V>>::DUPLICATE_WORK*u64::from(!self.is_empty())}
 /// 🔎️ Publishes the actual owned-key lookup cursor under the same native receipt.
 pub fn admit_owned_lookup_cursor(&self,key:&mut Option<SharedOwner<String>>,output:&mut Option<LookupCursor<V>>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{
  let before=effect.performed_boundary();let result=(||{if key.is_none()||output.is_some(){return Err(ValueError::invariant("ordered lookup requires original key and empty cursor output"))}let demand=self.owned_lookup_cursor_demand();let work=self.owned_lookup_cursor_work();if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<work{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(work)?{return Err(ValueError::invariant("ordered lookup constructor exceeded original Work"))}*output=Some(LookupCursor::new(self.clone(),key.take().expect("funded original lookup key")));Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same owned lookup constructor receipt")}
 }

 pub fn update_constructor_demand(&self)->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:size_of::<Option<UpdateCursor<V>>>()+size_of::<Option<SharedOwner<String>>>()+size_of::<Option<SharedOwner<V>>>()+2*usize::from(!self.is_empty())*SharedOwner::<Node<V>>::duplicate_copy_bytes(),depth:1,..Default::default()}}
 pub fn update_constructor_work(&self)->u64{1+2*SharedOwner::<Node<V>>::DUPLICATE_WORK*u64::from(!self.is_empty())}
 /// 🌱️ Publishes the actual private cursor directly after its native aliases and complete header are funded.
 pub fn admit_update_cursor(&self,key:&mut Option<SharedOwner<String>>,value:&mut Option<SharedOwner<V>>,output:&mut Option<UpdateCursor<V>>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{
  let before=effect.performed_boundary();let result=(||{if key.is_none()||output.is_some(){return Err(ValueError::invariant("ordered constructor requires original key and empty cursor output"))}let demand=self.update_constructor_demand();let work=self.update_constructor_work();if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<work{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(work)?{return Err(ValueError::invariant("ordered constructor exceeded original Work"))}*output=Some(UpdateCursor::new(self.clone(),key.take().expect("funded original update key"),value.take()));Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same ordered constructor receipt")}
 }
}

#[derive(Clone,Copy)]
enum Phase { Search, Successor, RebuildSuccessor, Rebuild, Complete }
struct Parent<V> { node: SharedOwner<Node<V>>, left: bool }

/// 🧵️ Retains both immutable roots until explicit result handoff and close; no payload Clone bound.
struct UpdateState<V> {
entry_input:Option<Entry<V>>,node_input:Option<Node<V>>,node_output:Root<V>,
    base: OrderedMap<V>, key: Option<SharedOwner<String>>, value: Option<SharedOwner<V>>, current: Root<V>, path: InlineStack<Parent<V>,MAX_AVL_HEIGHT>,
    successor_path: InlineStack<SharedOwner<Node<V>>,MAX_AVL_HEIGHT>, retired:InlineStack<SharedOwner<Node<V>>,UPDATE_RETIRED_ROOTS>, removed_node: Root<V>, successor_entry: Option<SharedOwner<Entry<V>>>,
    replacement: Root<V>, removed: Option<SharedOwner<V>>, result: Option<OrderedMap<V>>, phase: Phase, insert:bool,
    offset: usize, left_byte: Option<u8>, ordering: Option<Ordering>, retirement: Retirement<V>, closing: bool, last_close_work:u64, last_compared_bytes:usize,last_allocation_attempt:Option<usize>,pending_entry:Option<EntryBuilder<V>>,entry_arc:Option<SharedOwner<Entry<V>>>,pending_node:Option<NodeBuilder<V>>,plan:Option<BalancePlan<V>>,
}

impl<V> UpdateState<V> {
    fn new(base: OrderedMap<V>, key: SharedOwner<String>, value: Option<SharedOwner<V>>) -> Self {
        let insert=value.is_some();
        Self { entry_input:None,node_input:None,node_output:None,insert,current: (*base.root).clone(), base, key: Some(key), value, path: InlineStack::default(), successor_path: InlineStack::default(), retired:InlineStack::default(), removed_node: None,
            successor_entry: None, replacement: None, removed: None, result: None, phase: Phase::Search, offset: 0, left_byte: None, ordering: None, retirement: Retirement::default(), closing: false, last_close_work:0,last_compared_bytes:0,last_allocation_attempt:None,pending_entry:None,entry_arc:None,pending_node:None,plan:None }
    }

    fn compare(&mut self, maximum_bytes: usize) -> usize {
        compare_bytes(self.key.as_ref().unwrap().as_bytes(), self.current.as_ref().unwrap().entry.key.as_bytes(), &mut self.offset, &mut self.left_byte, &mut self.ordering, maximum_bytes)
    }


    fn native_compare_remaining(&self)->Option<usize>{
        if self.pending_entry.is_some()||self.pending_node.is_some()||self.plan.is_some()||!matches!(self.phase,Phase::Search)||self.ordering.is_some(){return None}
        self.current.as_ref().zip(self.key.as_ref()).map(|(node,key)|key.len().min(node.entry.key.len()).saturating_sub(self.offset).saturating_mul(2).saturating_sub(usize::from(self.left_byte.is_some())))
    }
    fn native_reads(&self,body:usize)->usize{self.native_compare_remaining().unwrap_or(0).min(body).min(64)}
    fn native_aliases(&self)->usize{
        if self.pending_entry.is_some()||self.pending_node.is_some()||self.plan.is_some(){return 0}
        match self.phase{
            Phase::Search=>{let Some(current)=self.current.as_ref()else{return 0};match self.ordering{
                None=>0,Some(Ordering::Less)=>usize::from(current.left.is_some()),Some(Ordering::Greater)=>usize::from(current.right.is_some()),
                Some(Ordering::Equal)=>if self.insert&&self.entry_arc.is_none(){0}else{1+if self.insert{usize::from(current.left.is_some())+usize::from(current.right.is_some())}else{usize::from(current.left.is_some()||current.right.is_some())}}
            }},
            Phase::Successor=>self.current.as_ref().map_or(0,|current|if current.left.is_some(){1}else{1+usize::from(current.right.is_some())}),
            Phase::Rebuild=>self.path.front().map_or(0,|parent|{let(left,right,sibling)=if parent.left{(&self.replacement,&parent.node.right,usize::from(parent.node.right.is_some()))}else{(&parent.node.left,&self.replacement,usize::from(parent.node.left.is_some()))};1+sibling+BalancePlan::new_aliases(left,right)}),
            Phase::RebuildSuccessor=>self.successor_path.front().map_or_else(||self.removed_node.as_ref().map_or(0,|removed|usize::from(removed.left.is_some())+BalancePlan::new_aliases(&removed.left,&self.replacement)),|parent|1+usize::from(parent.right.is_some())+BalancePlan::new_aliases(&self.replacement,&parent.right)),
            Phase::Complete=>0,
        }
    }
    fn native_copy_demand(&self,body:usize)->Result<usize,ValueError>{
        let headers=size_of::<Self>()-size_of::<InlineStack<Parent<V>,MAX_AVL_HEIGHT>>()-size_of::<InlineStack<SharedOwner<Node<V>>,MAX_AVL_HEIGHT>>()-size_of::<InlineStack<SharedOwner<Node<V>>,UPDATE_RETIRED_ROOTS>>()-size_of::<Retirement<V>>();
        headers.checked_add(4*size_of::<Option<Parent<V>>>()).and_then(|n|n.checked_add(4*size_of::<Option<SharedOwner<Node<V>>>>())).and_then(|n|n.checked_add(self.native_aliases()*SharedOwner::<Node<V>>::duplicate_copy_bytes())).and_then(|n|n.checked_add(body.min(64))).ok_or_else(||ValueError::invariant("ordered native metadata extent overflow"))
    }
    fn admission_growth(&self)->usize{
        if self.pending_entry.is_some()||self.pending_node.is_some(){return 1}if self.plan.is_some(){return 0}
        match self.phase{
            Phase::Search=>if self.current.is_none(){if self.value.is_some()&&self.entry_arc.is_none(){2}else{0}}else{let Some(current)=self.current.as_ref()else{return usize::MAX};match self.ordering{None=>0,Some(Ordering::Less)=>usize::from(current.left.is_some()),Some(Ordering::Greater)=>usize::from(current.right.is_some()),Some(Ordering::Equal)=>if self.value.is_some(){if self.entry_arc.is_none(){2}else{1+usize::from(current.left.is_some())+usize::from(current.right.is_some())}}else{1+usize::from(current.left.is_some()||current.right.is_some())}}},
            Phase::Successor=>{let Some(current)=self.current.as_ref()else{return usize::MAX};if current.left.is_some(){1}else{1+usize::from(current.right.is_some())}},
            Phase::Rebuild=>self.path.front().map_or(usize::from(self.replacement.is_none()),|parent|{let(left,right,sibling)=if parent.left{(&self.replacement,&parent.node.right,usize::from(parent.node.right.is_some()))}else{(&parent.node.left,&self.replacement,usize::from(parent.node.left.is_some()))};1+sibling+BalancePlan::new_aliases(left,right)}),
            Phase::RebuildSuccessor=>self.successor_path.front().map_or_else(||{let Some(removed)=self.removed_node.as_ref()else{return usize::MAX};let left=&removed.left;usize::from(left.is_some())+BalancePlan::new_aliases(left,&self.replacement)},|parent|1+usize::from(parent.right.is_some())+BalancePlan::new_aliases(&self.replacement,&parent.right)),Phase::Complete=>0,
        }
    }
    fn advance(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{
        self.last_compared_bytes=0;self.last_allocation_attempt=None;
        if let Some(staged)=self.pending_entry.as_mut(){let value=Entry{key:staged.key.take().expect("cold staged key"),value:staged.value.take().expect("cold staged value")};self.entry_arc=Some(SharedOwner::from_cold(value));self.pending_entry.take();return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:SharedOwner::<Entry<V>>::allocation_bytes(),..Default::default()})),work:0}}
        if let Some(staged)=self.pending_node.as_mut(){let value=staged.take_node().expect("cold staged node");self.plan.as_mut().expect("cold staged plan").accept(SharedOwner::from_cold(value));self.pending_node.take();return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:SharedOwner::<Node<V>>::allocation_bytes(),..Default::default()})),work:0}}
        if let Some(plan)=self.plan.as_mut(){if plan.ready(){let root=plan.take_result();let after=plan.after;self.plan.take();self.replacement=root;self.phase=after;}else{self.pending_node=Some(match plan.prepare(){Ok(node)=>node,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}});if let Err(error)=match self.pending_node.as_mut(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("pending_node absent").into()),work:0}}.validate(){return crate::RetainedWorkOutcome{result:Err(error.with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:self.last_compared_bytes,..Default::default()})),work:1}}}return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:self.last_compared_bytes,..Default::default()})),work:1}}
        match self.phase{
            Phase::Search=>{
                if self.current.is_none(){if self.insert{if let Some(entry)=self.entry_arc.take(){self.plan=Some(BalancePlan::single(entry,None,None,Phase::Rebuild));}else{if self.key.is_none()||self.value.is_none(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original insert entry owners absent")),work:0}}self.pending_entry=Some(EntryBuilder{key:self.key.take(),value:self.value.take()});}}else{self.phase=Phase::Rebuild;}}
                else if self.ordering.is_none(){self.last_compared_bytes=self.compare(grant.maximum_copy_bytes.min(64));}
                else if self.ordering==Some(Ordering::Equal)&&self.insert&&self.entry_arc.is_none(){if self.key.is_none()||self.value.is_none(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original replacement entry owners absent")),work:0}}self.pending_entry=Some(EntryBuilder{key:self.key.take(),value:self.value.take()});}
                else{let current=match self.current.take(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("current absent").into()),work:0}};match match self.ordering.take(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("ordering absent").into()),work:0}}{direction @ (Ordering::Less|Ordering::Greater)=>{let left=direction==Ordering::Less;self.current=if left{current.left.clone()}else{current.right.clone()};self.path.push_front(Parent{node:current,left});},Ordering::Equal=>{self.removed=Some(current.entry.value.clone());if self.insert{self.plan=Some(BalancePlan::single(match self.entry_arc.take(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("entry_arc absent").into()),work:0}},current.left.clone(),current.right.clone(),Phase::Rebuild));self.retired.push_front(current);}else if current.left.is_none()||current.right.is_none(){self.replacement=current.left.clone().or_else(||current.right.clone());self.retired.push_front(current);self.phase=Phase::Rebuild;}else{self.current=current.right.clone();self.removed_node=Some(current);self.phase=Phase::Successor;}}}self.offset=0;self.left_byte=None;}
            },
            Phase::Successor=>{let current=match self.current.take(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("current absent").into()),work:0}};if current.left.is_some(){self.current=current.left.clone();self.successor_path.push_front(current);}else{self.successor_entry=Some(SharedOwner::clone(&current.entry));self.replacement=current.right.clone();self.retired.push_front(current);self.phase=Phase::RebuildSuccessor;}},
            Phase::RebuildSuccessor=>{if let Some(parent)=self.successor_path.pop_front(){self.plan=Some(BalancePlan::balanced(SharedOwner::clone(&parent.entry),self.replacement.take(),parent.right.clone(),Phase::RebuildSuccessor,&mut self.retired));self.retired.push_front(parent);}else{let removed=match self.removed_node.take(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("removed_node absent").into()),work:0}};self.plan=Some(BalancePlan::balanced(match self.successor_entry.take(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("successor_entry absent").into()),work:0}},removed.left.clone(),self.replacement.take(),Phase::Rebuild,&mut self.retired));self.retired.push_front(removed);}},
            Phase::Rebuild=>{if let Some(parent)=self.path.pop_front(){let left=if parent.left{self.replacement.take()}else{parent.node.left.clone()};let right=if parent.left{parent.node.right.clone()}else{self.replacement.take()};self.plan=Some(BalancePlan::balanced(SharedOwner::clone(&parent.node.entry),left,right,Phase::Rebuild,&mut self.retired));self.retired.push_front(parent.node);}else{self.result=Some(OrderedMap{root:ManuallyDrop::new(self.replacement.take())});self.phase=Phase::Complete;}},
            Phase::Complete=>return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0},
        }
        crate::RetainedWorkOutcome{result:Ok(if self.is_complete(){RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:self.last_compared_bytes,..Default::default()})}else{RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:self.last_compared_bytes,..Default::default()})}),work:1+self.last_compared_bytes as u128}
    }

    pub fn is_complete(&self) -> bool { matches!(self.phase, Phase::Complete) }
    pub fn take_result(&mut self) -> Option<OrderedMap<V>> { self.result.take() }
    pub fn take_removed(&mut self) -> Option<SharedOwner<V>> { self.removed.take() }
    pub fn begin_close(&mut self) { self.closing = true; }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> {
        self.last_close_work=0;
        if self.terminal_is_empty(){return RetirementStep::Complete;}
        if !self.closing || grant.maximum_items == 0 { return RetirementStep::Blocked; }
        if !self.retirement.is_empty() { let step=self.retirement.advance(grant);self.last_close_work=self.retirement.last_turn_work();return step; }
        if grant.maximum_depth==0 {return RetirementStep::Failure(ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered close requires admitted metadata depth"));}
        self.last_close_work=1;
if self.entry_input.is_some(){*self.retirement.entry_value=self.entry_input.take();return RetirementStep::Progress{released_items:1,released_bytes:0}}
if self.node_input.is_some(){*self.retirement.node_value=self.node_input.take();return RetirementStep::Progress{released_items:1,released_bytes:0}}
if let Some(node)=self.node_output.take(){self.retirement.push(Owner::Node(node));return RetirementStep::Progress{released_items:1,released_bytes:0}}
        if let Some(node)=self.pending_node.as_mut(){if let Some(owner)=node.take_owner(){self.retirement.push(owner);}else{self.pending_node.take();}return RetirementStep::Progress{released_items:1,released_bytes:0};}
        if let Some(entry)=self.pending_entry.as_mut(){if let Some(owner)=entry.take_owner(){self.retirement.push(owner);}else{self.pending_entry.take();}return RetirementStep::Progress{released_items:1,released_bytes:0};}
        if let Some(entry)=self.entry_arc.take(){self.retirement.push(Owner::Entry(entry));return RetirementStep::Progress{released_items:1,released_bytes:0};}
        if let Some(plan)=self.plan.as_mut(){if let Some(owner)=plan.take_owner(){self.retirement.push(owner);}else{self.plan.take();}return RetirementStep::Progress{released_items:1,released_bytes:0};}
        if let Some(parent) = self.path.pop_front() { self.retirement.push(Owner::Node(parent.node)); }
        else if let Some(node) = self.successor_path.pop_front() { self.retirement.push(Owner::Node(node)); }
        else if let Some(node) = self.retired.pop_front() { self.retirement.push(Owner::Node(node)); }
        else if let Some(node) = self.current.take().or_else(|| self.removed_node.take()).or_else(|| self.replacement.take()).or_else(|| self.base.root.take()) { self.retirement.push(Owner::Node(node)); }
        else if let Some(mut map) = self.result.take() { if let Some(node) = map.root.take() { self.retirement.push(Owner::Node(node)); } }
        else if let Some(entry) = self.successor_entry.take() { self.retirement.push(Owner::Entry(entry)); }
        else if let Some(key) = self.key.take() { self.retirement.push(Owner::Key(key)); }
        else if let Some(value) = self.value.take().or_else(|| self.removed.take()) { self.retirement.push(Owner::Value(value)); }
        else { return RetirementStep::Complete; }
        RetirementStep::Progress { released_items: 1, released_bytes: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.entry_input.is_none()&&self.node_input.is_none()&&self.node_output.is_none()&&self.pending_entry.is_none()&&self.entry_arc.is_none()&&self.pending_node.is_none()&&self.plan.is_none()&&self.base.is_empty() && self.key.is_none() && self.value.is_none() && self.current.is_none() && self.path.is_empty()
            && self.successor_path.is_empty() && self.retired.is_empty() && self.removed_node.is_none() && self.successor_entry.is_none() && self.replacement.is_none()
            && self.removed.is_none() && self.result.is_none() && self.retirement.is_empty()
    }
    fn retained_depth(&self)->usize {
        self.pending_entry.as_ref().map_or(0,EntryBuilder::held)+usize::from(self.entry_arc.is_some())+self.pending_node.as_ref().map_or(0,NodeBuilder::held)+self.plan.as_ref().map_or(0,BalancePlan::held)+self.path.length+self.successor_path.length+self.retired.length
            +usize::from(!self.base.is_empty())+usize::from(self.key.is_some())+usize::from(self.value.is_some())
            +usize::from(self.current.is_some())+usize::from(self.removed_node.is_some())+usize::from(self.successor_entry.is_some())
            +usize::from(self.replacement.is_some())+usize::from(self.removed.is_some())+usize::from(self.result.is_some())
    }
}

/// 🔒️ Terminal guard prevents unbudgeted destruction of any live cursor alias, including unwinding.
#[must_use = "update cursors must finish explicit close before drop"]
pub struct UpdateCursor<V> { state: ManuallyDrop<UpdateState<V>> }
impl<V> UpdateCursor<V> {
    pub fn result_transfer_demand(&self)->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:2*size_of::<Option<OrderedMap<V>>>(),depth:1,..Default::default()}}
    /// 📤️ Publishes the actual completed root into its caller's original output slot.
    pub fn take_result_into(&mut self,output:&mut Option<OrderedMap<V>>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{
        let before=effect.performed_boundary();let result=(||{if output.is_some()||!self.state.is_complete(){return Err(ValueError::invariant("ordered result requires its completed cursor and empty output"))}if self.state.result.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.result_transfer_demand();if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::invariant("ordered output transfer exceeded original Work"))}*output=self.state.result.take();Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same ordered root transfer receipt")}
    }

    fn native_close_header_bytes(&self)->usize{
        let state=&self.state;let target=size_of::<Option<Owner<V>>>()+size_of::<usize>();
if state.entry_input.is_some(){return 2*size_of::<Option<Entry<V>>>()}if state.node_input.is_some(){return 2*size_of::<Option<Node<V>>>()}if state.node_output.is_some(){return size_of::<Root<V>>()+target}
        if let Some(node)=state.pending_node.as_ref(){return if node.held()==0{size_of::<Option<NodeBuilder<V>>>()}else{size_of::<Root<V>>()+target}}
        if let Some(entry)=state.pending_entry.as_ref(){return if entry.held()==0{size_of::<Option<EntryBuilder<V>>>()}else{size_of::<Option<SharedOwner<V>>>()+target}}
        if state.entry_arc.is_some(){return size_of::<Option<SharedOwner<Entry<V>>>>()+target}
        if let Some(plan)=state.plan.as_ref(){return if plan.held()==0{size_of::<Option<BalancePlan<V>>>()}else{size_of::<Root<V>>()+target}}
        if !state.path.is_empty(){return size_of::<Option<Parent<V>>>()+size_of::<usize>()+target}
        if !state.successor_path.is_empty()||!state.retired.is_empty(){return size_of::<Root<V>>()+size_of::<usize>()+target}
        if state.result.is_some(){return size_of::<Option<OrderedMap<V>>>()+size_of::<Root<V>>()+target}
        size_of::<Root<V>>()+target
    }

    fn new(base: OrderedMap<V>, key: SharedOwner<String>, value: Option<SharedOwner<V>>) -> Self { Self { state: ManuallyDrop::new(UpdateState::new(base, key, value)) } }
    pub fn next_copy_byte_demand(&self)->usize{let state=&self.state;if state.closing||state.is_complete()||state.pending_entry.is_some()||state.pending_node.is_some()||state.plan.is_some(){return 0}if matches!(state.phase,Phase::Search)&&state.ordering.is_none(){if let(Some(node),Some(key))=(state.current.as_ref(),state.key.as_ref()){return usize::from(state.offset<key.len().min(node.entry.key.len()))}}0}
    pub fn next_capacity_byte_demand(&self)->Result<usize,ValueError>{if self.state.closing||self.state.is_complete(){return Ok(0)}Ok(if self.state.pending_entry.is_some(){SharedOwner::<Entry<V>>::allocation_bytes()}else if self.state.pending_node.is_some(){SharedOwner::<Node<V>>::allocation_bytes()}else{0})}
    pub fn next_depth_demand(&self)->usize{if self.state.closing||self.state.is_complete(){0}else{self.state.retained_depth().saturating_add(self.state.admission_growth()).max(1)}}
    pub fn last_compared_bytes(&self)->usize{self.state.last_compared_bytes}
    pub fn last_allocation_attempt(&self)->Option<usize>{self.state.last_allocation_attempt}
    /// ⛽️ Quotes the exact maximum metadata and byte reads for the selected update turn.
    pub fn next_work_demand(&self,copy:usize)->Result<u64,ValueError>{
        let state=&self.state;
        if state.closing||state.is_complete()||copy<self.next_copy_byte_demand(){return Ok(0)}
        if state.pending_entry.is_none()&&state.pending_node.is_none()&&state.plan.is_none()&&matches!(state.phase,Phase::Search)&&state.ordering.is_none(){
            if let(Some(node),Some(key))=(state.current.as_ref(),state.key.as_ref()){
                let remaining=key.len().min(node.entry.key.len()).saturating_sub(state.offset).saturating_mul(2).saturating_sub(usize::from(state.left_byte.is_some()));
                return Ok(1+copy.min(64).min(remaining) as u64)
            }
        }
        Ok(1)
    }
    /// ⏩️ Every selected metadata probe or actual shared allocation is independently funded before effects.
    pub fn advance(&mut self,grant:RetainedCloneGrant,maximum_work_units:u64)->crate::RetainedWorkOutcome{
        let work=match self.next_work_demand(grant.maximum_copy_bytes){Ok(work)=>work,Err(error)=>{self.state.last_compared_bytes=0;self.state.last_allocation_attempt=None;return crate::RetainedWorkOutcome{result:Err(error),work:0}}};if work>maximum_work_units{self.state.last_compared_bytes=0;self.state.last_allocation_attempt=None;return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.state.last_compared_bytes=0;self.state.last_allocation_attempt=None;if self.state.closing{return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if self.state.is_complete(){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let capacity=match self.next_capacity_byte_demand(){Ok(capacity)=>capacity,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if grant.maximum_items==0||grant.maximum_capacity_bytes<capacity||grant.maximum_copy_bytes<self.next_copy_byte_demand(){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if grant.maximum_depth<self.next_depth_demand(){return crate::RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered update exceeds admitted metadata depth")),work:0}}self.state.advance(grant)
    }

    pub fn native_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
        let state=&self.state;
        if state.closing||state.is_complete()||(body==0&&state.native_compare_remaining().is_some_and(|n|n>0)){return Ok(Default::default())}
        if state.entry_input.is_some(){return SharedOwner::<Entry<V>>::source_admission_demand()}
        if state.node_input.is_some(){return SharedOwner::<Node<V>>::source_admission_demand()}
        let copy_bytes=if state.node_output.is_some(){size_of::<Root<V>>()*2+size_of::<usize>()+size_of::<Option<NodeBuilder<V>>>()}
        else if let Some(entry)=state.pending_entry.as_ref(){if entry.held()==0{size_of::<Option<EntryBuilder<V>>>()}else{2*size_of::<Option<SharedOwner<V>>>()+size_of::<Option<Entry<V>>>()}}
        else if let Some(node)=state.pending_node.as_ref(){if node.held()==0{size_of::<Option<NodeBuilder<V>>>()}else{size_of::<Option<Node<V>>>()+size_of::<Option<SharedOwner<Entry<V>>>>()+2*size_of::<Root<V>>()}}
        else{state.native_copy_demand(state.native_reads(body))?};
        Ok(crate::RetirementDemand{copy_bytes,depth:1,..Default::default()})
    }
    pub fn native_work_demand(&self,body:usize)->Result<u64,ValueError>{
        let state=&self.state;
        if state.closing||state.is_complete()||(body==0&&state.native_compare_remaining().is_some_and(|n|n>0)){return Ok(0)}
        if state.entry_input.is_some(){return Ok(SharedOwner::<Entry<V>>::source_admission_work_demand())}
        if state.node_input.is_some(){return Ok(SharedOwner::<Node<V>>::source_admission_work_demand())}
        if state.node_output.is_some()||state.pending_entry.is_some()||state.pending_node.is_some(){return Ok(1)}
        let reads=if state.plan.is_none()&&matches!(state.phase,Phase::Search)&&state.ordering.is_none(){state.current.as_ref().zip(state.key.as_ref()).map_or(0,|(node,key)|key.len().min(node.entry.key.len()).saturating_sub(state.offset).saturating_mul(2).saturating_sub(usize::from(state.left_byte.is_some())).min(body).min(64))}else{0};
        Ok(1+3*state.native_aliases()as u64+reads as u64)
    }
    /// 🎟️ Advances only the defining closed native AVL primitive on its caller's actual effect.
    pub fn advance_native(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{
        let before=effect.performed_boundary();let result=(||{
            self.state.last_compared_bytes=0;self.state.last_allocation_attempt=None;
            if self.state.closing{return Err(ValueError::invariant("ordered updater is closing"))}
            if self.state.is_complete(){return Ok(RetainedCloneStep::Complete(Default::default()))}
            if body==0&&self.state.native_compare_remaining().is_some_and(|n|n>0){return Ok(RetainedCloneStep::Progress(Default::default()))}let demand=self.native_demands(body)?;let work=self.native_work_demand(body)?;
            if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<work{return Ok(RetainedCloneStep::Progress(Default::default()))}
            let state=&mut*self.state;
            if state.entry_input.is_some(){let turn=SharedOwner::admit_from(&mut state.entry_input,&mut state.entry_arc,effect);state.last_allocation_attempt=*effect.actual();return turn.result.map(|p|RetainedCloneStep::Progress(p.progress()))}
            if state.node_input.is_some(){let turn=SharedOwner::admit_from(&mut state.node_input,&mut state.node_output,effect);state.last_allocation_attempt=*effect.actual();return turn.result.map(|p|RetainedCloneStep::Progress(p.progress()))}
            if state.node_output.is_some(){if !effect.performed(1)?{return Err(ValueError::invariant("ordered node placement exceeded original Work"))}state.plan.as_mut().ok_or_else(||ValueError::invariant("ordered native node has no original plan"))?.accept(state.node_output.take().expect("funded original node"));state.pending_node.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
            if let Some(entry)=state.pending_entry.as_mut(){if !effect.performed(1)?{return Err(ValueError::invariant("ordered entry preparation exceeded original Work"))}if entry.held()==0{state.pending_entry.take();}else{state.entry_input=Some(Entry{key:entry.key.take().expect("original prepared key"),value:entry.value.take().expect("original prepared value")});}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
            if let Some(node)=state.pending_node.as_mut(){if !effect.performed(1)?{return Err(ValueError::invariant("ordered node preparation exceeded original Work"))}if node.held()==0{state.pending_node.take();}else{state.node_input=Some(node.take_node().ok_or_else(||ValueError::invariant("original node entry is absent"))?);}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
            if !effect.performed(1+3*state.native_aliases()as u64)?{return Err(ValueError::invariant("ordered metadata exceeded original Work"))}
            if state.plan.is_none()&&matches!(state.phase,Phase::Search)&&state.ordering.is_none()&&state.current.is_some(){
                let fixed=state.native_copy_demand(0)?;let limit=state.native_reads(body);let key=state.key.as_ref().ok_or_else(||ValueError::invariant("ordered original comparison key absent"))?.as_bytes();let other=state.current.as_ref().expect("selected native comparison").entry.key.as_bytes();
                if state.offset==key.len().min(other.len()){state.ordering=Some(key.len().cmp(&other.len()));}
                while state.ordering.is_none()&&state.last_compared_bytes<limit{
                    if !effect.performed(1)?{return Err(ValueError::invariant("ordered byte read exceeded original Work"))}
                    if let Some(left)=state.left_byte.take(){let right=other[state.offset];state.offset+=1;if left!=right{state.ordering=Some(left.cmp(&right));}}else{state.left_byte=Some(key[state.offset]);}
                    state.last_compared_bytes+=1;if state.ordering.is_none()&&state.offset==key.len().min(other.len()){state.ordering=Some(key.len().cmp(&other.len()));}
                }
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:fixed+state.last_compared_bytes,..Default::default()}))
            }
            let turn=state.advance(effect.grant());turn.result.map(|step|{let p=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};if matches!(step,RetainedCloneStep::Complete(_)){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)}})
        })();crate::RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same ordered native updater receipt")}
    }
    pub fn is_complete(&self) -> bool { self.state.is_complete() }
    pub fn take_result(&mut self) -> Option<OrderedMap<V>> { self.state.take_result() }
    /// 📤️ Explicit shared-value handoff; the recipient owns its eventual domain retirement.
    pub fn take_removed(&mut self) -> Option<SharedOwner<V>> { self.state.take_removed() }
    pub fn begin_close(&mut self) { self.state.begin_close(); }
    pub fn next_close_work_demand(&self)->u64{if !self.state.closing||self.state.terminal_is_empty(){0}else if !self.state.retirement.is_empty(){self.state.retirement.next_work_demand()}else{1}}
    pub fn close_step(&mut self, grant: RetainedCloneGrant, maximum_work_units:u64) -> RetirementStep<V> { if self.next_close_work_demand()>maximum_work_units{self.state.last_close_work=0;return RetirementStep::Blocked} self.state.close_step(grant) }
    pub fn last_close_work(&self)->u64{self.state.last_close_work}
    pub fn next_close_byte_demand(&self)->Result<usize,ValueError>{self.state.retirement.next_close_byte_demand()}
    pub fn next_close_copy_byte_demand(&self)->usize{self.state.retirement.next_copy_byte_demand()}
    pub fn next_close_depth_demand(&self)->usize{(self.state.retained_depth()+self.state.retirement.next_depth_demand()).max(usize::from(!self.terminal_is_empty()))}
    pub fn terminal_is_empty(&self) -> bool { self.state.terminal_is_empty() }
}

fn cold_update_grant<V>(cursor:&UpdateCursor<V>)->RetainedCloneGrant {RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:cursor.next_capacity_byte_demand().expect("ordered cold update birth demand"),maximum_release_bytes:0,maximum_depth:cursor.next_depth_demand()}}
impl<V> Drop for UpdateCursor<V> {
    fn drop(&mut self) {
        if !self.state.terminal_is_empty() {
            assert!(std::thread::panicking(), "ordered-map update must finish explicit close before drop"); return;
        }
        unsafe { ManuallyDrop::drop(&mut self.state); }
    }
}

};}

#[path="🌳️tree/🦀️.rs"]mod native_tree;
pub use native_tree::{Root,Node,Entry,native_height,native_len};
#[path="🔎️lookup/🦀️.rs"]mod native_lookup;
pub use native_lookup::{BorrowedLookupCursor,native_compare_bytes};
