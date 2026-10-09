//! 🌊️ Validated borrowed Flow entities and explicitly owned neural reconstruction.
use crate::*;
use neural::{Atom, ColdDictionaryBuilder, ColdOwner, ColdRetire, Dictionary, Neuron, Tree, Value};
use semio_framework_value::{NativeDecodeControl, ValueError, ValueRefusalKind};

use store::sqlite_snapshot::{
    artifact::{FloatColumn, FloatRow, Reconstruction},
    SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue,
};
const SQL: &str = include_str!("../🗄️.sql");
const NUMBER: &[FloatColumn] = &[FloatColumn::Binary64(2)];
const WIDTHS: &[(&str, usize)] = &[
    ("flow_document", 11),
    ("flow_widget", 5),
    ("flow_neuron_widget", 5),
    ("flow_widget_port", 5),
    ("flow_slider_widget", 15),
    ("flow_note_widget", 3),
    ("flow_image_widget", 3),
    ("flow_variable_widget", 4),
    ("flow_preview_widget", 3),
    ("flow_widget_expanded", 4),
    ("flow_action_widget", 3),
    ("flow_export_widget", 3),
    ("flow_cluster_widget", 5),
    ("flow_synapse", 8),
    ("flow_layout", 10),
    ("flow_gui", 10),
    ("flow_gui_node", 11),
    ("flow_chrome_plain", 3),
    ("flow_chrome_slider", 15),
    ("flow_chrome_note", 3),
    ("flow_chrome_image", 3),
    ("flow_chrome_variable", 4),
    ("flow_gui_preview", 6),
    ("flow_preview_source", 4),
    ("flow_preview_layout", 8),
    ("flow_gui_expanded", 4),
    ("flow_tree", 1),
    ("flow_tree_neuron", 7),
    ("flow_tree_synapse", 8),
    ("flow_dictionary", 1),
    ("flow_dictionary_entry", 5),
    ("flow_neural_value", 2),
    ("flow_neural_boolean", 3),
    ("flow_neural_integer", 3),
    ("flow_neural_decimal", 5),
    ("flow_neural_text", 3),
    ("flow_neural_dictionary", 3),
];
type Owners<'a> = Vec<&'a SqliteRow>;
struct Key<'a>{table:&'static str,id:i64,row:&'a SqliteRow,used:bool}
struct Group<'a>{table:&'static str,column:usize,rows:Owners<'a>}
pub(super) struct Read<'a,'c,'p>{database:&'a SqliteDatabase,control:&'c mut SqliteSnapshotControl<'p>,keys:Vec<Key<'a>>,groups:Vec<Group<'a>>,work:usize}
impl<'a,'c,'p> Read<'a,'c,'p>{
 pub(super) fn new(database:&'a SqliteDatabase,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{
  store::sqlite_snapshot::validate_sqlite_database_schema_controlled(database,SQL,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
  control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
  let count=WIDTHS.iter().try_fold(0usize,|sum,(name,_)|sum.checked_add(database.table(name)?.rows.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Flow entity count overflow")))?;
  let keys=store::sqlite_snapshot::transfer::reserve(count,control)?;
  let groups=store::sqlite_snapshot::transfer::reserve(WIDTHS.len(),control)?;
  let mut read=Self{database,control,keys,groups,work:0};
  for &(table,width)in WIDTHS{for row in &database.table(table)?.rows{if row.rowid<=0||row.values.len()!=width||row.integer(0)?!=row.rowid{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow entity identity or width differs"));}read.keys.push(Key{table,id:row.rowid,row,used:false});read.step()?;}}
  read.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,read.work,0)?;
  read.keys.sort_unstable_by_key(|key|(key.table,key.id));
  for(index,key)in read.keys.iter().enumerate(){if index>0&&(key.table,key.id)==(read.keys[index-1].table,read.keys[index-1].id){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow entity identity repeated"));}}
  Ok(read)
 }
 fn step(&mut self)->Result<(),ValueError>{self.work=self.work.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Flow reconstruction work overflow"))?;if self.work%256==0{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.work,0)?;}Ok(())}
 fn push<T>(&mut self,values:&mut Vec<T>,value:T)->Result<(),ValueError>{if values.len()==values.capacity(){let count=values.capacity().max(1).checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Flow frontier capacity overflow"))?;let mut next=self.allocate(count)?;let total=values.len();for(index,value)in values.drain(..).enumerate(){next.push(value);if(index+1)%256==0{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index+1,total)?;}}*values=next;}values.push(value);Ok(())}
 pub(super) fn take(&mut self,table:&'static str,id:i64)->Result<&'a SqliteRow,ValueError>{let index=self.keys.binary_search_by_key(&(table,id),|key|(key.table,key.id)).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Flow entity missing"))?;if self.keys[index].used{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow entity has repeated ownership or a cycle"));}self.keys[index].used=true;let row=self.keys[index].row;self.step()?;Ok(row)}
 fn relations(&mut self,table:&'static str,column:usize,owner:i64)->Result<Vec<&'a SqliteRow>,ValueError>{
  let index=match self.groups.iter().position(|group|group.table==table&&group.column==column){Some(index)=>index,None=>{
   let source=&self.database.table(table)?.rows;let mut rows=self.allocate(source.len())?;
   for row in source{if row.integer(column)?<=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow relationship owner must be positive"));}rows.push(row);self.step()?;}
   self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.work,0)?;
   rows.sort_unstable_by_key(|row|row.integer(column).expect("validated Flow owner word"));
   if self.groups.len()==self.groups.capacity(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Flow relationship schema index exceeds authored table count"));}
   self.groups.push(Group{table,column,rows});self.groups.len()-1
  }};
  let begin=self.groups[index].rows.partition_point(|row|row.integer(column).expect("validated Flow owner word")<owner);
  let end=self.groups[index].rows.partition_point(|row|row.integer(column).expect("validated Flow owner word")<=owner);
  let mut rows=self.allocate(end-begin)?;
  for position in begin..end{let row=self.groups[index].rows[position];self.take(table,row.rowid)?;rows.push(row);}
  Ok(rows)
 }
 pub(super) fn child(&mut self,table:&'static str,owner:i64)->Result<&'a SqliteRow,ValueError>{let mut rows=self.relations(table,1,owner)?;if rows.len()!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow required variant entity differs"));}Ok(rows.pop().unwrap())}
 pub(super) fn optional_child(&mut self,table:&'static str,owner:i64)->Result<Option<&'a SqliteRow>,ValueError>{let mut rows=self.relations(table,1,owner)?;if rows.len()>1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow optional entity has repeated owners"));}Ok(rows.pop())}
 pub(super) fn ordered(&mut self,table:&'static str,column:usize,owner:i64,ordinal:usize)->Result<Vec<&'a SqliteRow>,ValueError>{let mut rows=self.relations(table,column,owner)?;for row in &rows{if row.integer(ordinal)?<0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow ordinal is negative"));}self.step()?;}rows.sort_unstable_by_key(|row|row.integer(ordinal).expect("validated Flow ordinal"));for(index,row)in rows.iter().enumerate(){if row.integer(ordinal)?!=i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Flow ordinal exceeds signed64"))?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow ordinals must be contiguous and unique"));}self.step()?;}Ok(rows)}
    pub(super) fn text(&mut self, row: &SqliteRow, column: usize) -> Result<String, ValueError> {
        Reconstruction::new(self.control)?.text(row.text(column)?)
    }
    pub(super) fn optional_text(&mut self, row: &SqliteRow, column: usize) -> Result<Option<String>, ValueError> {
        row.optional_text(column)?.map(|value| Reconstruction::new(self.control)?.text(value)).transpose()
    }
    pub(super) fn boolean(&mut self, row: &SqliteRow, column: usize) -> Result<bool, ValueError> {
        self.step()?;
        match row.integer(column)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow boolean is not zero or one")),
        }
    }
    pub(super) fn optional_boolean(&mut self, row: &SqliteRow, column: usize) -> Result<Option<bool>, ValueError> {
        match row.values.get(column) {
            Some(SqliteValue::Null) => Ok(None),
            Some(SqliteValue::Integer(_)) => self.boolean(row, column).map(Some),
            _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow optional boolean storage differs")),
        }
    }
    pub(super) fn unsigned(&mut self, row: &SqliteRow, column: usize) -> Result<u64, ValueError> {
        self.step()?;
        let high = u32::try_from(row.integer(column)?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Flow unsigned high word exceeds32bits"))?;
        let low = u32::try_from(row.integer(column + 1)?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Flow unsigned low word exceeds32bits"))?;
        Ok((u64::from(high) << 32) | u64::from(low))
    }
    pub(super) fn finish(self) -> Result<(),ValueError>{for key in &self.keys{if !key.used{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow unowned entity remains"));}}self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.work,self.work)}
}
struct Slots<T>{values:Vec<(i64,T)>,retire:fn(T)}
impl<T> Slots<T>{
 fn new(retire:fn(T))->Self{Self{values:Vec::new(),retire}}
 fn is_empty(&self)->bool{self.values.is_empty()}
 fn remove(&mut self,id:&i64)->Option<T>{self.values.binary_search_by_key(id,|(id,_)|*id).ok().map(|index|self.values.remove(index).1)}
 fn insert(&mut self,id:i64,value:T,control:&mut SqliteSnapshotControl<'_>)->Result<Option<T>,ValueError>{
  let value=Owned::new(value,self.retire);
  match self.values.binary_search_by_key(&id,|(id,_)|*id){Ok(index)=>Ok(Some(std::mem::replace(&mut self.values[index].1,value.take()))),Err(index)=>{
   if self.values.len()==self.values.capacity(){let capacity=self.values.capacity().max(1).checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Flow owned frontier capacity overflow"))?;let mut next=Self{values:store::sqlite_snapshot::transfer::reserve(capacity,control)?,retire:self.retire};let total=self.values.len();for(index,value)in self.values.drain(..).enumerate(){next.values.push(value);if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index+1,total)?;}}self.values=std::mem::take(&mut next.values);}
   self.values.insert(index,(id,value.take()));Ok(None)
  }}
 }
}
impl<T> Drop for Slots<T>{fn drop(&mut self){for(_,value)in self.values.drain(..){(self.retire)(value);}}}
struct Dictionaries(Slots<Dictionary>);
struct Values(Slots<Value>);
enum DictionaryFrame<'a> {
    Dictionary(i64),
    Value(i64),
    Assemble(i64, Vec<&'a SqliteRow>),
    Wrap(i64, i64),
}
impl<'a, 'c, 'p> Read<'a, 'c, 'p> {
    fn dictionary(&mut self, root: i64) -> Result<Dictionary, ValueError> {
        let mut pending = self.allocate(1)?; pending.push(DictionaryFrame::Dictionary(root));
        let mut dictionaries = Dictionaries(Slots::new(Dictionary::retire_cold));
        let mut values = Values(Slots::new(Value::retire_cold));
        while let Some(frame) = pending.pop() {
            self.step()?;
            match frame {
                DictionaryFrame::Dictionary(id) => {
                    self.take("flow_dictionary", id)?;
                    let rows = self.ordered("flow_dictionary_entry", 1, id, 2)?;
                    let mut children = self.allocate(rows.len())?; for row in &rows { children.push(row.integer(4)?); self.step()?; }
                    self.push(&mut pending,DictionaryFrame::Assemble(id, rows))?;
                    for child in children.into_iter().rev(){self.push(&mut pending,DictionaryFrame::Value(child))?;}
                }
                DictionaryFrame::Value(id) => {
                    let row = self.take("flow_neural_value", id)?;
                    let value = match row.text(1)? {
                        "null" => Value::Atom(Atom::Null),
                        "boolean" => {
                            let body = self.child("flow_neural_boolean", id)?;
                            Value::Atom(Atom::Boolean(self.boolean(body, 2)?))
                        }
                        "integer" => {
                            let body = self.child("flow_neural_integer", id)?;
                            Value::Atom(Atom::Integer(body.integer(2)?))
                        }
                        "decimal" => {
                            let body = self.child("flow_neural_decimal", id)?;
                            Value::Atom(Atom::Decimal(FloatRow::new(body, NUMBER)?.real(2)?))
                        }
                        "text" => {
                            let body = self.child("flow_neural_text", id)?;
                            Value::Atom(Atom::String(self.text(body, 2)?))
                        }
                        "dictionary" => {
                            let body = self.child("flow_neural_dictionary", id)?;
                            let child = body.integer(2)?;
                            self.push(&mut pending,DictionaryFrame::Wrap(id, child))?;
                            self.push(&mut pending,DictionaryFrame::Dictionary(child))?;
                            continue;
                        }
                        _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow neural value variant differs")),
                    };
                    if let Some(value) = values.0.insert(id, value, self.control)? {
                        value.retire_cold();
                        return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow value reused"));
                    }
                }
                DictionaryFrame::Wrap(id, child) => {
                    let dictionary = dictionaries.0.remove(&child).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Flow dictionary frontier missing"))?;
                    if let Some(value) = values.0.insert(id, Value::Dictionary(dictionary), self.control)? {
                        value.retire_cold();
                        return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow value reused"));
                    }
                }
                DictionaryFrame::Assemble(id, entries) => {
                    let mut builder = ColdDictionaryBuilder::new();
                    for row in entries {
                        let key = self.text(row, 3)?;
                        let value = ColdOwner::new(values.0.remove(&row.integer(4)?).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Flow value frontier missing"))?);

                        self.control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,checkpoint,allocation|{
                            let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(event.completed,event.total);
                            let mut native_allocation=|request:semio_framework_value::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=NativeDecodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);
                            let prior=builder.dictionary().len();
                            let result=builder.insert_controlled(key,value.into_inner(),&mut native).and_then(|_|if builder.dictionary().len()==prior{Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow dictionary key repeated"))}else{Ok(())});
                            (result,native.owned_bytes())
                        })??;

                    }
                    if let Some(value) = dictionaries.0.insert(id, builder.finish(), self.control)? {
                        value.retire_cold();
                        return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow dictionary reused"));
                    }
                }
            }
        }
        if !values.0.is_empty() {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow detached neural value remains"));
        }
        let result = ColdOwner::new(dictionaries.0.remove(&root).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Flow root dictionary missing"))?);
        if !dictionaries.0.is_empty() {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow detached dictionary remains"));
        }
        Ok(result.into_inner())
    }
}
struct Owned<T> {
    value: Option<T>,
    retire: fn(T),
}
impl<T> Owned<T> {
    fn new(value: T, retire: fn(T)) -> Self {
        Self { value: Some(value), retire }
    }
    fn take(mut self) -> T {
        self.value.take().unwrap()
    }
}
impl<T> std::ops::Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.value.as_ref().unwrap()
    }
}
impl<T> std::ops::DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.as_mut().unwrap()
    }
}
impl<T> Drop for Owned<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            (self.retire)(value);
        }
    }
}
fn retire_tree(tree: Tree) {
    crate::retained::FlowRetirement::from_owner(crate::retained::FlowOwner::Tree(tree)).retire_cold();
}
fn retire_neuron(neuron: Neuron) {
    neural::ColdRetire::retire_cold(neuron);
}
struct Trees(Slots<Tree>);
enum TreeFrame<'a> {
    Enter(i64),
    Assemble(i64, Vec<&'a SqliteRow>, Vec<&'a SqliteRow>),
}
impl<'a, 'c, 'p> Read<'a, 'c, 'p> {
    fn allocate<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.work,0)?;store::sqlite_snapshot::transfer::reserve(count,self.control)}
    fn synapse(&mut self, row: &SqliteRow) -> Result<neural::Synapse, ValueError> {
        Ok(neural::Synapse { id: self.text(row, 3)?, from: self.text(row, 4)?, to: self.text(row, 5)?, from_port: self.text(row, 6)?, to_port: self.text(row, 7)? })
    }
    fn tree(&mut self, root: i64) -> Result<Tree, ValueError> {
        let mut pending = self.allocate(1)?; pending.push(TreeFrame::Enter(root));
        let mut trees = Trees(Slots::new(retire_tree));
        while let Some(frame) = pending.pop() {
            self.step()?;
            match frame {
                TreeFrame::Enter(id) => {
                    self.take("flow_tree", id)?;
                    let neurons = self.ordered("flow_tree_neuron", 1, id, 2)?;
                    let synapses = self.ordered("flow_tree_synapse", 1, id, 2)?;
                    let mut children = self.allocate(neurons.len())?;
                    for neuron in &neurons {
                        match neuron.values.get(6) {
                            Some(SqliteValue::Null) => {}
                            Some(SqliteValue::Integer(id)) if *id > 0 => children.push(*id),
                            _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow optional child tree identity differs")),
                        }
                    }
                    self.push(&mut pending,TreeFrame::Assemble(id, neurons, synapses))?;
                    for child in children.into_iter().rev(){self.push(&mut pending,TreeFrame::Enter(child))?;}
                }
                TreeFrame::Assemble(id, neurons, synapses) => {
                    let mut tree = Owned::new(Tree { neurons: self.allocate(neurons.len())?, synapses: self.allocate(synapses.len())? }, retire_tree);
                    for row in neurons {
                        let mut neuron = Owned::new(Neuron { id: self.text(row, 3)?, kind: self.text(row, 4)?, params: Dictionary::new(), tree: None }, retire_neuron);
                        neuron.params = self.dictionary(row.integer(5)?)?;
                        if let Some(SqliteValue::Integer(child)) = row.values.get(6) {
                            let child = Owned::new(trees.0.remove(child).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Flow child tree frontier missing"))?, retire_tree);
                            self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, self.work, 0)?;
                            self.control.admit_allocation_bytes(std::mem::size_of::<Tree>())?;
                            neuron.tree = Some(Box::new(child.take()));
                        }
                        tree.neurons.push(neuron.take());
                        self.step()?;
                    }
                    for row in synapses {
                        tree.synapses.push(self.synapse(row)?);
                        self.step()?;
                    }
                    if let Some(tree) = trees.0.insert(id, tree.take(), self.control)? {
                        retire_tree(tree);
                        return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow tree reused"));
                    }
                }
            }
        }
        let root = Owned::new(trees.0.remove(&root).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Flow root tree missing"))?, retire_tree);
        if !trees.0.is_empty() {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow detached tree remains"));
        }
        Ok(root.take())
    }
}
fn retire_ordered<V>(map: OrderedMap<V>, retire: fn(V)) {
    use semio_framework_value::{retained_clone::RetainedCloneGrant,ordered::RetirementStep};
    let mut cursor = map.retire();
    loop {
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:cursor.next_close_byte_demand().expect("Flow ordered release demand"),maximum_depth:cursor.next_depth_demand()};
        match cursor.advance(grant) {
            RetirementStep::OwnedValue(value) => retire(value),
            RetirementStep::Complete => break,
            RetirementStep::Progress { .. } | RetirementStep::ProcessedBytes(_) => {},
            RetirementStep::Failure(error)=>panic!("Flow ordered release refused: {error}"),
            RetirementStep::Blocked => unreachable!("Flow ordered release grants exact demand"),
        }
    }
}
impl<'a, 'c, 'p> Read<'a, 'c, 'p> {
    fn insert_ordered<V>(&mut self,map:&mut OrderedMap<V>,key:String,value:V,retire:fn(V))->Result<(),ValueError>{
        use semio_framework_value::{retained_clone::RetainedCloneGrant,ordered::RetirementStep};
        let value=Owned::new(value,retire);
        self.control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,checkpoint,allocation|{
            let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(event.completed,event.total);
            let mut native_allocation=|request:semio_framework_value::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=NativeDecodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);
            let result=(||->Result<(),ValueError>{
                native.charge(std::mem::size_of::<String>()+std::mem::size_of::<V>()+4*std::mem::size_of::<usize>())?;
                let mut cursor=map.begin_set(key,value.take());
                let result=(||->Result<(),ValueError>{while !cursor.is_complete(){let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:cursor.next_capacity_byte_demand()?,maximum_release_bytes:0,maximum_depth:cursor.next_depth_demand()};cursor.advance_insert_controlled(grant,&mut native)?;}let next=cursor.take_result().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Flow ordered result missing"))?;if next.len()<=map.len(){retire_ordered(next,retire);return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow ordered map key repeated"));}retire_ordered(std::mem::replace(map,next),retire);Ok(())})();
                cursor.begin_close();loop{let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:cursor.next_close_byte_demand().expect("Flow ordered close release demand"),maximum_depth:cursor.next_close_depth_demand()};match cursor.close_step(grant){RetirementStep::OwnedValue(value)=>retire(value),RetirementStep::Complete=>break,RetirementStep::Failure(error)=>panic!("Flow ordered close refused: {error}"),_=>{}}}
                assert!(cursor.terminal_is_empty());result
            })();
            (result,native.owned_bytes())
        })?
    }
    fn paths(&mut self, table: &'static str, owner: i64) -> Result<OrderedSet, ValueError> {
        let mut map = Owned::new(OrderedMap::new(), |map| retire_ordered(map, std::mem::drop));
        for row in self.ordered(table, 1, owner, 2)? {
            let key = self.text(row, 3)?;
            self.insert_ordered(&mut map, key, (), std::mem::drop)?;
        }
        Ok(OrderedSet::from_map(map.take()))
    }
    fn chrome(&mut self, row: &SqliteRow) -> Result<NodeChrome, ValueError> {
        Ok(match row.text(6)? {
            "plain" => {
                let body = self.child("flow_chrome_plain", row.rowid)?;
                NodeChrome::Plain { preview: self.boolean(body, 2)? }
            }
            "slider" => {
                let body = self.child("flow_chrome_slider", row.rowid)?;
                let words = FloatRow::new(body, &[FloatColumn::Binary64(3), FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6)])?;
                NodeChrome::Slider { label: self.text(body, 2)?, min: words.real(3)?, max: words.real(4)?, step: words.real(5)?, value: words.real(6)? }
            }
            "note" => {
                let body = self.child("flow_chrome_note", row.rowid)?;
                NodeChrome::Note { text: self.text(body, 2)? }
            }
            "image" => {
                let body = self.child("flow_chrome_image", row.rowid)?;
                NodeChrome::Image { src: self.text(body, 2)? }
            }
            "variable" => {
                let body = self.child("flow_chrome_variable", row.rowid)?;
                NodeChrome::Variable { name: self.text(body, 2)?, schema: self.text(body, 3)? }
            }
            _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow GUI chrome variant differs")),
        })
    }
    fn gui(&mut self, id: i64) -> Result<FlowUi, ValueError> {
        let row = self.take("flow_gui", id)?;
        let camera = FloatRow::new(row, &[FloatColumn::Binary64(1), FloatColumn::Binary64(2), FloatColumn::Binary64(3)])?;
        let mut gui = Owned::new(FlowUi { camera: CameraJson { x: camera.real(1)?, y: camera.real(2)?, zoom: camera.real(3)? }, nodes: OrderedMap::new(), previews: Vec::new() }, FlowUi::retire_cold);
        for row in self.ordered("flow_gui_node", 1, id, 2)? {
            let key = self.text(row, 3)?;
            let words = FloatRow::new(row, &[FloatColumn::Binary64(4), FloatColumn::Binary64(5)])?;
            let node = FlowNodeGui { layout: WidgetLayout { x: words.real(4)?, y: words.real(5)? }, chrome: self.chrome(row)? };
            self.insert_ordered(&mut gui.nodes, key, node, std::mem::drop)?;
        }
        let previews = self.ordered("flow_gui_preview", 1, id, 2)?;
        gui.previews = self.allocate(previews.len())?;
        for row in previews {
            let preview = FlowPreviewGui { id: self.text(row, 3)?, mode: self.text(row, 4)?, source: None, preview: Dictionary::new(), expanded: OrderedSet::new(), layout: None };
            gui.previews.push(preview);
            let preview = gui.previews.last_mut().unwrap();
            if let Some(source) = self.optional_child("flow_preview_source", row.rowid)? {
                preview.source = Some(FlowChannelRef { neuron: self.text(source, 2)?, channel: self.text(source, 3)? });
            }
            if let Some(layout) = self.optional_child("flow_preview_layout", row.rowid)? {
                let words = FloatRow::new(layout, &[FloatColumn::Binary64(2), FloatColumn::Binary64(3)])?;
                preview.layout = Some(WidgetLayout { x: words.real(2)?, y: words.real(3)? });
            }
            preview.preview = self.dictionary(row.integer(5)?)?;
            preview.expanded = self.paths("flow_gui_expanded", row.rowid)?;
        }
        Ok(gui.take())
    }
}
impl<'a, 'c, 'p> Read<'a, 'c, 'p> {
    fn ports(&mut self,owner:i64)->Result<(Vec<String>,Vec<String>),ValueError>{
        let rows=self.relations("flow_widget_port",1,owner)?;
        let mut inputs=0usize;let mut outputs=0usize;
        for row in &rows{match row.text(2)?{"input"=>inputs+=1,"output"=>outputs+=1,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow port side differs"))}if row.integer(3)?<0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow port ordinal negative"));}self.step()?;}
        let mut input_rows=self.allocate(inputs)?;let mut output_rows=self.allocate(outputs)?;
        for row in rows{if row.text(2)?=="input"{input_rows.push(row)}else{output_rows.push(row)}self.step()?;}
        let mut input=self.allocate(inputs)?;let mut output=self.allocate(outputs)?;
        for(rows,values)in [(&mut input_rows,&mut input),(&mut output_rows,&mut output)]{
            rows.sort_unstable_by_key(|row|row.integer(3).expect("validated Flow port ordinal"));
            for(index,row)in rows.iter().enumerate(){if row.integer(3)?!=i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Flow port ordinal exceeds signed64"))?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow port ordinals must be contiguous and unique"));}values.push(self.text(row,4)?);self.step()?;}
        }
        Ok((input,output))
    }
    fn widget(&mut self, row: &SqliteRow) -> Result<Widget, ValueError> {
        let id = self.text(row, 3)?;
        Ok(match row.text(4)? {
            "neuron" => {
                let body = self.child("flow_neuron_widget", row.rowid)?;
                let mut widget = Owned::new(Widget::Neuron { id, neuron_kind: self.text(body, 2)?, params: Dictionary::new(), input_ports: Vec::new(), output_ports: Vec::new(), preview: self.boolean(body, 4)? }, Widget::retire_cold);
                if let Widget::Neuron { params, input_ports, output_ports, .. } = &mut *widget {
                    *params = self.dictionary(body.integer(3)?)?;
                    let ports = self.ports(body.rowid)?;
                    *input_ports = ports.0;
                    *output_ports = ports.1;
                }
                widget.take()
            }
            "inputSlider" => {
                let body = self.child("flow_slider_widget", row.rowid)?;
                let words = FloatRow::new(body, &[FloatColumn::Binary64(3), FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6)])?;
                Widget::InputSlider { id, label: self.text(body, 2)?, value: words.real(3)?, min: words.real(4)?, max: words.real(5)?, step: words.real(6)? }
            }
            "inputNote" => {
                let body = self.child("flow_note_widget", row.rowid)?;
                Widget::InputNote { id, text: self.text(body, 2)? }
            }
            "inputImage" => {
                let body = self.child("flow_image_widget", row.rowid)?;
                Widget::InputImage { id, src: self.text(body, 2)? }
            }
            "variable" => {
                let body = self.child("flow_variable_widget", row.rowid)?;
                Widget::Variable { id, name: self.text(body, 2)?, schema: self.text(body, 3)? }
            }
            "outputPreview" => {
                let body = self.child("flow_preview_widget", row.rowid)?;
                let mut widget = Owned::new(Widget::OutputPreview { id, preview: Dictionary::new(), expanded: OrderedSet::new() }, Widget::retire_cold);
                if let Widget::OutputPreview { preview, expanded, .. } = &mut *widget {
                    *preview = self.dictionary(body.integer(2)?)?;
                    *expanded = self.paths("flow_widget_expanded", body.rowid)?;
                }
                widget.take()
            }
            "outputAction" => {
                let body = self.child("flow_action_widget", row.rowid)?;
                Widget::OutputAction { id, action: self.text(body, 2)? }
            }
            "outputExport" => {
                let body = self.child("flow_export_widget", row.rowid)?;
                Widget::OutputExport { id, format: self.text(body, 2)? }
            }
            "cluster" => {
                let body = self.child("flow_cluster_widget", row.rowid)?;
                let mut widget = Owned::new(Widget::Cluster { id, name: self.text(body, 2)?, tree: Tree::default(), flow: FlowUi::default() }, Widget::retire_cold);
                if let Widget::Cluster { tree, flow, .. } = &mut *widget {
                    *tree = self.tree(body.integer(3)?)?;
                    *flow = self.gui(body.integer(4)?)?;
                }
                widget.take()
            }
            _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Flow widget variant differs")),
        })
    }
}
pub(super) fn reconstruct(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<FlowHostSnapshot, ValueError> {
    let mut read = Read::new(database, control)?;
    let row = database.table("flow_document")?.single_row()?;
    let row = read.take("flow_document", row.rowid)?;
    let camera = FloatRow::new(row, &[FloatColumn::Binary64(2), FloatColumn::Binary64(3), FloatColumn::Binary64(4)])?;
    let widgets = read.ordered("flow_widget", 1, row.rowid, 2)?;
    let synapses = read.ordered("flow_synapse", 1, row.rowid, 2)?;
    let mut snapshot = Owned::new(
        FlowHostSnapshot {
            schema: read.text(row, 1)?,
            camera: CameraJson { x: camera.real(2)?, y: camera.real(3)?, zoom: camera.real(4)? },
            widgets: read.allocate(widgets.len())?,
            synapses: read.allocate(synapses.len())?,
            layout: OrderedMap::new(),
        },
        FlowHostSnapshot::retire_cold,
    );
    for widget in widgets {
        snapshot.widgets.push(read.widget(widget)?);
        read.step()?;
    }
    for row in synapses {
        snapshot.synapses.push(SynapseSpec { id: read.text(row, 3)?, from: read.text(row, 4)?, to: read.text(row, 5)?, from_port: read.text(row, 6)?, to_port: read.text(row, 7)? });
        read.step()?;
    }
    for row in read.ordered("flow_layout", 1, row.rowid, 2)? {
        let key = read.text(row, 3)?;
        let words = FloatRow::new(row, &[FloatColumn::Binary64(4), FloatColumn::Binary64(5)])?;
        read.insert_ordered(&mut snapshot.layout, key, WidgetLayout { x: words.real(4)?, y: words.real(5)? }, std::mem::drop)?;
    }
    read.finish()?;
    Ok(snapshot.take())
}
