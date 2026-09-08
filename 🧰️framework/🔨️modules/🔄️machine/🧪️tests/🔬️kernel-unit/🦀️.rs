mod tests {
    use super::super::inspect::NullInspector;
    use super::*;
    use crate::{ActionId, BitSet, EventId, GuardId, NodeId};

    //#region 🔖️ToggleMachine

    #[derive(Clone, Debug, PartialEq)]
    enum ToggleEvent {
        Flip,
    }

    impl StatechartEvent for ToggleEvent {
        const EVENT_COUNT: u16 = 1;
        fn event_id(&self) -> EventId {
            EventId(0)
        }
        fn event_name(id: EventId) -> &'static str {
            match id.0 {
                0 => "Flip",
                _ => "?",
            }
        }
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct ToggleContext {
        count: u32,
        allow: bool,
    }

    fn toggle_inc(ctx: &mut ToggleContext, _event: Option<&ToggleEvent>, _sink: &mut Vec<Command<ToggleMachine>>) {
        ctx.count += 1;
    }

    fn toggle_allowed(ctx: &ToggleContext, _event: Option<&ToggleEvent>) -> bool {
        ctx.allow
    }

    const TOGGLE_NODES: &[NodeDef] = &[
        NodeDef { stable_id: "root", kind: NodeKind::Compound, parent: None, initial: Some(NodeId(1)), children: &[NodeId(1), NodeId(2)], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 0 },
        NodeDef { stable_id: "off", kind: NodeKind::Atomic, parent: Some(ROOT), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 1 },
        NodeDef { stable_id: "on", kind: NodeKind::Atomic, parent: Some(ROOT), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 2 },
    ];

    const TOGGLE_TRANSITIONS: &[TransitionDef] = &[
        TransitionDef { source: NodeId(1), trigger: Trigger::Event(EventId(0)), guard: None, targets: &[NodeId(2)], kind: TransitionKind::External, actions: &[ActionId(0)], doc_index: 0 },
        TransitionDef { source: NodeId(2), trigger: Trigger::Event(EventId(0)), guard: Some(GuardId(0)), targets: &[NodeId(1)], kind: TransitionKind::External, actions: &[ActionId(0)], doc_index: 1 },
    ];

    struct ToggleMachine;
    impl Machine for ToggleMachine {
        type Context = ToggleContext;
        type Event = ToggleEvent;
        type Input = bool;
        type Output = ();
        type Effect = ();
        type Config = BitSet<1>;
        fn definition() -> &'static MachineDefinition<Self> {
            static DEF: MachineDefinition<ToggleMachine> = MachineDefinition {
                id: "toggle",
                nodes: TOGGLE_NODES,
                transitions: TOGGLE_TRANSITIONS,
                context_from_input: |allow| ToggleContext { count: 0, allow },
                make_output: None,
                guards: &[toggle_allowed],
                actions: &[toggle_inc],
                fingerprint: 1,
                manifest_json: "{}",
            };
            &DEF
        }
    }

    #[test]
    fn flat_machine_toggles_and_counts() {
        let mut sink: Vec<Command<ToggleMachine>> = Vec::new();
        let mut snapshot = init::<ToggleMachine>(true, &mut sink);
        assert!(snapshot.matches("off"));
        let mut inspector = NullInspector;
        macrostep(&mut snapshot, ToggleEvent::Flip, &mut sink, &mut inspector);
        assert!(snapshot.matches("on"));
        assert_eq!(snapshot.context.count, 1);
        macrostep(&mut snapshot, ToggleEvent::Flip, &mut sink, &mut inspector);
        assert!(snapshot.matches("off"));
        assert_eq!(snapshot.context.count, 2);
    }

    #[test]
    fn guard_blocks_transition_when_false() {
        let mut sink: Vec<Command<ToggleMachine>> = Vec::new();
        let mut snapshot = init::<ToggleMachine>(false, &mut sink);
        let mut inspector = NullInspector;
        macrostep(&mut snapshot, ToggleEvent::Flip, &mut sink, &mut inspector);
        assert!(snapshot.matches("on"));
        // guard on the On->Off transition requires ctx.allow, which is false.
        macrostep(&mut snapshot, ToggleEvent::Flip, &mut sink, &mut inspector);
        assert!(snapshot.matches("on"), "guard should have blocked the transition back to off");
        assert_eq!(snapshot.context.count, 1);
    }

    //#endregion 🔖️ToggleMachine

    //#region 🔖️PlayerMachine

    #[derive(Clone, Debug, PartialEq)]
    enum PlayerEvent {
        Open,
        Pause,
        Play,
        Stop,
        Resume,
    }

    impl StatechartEvent for PlayerEvent {
        const EVENT_COUNT: u16 = 5;
        fn event_id(&self) -> EventId {
            match self {
                PlayerEvent::Open => EventId(0),
                PlayerEvent::Pause => EventId(1),
                PlayerEvent::Play => EventId(2),
                PlayerEvent::Stop => EventId(3),
                PlayerEvent::Resume => EventId(4),
            }
        }
        fn event_name(id: EventId) -> &'static str {
            match id.0 {
                0 => "Open",
                1 => "Pause",
                2 => "Play",
                3 => "Stop",
                4 => "Resume",
                _ => "?",
            }
        }
    }

    #[derive(Clone, Debug, Default)]
    struct PlayerContext;

    const PLAYER_NODES: &[NodeDef] = &[
        NodeDef { stable_id: "root", kind: NodeKind::Compound, parent: None, initial: Some(NodeId(1)), children: &[NodeId(1), NodeId(3)], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 0 },
        NodeDef { stable_id: "closed", kind: NodeKind::Atomic, parent: Some(ROOT), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 1 },
        NodeDef { stable_id: "playing", kind: NodeKind::Atomic, parent: Some(NodeId(3)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 3 },
        NodeDef { stable_id: "open", kind: NodeKind::Compound, parent: Some(ROOT), initial: Some(NodeId(2)), children: &[NodeId(2), NodeId(4), NodeId(5)], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 2 },
        NodeDef { stable_id: "paused", kind: NodeKind::Atomic, parent: Some(NodeId(3)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 4 },
        NodeDef { stable_id: "open.history", kind: NodeKind::HistoryShallow, parent: Some(NodeId(3)), initial: Some(NodeId(2)), children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 5 },
    ];

    const PLAYER_TRANSITIONS: &[TransitionDef] = &[
        TransitionDef { source: NodeId(1), trigger: Trigger::Event(EventId(0)), guard: None, targets: &[NodeId(3)], kind: TransitionKind::External, actions: &[], doc_index: 0 },
        TransitionDef { source: NodeId(2), trigger: Trigger::Event(EventId(1)), guard: None, targets: &[NodeId(4)], kind: TransitionKind::External, actions: &[], doc_index: 1 },
        TransitionDef { source: NodeId(4), trigger: Trigger::Event(EventId(2)), guard: None, targets: &[NodeId(2)], kind: TransitionKind::External, actions: &[], doc_index: 2 },
        TransitionDef { source: NodeId(3), trigger: Trigger::Event(EventId(3)), guard: None, targets: &[NodeId(1)], kind: TransitionKind::External, actions: &[], doc_index: 3 },
        TransitionDef { source: NodeId(1), trigger: Trigger::Event(EventId(4)), guard: None, targets: &[NodeId(5)], kind: TransitionKind::External, actions: &[], doc_index: 4 },
    ];

    struct PlayerMachine;
    impl Machine for PlayerMachine {
        type Context = PlayerContext;
        type Event = PlayerEvent;
        type Input = ();
        type Output = ();
        type Effect = ();
        type Config = BitSet<1>;
        fn definition() -> &'static MachineDefinition<Self> {
            static DEF: MachineDefinition<PlayerMachine> =
                MachineDefinition { id: "player", nodes: PLAYER_NODES, transitions: PLAYER_TRANSITIONS, context_from_input: |_| PlayerContext, make_output: None, guards: &[], actions: &[], fingerprint: 2, manifest_json: "{}" };
            &DEF
        }
    }

    #[test]
    fn hierarchical_machine_enters_default_descendant() {
        let mut sink: Vec<Command<PlayerMachine>> = Vec::new();
        let snapshot = init::<PlayerMachine>((), &mut sink);
        assert!(snapshot.matches("closed"));
        assert!(!snapshot.matches("open"));
    }

    #[test]
    fn hierarchical_machine_transitions_into_compound_default() {
        let mut sink: Vec<Command<PlayerMachine>> = Vec::new();
        let mut snapshot = init::<PlayerMachine>((), &mut sink);
        let mut inspector = NullInspector;
        macrostep(&mut snapshot, PlayerEvent::Open, &mut sink, &mut inspector);
        assert!(snapshot.matches("open"));
        assert!(snapshot.matches("playing"));
    }

    #[test]
    fn shallow_history_restores_last_active_child() {
        let mut sink: Vec<Command<PlayerMachine>> = Vec::new();
        let mut snapshot = init::<PlayerMachine>((), &mut sink);
        let mut inspector = NullInspector;
        macrostep(&mut snapshot, PlayerEvent::Open, &mut sink, &mut inspector);
        macrostep(&mut snapshot, PlayerEvent::Pause, &mut sink, &mut inspector);
        assert!(snapshot.matches("paused"));
        macrostep(&mut snapshot, PlayerEvent::Stop, &mut sink, &mut inspector);
        assert!(snapshot.matches("closed"));
        assert!(!snapshot.matches("open"));
        macrostep(&mut snapshot, PlayerEvent::Resume, &mut sink, &mut inspector);
        assert!(snapshot.matches("open"));
        assert!(snapshot.matches("paused"), "shallow history should restore Paused, not the default Playing");
        assert!(!snapshot.matches("playing"));
        macrostep(&mut snapshot, PlayerEvent::Play, &mut sink, &mut inspector);
        assert!(snapshot.matches("playing"));
        assert!(!snapshot.matches("paused"));
    }

    //#endregion 🔖️PlayerMachine

    //#region 🔖️RecorderMachine

    #[derive(Clone, Debug, PartialEq)]
    enum RecorderEvent {
        Start,
        AudioStop,
        VideoStop,
    }

    impl StatechartEvent for RecorderEvent {
        const EVENT_COUNT: u16 = 3;
        fn event_id(&self) -> EventId {
            match self {
                RecorderEvent::Start => EventId(0),
                RecorderEvent::AudioStop => EventId(1),
                RecorderEvent::VideoStop => EventId(2),
            }
        }
        fn event_name(id: EventId) -> &'static str {
            match id.0 {
                0 => "Start",
                1 => "AudioStop",
                2 => "VideoStop",
                _ => "?",
            }
        }
    }

    #[derive(Clone, Debug, Default)]
    struct RecorderContext;

    const RECORDER_NODES: &[NodeDef] = &[
        NodeDef { stable_id: "root", kind: NodeKind::Compound, parent: None, initial: Some(NodeId(1)), children: &[NodeId(1), NodeId(2)], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 0 },
        NodeDef { stable_id: "idle", kind: NodeKind::Atomic, parent: Some(ROOT), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 1 },
        NodeDef { stable_id: "recording", kind: NodeKind::Parallel, parent: Some(ROOT), initial: None, children: &[NodeId(3), NodeId(6)], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 2 },
        NodeDef { stable_id: "audio", kind: NodeKind::Compound, parent: Some(NodeId(2)), initial: Some(NodeId(4)), children: &[NodeId(4), NodeId(5)], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 3 },
        NodeDef { stable_id: "audio.capturing", kind: NodeKind::Atomic, parent: Some(NodeId(3)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 4 },
        NodeDef { stable_id: "audio.done", kind: NodeKind::Final, parent: Some(NodeId(3)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 5 },
        NodeDef { stable_id: "video", kind: NodeKind::Compound, parent: Some(NodeId(2)), initial: Some(NodeId(7)), children: &[NodeId(7), NodeId(8)], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 6 },
        NodeDef { stable_id: "video.capturing", kind: NodeKind::Atomic, parent: Some(NodeId(6)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 7 },
        NodeDef { stable_id: "video.done", kind: NodeKind::Final, parent: Some(NodeId(6)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 8 },
    ];

    const RECORDER_TRANSITIONS: &[TransitionDef] = &[
        TransitionDef { source: NodeId(1), trigger: Trigger::Event(EventId(0)), guard: None, targets: &[NodeId(2)], kind: TransitionKind::External, actions: &[], doc_index: 0 },
        TransitionDef { source: NodeId(4), trigger: Trigger::Event(EventId(1)), guard: None, targets: &[NodeId(5)], kind: TransitionKind::External, actions: &[], doc_index: 1 },
        TransitionDef { source: NodeId(7), trigger: Trigger::Event(EventId(2)), guard: None, targets: &[NodeId(8)], kind: TransitionKind::External, actions: &[], doc_index: 2 },
        TransitionDef { source: NodeId(2), trigger: Trigger::Done(NodeId(2)), guard: None, targets: &[NodeId(1)], kind: TransitionKind::External, actions: &[], doc_index: 3 },
    ];

    struct RecorderMachine;
    impl Machine for RecorderMachine {
        type Context = RecorderContext;
        type Event = RecorderEvent;
        type Input = ();
        type Output = ();
        type Effect = ();
        type Config = BitSet<1>;
        fn definition() -> &'static MachineDefinition<Self> {
            static DEF: MachineDefinition<RecorderMachine> =
                MachineDefinition { id: "recorder", nodes: RECORDER_NODES, transitions: RECORDER_TRANSITIONS, context_from_input: |_| RecorderContext, make_output: None, guards: &[], actions: &[], fingerprint: 3, manifest_json: "{}" };
            &DEF
        }
    }

    #[test]
    fn parallel_regions_enter_together() {
        let mut sink: Vec<Command<RecorderMachine>> = Vec::new();
        let mut snapshot = init::<RecorderMachine>((), &mut sink);
        let mut inspector = NullInspector;
        macrostep(&mut snapshot, RecorderEvent::Start, &mut sink, &mut inspector);
        assert!(snapshot.matches("recording"));
        assert!(snapshot.matches("audio.capturing"));
        assert!(snapshot.matches("video.capturing"));
    }

    #[test]
    fn parallel_done_bubbles_only_once_every_region_finishes() {
        let mut sink: Vec<Command<RecorderMachine>> = Vec::new();
        let mut snapshot = init::<RecorderMachine>((), &mut sink);
        let mut inspector = NullInspector;
        macrostep(&mut snapshot, RecorderEvent::Start, &mut sink, &mut inspector);
        macrostep(&mut snapshot, RecorderEvent::AudioStop, &mut sink, &mut inspector);
        assert!(snapshot.matches("audio.done"));
        assert!(snapshot.matches("recording"), "video region still capturing — on_done must not fire yet");
        macrostep(&mut snapshot, RecorderEvent::VideoStop, &mut sink, &mut inspector);
        assert!(snapshot.matches("idle"), "on_done should bubble once both regions reach final");
        assert!(!snapshot.matches("recording"));
    }

    //#endregion 🔖️RecorderMachine
}
