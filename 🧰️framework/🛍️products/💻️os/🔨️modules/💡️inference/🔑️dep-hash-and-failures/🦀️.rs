//! 🔑️ Dependency hash and failures of the inference cache: the merkle dependency hash, the inference step and fault records, the work failure with its domain projections and the inference error. See [`super`].


//#region 🔖️DepHash
/// 🔑 Per-entity dependency hash — one link in a merkle dependency chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(semio_framework_value::RetireOwned)]
pub struct DepHash(pub [u8; 32]);

impl DepHash {
    /// 🏗️ Roots a chain: `blake3(field_id ‖ 0 ‖ schema_version ‖ 0 ‖ input)`, no parent hashes folded in.
    pub fn root(field_id: &str, schema_version: u32, input: &[u8]) -> Self {
        let mut data = field_id.as_bytes().to_vec();
        data.push(0);
        data.extend_from_slice(&schema_version.to_le_bytes());
        data.push(0);
        data.extend_from_slice(input);
        Self(*semio_framework_hash::hash(&data).as_bytes())
    }

    /// 🔗 Extends a chain: folds `parents` (order-independent — sorted by their own bytes, so two entities with
    /// the same parent SET in different orders hash identically) into `input` under the same `(field_id, schema_version)`
    /// salt as [`root`](Self::root). The fold is `semio_framework_hash::merkle_node` over the hex of the sorted parents, without its strings.
    pub fn chain(field_id: &str, schema_version: u32, input: &[u8], parents: &[DepHash]) -> Self {
        let mut own = field_id.as_bytes().to_vec();
        own.push(0);
        own.extend_from_slice(&schema_version.to_le_bytes());
        own.push(0);
        own.extend_from_slice(input);
        let mut sorted: Vec<[u8; 32]> = parents.iter().map(|parent| parent.0).collect();
        sorted.sort_unstable();
        let mut data = Vec::with_capacity(65 * (1 + sorted.len()));
        push_hex(&mut data, semio_framework_hash::hash(&own).as_bytes());
        data.push(0x1f);
        for parent in &sorted {
            push_hex(&mut data, parent);
            data.push(0x1f);
        }
        Self(*semio_framework_hash::hash(&data).as_bytes())
    }
}

fn push_hex(out: &mut Vec<u8>, bytes: &[u8; 32]) {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    for byte in bytes {
        out.push(DIGITS[usize::from(byte >> 4)]);
        out.push(DIGITS[usize::from(byte & 15)]);
    }
}
//#endregion 🔖️DepHash

//#region 🔖️InferredField
/// 🧭 One entity in a field's deterministic evaluation plan (roots first).
#[derive(Clone, Debug)]
pub struct InferenceStep<K> {
    pub key: K,
    pub parents: Vec<K>,
}

/// 🚫 A compute that could not produce a value: a stable machine code and a readable message.
#[derive(Clone, Debug, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
pub struct InferenceFault {
    pub code: String,
    pub message: String,
}

impl InferenceFault {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code: code.into(), message: message.into() }
    }
}

/// 🪢️ Keeps each original field, driver or native turn refusal as its complete retained owner.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum InferenceWorkFailure{Field(InferenceFault),Driver(InferenceError),Original(semio_framework_job::OriginalTurnFailure)}

impl From<semio_framework_diagnostic::Fault> for InferenceWorkFailure{fn from(primary:semio_framework_diagnostic::Fault)->Self{Self::Original(primary.into())}}

impl From<InferenceFault> for InferenceWorkFailure{fn from(primary:InferenceFault)->Self{Self::Field(primary)}}

impl From<InferenceError> for InferenceWorkFailure{fn from(primary:InferenceError)->Self{Self::Driver(primary)}}

impl From<semio_framework_value::ValueError> for InferenceWorkFailure{fn from(primary:semio_framework_value::ValueError)->Self{Self::Original(primary.into())}}

impl From<semio_framework_job::OriginalTurnFailure> for InferenceWorkFailure{fn from(primary:semio_framework_job::OriginalTurnFailure)->Self{Self::Original(primary)}}

