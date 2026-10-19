    /// 🔎️ Census oracles and fixture machine: the typed command census decision with its owned and serde oracles and the owned fixture transition machine.

    use serde_json::Value;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct TypedCommandCensusDecision {
        pub bytes: usize,
        pub accepted: bool,
    }

    trait TypedCommandCensusOracle {
        fn decide(&self, description: &str, maximum: usize) -> TypedCommandCensusDecision;
    }

    struct OwnedTypedCommandCensus;

    impl TypedCommandCensusOracle for OwnedTypedCommandCensus {
        fn decide(&self, description: &str, maximum: usize) -> TypedCommandCensusDecision {
            let mut bytes = 0usize;
            let mut accepted = true;
            for _ in description.as_bytes() {
                accepted &= bytes.checked_add(1).is_some_and(|next| {
                    bytes = next;
                    next <= maximum
                });
            }
            accepted &= bytes.checked_add(1).is_some_and(|next| {
                bytes = next;
                next <= maximum
            });
            TypedCommandCensusDecision { bytes, accepted }
        }
    }

    struct SerdeJsonTypedCommandCensus;

    impl TypedCommandCensusOracle for SerdeJsonTypedCommandCensus {
        fn decide(&self, description: &str, maximum: usize) -> TypedCommandCensusDecision {
            let value = serde_json::json!({ "description": description, "uiScopeFields": 1 });
            let bytes = value["description"].as_str().expect("oracle description").as_bytes().len() + value["uiScopeFields"].as_u64().expect("oracle UI scope") as usize;
            TypedCommandCensusDecision { bytes, accepted: bytes <= maximum }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum FixtureTransition {
        Yield,
        Cancelled,
        Checkpoint,
        Advance,
        Retain,
        Fault,
    }

    struct OwnedFixtureMachine {
        pub phase: usize,
        pub advanced: usize,
        retained_page: bool,
        pub owners: usize,
        pub terminal: bool,
    }

    impl OwnedFixtureMachine {
        pub fn new(phase: usize) -> Self {
            Self { phase, advanced: 0, retained_page: true, owners: 0, terminal: false }
        }

        pub fn grant(&mut self, fuel: u64, expired: bool, cancelled: bool) -> FixtureTransition {
            if cancelled {
                return FixtureTransition::Cancelled;
            }
            if fuel == 0 || expired {
                return FixtureTransition::Yield;
            }
            self.phase = self.phase.saturating_add(1);
            self.advanced += 1;
            FixtureTransition::Checkpoint
        }

        pub fn freshness(operation_revision: u64, live_revision: u64, operation_generation: u64, live_generation: u64) -> bool {
            operation_revision == live_revision && operation_generation == live_generation
        }

        pub fn admission(capacity: usize, occupied: usize, roots: usize, required_roots: usize) -> bool {
            occupied < capacity && roots == required_roots
        }

        pub fn publish(&mut self, attempt: u8, acknowledged: bool, maximum_retries: u8) -> FixtureTransition {
            if acknowledged {
                self.retained_page = false;
                FixtureTransition::Advance
            } else if attempt > maximum_retries {
                FixtureTransition::Fault
            } else {
                FixtureTransition::Retain
            }
        }

        fn close_scalar(value: &mut String, maximum_bytes: usize) -> (usize, usize) {
            let Some(bytes) = value.chars().next_back().map(char::len_utf8) else { return (0, 0) };
            if bytes > maximum_bytes {
                return (0, 0);
            }
            value.pop();
            (0, bytes)
        }

        pub fn close_owner(&mut self, maximum_items: usize) -> (usize, usize) {
            if self.owners == 0 {
                self.terminal = true;
                return (0, 0);
            }
            if maximum_items == 0 {
                return (0, 0);
            }
            self.owners -= 1;
            self.terminal = self.owners == 0;
            (1, 0)
        }
    }

    struct SerdeFixtureOracle<'a>(&'a Value);

    impl SerdeFixtureOracle<'_> {
        pub fn transition(&self) -> FixtureTransition {
            match self.0["expected"].as_str().expect("fixture expected transition") {
                "yield" => FixtureTransition::Yield,
                "cancelled" => FixtureTransition::Cancelled,
                "checkpoint" => FixtureTransition::Checkpoint,
                "advance" => FixtureTransition::Advance,
                "retain" => FixtureTransition::Retain,
                "fault" => FixtureTransition::Fault,
                unexpected => panic!("unknown fixture transition {unexpected}"),
            }
        }
    }
