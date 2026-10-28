//! 🧵 Retained EXPRESS continuations execute over nonrecursive values under their initiating original authority.
use super::{ast::*, metadata, overlay::*, receive::PopulationReceiver, retained::*};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{
    RetirementDemand, ValueError,
    retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneStep},
};
use std::{cmp::Ordering, mem::size_of};

type Handle = usize;
const NONE: usize = usize::MAX;
const MAXIMUM_DEPTH: usize = 256;
const MAXIMUM_NODES: usize = 10_000_000;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Logical {
    False,
    Unknown,
    True,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum AggregateKind {
    List,
    Set,
    Bag,
    Array,
}
#[derive(Clone, Copy, Debug)]
enum Text {
    Static(&'static str),
    Source(usize),
    Owned(usize),
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Entity {
    Original(usize),
    Constructed(usize),
    View(usize),
}
#[derive(Clone, Copy, Default, Debug)]
struct Cursor {
    source: bool,
    next: Option<usize>,
}
#[derive(Clone, Copy, Debug)]
struct Aggregate {
    kind: AggregateKind,
    lower: i128,
    first: Option<usize>,
    last: Option<usize>,
    length: usize,
}
impl Aggregate {
    fn empty(kind: AggregateKind) -> Self {
        Self { kind, lower: 1, first: None, last: None, length: 0 }
    }
}
#[derive(Clone, Copy, Debug)]
enum Value {
    Missing,
    Logical(Logical),
    Integer(i128),
    Real(f64),
    String(Text),
    Binary(Text),
    Enum(Text),
    Entity(Entity),
    Aggregate(Aggregate),
    Source(usize),
    Tagged { base: Handle, domain: Option<&'static str>, aggregate: Option<(AggregateKind, i128)> },
    Qualified { base: Handle, owner: &'static str },
    Local { base: Handle, view: usize },
}
#[derive(Clone, Copy, Debug)]
enum Raw {
    Missing,
    Logical(Logical),
    Integer(i128),
    Real(f64),
    String(Text),
    Binary(Text),
    Enum(Text),
    Entity(Entity),
    Aggregate { cursor: Cursor, kind: AggregateKind, lower: i128, length: usize },
}
#[derive(Clone, Copy)]
struct Edge {
    value: Handle,
    next: Option<usize>,
}
#[derive(Clone, Copy)]
struct Scope {
    parent: Option<usize>,
    bindings: Option<usize>,
    root: usize,
    memo: Option<usize>,
}
#[derive(Clone, Copy)]
struct Binding {
    name: &'static str,
    value: Handle,
    declaration: Option<&'static str>,
    next: Option<usize>,
}
#[derive(Clone, Copy)]
struct Constructed {
    declaration: &'static metadata::StaticEntity,
    fields: [Option<(&'static str, Handle)>; 32],
    length: usize,
    version: u64,
    view: Option<usize>,
}
#[derive(Clone, Copy)]
struct LocalView {
    original: usize,
    root: usize,
    next: Option<usize>,
    record: Constructed,
}
#[derive(Clone, Copy)]
struct Memo {
    entity: Entity,
    rule: usize,
    value: Option<Handle>,
    active: bool,
    root: usize,
    parent: Option<usize>,
    dependencies: Option<usize>,
    cacheable: bool,
}
#[derive(Clone, Copy)]
struct Dependency {
    entity: Entity,
    version: u64,
    next: Option<usize>,
}
#[derive(Clone, Copy)]
struct DomainSlot {
    expression: Option<&'static metadata::StaticTypeExpression>,
    name: Option<&'static str>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedRuleKind {
    Where,
    Global,
    Bound,
    Inverse,
    Unique,
    Admission,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedRuleOutcome {
    True,
    False,
    Unknown,
    Incomplete,
}
#[derive(Clone, Copy, Debug)]
pub struct RetainedRuleRecord {
    pub owner: Option<&'static str>,
    pub rule: Option<&'static str>,
    pub kind: RetainedRuleKind,
    pub outcome: RetainedRuleOutcome,
    pub entity_id: Option<u64>,
    pub offset: Option<usize>,
    pub code: Option<&'static str>,
}
pub struct RetainedPopulationReport<'a> {
    pub complete: bool,
    pub valid: bool,
    pub results: &'a [RetainedRuleRecord],
    pub diagnostics: &'a [RetainedRuleRecord],
    pub work: u64,
}
#[derive(Clone, Copy)]
enum Kind {
    Eval,
    EvalInner,
    RootRead,
    Dependency,
    PreparedQuery,
    Lookup,
    Unary,
    BinaryLeft,
    BinaryRight,
    Aggregate,
    AggregateRepeat,
    CallArgs,
    Call,
    Function,
    Declare,
    Coerce,
    Body,
    Statement,
    Assign,
    Case,
    Repeat,
    Query,
    Index,
    Slice,
    TextCopy,
    TextCompare,
    CompareResult,
    Equal,
    EqualSequence,
    Contains,
    AggregateBinary,
    Match,
    Attribute,
    TypeOf,
    UsedIn,
    Header,
    Normalize,
    Domain,
    Select,
    Population,
    TypeRules,
    Bound,
    InverseRule,
    UniqueRule,
    Global,
    Record,
    AggregateUnique,
}
#[derive(Clone, Copy)]
struct Frame {
    kind: Kind,
    expression: Option<&'static ExpressExpression>,
    statement: Option<&'static ExpressStatement>,
    function: Option<&'static ExpressFunction>,
    body: &'static [ExpressStatement],
    declarations: &'static [ExpressVariable],
    name: &'static str,
    scope: usize,
    depth: usize,
    stage: usize,
    position: usize,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    i: i128,
    j: i128,
    k: i128,
    cursor: Cursor,
    other: Cursor,
    truth: Logical,
    identity: bool,
    type_expression: Option<&'static metadata::StaticTypeExpression>,
    entity: Option<&'static metadata::StaticEntity>,
    owner: &'static str,
    record_entity: Option<Entity>,
}
impl Frame {
    fn new(kind: Kind, scope: usize, depth: usize) -> Self {
        Self {
            kind,
            expression: None,
            statement: None,
            function: None,
            body: &[],
            declarations: &[],
            name: "",
            scope,
            depth,
            stage: 0,
            position: 0,
            a: NONE,
            b: NONE,
            c: NONE,
            d: NONE,
            i: 0,
            j: 0,
            k: 0,
            cursor: Cursor::default(),
            other: Cursor::default(),
            truth: Logical::True,
            identity: false,
            type_expression: None,
            entity: None,
            owner: "",
            record_entity: None,
        }
    }
    fn eval(expression: &'static ExpressExpression, scope: usize, depth: usize) -> Self {
        Self { expression: Some(expression), ..Self::new(Kind::Eval, scope, depth) }
    }
}
semio_framework_value::artifact_retire_leaf!(Value, Edge, Scope, Binding, Constructed, LocalView, Memo, Dependency, DomainSlot, RetainedRuleRecord, Frame);
#[derive(Default, semio_framework_value::RetireOwned)]
struct Store {
    values: Vec<Value>,
    edges: Vec<Edge>,
    tasks: Vec<Frame>,
    results: Vec<Handle>,
    scopes: Vec<Scope>,
    bindings: Vec<Binding>,
    entities: Vec<Constructed>,
    originals: Vec<Constructed>,
    memos: Vec<Memo>,
    texts: Vec<String>,
    scratch: Vec<usize>,
    domains: Vec<DomainSlot>,
    records: Vec<RetainedRuleRecord>,
    diagnostics: Vec<RetainedRuleRecord>,
    views: Vec<LocalView>,
    dependencies: Vec<Dependency>,
}
#[derive(Clone, Copy)]
enum Initial {
    Expression(&'static ExpressExpression),
    Call(&'static str, &'static [i128]),
    Population,
    Local(usize),
    #[cfg(test)]
    Prepared(&'static ExpressExpression, &'static [(&'static str, u64)]),
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Flow {
    None,
    Return(Handle),
    Escape,
    Skip,
}

pub struct MachineEnvironment<'a> {
    pub overlay: Option<&'a PopulationOverlay>,
    pub receiver: Option<&'a PopulationReceiver>,
    pub source: Option<&'a IfcSnapshot>,
    pub mutations: &'a [IfcMutation],
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct EnvironmentIdentity {
    overlay: usize,
    receiver: usize,
    source: usize,
    mutations: usize,
    length: usize,
}
impl MachineEnvironment<'_> {
    pub const fn empty() -> Self {
        Self { overlay: None, receiver: None, source: None, mutations: &[] }
    }
    fn source_value(&self, node: usize) -> Result<&IfcValue, ValueError> {
        self.receiver.ok_or_else(|| refusal("express.runtime-source"))?.source_value(self.overlay.ok_or_else(|| refusal("express.runtime-overlay"))?, self.source.ok_or_else(|| refusal("express.runtime-snapshot"))?, self.mutations, node)
    }
    fn identity(&self) -> Result<EnvironmentIdentity, ValueError> {
        match (self.overlay, self.receiver, self.source) {
            (Some(overlay), Some(receiver), Some(source)) => {
                overlay.check_authority(source, self.mutations)?;
                if !overlay.is_ready() || receiver.arena().is_none() {
                    return Err(refusal("express.runtime-source-incomplete"));
                }
            }
            (None, None, None) if self.mutations.is_empty() => {}
            _ => return Err(refusal("express.runtime-source-authority")),
        }
        Ok(EnvironmentIdentity {
            overlay: self.overlay.map_or(0, |value| value as *const PopulationOverlay as usize),
            receiver: self.receiver.map_or(0, |value| value as *const PopulationReceiver as usize),
            source: self.source.map_or(0, |value| value as *const IfcSnapshot as usize),
            mutations: if self.mutations.is_empty() { 0 } else { self.mutations.as_ptr() as usize },
            length: self.mutations.len(),
        })
    }
}
pub struct RetainedMachine {
    program: &'static ExpressProgram,
    store: Option<Store>,
    retirement: RetainedCloneClose,
    initial: Option<Initial>,
    initialized: bool,
    failed: bool,
    closing: bool,
    iterations: usize,
    work: u64,
    flow: Flow,
    environment: Option<EnvironmentIdentity>,
    population_complete: bool,
    population_valid: bool,
    population_finished: bool,
}
impl RetainedMachine {
    pub fn birth_demands() -> RetirementDemand {
        fixed(size_of::<Store>() + size_of::<Initial>() + size_of::<Self>())
    }
    pub fn new(program: &'static ExpressProgram) -> Self {
        Self {
            program,
            store: None,
            retirement: Default::default(),
            initial: None,
            initialized: false,
            failed: false,
            closing: false,
            iterations: 0,
            work: 0,
            flow: Flow::None,
            environment: None,
            population_complete: true,
            population_valid: true,
            population_finished: false,
        }
    }
    fn store(&self) -> &Store {
        self.store.as_ref().expect("retained semantic store")
    }
    fn store_mut(&mut self) -> &mut Store {
        self.store.as_mut().expect("retained semantic store")
    }
    fn birth(&mut self, initial: Initial, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.store.is_some() || self.closing {
            return Err(refusal("express.runtime-already-started").into());
        }
        let demand = Self::birth_demands();
        if !admitted(cx, demand)? {
            return Ok(false);
        }
        self.store = Some(Store::default());
        self.initial = Some(initial);
        debit(cx, demand)?;
        Ok(true)
    }
    pub fn start(&mut self, expression: &'static ExpressExpression, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.birth(Initial::Expression(expression), cx)
    }
    pub fn start_call(&mut self, name: &'static str, arguments: &'static [i128], cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.birth(Initial::Call(name, arguments), cx)
    }
    pub fn start_population(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.birth(Initial::Population, cx)
    }
    pub(super) fn start_local(&mut self, row: usize, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if !std::ptr::eq(self.program, super::ifc4x3_express_program()) {
            return Err(refusal("express.local-program-authority").into());
        }
        self.birth(Initial::Local(row), cx)
    }
    pub(super) fn local_status(&self) -> Option<&'static str> {
        if !matches!(self.initial, Some(Initial::Local(_))) || !self.population_finished || self.closing {
            return None;
        }
        Some(if !self.population_complete {
            "incomplete"
        } else if !self.population_valid {
            "fail"
        } else {
            "pass"
        })
    }
    pub fn report(&self) -> Option<RetainedPopulationReport<'_>> {
        if !self.population_finished || self.closing {
            return None;
        }
        let store = self.store.as_ref()?;
        Some(RetainedPopulationReport { complete: self.population_complete, valid: self.population_complete && self.population_valid, results: &store.records, diagnostics: &store.diagnostics, work: self.work })
    }
    fn growth<T>(vector: &Vec<T>, additional: usize) -> Result<Option<RetirementDemand>, ValueError> {
        if vector.capacity() - vector.len() >= additional {
            return Ok(None);
        }
        let target = vector.len().checked_add(additional).ok_or_else(|| refusal("express.runtime-capacity-overflow"))?.max(vector.capacity().saturating_mul(2)).max(16);
        if target > MAXIMUM_NODES {
            return Err(refusal("express.node-budget"));
        }
        Ok(Some(RetirementDemand { copy_bytes: capacity(vector.len(), size_of::<T>())?, capacity_bytes: capacity(target, size_of::<T>())?, release_bytes: capacity(vector.capacity(), size_of::<T>())?, depth: 1 }))
    }
    fn reserve<T>(vector: &mut Vec<T>, additional: usize) {
        let target = (vector.len() + additional).max(vector.capacity().saturating_mul(2)).max(16);
        vector.reserve_exact(target - vector.len());
    }
    fn next_growth(&self) -> Result<Option<(usize, RetirementDemand)>, ValueError> {
        let store = self.store();
        for (index, demand) in [
            Self::growth(&store.values, 8)?,
            Self::growth(&store.edges, 4)?,
            Self::growth(&store.tasks, 4)?,
            Self::growth(&store.results, 4)?,
            Self::growth(&store.scopes, 2)?,
            Self::growth(&store.bindings, 4)?,
            Self::growth(&store.entities, 1)?,
            Self::growth(&store.texts, 1)?,
            Self::growth(&store.scratch, 8)?,
            Self::growth(&store.originals, 1)?,
            Self::growth(&store.memos, 1)?,
            Self::growth(&store.domains, 1)?,
            Self::growth(&store.records, 1)?,
            Self::growth(&store.diagnostics, 1)?,
            Self::growth(&store.views, 1)?,
            Self::growth(&store.dependencies, 1)?,
        ]
        .into_iter()
        .enumerate()
        {
            if let Some(demand) = demand {
                return Ok(Some((index, demand)));
            }
        }
        Ok(None)
    }
    fn reserve_table(&mut self, index: usize) {
        let store = self.store_mut();
        match index {
            0 => Self::reserve(&mut store.values, 8),
            1 => Self::reserve(&mut store.edges, 4),
            2 => Self::reserve(&mut store.tasks, 4),
            3 => Self::reserve(&mut store.results, 4),
            4 => Self::reserve(&mut store.scopes, 2),
            5 => Self::reserve(&mut store.bindings, 4),
            6 => Self::reserve(&mut store.entities, 1),
            7 => Self::reserve(&mut store.texts, 1),
            8 => Self::reserve(&mut store.scratch, 8),
            9 => Self::reserve(&mut store.originals, 1),
            10 => Self::reserve(&mut store.memos, 1),
            11 => Self::reserve(&mut store.domains, 1),
            12 => Self::reserve(&mut store.records, 1),
            13 => Self::reserve(&mut store.diagnostics, 1),
            14 => Self::reserve(&mut store.views, 1),
            _ => Self::reserve(&mut store.dependencies, 1),
        }
    }
    /// 🎟️ Table births/reallocations and semantic actions are distinct actual original transitions.
    pub fn demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.store.is_none() || self.closing {
            return Err(refusal("express.runtime-not-live"));
        }
        if self.failed && !self.population_finished {
            return Err(refusal("express.runtime-failed"));
        }
        if self.population_finished || self.initialized && self.store().tasks.is_empty() {
            return Ok(fixed(0));
        }
        if self.environment.is_none() {
            return Ok(fixed(size_of::<EnvironmentIdentity>()));
        }
        if let Some((_, demand)) = self.next_growth()? {
            return Ok(demand);
        }
        let mut demand = fixed(
            4 * size_of::<Frame>()
                + 8 * size_of::<Value>()
                + 4 * size_of::<Edge>()
                + 4 * size_of::<Binding>()
                + 2 * size_of::<Scope>()
                + 2 * size_of::<LocalView>()
                + 2 * size_of::<Constructed>()
                + 2 * size_of::<Memo>()
                + 2 * size_of::<Dependency>()
                + 2048,
        );
        if let Some(frame) = self.store().tasks.last() {
            demand.depth = frame.depth.max(1);
            if matches!(frame.kind, Kind::TextCopy) && frame.stage == 0 {
                demand.capacity_bytes = usize::try_from(frame.i).map_err(|_| refusal("express.text-budget"))?;
            }
        }
        Ok(demand)
    }
    fn make(&mut self, value: Value) -> Handle {
        let id = self.store().values.len();
        self.store_mut().values.push(value);
        id
    }
    fn result(&mut self, value: Value) {
        let id = self.make(value);
        self.store_mut().results.push(id);
    }
    fn pop(&mut self) -> Result<Handle, ValueError> {
        self.store_mut().results.pop().ok_or_else(|| refusal("express.runtime-result-stack"))
    }
    fn push(&mut self, frame: Frame) -> Result<(), ValueError> {
        if frame.depth > MAXIMUM_DEPTH {
            return Err(refusal("express.depth"));
        }
        self.store_mut().tasks.push(frame);
        Ok(())
    }
    fn bind(&mut self, scope: usize, name: &'static str, value: Handle, declaration: Option<&'static str>) -> usize {
        let next = self.store().scopes[scope].bindings;
        let binding = self.store().bindings.len();
        self.store_mut().bindings.push(Binding { name, value, declaration, next });
        self.store_mut().scopes[scope].bindings = Some(binding);
        binding
    }
    fn scope(&mut self, parent: Option<usize>) -> usize {
        let index = self.store().scopes.len();
        let (root, memo) = parent
            .map(|parent| {
                let scope = self.store().scopes[parent];
                (scope.root, scope.memo)
            })
            .unwrap_or((index, None));
        self.store_mut().scopes.push(Scope { parent, bindings: None, root, memo });
        index
    }
    fn detached_scope(&mut self, source: usize) -> usize {
        let scope = self.store().scopes[source];
        let index = self.scope(None);
        self.store_mut().scopes[index].root = scope.root;
        self.store_mut().scopes[index].memo = scope.memo;
        index
    }
    fn root_scope(&mut self, parent: usize) -> usize {
        let index = self.scope(Some(parent));
        self.store_mut().scopes[index].root = index;
        self.store_mut().scopes[index].memo = None;
        index
    }
    fn append(&mut self, aggregate: Handle, value: Handle) -> Result<(), ValueError> {
        let Value::Aggregate(mut items) = self.store().values[aggregate] else {
            return Err(refusal("express.aggregate-builder"));
        };
        let index = self.store().edges.len();
        self.store_mut().edges.push(Edge { value, next: None });
        if let Some(last) = items.last {
            self.store_mut().edges[last].next = Some(index);
        } else {
            items.first = Some(index);
        }
        items.last = Some(index);
        items.length += 1;
        self.store_mut().values[aggregate] = Value::Aggregate(items);
        Ok(())
    }
    fn item(&mut self, cursor: &mut Cursor, env: &MachineEnvironment<'_>) -> Result<Option<Handle>, ValueError> {
        let Some(index) = cursor.next else { return Ok(None) };
        if cursor.source {
            let edge = env.receiver.and_then(PopulationReceiver::arena).and_then(|arena| arena.edges.get(index)).ok_or_else(|| refusal("express.runtime-source-edge"))?;
            cursor.next = edge.next;
            Ok(Some(self.make(Value::Source(edge.value))))
        } else {
            let edge = self.store().edges[index];
            cursor.next = edge.next;
            Ok(Some(edge.value))
        }
    }
    fn raw(&self, mut handle: Handle, env: &MachineEnvironment<'_>) -> Result<Raw, ValueError> {
        let mut override_aggregate = None;
        for _ in 0..MAXIMUM_DEPTH {
            let value = *self.store().values.get(handle).ok_or_else(|| refusal("express.foreign-value"))?;
            let raw = match value {
                Value::Tagged { base, aggregate, .. } => {
                    if override_aggregate.is_none() {
                        override_aggregate = aggregate;
                    }
                    handle = base;
                    continue;
                }
                Value::Qualified { base, .. } => {
                    handle = base;
                    continue;
                }
                Value::Local { view, .. } => Raw::Entity(Entity::View(view)),
                Value::Missing => Raw::Missing,
                Value::Logical(value) => Raw::Logical(value),
                Value::Integer(value) => Raw::Integer(value),
                Value::Real(value) => Raw::Real(value),
                Value::String(text) => Raw::String(text),
                Value::Binary(text) => Raw::Binary(text),
                Value::Enum(text) => Raw::Enum(text),
                Value::Entity(entity) => Raw::Entity(entity),
                Value::Aggregate(value) => Raw::Aggregate { cursor: Cursor { source: false, next: value.first }, kind: value.kind, lower: value.lower, length: value.length },
                Value::Source(mut node) => {
                    let arena = env.receiver.and_then(PopulationReceiver::arena).ok_or_else(|| refusal("express.runtime-source-arena"))?;
                    let mut source = env.source_value(node)?;
                    for _ in 0..MAXIMUM_DEPTH {
                        if !matches!(source, IfcValue::TypedValue(typed) if typed.items.len() == 1) {
                            break;
                        }
                        node = arena.nodes[node].first_edge.and_then(|edge| arena.edges.get(edge)).map(|edge| edge.value).ok_or_else(|| refusal("express.runtime-source-typed"))?;
                        source = env.source_value(node)?;
                    }
                    match source {
                        IfcValue::Unset | IfcValue::Derived => Raw::Missing,
                        IfcValue::Integer(value) => Raw::Integer(*value as i128),
                        IfcValue::Real(value) => Raw::Real(*value),
                        IfcValue::String(_) => Raw::String(Text::Source(node)),
                        IfcValue::Binary(_) => Raw::Binary(Text::Source(node)),
                        IfcValue::Enum(value) if value == "T" => Raw::Logical(Logical::True),
                        IfcValue::Enum(value) if value == "F" => Raw::Logical(Logical::False),
                        IfcValue::Enum(value) if value == "U" => Raw::Logical(Logical::Unknown),
                        IfcValue::Enum(_) => Raw::Enum(Text::Source(node)),
                        IfcValue::Reference(id) => Raw::Entity(Entity::Original(env.overlay.and_then(|overlay| overlay.find(*id)).ok_or_else(|| refusal("express.runtime-reference"))?)),
                        IfcValue::Aggregate(items) => {
                            let expression = self.source_aggregate_expression(node)?;
                            let kind = aggregate_kind(expression.and_then(|expression| expression.aggregate));
                            let lower = if kind == AggregateKind::Array { expression.and_then(|expression| expression.lower).unwrap_or(1) as i128 } else { 1 };
                            Raw::Aggregate { cursor: Cursor { source: true, next: arena.nodes[node].first_edge }, kind, lower, length: items.len() }
                        }
                        IfcValue::TypedValue(_) => return Err(refusal("express.runtime-typed-arity")),
                    }
                }
            };
            return Ok(match (raw, override_aggregate) {
                (Raw::Aggregate { cursor, length, .. }, Some((kind, lower))) => Raw::Aggregate { cursor, kind, lower, length },
                _ => raw,
            });
        }
        Err(refusal("express.depth"))
    }
    fn text<'a>(&'a self, text: Text, env: &'a MachineEnvironment<'_>) -> Result<&'a str, ValueError> {
        match text {
            Text::Static(value) => Ok(value),
            Text::Owned(index) => self.store().texts.get(index).map(String::as_str).ok_or_else(|| refusal("express.runtime-text")),
            Text::Source(node) => match env.source_value(node)? {
                IfcValue::String(text) | IfcValue::Enum(text) | IfcValue::Binary(text) => Ok(text),
                _ => Err(refusal("express.runtime-source-text")),
            },
        }
    }
    fn truth(&self, handle: Handle, env: &MachineEnvironment<'_>) -> Result<Logical, ValueError> {
        Ok(if let Raw::Logical(value) = self.raw(handle, env)? { value } else { Logical::Unknown })
    }
    fn integer(&self, handle: Handle, env: &MachineEnvironment<'_>) -> Result<Option<i128>, ValueError> {
        Ok(if let Raw::Integer(value) = self.raw(handle, env)? { Some(value) } else { None })
    }
    /// 🧭 Execute one continuation or one exact native table growth without recursive semantic calls.
    pub fn advance(&mut self, env: &MachineEnvironment<'_>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if matches!(self.initial, Some(Initial::Local(_))) {
            return Err(refusal("express.local-authority-required").into());
        }
        self.advance_inner(env, cx)
    }
    pub(super) fn advance_local(&mut self, env: &MachineEnvironment<'_>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if !matches!(self.initial, Some(Initial::Local(_))) {
            return Err(refusal("express.local-not-started").into());
        }
        self.advance_inner(env, cx)
    }
    fn advance_inner(&mut self, env: &MachineEnvironment<'_>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.closing || self.store.is_none() {
            return Err(refusal("express.runtime-not-live").into());
        }
        let identity = env.identity()?;
        if self.environment.is_some_and(|original| original != identity) {
            return Err(refusal("express.foreign-value").into());
        }
        if self.failed && !self.population_finished {
            return Err(refusal("express.runtime-failed").into());
        }
        if self.population_finished || self.initialized && self.store().tasks.is_empty() {
            return Ok(admitted(cx, fixed(0))?);
        }
        if self.work >= 10_000_000 {
            return Err(refusal("express.work-budget").into());
        }
        let demand = self.demands()?;
        if !admitted(cx, demand)? {
            return Ok(false);
        }
        if self.environment.is_none() {
            self.environment = Some(identity);
            debit(cx, demand)?;
            return Ok(false);
        }
        if let Some((index, _)) = self.next_growth()? {
            self.reserve_table(index);
            debit(cx, demand)?;
            return Ok(false);
        }
        let action = if !self.initialized {
            self.initialize()
        } else if let Some(frame) = self.store_mut().tasks.pop() {
            self.execute(frame, env)
        } else {
            Ok(())
        };
        debit(cx, demand)?;
        self.work += 1;
        if let Err(error) = action {
            self.failed = true;
            if matches!(self.initial, Some(Initial::Population | Initial::Local(_))) {
                let code = match &error.message {
                    std::borrow::Cow::Borrowed(code) => *code,
                    std::borrow::Cow::Owned(_) => return Err(error.into()),
                };
                self.population_complete = false;
                self.population_valid = false;
                self.store_mut().diagnostics.push(RetainedRuleRecord { owner: None, rule: None, kind: RetainedRuleKind::Admission, outcome: RetainedRuleOutcome::Incomplete, entity_id: None, offset: None, code: Some(code) });
                self.store_mut().tasks.clear();
                self.flow = Flow::None;
                self.population_finished = true;
            } else {
                return Err(error.into());
            }
        }
        Ok(self.population_finished || self.initialized && self.store().tasks.is_empty())
    }
    fn initialize(&mut self) -> Result<(), ValueError> {
        let scope = self.scope(None);
        match self.initial.ok_or_else(|| refusal("express.runtime-initial"))? {
            Initial::Expression(expression) => {
                self.push(Frame::eval(expression, scope, 1))?;
                self.initialized = true;
            }
            Initial::Call(name, arguments) => {
                let builder = self.make(Value::Aggregate(Aggregate::empty(AggregateKind::List)));
                let frame = Frame { name, a: builder, position: 0, stage: 100, ..Frame::new(Kind::CallArgs, scope, 1) };
                if arguments.is_empty() {
                    self.push(Frame { stage: 0, kind: Kind::Call, ..frame })?;
                } else {
                    self.push(frame)?;
                }
                self.initialized = true;
            }
            Initial::Population | Initial::Local(_) => {
                if self.program.schema != "IFC4X3_ADD2" {
                    return Err(refusal("express.program-schema"));
                }
                if std::ptr::eq(self.program, super::ifc4x3_express_program()) && super::program_index::PROGRAM_SHA256 != super::EXPRESS_PROGRAM_SHA256 {
                    return Err(refusal("express.program-index-stale"));
                }
                let frame = match self.initial {
                    Some(Initial::Local(row)) => Frame { a: row, b: 0, identity: true, ..Frame::new(Kind::Population, scope, 1) },
                    _ => Frame::new(Kind::Population, scope, 1),
                };
                self.push(frame)?;
                self.push(Frame::new(Kind::Normalize, scope, 1))?;
                self.push(Frame::new(Kind::Header, scope, 1))?;
                self.initialized = true;
            }
            #[cfg(test)]
            Initial::Prepared(expression, _) => {
                self.push(Frame { expression: Some(expression), ..Frame::new(Kind::PreparedQuery, scope, 1) })?;
                self.push(Frame::new(Kind::Normalize, scope, 1))?;
                self.push(Frame::new(Kind::Header, scope, 1))?;
                self.initialized = true;
            }
        }
        Ok(())
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
    }
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.retirement.next_owner_demands::<Store>(self.store.is_some(), &None, body)
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Err(refusal("express.runtime-not-closing"));
        }
        if self.store.is_some() {
            self.retirement.begin_granted(&mut self.store, grant)?.ok_or_else(|| refusal("express.runtime-close-owner"))
        } else if !self.retirement.is_empty() {
            self.retirement.step_granted(grant)
        } else {
            Ok(RetainedCloneStep::Complete(Default::default()))
        }
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.store.is_none() && self.retirement.is_empty()
    }
}
impl Default for RetainedMachine {
    fn default() -> Self {
        Self::new(super::ifc4x3_express_program())
    }
}

include!("machine-expression.rs");
include!("machine-operation.rs");
include!("machine-statement.rs");
include!("machine-population.rs");
include!("machine-domain.rs");
include!("machine-rules.rs");
include!("machine-root.rs");
#[cfg(test)]
#[path = "machine-local-tests.rs"]
mod local_tests;

#[cfg(test)]
#[path = "machine-custody-tests.rs"]
mod custody_tests;

#[cfg(test)]
#[path = "machine-aggregate-tests.rs"]
mod aggregate_tests;

#[cfg(test)]
#[path = "machine-typeof-tests.rs"]
mod typeof_tests;

#[cfg(test)]
#[path = "machine-root-tests.rs"]
mod root_tests;

pub enum RetainedScalar<'a> {
    Missing,
    Logical(Option<bool>),
    Integer(i128),
    Real(f64),
    String(&'a str),
    Binary(&'a str),
    Enumeration(&'a str),
    Aggregate,
    Entity,
}
impl RetainedScalar<'_> {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Missing => "unknown",
            Self::Logical(_) => "logical",
            Self::Integer(_) => "integer",
            Self::Real(_) => "real",
            Self::String(_) => "string",
            Self::Binary(_) => "binary",
            Self::Enumeration(_) => "enum",
            Self::Aggregate => "aggregate",
            Self::Entity => "entity",
        }
    }
    pub fn integer(&self) -> Option<i128> {
        if let Self::Integer(value) = self { Some(*value) } else { None }
    }
    #[cfg(test)]
    pub fn text(&self) -> String {
        match self {
            Self::Logical(value) => match value {
                Some(true) => "true",
                Some(false) => "false",
                None => "unknown",
            }
            .into(),
            Self::Integer(value) => value.to_string(),
            Self::Real(value) => value.to_string(),
            Self::String(value) | Self::Binary(value) | Self::Enumeration(value) => (*value).into(),
            _ => String::new(),
        }
    }
}
impl RetainedMachine {
    /// 👁 Borrow a completed scalar without allocating or transferring its original arena owner.
    pub fn result_scalar<'a>(&'a self, env: &'a MachineEnvironment<'_>) -> Result<RetainedScalar<'a>, ValueError> {
        if self.closing || self.failed || self.store.is_none() || self.environment != Some(env.identity()?) || !self.initialized || !self.store().tasks.is_empty() {
            return Err(refusal("express.runtime-result-not-ready"));
        }
        let value = *self.store().results.last().ok_or_else(|| refusal("express.runtime-result-stack"))?;
        Ok(match self.raw(value, env)? {
            Raw::Missing => RetainedScalar::Missing,
            Raw::Logical(value) => RetainedScalar::Logical(match value {
                Logical::True => Some(true),
                Logical::False => Some(false),
                _ => None,
            }),
            Raw::Integer(value) => RetainedScalar::Integer(value),
            Raw::Real(value) => RetainedScalar::Real(value),
            Raw::String(text) => RetainedScalar::String(self.text(text, env)?),
            Raw::Binary(text) => RetainedScalar::Binary(self.text(text, env)?),
            Raw::Enum(text) => RetainedScalar::Enumeration(self.text(text, env)?),
            Raw::Aggregate { .. } => RetainedScalar::Aggregate,
            Raw::Entity(_) => RetainedScalar::Entity,
        })
    }
}