/// 🎟️ Moves an already admitted native envelope unchanged, or admits its entire defining inference source once.
pub fn admit_inference_failure(original:&mut Option<InferenceWorkFailure>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::OriginalTurnFailure>,semio_framework_job::OriginalTurnFailure>{
 if original.as_ref().is_some_and(|original|matches!(original,InferenceWorkFailure::Original(_))){let Some(InferenceWorkFailure::Original(original))=original.take()else{unreachable!()};return Ok(Some(original))}
 semio_framework_job::admit_original_domain_failure(original,cx)
}

impl semio_framework_job::OriginalDomainProjection for InferenceWorkFailure{
 fn projection_view(&self,path:&[usize])->Result<semio_framework_job::OriginalDomainView<'_>,semio_framework_value::ValueError>{
  use semio_framework_job::OriginalDomainView as V;
  if path.is_empty(){return Ok(V::IntrinsicObject(2))}
  match path[0]{
   0 if path.len()==1=>Ok(V::Text(match self{Self::Field(_)=>"field",Self::Driver(_)=>"driver",Self::Original(_)=>"original"})),
   1=>match self{
    Self::Original(original)=>semio_framework_job::project_original_turn_failure(original,&path[1..]),
    Self::Field(original)=>match &path[1..]{[]=>Ok(V::IntrinsicObject(2)),[0]=>Ok(V::Text(&original.code)),[1]=>Ok(V::Text(&original.message)),_=>Err(inference_projection_refusal())},
    Self::Driver(original)=>match (original,&path[1..]){
     (InferenceError::MissingParent{..},[])=>Ok(V::IntrinsicObject(3)),(InferenceError::Compute{..},[])=>Ok(V::IntrinsicObject(3)),(InferenceError::Cancelled,[])=>Ok(V::IntrinsicObject(1)),
     (InferenceError::MissingParent{..},[0])=>Ok(V::Text("missingParent")),(InferenceError::Compute{..},[0])=>Ok(V::Text("compute")),(InferenceError::Cancelled,[0])=>Ok(V::Text("cancelled")),
     (InferenceError::MissingParent{key,..}|InferenceError::Compute{key,..},[1])=>Ok(V::Text(key)),(InferenceError::MissingParent{parent,..},[2])=>Ok(V::Text(parent)),
     (InferenceError::Compute{..},[2])=>Ok(V::IntrinsicObject(2)),(InferenceError::Compute{fault,..},[2,0])=>Ok(V::Text(&fault.code)),(InferenceError::Compute{fault,..},[2,1])=>Ok(V::Text(&fault.message)),_=>Err(inference_projection_refusal()),
    },
   },_=>Err(inference_projection_refusal()),
  }
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,semio_framework_value::ValueError>{
  if path.is_empty(){return ["kind","primary"].get(index).copied().ok_or_else(inference_projection_refusal)}
  if path[0]!=1{return Err(inference_projection_refusal())}
  match self{
   Self::Original(original)=>semio_framework_job::key_original_turn_failure(original,&path[1..],index),
   Self::Field(_)=>if path.len()==1{["code","message"].get(index).copied().ok_or_else(inference_projection_refusal)}else{Err(inference_projection_refusal())},
   Self::Driver(original)=>match (original,&path[1..]){(InferenceError::MissingParent{..},[])=>["kind","key","parent"].get(index).copied().ok_or_else(inference_projection_refusal),(InferenceError::Compute{..},[])=>["kind","key","fault"].get(index).copied().ok_or_else(inference_projection_refusal),(InferenceError::Cancelled,[]) if index==0=>Ok("kind"),(InferenceError::Compute{..},[2])=>["code","message"].get(index).copied().ok_or_else(inference_projection_refusal),_=>Err(inference_projection_refusal())},
  }
 }
}

impl semio_framework_job::OriginalDomainFailure for InferenceWorkFailure{
 fn domain_code(&self)->&str{match self{Self::Field(original)=>&original.code,Self::Driver(_)=>"inference.driver",Self::Original(_)=>"inference.original"}}
 fn domain_message(&self)->&str{match self{Self::Field(original)=>&original.message,Self::Driver(InferenceError::MissingParent{..})=>"inference plan parent is missing",Self::Driver(InferenceError::Compute{fault,..})=>&fault.message,Self::Driver(InferenceError::Cancelled)=>"inference cancelled",Self::Original(original)=>original.primary.message()}}
 fn domain_retained_progress(&self)->semio_framework_value::RetainedCloneProgress{match self{Self::Original(original)=>original.retained_progress(),_=>Default::default()}}
}

fn inference_projection_refusal()->semio_framework_value::ValueError{semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inference failure projection ordinal changed")}

/// 🛑 Why a driver call could not finish: a plan that names a parent before computing it, a compute that failed, or a cancelled run.
#[derive(Clone, Debug, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
pub enum InferenceError {
    MissingParent { key: String, parent: String },
    Compute { key: String, fault: InferenceFault },
    Cancelled,
}

impl std::fmt::Display for InferenceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingParent { key, parent } => write!(formatter, "inference plan violation: {key} names parent {parent} before it is computed"),
            Self::Compute { key, fault } => write!(formatter, "inference compute of {key} failed: {}: {}", fault.code, fault.message),
            Self::Cancelled => write!(formatter, "inference cancelled"),
        }
    }
}

impl std::error::Error for InferenceError {}
//#endregion 🔖️InferredField
