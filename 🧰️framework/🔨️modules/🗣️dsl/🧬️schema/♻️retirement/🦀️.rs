//! ♻️ Finite Record ownership uses the neutral typed constructor and release authorities.
use super::{ExprValue,FieldSpec,FieldValue,RecordSpec,RecordSpecProducer,RecordValue,Shape,WireEdgeLabel,WireNode,WireValue};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,deferred,deferred_birth_bytes_for,leaf,leaf_birth_bytes,sequence,sequence_birth_bytes};

impl RetireOwned for RecordValue{
    fn retirement(self)->Box<dyn RetirementCursor>{self.fields.retirement()}
    fn retirement_birth_bytes(&self)->Option<usize>{self.fields.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool{true}
}
impl RetireOwned for FieldValue{
    fn retirement(self)->Box<dyn RetirementCursor>{match self{
        Self::Text(value)=>value.retirement(),Self::Bytes64(value)=>value.retirement(),Self::Tuple(value)|Self::List(value)=>value.retirement(),Self::Record(value)=>value.retirement(),Self::Block(value)=>value.retirement(),Self::Statements(value)=>value.retirement(),Self::Map(value)=>value.retirement(),Self::Value(value)=>value.retirement(),Self::Wire(value)=>value.retirement(),Self::Expr(value)=>value.retirement(),
        Self::Bool(_)|Self::Int(_)|Self::UInt(_)|Self::Float(_)|Self::Enum(_)|Self::Absent=>leaf(()),
    }}
    fn retirement_birth_bytes(&self)->Option<usize>{match self{
        Self::Text(value)=>value.retirement_birth_bytes(),Self::Bytes64(value)=>value.retirement_birth_bytes(),Self::Tuple(value)|Self::List(value)=>value.retirement_birth_bytes(),Self::Record(value)=>value.retirement_birth_bytes(),Self::Block(value)=>value.retirement_birth_bytes(),Self::Statements(value)=>value.retirement_birth_bytes(),Self::Map(value)=>value.retirement_birth_bytes(),Self::Value(value)=>value.retirement_birth_bytes(),Self::Wire(value)=>value.retirement_birth_bytes(),Self::Expr(value)=>value.retirement_birth_bytes(),
        Self::Bool(_)|Self::Int(_)|Self::UInt(_)|Self::Float(_)|Self::Enum(_)|Self::Absent=>Some(leaf_birth_bytes::<()>()),
    }}
    fn controlled_retirement_supported()->bool{true}
}
impl RetireOwned for ExprValue{
    fn retirement(self)->Box<dyn RetirementCursor>{match self{
        Self::Var(value)=>value.retirement(),Self::Neg(value)=>value.retirement(),Self::Binary(_,left,right)=>sequence(vec![deferred(left),deferred(right)]),Self::Call(name,items)=>sequence(vec![deferred(name),deferred(items)]),Self::Num(_)=>leaf(()),
    }}
    fn retirement_birth_bytes(&self)->Option<usize>{match self{
        Self::Var(value)=>value.retirement_birth_bytes(),Self::Neg(value)=>value.retirement_birth_bytes(),Self::Binary(_,left,right)=>sequence_birth_bytes(&[deferred_birth_bytes_for(left),deferred_birth_bytes_for(right)]),Self::Call(name,items)=>sequence_birth_bytes(&[deferred_birth_bytes_for(name),deferred_birth_bytes_for(items)]),Self::Num(_)=>Some(leaf_birth_bytes::<()>()),
    }}
    fn controlled_retirement_supported()->bool{true}
}
semio_framework_value::artifact_retire_struct!(WireNode{id,kind,port});
semio_framework_value::artifact_retire_struct!(WireEdgeLabel{id,kind});
semio_framework_value::artifact_retire_struct!(WireValue{from,edge,edge_label,properties});
semio_framework_value::artifact_retire_leaf!(RecordSpecProducer);

impl RetireOwned for Shape{
    fn retirement(self)->Box<dyn RetirementCursor>{match self{
        Self::Enum(items)=>items.retirement(),Self::Tuple(shape,_)|Self::List(shape)|Self::Block(shape)|Self::Map(shape)=>shape.retirement(),Self::Record(make)|Self::Table(make)=>make.retirement(),Self::Statements(items)=>items.retirement(),
        Self::Bool|Self::Int|Self::UInt|Self::Float|Self::Text|Self::Bytes64|Self::Value|Self::Wire|Self::Quantity(_)|Self::Angle(_)|Self::Ref(_)|Self::Coord(_)|Self::Dir|Self::Dim(_)|Self::Range|Self::Count|Self::Expr|Self::Embed(_)|Self::EmbedFrom(_)=>leaf(()),
    }}
    fn retirement_birth_bytes(&self)->Option<usize>{match self{
        Self::Enum(items)=>items.retirement_birth_bytes(),Self::Tuple(shape,_)|Self::List(shape)|Self::Block(shape)|Self::Map(shape)=>shape.retirement_birth_bytes(),Self::Record(make)|Self::Table(make)=>make.retirement_birth_bytes(),Self::Statements(items)=>items.retirement_birth_bytes(),
        Self::Bool|Self::Int|Self::UInt|Self::Float|Self::Text|Self::Bytes64|Self::Value|Self::Wire|Self::Quantity(_)|Self::Angle(_)|Self::Ref(_)|Self::Coord(_)|Self::Dir|Self::Dim(_)|Self::Range|Self::Count|Self::Expr|Self::Embed(_)|Self::EmbedFrom(_)=>Some(leaf_birth_bytes::<()>()),
    }}
    fn controlled_retirement_supported()->bool{true}
}
impl RetireOwned for FieldSpec{
    fn retirement(self)->Box<dyn RetirementCursor>{sequence(vec![deferred(self.key),deferred(self.shape)])}
    fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.key),deferred_birth_bytes_for(&self.shape)])}
    fn controlled_retirement_supported()->bool{true}
}
impl RetireOwned for RecordSpec{
    fn retirement(self)->Box<dyn RetirementCursor>{sequence(vec![deferred(self.keyword),deferred(self.fields)])}
    fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.keyword),deferred_birth_bytes_for(&self.fields)])}
    fn controlled_retirement_supported()->bool{true}
}
/// 📋️ Retains the original writer planning buffer until its physical backing is admitted.
pub(super) struct RecordPlanningRetirement<T:Copy+Send+'static>(std::mem::ManuallyDrop<Vec<T>>);
impl<T:Copy+Send+'static> RecordPlanningRetirement<T> {
    pub(super) fn new(indices:Vec<T>)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(Self(std::mem::ManuallyDrop::new(indices)))}
    pub(super) const fn birth_bytes()->usize{std::mem::size_of::<Self>()}
}
impl<T:Copy+Send+'static> semio_framework_value::retirement::RetirementCursor for RecordPlanningRetirement<T> {
    fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{
        use semio_framework_value::retirement::RetirementStep;
        if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}
        if !self.0.is_empty(){self.0.clear();return RetirementStep::Advanced;}
        let bytes=self.0.capacity()*std::mem::size_of::<T>();
        if bytes==0{return RetirementStep::Complete;}
        if bytes>grant.maximum_release_bytes{return RetirementStep::BudgetExhausted;}
        drop(std::mem::take(&mut*self.0));RetirementStep::Bytes(bytes)
    }
    fn terminal_is_empty(&self)->bool{self.0.capacity()==0}
    fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
    fn next_close_byte_demand(&self)->Option<usize>{Some(if self.0.is_empty(){self.0.capacity()*std::mem::size_of::<T>()}else{0})}
    fn terminal_release_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self>())}
}
impl<T:Copy+Send+'static> Drop for RecordPlanningRetirement<T> {
    fn drop(&mut self){use semio_framework_value::retirement::RetirementCursor;assert!(std::thread::panicking()||self.terminal_is_empty(),"record planning buffer requires terminal retirement");unsafe{std::mem::ManuallyDrop::drop(&mut self.0)};}
}
