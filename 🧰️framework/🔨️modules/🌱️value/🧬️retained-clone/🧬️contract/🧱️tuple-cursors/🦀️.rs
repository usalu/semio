//! 🧱️ Tuple cursors of the retained clone contract: the tuple retained clone macro and its invocations for the pair and triple cursors. See [`super`].

use super::*;

macro_rules! retained_clone_tuple {
    ($cursor:ident, $count:expr, $(($index:tt, $type:ident, $child:ident, $value:ident)),+ $(,)?) => {
        pub struct $cursor<$($type: RetainedClone),+> {
            $($child: $type::Cursor, $value: Option<$type>,)+
            phase: usize,
            source: Option<RetainedCloneBinding>,
            output: Option<($($type,)+)>,
            spent: bool,
            draining: bool,
            closing: bool,
            child_closing:[bool;$count],
            close: RetainedCloneClose,
        }

        impl<$($type: RetainedClone),+> Default for $cursor<$($type),+> {
            fn default() -> Self {
                Self {
                    $($child: $type::retained_clone_cursor(), $value: None,)+
                    phase: 0,
                    source: None,
                    output: None,
                    spent: false,
                    draining: false,
                    closing: false,
                    child_closing:[false;$count],
                    close: RetainedCloneClose::default(),
                }
            }
        }

        impl<$($type: RetainedClone),+> RetainedCloneCursor<($($type,)+)> for $cursor<$($type),+> {
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_demand(self.closing))}
fn begin_close_work_demand(&self)->Result<u64,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_work_demand(self.closing))}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::retained_clone::RetainedCloneProgress>{crate::retained_clone::start_retained_close(&mut self.closing,effect)}
fn close_started(&self)->bool{self.closing}
fn output_ready(&self)->bool{!self.closing&&!self.spent&&self.output.is_some()}
            fn advance_demands(&self,source:RetainedCloneRef<'_,($($type,)+)>,body:usize,maximum_work_units:u64)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"tuple original clone is closing or spent"))}if self.output.is_some(){return Ok(Default::default())}match self.phase{$($index=>if self.draining{if self.child_closing[$index]{clone_cursor_close_demand::<$type>(&self.$child,body)}else{child_close_start_demand(&self.$child)}}else if self.$child.output_ready(){self.$child.take_demand()}else{self.$child.advance_demands(source.project($index+1,|value|&value.$index),body,maximum_work_units)},)+_=>Ok(crate::RetirementDemand{depth:1,..Default::default()})}}

            fn advance(&mut self, source: RetainedCloneRef<'_, ($($type,)+)>, body: usize,maximum_work_units:u64, effect:&mut crate::RetainedAllocationEffect<'_>) ->crate::OriginalRetirementOutcome{let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
                if self.closing { return Err((crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone cursor is closing")).into()); }
                if self.spent { return Err((crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone cursor is spent")).into()); }
                if !retained_clone_grant_admits(grant,self.advance_demands(source,body,maximum_work_units)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
                if self.output.is_some() { return Ok(RetainedCloneStep::Complete(Default::default())); }
                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)={let turn=source.bind(&mut self.source,grant);work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
                if self.draining {
                    match self.phase {
                        $($index => {
                            if !self.child_closing[$index]{let turn=start_child_close(&mut self.$child,&mut self.child_closing[$index],grant);work+=turn.work;return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}
                            if !self.$child.terminal_is_empty() {
                                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                                let step = {let turn=self.$child.close_step(&mut effect.reborrow(grant,effect.maximum_work_units().saturating_sub(work))?);work+=turn.work;turn.result}?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.$child.terminal_is_empty(), "retained tuple child scaffold close")?.progress()));
                            }
                            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                            self.draining = false;work+=1;
                            self.phase += 1;
                            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                        },)+
                        _ => return Err((crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained tuple drain state is invalid")).into()),
                    }
                }
                match self.phase {
                    $($index => {
                     if self.$child.output_ready(){let turn=self.$child.take(grant);work+=turn.work;let Some((value,p))=turn.result?else{return Ok(RetainedCloneStep::Progress(Default::default()))};self.$value=Some(value);self.draining=true;return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant,p,"tuple child output handoff")?))}
                     let turn=self.$child.advance(source.project($index+1,|value|&value.$index),body,maximum_work_units,&mut effect.reborrow(grant,maximum_work_units)?);work+=turn.work;Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant,turn.result?.progress(),"retained tuple child")?))
                    },)+
                    $count => {
                        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                        work+=1;self.output = Some(($(self.$value.take().ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained tuple owner is missing"))?,)+));
                        self.phase += 1;
                        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
                    }
                    _ => Err((crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone state is invalid")).into()),
                }
            })();crate::RetainedWorkOutcome{result,work}}

            fn take_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{crate::retained_clone::retained_clone_cursor_take_demand::<($($type,)+),bool>()}
    fn take<'reply>(&mut self,output:&mut Option<($($type,)+)>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{crate::retained_clone::take_retained_clone_cursor_output!(self.output_ready(),self.output,output,self.spent,true,effect,reply,($($type,)+),bool)}

            

            

            fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
                if !self.closing{return Ok(Default::default());}
                $(if !self.$child.terminal_is_empty(){return if self.child_closing[$index]{self.$child.close_demands(body)}else{child_close_start_demand(&self.$child)};})+
                if !self.close.is_empty(){return self.close.next_owner_demands::<()>(false,&None,body);}
                $(if self.$value.is_some(){return self.close.next_owner_demands::<$type>(true,&None,body);})+
                self.close.next_owner_demands::<($($type,)+)>(self.output.is_some(),&self.source,body)
            }

            fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let mut work=0;let result=(||{
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                if !self.closing { return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "tuple must begin close before granted retirement").into()); }
                if !retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
                $(if !self.$child.terminal_is_empty() {
                    if !self.child_closing[$index]{let turn=start_child_close(&mut self.$child,&mut self.child_closing[$index],grant);work+=turn.work;return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}
                    let turn=self.$child.close_step(&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"));work+=turn.work;let step=turn.result?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.$child.terminal_is_empty(), "retained child close")?.progress()));
                })+
                if !self.close.is_empty() { let turn=self.close.step_granted(effect);work+=turn.work;return turn.result; }
                $({let turn=self.close.begin_granted(&mut self.$value,effect);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}})+
                {let turn=self.close.begin_granted(&mut self.output,effect);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}}
                {let turn=close_retained_binding(&mut self.source,effect);work+=turn.work;turn.result}
            })();crate::RetainedWorkOutcome{result,work}}

            fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} $(if !self.$child.terminal_is_empty(){return self.$child.next_close_depth_demand();})+ self.close.next_owner_depth_with_binding(self.output.is_some()$(||self.$value.is_some())+,&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
                if !self.closing { return Ok(0); }
                $(if !self.$child.terminal_is_empty() { return self.$child.next_close_copy_byte_demand(); })+
                self.close.next_copy_with_binding(&self.source)
            }
            fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
                if !self.closing { return Ok(0); }
                $(if !self.$child.terminal_is_empty() { return self.$child.next_close_capacity_byte_demand(maximum_release_bytes); })+
                if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
                $(if self.$value.is_some() { return self.close.next_owner_capacity_with_binding::<$type>(true,maximum_release_bytes,&self.source); })+
                self.close.next_owner_capacity_with_binding::<($($type,)+)>(self.output.is_some(),maximum_release_bytes,&self.source)
            }
            fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
                if !self.closing { return Ok(0); }
                $(if !self.$child.terminal_is_empty() { return self.$child.next_close_release_byte_demand(); })+
                self.close.next_release_with_binding(&self.source)
            }
            fn terminal_is_empty(&self) -> bool {
                self.closing && self.output.is_none() && self.close.is_empty() && self.source.is_none() $(&& self.$value.is_none() && self.$child.terminal_is_empty())+
            }
        }

        impl<$($type: RetainedClone),+> RetainedClone for ($($type,)+) {
            type Cursor = $cursor<$($type),+>;
            fn retained_clone_cursor() -> Self::Cursor { Default::default() }
        }
    };
}

retained_clone_tuple!(Tuple2Cursor, 2, (0, A, first_cursor, first_value), (1, B, second_cursor, second_value));

retained_clone_tuple!(Tuple3Cursor, 3, (0, A, first_cursor, first_value), (1, B, second_cursor, second_value), (2, C, third_cursor, third_value));
