//! 🔗️ Widget graph editing actions 2550 to 2560: adding, ghosting, removing and moving widgets, insert between, make space, neuron params, connect, disconnect and undo. See [`super`].

use super::*;

pub struct FlowAction2550 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2550 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_550
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_550),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.add_widget(text(args, "descriptorJson")?, number(args, "worldX")?, number(args, "worldY")?).map(String::into_bytes).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_550),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2551 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2551 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_551
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_551),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.set_ghost_widget(text(args, "descriptorJson")?, number(args, "worldX")?, number(args, "worldY")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_551),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2552 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2552 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_552
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_552),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.clear_ghost_widget();
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_552),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2553 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2553 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_553
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_553),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.remove_widget(text(args, "widgetId")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_553),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2554 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2554 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_554
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_554),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.move_widget(text(args, "widgetId")?, number(args, "x")?, number(args, "y")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_554),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2555 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2555 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_555
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_555),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> =
                    flow_result! { domain.host.insert_between(text(args, "anchorId")?, text(args, "anchorOutPort")?, text(args, "midId")?, text(args, "midInPort")?, text(args, "midOutPort")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_555),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2556 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2556 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_556
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_556),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.make_space(text(args, "anchorId")?, number(args, "dx")?, number(args, "dy")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_556),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2557 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2557 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_557
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_557),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.set_neuron_params(text(args, "widgetId")?, text(args, "paramsJson")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_557),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2558 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2558 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_558
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_558),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.connect(text(args, "fromId")?, text(args, "toId")?).map(String::into_bytes).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_558),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2559 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2559 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_559
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_559),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.disconnect(text(args, "synapseId")?).map(|_| Vec::new()).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_559),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2560 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2560 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_560
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_560),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(vec![u8::from(domain.host.undo())]) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_560),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}
