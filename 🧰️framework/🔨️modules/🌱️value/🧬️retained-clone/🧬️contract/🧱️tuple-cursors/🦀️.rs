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
    fn output_ready(&self)->bool{!self.closing&&!self.spent&&self.output.is_some()}
            fn advance_demands(&self,source:RetainedCloneRef<'_,($($type,)+)>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"tuple original clone is closing or spent"))}if self.output.is_some(){return Ok(Default::default())}match self.phase{$($index=>if self.draining{clone_cursor_close_demand::<$type>(&self.$child,body)}else{self.$child.advance_demands(source.project($index+1,|value|&value.$index),body)},)+_=>Ok(crate::RetirementDemand{depth:1,..Default::default()})}}

            fn advance(&mut self, source: RetainedCloneRef<'_, ($($type,)+)>, body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
                if self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone cursor is closing")); }
                if self.spent { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone cursor is spent")); }
                if !retained_clone_grant_admits(grant,self.advance_demands(source,body)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
                if self.output.is_some() { return Ok(RetainedCloneStep::Complete(Default::default())); }
                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
                if self.draining {
                    match self.phase {
                        $($index => {
                            if !self.$child.terminal_is_empty() {
                                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                                let step = self.$child.close_step(grant)?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.$child.terminal_is_empty(), "retained tuple child scaffold close")?.progress()));
                            }
                            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                            self.draining = false;
                            self.phase += 1;
                            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                        },)+
                        _ => return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple drain state is invalid")),
                    }
                }
                match self.phase {
                    $($index => match self.$child.advance(source.project($index + 1, |value| &value.$index),body, grant)? {
                        RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "retained tuple child")?)),
                        RetainedCloneStep::Complete(progress) => {
                            let progress = admit_retained_clone_progress(grant, progress, "retained tuple child")?;
                            if progress.copied_items >= grant.maximum_items {
                                return Ok(RetainedCloneStep::Progress(progress));
                            }
                            self.$value = Some(self.$child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple child completed without an owner"))?);
                            let _ = self.$child.begin_close();self.child_closing[$index]=true;
                            self.draining = true;
                            Ok(RetainedCloneStep::Progress(progress.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?))
                        }
                    },)+
                    $count => {
                        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                        self.output = Some(($(self.$value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple owner is missing"))?,)+));
                        self.phase += 1;
                        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
                    }
                    _ => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone state is invalid")),
                }
            }

            fn take(&mut self) -> Option<($($type,)+)> {
                if !self.output_ready(){return None;}
                let output = self.output.take();
                if output.is_some() { self.spent = true; }
                output
            }

            fn begin_close(&mut self) -> bool {
                if self.closing { return false; }
                self.closing = true;
                true
            }

            

            fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
                if !self.closing{return Ok(Default::default());}
                $(if !self.$child.terminal_is_empty(){return if self.child_closing[$index]{self.$child.close_demands(body)}else{Ok(crate::RetirementDemand{depth:1,..Default::default()})};})+
                if !self.close.is_empty(){return self.close.next_owner_demands::<()>(false,&None,body);}
                $(if self.$value.is_some(){return self.close.next_owner_demands::<$type>(true,&None,body);})+
                self.close.next_owner_demands::<($($type,)+)>(self.output.is_some(),&self.source,body)
            }

            fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "tuple must begin close before granted retirement")); }
                if !retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
                $(if !self.$child.terminal_is_empty() {
                    if !self.child_closing[$index] {self.$child.begin_close();self.child_closing[$index]=true; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
                    let step = self.$child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.$child.terminal_is_empty(), "retained child close")?.progress()));
                })+
                if !self.close.is_empty() { return self.close.step_granted(grant); }
                $(if let Some(step) = self.close.begin_granted(&mut self.$value, grant)? { return Ok(step); })+
                if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
                close_retained_binding(&mut self.source, grant)
            }

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
