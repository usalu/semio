//! 🖱️ Computing, hover and preview actions 2516 to 2529: the dag input failure mapping, the computing widget ids, the preview text, hover and hover channel and the preview off set. See [`super`].

use super::*;
use semio_framework_abi::ABI_MAX_BODY_BYTES;
use semio_os_infinite::board::schema::dag_input::DagChannelRef;

pub(crate) fn dag_input_failure(error:semio_framework_value::ValueError)->FlowFailure{
    let code=match error.kind{semio_framework_value::ValueRefusalKind::Canceled=>AbiErrorCode::Cancelled,semio_framework_value::ValueRefusalKind::OwnershipLimit=>AbiErrorCode::NoCredit,_=>AbiErrorCode::MalformedTag};
    flow_close_failure(code).with_retained_progress(error.retained_progress())
}

pub struct FlowAction2516 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2516 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_516
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
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_516),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.clear_computing_widget_ids();
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_516),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2517 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2517 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_517
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
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_517),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.preview_text().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_517),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2521 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2521 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_521
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
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_521),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.hovered_widget_id().unwrap_or_default().into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_521),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2524 {
    pub program: FlowProgramState,
    pub items: FlowStringArrayCursor,
}

impl FlowActionState for FlowAction2524 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_524
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
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_524),
            FlowProgramPhase::Domain => {
                self.program.domain_cursor += 1;
                match self.items.step(&FlowPreviewOffSource(&domain.host)) {
                    Ok(FlowStringArrayStep::Census { bytes }) => match self.program.begin_incremental_output(bytes) {
                        Ok(()) => FlowFeatureStep::Progress { completed: 0, total: bytes as u64 },
                        Err(failure) => FlowFeatureStep::Failed(failure),
                    },
                    Ok(FlowStringArrayStep::Byte(byte)) => {
                        self.program.output.push(byte);
                        FlowFeatureStep::Progress { completed: self.program.output.len() as u64, total: self.program.output.capacity() as u64 }
                    }
                    Ok(FlowStringArrayStep::Progress { completed, total }) => FlowFeatureStep::Progress { completed: completed as u64, total: total as u64 },
                    Ok(FlowStringArrayStep::Complete) => self.program.finish_incremental_output(),
                    Err(failure) => FlowFeatureStep::Failed(failure),
                }
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_524),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2526 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2526 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_526
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
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_526),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        if let Some(edge) = optional_text(args,"edgeId")? {
                            domain.host.dag.graph.set_hover_edge(Some(edge));
                        } else {
                            domain.host.set_hover(optional_text(args,"widgetId")?);
                        }
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_526),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

/// 🧷️ Decodes within the enclosing ABI body limit and the graph hover roster capacity.
pub(super) fn decode_hover_ports(json: Option<&str>) -> Result<Vec<DagChannelRef>, FlowFailure> {
    let Some(json) = json else { return Ok(Vec::new()); };
    let malformed = || FlowFailure::new(AbiErrorCode::MalformedTag, "portsJson");
    if json.len() > ABI_MAX_BODY_BYTES { return Err(malformed()); }
    let ports = semio_framework_pack_json::from_json_str::<Vec<DagChannelRef>>(json, semio_framework_pack_json::JsonMemberPolicy::Reject)
        .map_err(|_| malformed())?;
    if ports.len() > 1024 { return Err(malformed()); }
    Ok(ports)
}

pub struct FlowAction2527 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2527 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_527
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
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_527),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        use semio_os_infinite::board::schema::dag_input::DagChannelDirection;
                        let direction = match optional_text(args, "direction")? {
                            None => None,
                            Some("in") => Some(DagChannelDirection::In),
                            Some("out") => Some(DagChannelDirection::Out),
                            Some(_) => return Err(FlowFailure::new(AbiErrorCode::MalformedTag, "direction")),
                        };
                        let ports = decode_hover_ports(optional_text(args, "portsJson")?)?;
                        domain.host.set_hover_channel_direction(optional_text(args, "widgetId")?, optional_text(args, "port")?, direction);
                        domain.host.dag.graph.set_hover_ports(&ports);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_527),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2529 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2529 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_529
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
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_529),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.set_preview_off_json(text(args, "json")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_529),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

#[cfg(test)]
#[test]
fn full_hover_roster_fits_the_shared_abi_body() {
    use semio_framework_pack_json::{JsonMemberPolicy, Value, object, parse, to_string};
    let fixture = parse(include_str!("../../🧪️tests/🧫️fixtures/🎯️hover-group-capacity.json"), JsonMemberPolicy::Reject).unwrap();
    let count = fixture["count"].as_u64().unwrap() as usize;
    let old_limit = fixture["oldLimitBytes"].as_u64().unwrap() as usize;
    let width = fixture["widgetBytes"].as_u64().unwrap() as usize;
    let prefix = fixture["widgetPrefix"].as_str().unwrap();
    let sample = fixture["escapeSample"].as_str().unwrap();
    let port = fixture["port"].as_str().unwrap();
    let widget = |index: usize| {
        let head = format!("{prefix}{index:04}-{sample}");
        format!("{head}{}", "q".repeat(width - head.len()))
    };
    let row = |index: usize| object([
        ("widgetId".into(), Value::String(widget(index))),
        ("port".into(), Value::String(port.into())),
        ("direction".into(), Value::String(if index % 2 == 0 { "in" } else { "out" }.into())),
    ]);
    let json = to_string(&Value::Array((0..count).map(row).collect()));
    assert!(json.len() > old_limit && json.len() < ABI_MAX_BODY_BYTES);
    assert_eq!(ABI_MAX_BODY_BYTES as u64, fixture["maximumBodyBytes"].as_u64().unwrap());
    let result = decode_hover_ports(Some(&json)).ok().unwrap();
    assert_eq!(result.len(), count);
    for (index, channel) in result.iter().enumerate() {
        assert_eq!(channel.widget_id, widget(index));
        assert_eq!(channel.port, port);
        assert_eq!(channel.direction == semio_os_infinite::board::schema::dag_input::DagChannelDirection::In, index % 2 == 0);
    }
    let too_many = to_string(&Value::Array((0..=count).map(row).collect()));
    assert!(decode_hover_ports(Some(&too_many)).is_err());
    assert!(decode_hover_ports(Some(&" ".repeat(ABI_MAX_BODY_BYTES + 1))).is_err());
    assert!(decode_hover_ports(None).ok().unwrap().is_empty());
}
