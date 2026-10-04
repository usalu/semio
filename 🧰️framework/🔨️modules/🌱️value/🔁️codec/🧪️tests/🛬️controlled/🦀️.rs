use super::*;
use crate::native_decoding::NativeDecodeControl;

#[derive(Debug, PartialEq, crate::FromValue, serde::Deserialize)]
#[value(crate="crate", rename_all="camelCase", deny_unknown_fields)]
#[serde(rename_all="camelCase", deny_unknown_fields)]
struct Entry { label:String, samples:Vec<i64>, optional:Option<bool> }

#[test]
fn controlled_value_neutral_constructor_matches_serde() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();
    for input in fixture["entries"].as_array().unwrap() {
        let expected:Entry=serde_json::from_value(input.clone()).unwrap();
        let value=DslValue::from(input);
        let mut callback=|_|true; let mut control=NativeDecodeControl::new(65536,&mut callback);
        assert_eq!(Entry::from_value_controlled(&value,&mut control).unwrap(),expected);
        if !expected.label.is_empty()||!expected.samples.is_empty(){assert!(control.owned_bytes()>0);}
    }
}

#[test]
fn controlled_value_custom_owner_refuses_without_constructor() {
    struct Missing;
    impl FromValue for Missing { fn from_value(_:DslValue)->Result<Self,ValueError>{Ok(Self)} }
    let mut callback=|_|true;let mut control=NativeDecodeControl::new(32,&mut callback);
    assert!(Missing::from_value_controlled(&DslValue::Null,&mut control).is_err());
}

#[derive(Debug,PartialEq,crate::FromValue,serde::Deserialize)]
#[value(crate="crate",tag="kind",content="payload",rename_all="camelCase",deny_unknown_fields)]
#[serde(tag="kind",content="payload",rename_all="camelCase",deny_unknown_fields)]
enum Adjacent { Empty, Label(String), Record{required:i64,optional:Option<String>} }
#[derive(Debug,PartialEq,crate::FromValue,serde::Deserialize)]
#[value(crate="crate",tag="kind",rename_all="camelCase",deny_unknown_fields)]
#[serde(tag="kind",rename_all="camelCase",deny_unknown_fields)]
enum Internal { Empty, Record{required:i64,optional:Option<String>} }
#[derive(Debug,PartialEq,crate::FromValue,serde::Deserialize)]
#[value(crate="crate",rename_all="camelCase")]
#[serde(rename_all="camelCase")]
enum External { Empty, Label(String), Record{required:i64,optional:Option<String>} }
#[derive(Debug,PartialEq,crate::FromValue,serde::Deserialize)]
#[value(crate="crate",transparent)]
#[serde(transparent)]
struct Transparent { label:String }
#[derive(Debug,PartialEq,crate::FromValue,serde::Deserialize)]
#[value(crate="crate")]
struct Flattened { known:i64, #[value(flatten)] #[serde(flatten)] extra:std::collections::BTreeMap<String,String> }
#[derive(Debug,PartialEq,crate::FromValue,serde::Deserialize)]
#[value(crate="crate")]
struct Defaults { #[value(default)] #[serde(default)] count:i64, optional:Option<String> }

fn decode<T:FromValue>(input:&serde_json::Value)->Result<T,ValueError>{let mut callback=|_|true;T::from_value_controlled(&DslValue::from(input),&mut NativeDecodeControl::new(1024*1024,&mut callback))}

fn plain_json_bridge(value:DslValue)->Result<serde_json::Value,ValueError>{Ok(serde_json::Value::from(value))}
fn plain_optional_json_bridge(value:DslValue)->Result<Option<serde_json::Value>,ValueError>{if matches!(value,DslValue::Null){Ok(None)}else{plain_json_bridge(value).map(Some)}}
#[derive(crate::FromValue,serde::Deserialize)]
#[value(crate="crate")]
struct CustomBareDefault { #[value(default,deserialize_with="plain_json_bridge",retire_with="std::mem::drop")] #[serde(default)] payload:serde_json::Value }
#[derive(crate::FromValue,serde::Deserialize)]
#[value(crate="crate")]
struct CustomOptionDefault { #[value(deserialize_with="plain_optional_json_bridge",retire_with="std::mem::drop")] payload:Option<serde_json::Value> }
#[derive(crate::FromValue,serde::Deserialize)]
#[value(crate="crate",default)]
#[serde(default)]
#[derive(Default)]
struct CustomContainerDefault { #[value(deserialize_with="plain_json_bridge",retire_with="std::mem::drop")] payload:serde_json::Value }
#[derive(Default)]
struct CustomText(String);
fn plain_text_bridge(value:DslValue)->Result<CustomText,ValueError>{String::from_value(value).map(CustomText)}
fn controlled_text_bridge(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<CustomText,ValueError>{String::from_value_controlled(value,control).map(CustomText)}
fn controlled_text_default(control:&mut NativeDecodeControl<'_>)->Result<CustomText,ValueError>{String::default_value_controlled(control).map(CustomText)}
#[derive(crate::FromValue)]
#[value(crate="crate")]
struct CustomControlledDefault { #[value(default,deserialize_with="plain_text_bridge",deserialize_controlled_with="controlled_text_bridge",default_controlled="controlled_text_default",retire_with="std::mem::drop")] payload:CustomText }
#[derive(serde::Deserialize)]
struct CustomTextOracle { #[serde(default)] payload:String }

#[test]
fn controlled_value_custom_defaults_require_owned_constructor_without_foreign_codec(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();
    for row in fixture["customDefaults"].as_array().unwrap(){
        let input=&row["input"];
        let(actual,plain,oracle)=match row["mode"].as_str().unwrap(){
            "bare"=>(decode::<CustomBareDefault>(input).map(|v|v.payload),CustomBareDefault::from_value(DslValue::from(input)).unwrap().payload,serde_json::from_value::<CustomBareDefault>(input.clone()).unwrap().payload),
            "option"=>(decode::<CustomOptionDefault>(input).map(|v|v.payload.unwrap_or_default()),CustomOptionDefault::from_value(DslValue::from(input)).unwrap().payload.unwrap_or_default(),serde_json::from_value::<CustomOptionDefault>(input.clone()).unwrap().payload.unwrap_or_default()),
            "container"=>(decode::<CustomContainerDefault>(input).map(|v|v.payload),CustomContainerDefault::from_value(DslValue::from(input)).unwrap().payload,serde_json::from_value::<CustomContainerDefault>(input.clone()).unwrap().payload),
            "controlled"=>(decode::<CustomControlledDefault>(input).map(|v|serde_json::Value::String(v.payload.0)),serde_json::Value::String(CustomControlledDefault::from_value(DslValue::from(input)).unwrap().payload.0),serde_json::Value::String(serde_json::from_value::<CustomTextOracle>(input.clone()).unwrap().payload)),
            _=>panic!("unsupported corpus mode")
        };
        assert_eq!(plain,oracle);assert_eq!(plain,row["plainPayload"]);assert_eq!(actual.is_ok(),row["controlledAccepted"].as_bool().unwrap());
        match actual{Ok(value)=>assert_eq!(value,oracle),Err(error)=>assert_eq!(error.message,row["controlledError"].as_str().unwrap())}
    }
}

#[test]
fn controlled_value_hash_set_vectors_match_serde_and_first_wire_error(){
    use std::collections::HashSet;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();
    for row in fixture["hashSets"].as_array().unwrap(){
        let input=&row["input"];
        let oracle=serde_json::from_value::<HashSet<u32>>(input.clone());
        let plain=HashSet::<u32>::from_value(DslValue::from(input));
        let controlled=decode::<HashSet<u32>>(input);
        assert_eq!(oracle.is_ok(),row["accepted"].as_bool().unwrap(),"{}",row["id"]);
        assert_eq!(plain.is_ok(),oracle.is_ok());assert_eq!(controlled.is_ok(),oracle.is_ok());
        if let Ok(oracle)=oracle{
            let actual=plain.unwrap();assert_eq!(actual,oracle);assert_eq!(controlled.unwrap(),oracle);
            let encoded:HashSet<u32>=serde_json::from_value(serde_json::Value::from(actual.to_value())).unwrap();assert_eq!(encoded,oracle);
        }else if let Some(index)=row["invalidIndex"].as_u64(){
            let value=&input[index as usize];
            assert_eq!(plain.err().unwrap(),u32::from_value(DslValue::from(value)).unwrap_err().under(index));
            assert_eq!(controlled.err().unwrap(),decode::<u32>(value).unwrap_err().under(index));
        }
    }
}

#[test]
fn controlled_value_hash_set_admits_budget_before_allocation_and_cancels(){
    use std::collections::HashSet;
    let value=DslValue::Array((0..1000).map(DslValue::uint).collect());
    let mut callback=|_|true;let mut control=NativeDecodeControl::new(1,&mut callback);
    assert!(HashSet::<u32>::from_value_controlled(&value,&mut control).is_err());assert_eq!(control.owned_bytes(),0);
    let mut observed=false;
    let mut callback=|progress:crate::native_decoding::NativeDecodeProgress|{if progress.total==1000&&progress.completed==256{observed=true;false}else{true}};
    assert!(HashSet::<u32>::from_value_controlled(&value,&mut NativeDecodeControl::new(1024*1024,&mut callback)).is_err());assert!(observed);
}

static SET_RETIREMENTS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
#[derive(Eq,PartialEq,Hash)]
struct SetMember(u32);
impl FromValue for SetMember{
    fn from_value(value:DslValue)->Result<Self,ValueError>{u32::from_value(value).map(Self)}
    fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{u32::from_value_controlled(value,control).map(Self)}
    fn retire_decoded(self){SET_RETIREMENTS.fetch_add(1,std::sync::atomic::Ordering::SeqCst);}
}
#[test]
fn controlled_value_hash_set_retires_duplicates_and_partial_members_once(){
    use std::collections::HashSet;
    SET_RETIREMENTS.store(0,std::sync::atomic::Ordering::SeqCst);
    let value=serde_json::json!([1,1,2,"invalid"]);
    assert!(HashSet::<SetMember>::from_value(DslValue::from(&value)).is_err());
    assert_eq!(SET_RETIREMENTS.load(std::sync::atomic::Ordering::SeqCst),3);
    assert!(decode::<HashSet<SetMember>>(&value).is_err());
    assert_eq!(SET_RETIREMENTS.load(std::sync::atomic::Ordering::SeqCst),6);
}

static SECRET_DROPS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
#[derive(crate::FromValue)]
#[value(crate="crate",retire_with="std::mem::drop")]
struct SecretOwner { secret:String }
impl Drop for SecretOwner{fn drop(&mut self){for byte in unsafe{self.secret.as_bytes_mut()}{unsafe{std::ptr::write_volatile(byte,0)};}SECRET_DROPS.fetch_add(1,std::sync::atomic::Ordering::SeqCst);}}
#[derive(crate::FromValue)]
#[value(crate="crate")]
struct SecretParent { owner:SecretOwner, later:u32 }
#[derive(serde::Deserialize)]
struct SecretOracle { secret:String, later:u32 }
#[test]
fn controlled_value_drop_owner_preserves_destructor_on_success_and_partial_failure(){
    SECRET_DROPS.store(0,std::sync::atomic::Ordering::SeqCst);
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();
    for row in fixture["dropOwners"].as_array().unwrap(){
        let input=&row["input"];let oracle=serde_json::from_value::<SecretOracle>(input.clone());
        let parent=serde_json::json!({"owner":{"secret":input["secret"]},"later":input["later"]});
        let actual=decode::<SecretParent>(&parent);assert_eq!(actual.is_ok(),oracle.is_ok());assert_eq!(actual.is_ok(),row["accepted"].as_bool().unwrap());
        if let(Ok(actual),Ok(oracle))=(actual,oracle){assert_eq!(actual.owner.secret,oracle.secret);assert_eq!(actual.later,oracle.later);SecretParent::retire_decoded(actual);}
    }
    assert_eq!(SECRET_DROPS.load(std::sync::atomic::Ordering::SeqCst),2);
}

#[test]
fn controlled_value_derived_shapes_match_independent_serde(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();
    macro_rules! check{($ty:ty,$value:expr)=>{assert_eq!(decode::<$ty>($value).unwrap(),serde_json::from_value::<$ty>($value.clone()).unwrap())};}
    for case in fixture["representations"].as_array().unwrap(){let input=&case["value"];match case["kind"].as_str().unwrap(){
        "adjacent"=>check!(Adjacent,input),"internal"=>check!(Internal,input),"external"=>check!(External,input),"transparent"=>check!(Transparent,input),"flattened"=>check!(Flattened,input),"defaults"=>check!(Defaults,input),_=>unreachable!()
    }}
    assert!(decode::<Internal>(&serde_json::json!({"kind":"record","required":1,"unknown":false})).is_err());
}

#[test]
fn controlled_value_intrinsic_copy_is_exact_and_iterative(){
    let mut value=DslValue::String("leaf雪".into());for _ in 0..2048{value=DslValue::Array(vec![value]);}
    let mut callback=|_|true;let mut control=NativeDecodeControl::new(8*1024*1024,&mut callback);
    let copied=DslValue::from_value_controlled(&value,&mut control).unwrap();let mut cursor=&copied;for _ in 0..2048{let DslValue::Array(children)=cursor else{panic!("array")};cursor=&children[0];}assert_eq!(cursor,&DslValue::String("leaf雪".into()));
    DslValue::retire_decoded(copied);
    let mut callback=|progress:crate::native_decoding::NativeDecodeProgress|progress.completed<256;
    assert!(DslValue::from_value_controlled(&value,&mut NativeDecodeControl::new(8*1024*1024,&mut callback)).is_err());
    DslValue::retire_decoded(value);
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();
    for word in fixture["floatWords"].as_array().unwrap(){let bits=u64::from_str_radix(word.as_str().unwrap(),16).unwrap();
        let value=DslValue::Number(Number::Float(f64::from_bits(bits)));let mut callback=|_|true;
        let DslValue::Number(Number::Float(number))=DslValue::from_value_controlled(&value,&mut NativeDecodeControl::new(4096,&mut callback)).unwrap()else{panic!("float")};assert_eq!(number.to_bits(),bits);
    }
    let bytes=DslValue::Bytes(serde_json::from_value(fixture["octets"].clone()).unwrap());let mut callback=|_|true;assert_eq!(DslValue::from_value_controlled(&bytes,&mut NativeDecodeControl::new(4096,&mut callback)).unwrap(),bytes);
}

#[test]
fn controlled_value_budget_precedes_large_copy_and_cancels_interior(){
    let value=DslValue::String("x".repeat(100000));let mut callback=|_|true;let mut control=NativeDecodeControl::new(128,&mut callback);assert!(String::from_value_controlled(&value,&mut control).is_err());assert_eq!(control.owned_bytes(),0);
    let value=DslValue::Array((0..1000).map(DslValue::int).collect());let mut observed=Vec::new();let mut callback=|progress:crate::native_decoding::NativeDecodeProgress|{observed.push(progress);progress.total!=1000||progress.completed<256};let mut control=NativeDecodeControl::new(100000,&mut callback);assert!(Vec::<i64>::from_value_controlled(&value,&mut control).is_err());assert!(observed.iter().any(|p|p.total==1000&&p.completed==256));
}

#[test]
fn controlled_value_intrinsic_frontier_cancels_before_materializing_wide_input(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();
    let count=fixture["limits"]["nullableSlots"].as_u64().unwrap() as usize;
    let value=DslValue::Array(vec![DslValue::Null;count]);let mut observed=None;
    let mut callback=|progress:crate::native_decoding::NativeDecodeProgress|{if progress.completed>=256{observed=Some(progress);false}else{true}};
    let error=DslValue::from_value_controlled(&value,&mut NativeDecodeControl::new(128*1024,&mut callback)).unwrap_err();
    assert!(error.to_string().contains("canceled"),"{error}");assert!(observed.unwrap().owned_bytes<65536);
}

static RETIRED:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
static DROPPED:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
struct Retained;
impl Drop for Retained{fn drop(&mut self){DROPPED.fetch_add(1,std::sync::atomic::Ordering::SeqCst);}}
impl FromValue for Retained{
    fn from_value(_:DslValue)->Result<Self,ValueError>{Ok(Self)}
    fn from_value_controlled(_: &DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Ok(Self)}
    fn retire_decoded(self){RETIRED.fetch_add(1,std::sync::atomic::Ordering::SeqCst);std::mem::forget(self);}
}
#[derive(crate::FromValue)]
#[value(crate="crate")]
struct Partial { first:Retained, later:i64 }
#[test]
fn controlled_value_partial_children_use_owner_retirement(){
    RETIRED.store(0,std::sync::atomic::Ordering::SeqCst);DROPPED.store(0,std::sync::atomic::Ordering::SeqCst);
    assert!(decode::<Partial>(&serde_json::json!({"first":null,"later":"invalid"})).is_err());
    assert_eq!(RETIRED.load(std::sync::atomic::Ordering::SeqCst),1);assert_eq!(DROPPED.load(std::sync::atomic::Ordering::SeqCst),0);
}

#[derive(crate::FromValue)]
#[value(crate="crate",retire_with="retire_recursive")]
struct Recursive { children:Vec<Recursive> }
fn retire_recursive(value:Recursive){let mut pending=vec![value];while let Some(value)=pending.pop(){pending.extend(value.children);}}
#[test]
fn controlled_value_recursive_derive_has_own_depth_bound(){
    let mut value=serde_json::json!({"children":[]});for _ in 0..80{value=serde_json::json!({"children":[value]});}
    let value=DslValue::from(&value);let mut callback=|_|true;let mut control=NativeDecodeControl::new(1024*1024,&mut callback);let error=Recursive::from_value_controlled(&value,&mut control).err().unwrap();assert!(error.message.contains("depth limit"));
    let success=DslValue::Object(vec![("children".into(),DslValue::Array(vec![]))]);let result=Recursive::from_value_controlled(&success,&mut control).unwrap();Recursive::retire_decoded(result);DslValue::retire_decoded(value);
}

#[derive(Debug,PartialEq,crate::FromValue,serde::Deserialize)]
#[value(crate="crate",rename_all="camelCase")]
#[serde(rename_all="camelCase")]
struct Attributes {
    #[value(rename="display",required)] #[serde(rename="display")] label:String,
    #[value(default)] #[serde(default)] flag:bool,
    #[value(deserialize_with="ordinary_custom",deserialize_controlled_with="controlled_custom",retire_with="std::mem::drop")] #[serde(deserialize_with="serde_custom")] converted:String,
    #[value(skip,default="ordinary_default",default_controlled="controlled_default",retire_with="std::mem::drop")] #[serde(skip,default="ordinary_default")] skipped:String,
}
fn serde_custom<'de,D:serde::Deserializer<'de>>(decoder:D)->Result<String,D::Error>{<String as serde::Deserialize>::deserialize(decoder)}
fn ordinary_custom(value:DslValue)->Result<String,ValueError>{String::from_value(value)}
fn controlled_custom(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<String,ValueError>{String::from_value_controlled(value,control)}
fn ordinary_default()->String{"owned-default".into()}
fn controlled_default(control:&mut NativeDecodeControl<'_>)->Result<String,ValueError>{control.copy_text("owned-default")}
#[derive(crate::FromValue)]
#[value(crate="crate")]
struct UncontrolledCustom { #[value(deserialize_with="ordinary_custom")] converted:String }
#[test]
fn controlled_value_declared_custom_defaults_and_required_rename_are_admitted(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();let input=fixture["attributes"].clone();
    let ordinary=Attributes::from_value(DslValue::from(&input)).unwrap();assert_eq!(decode::<Attributes>(&input).unwrap(),ordinary);assert_eq!(ordinary,serde_json::from_value::<Attributes>(input.clone()).unwrap());
    assert!(!ordinary.flag);assert_eq!(ordinary.skipped,"owned-default");
    assert!(decode::<Attributes>(&serde_json::json!({"converted":"exact"})).is_err());
    assert!(decode::<UncontrolledCustom>(&serde_json::json!({"converted":"exact"})).is_err());
    let mut callback=|_|true;let mut control=NativeDecodeControl::new(128,&mut callback);
    assert!(Attributes::from_value_controlled(&DslValue::from(&input),&mut control).is_err());assert!(control.owned_bytes()<=128);
}

#[test]
fn controlled_value_all_authored_container_domains_are_exact(){
    let input=DslValue::Array(vec![DslValue::int(1),DslValue::int(2)]);
    let mut callback=|_|true;let mut control=NativeDecodeControl::new(1024*1024,&mut callback);
    assert_eq!(<[i64;2]>::from_value_controlled(&input,&mut control).unwrap(),[1,2]);
    assert!(<[i64;3]>::from_value_controlled(&input,&mut control).is_err());
    assert_eq!(<(i64,i64)>::from_value_controlled(&input,&mut control).unwrap(),(1,2));
    assert_eq!(std::collections::VecDeque::<i64>::from_value_controlled(&input,&mut control).unwrap(),std::collections::VecDeque::from([1,2]));
    assert_eq!(std::collections::BTreeSet::<i64>::from_value_controlled(&input,&mut control).unwrap(),std::collections::BTreeSet::from([1,2]));
    let map=DslValue::Object(vec![("1".into(),DslValue::String("first".into())),("2".into(),DslValue::String("second".into()))]);
    assert_eq!(std::collections::HashMap::<i64,String>::from_value_controlled(&map,&mut control).unwrap(),std::collections::HashMap::from([(1,"first".into()),(2,"second".into())]));
    assert_eq!(std::collections::BTreeMap::<String,String>::from_value_controlled(&map,&mut control).unwrap(),std::collections::BTreeMap::from([("1".into(),"first".into()),("2".into(),"second".into())]));
    assert_eq!(*Box::<String>::from_value_controlled(&DslValue::String("boxed".into()),&mut control).unwrap(),"boxed");
    assert!(Option::<String>::from_value_controlled(&DslValue::Null,&mut control).unwrap().is_none());
}

#[test]
fn controlled_value_scalar_domains_preserve_exact_types_and_words(){
    let mut callback=|_|true;let mut control=NativeDecodeControl::new(65536,&mut callback);
    assert_eq!(u64::from_value_controlled(&DslValue::uint(u64::MAX),&mut control).unwrap(),u64::MAX);
    assert_eq!(i64::from_value_controlled(&DslValue::int(i64::MIN),&mut control).unwrap(),i64::MIN);
    for value in [DslValue::float(1.0),DslValue::float(f64::NAN),DslValue::int(-1)]{assert!(u64::from_value_controlled(&value,&mut control).is_err());}
    let word=0xfff8000000000042;assert_eq!(f64::from_value_controlled(&DslValue::float(f64::from_bits(word)),&mut control).unwrap().to_bits(),word);
    assert_eq!(bool::from_value_controlled(&DslValue::Bool(true),&mut control).unwrap(),true);
    assert_eq!(<()>::from_value_controlled(&DslValue::Null,&mut control).unwrap(),());
    assert_eq!(std::path::PathBuf::from_value_controlled(&DslValue::String("snow/雪".into()),&mut control).unwrap(),std::path::PathBuf::from("snow/雪"));
    let _:std::marker::PhantomData<String>=std::marker::PhantomData::from_value_controlled(&DslValue::Null,&mut control).unwrap();
}

#[derive(Hash,Eq,PartialEq)]
struct UncontrolledKey(i64);
impl std::str::FromStr for UncontrolledKey{type Err=std::num::ParseIntError;fn from_str(input:&str)->Result<Self,Self::Err>{input.parse().map(Self)}}
impl FromValue for UncontrolledKey{fn from_value(value:DslValue)->Result<Self,ValueError>{i64::from_value(value).map(Self)}}
#[test]
fn controlled_value_custom_map_keys_refuse_uncontrolled_parsing(){
    let value=DslValue::Object(vec![("1".into(),DslValue::Bool(true))]);let mut callback=|_|true;let mut control=NativeDecodeControl::new(4096,&mut callback);
    let error=std::collections::HashMap::<UncontrolledKey,bool>::from_value_controlled(&value,&mut control).err().unwrap();assert!(error.message.contains("key owner"));
}

#[test]
fn controlled_value_wide_nullable_slots_are_admitted_before_allocation(){
    let value=DslValue::Array(vec![DslValue::Null;20000]);let mut callback=|_|true;let mut control=NativeDecodeControl::new(128,&mut callback);assert!(Vec::<Option<u64>>::from_value_controlled(&value,&mut control).is_err());assert_eq!(control.owned_bytes(),0);
}

mod custom_module {
    use super::*;
    pub fn to_value(value:&String)->DslValue{value.to_value()}
    pub fn from_value(value:DslValue)->Result<String,ValueError>{String::from_value(value)}
    pub fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<String,ValueError>{String::from_value_controlled(value,control)}
}
#[derive(Debug,PartialEq,crate::ToValue,crate::FromValue)]
#[value(crate="crate")]
struct WithModule { #[value(with="custom_module",deserialize_controlled_with="custom_module::from_value_controlled",retire_with="std::mem::drop")] label:String }
#[test]
fn controlled_value_custom_module_matches_ordinary_representation(){
    let input=serde_json::json!({"label":"exact雪"});let actual=decode::<WithModule>(&input).unwrap();assert_eq!(actual,WithModule::from_value(DslValue::from(&input)).unwrap());assert_eq!(serde_json::Value::from(actual.to_value()),input);
}

#[derive(Debug,PartialEq,Default,crate::FromValue,serde::Deserialize)]
#[value(crate="crate",default_controlled="mode_default")]
enum Mode { #[default] Off, On }
fn mode_default(control:&mut NativeDecodeControl<'_>)->Result<Mode,ValueError>{bool::default_value_controlled(control)?;Ok(Mode::Off)}
#[derive(Debug,PartialEq,crate::FromValue,serde::Deserialize)]
#[value(crate="crate")]
struct ModeParent { #[value(default)] #[serde(default)] mode:Mode }
#[test]
fn controlled_value_declared_container_default_preserves_serde_semantics(){
    let input=serde_json::json!({});assert_eq!(decode::<ModeParent>(&input).unwrap(),serde_json::from_value::<ModeParent>(input).unwrap());
}

struct ObservedAllocator;
static OBSERVE_ALLOCATIONS:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);
static LARGEST_ALLOCATION:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
unsafe impl std::alloc::GlobalAlloc for ObservedAllocator {
    unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8{if OBSERVE_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed){LARGEST_ALLOCATION.fetch_max(layout.size(),std::sync::atomic::Ordering::Relaxed);}unsafe{std::alloc::GlobalAlloc::alloc(&std::alloc::System,layout)}}
    unsafe fn alloc_zeroed(&self,layout:std::alloc::Layout)->*mut u8{if OBSERVE_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed){LARGEST_ALLOCATION.fetch_max(layout.size(),std::sync::atomic::Ordering::Relaxed);}unsafe{std::alloc::GlobalAlloc::alloc_zeroed(&std::alloc::System,layout)}}
    unsafe fn realloc(&self,pointer:*mut u8,layout:std::alloc::Layout,size:usize)->*mut u8{if OBSERVE_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed){LARGEST_ALLOCATION.fetch_max(size,std::sync::atomic::Ordering::Relaxed);}unsafe{std::alloc::GlobalAlloc::realloc(&std::alloc::System,pointer,layout,size)}}
    unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout){unsafe{std::alloc::GlobalAlloc::dealloc(&std::alloc::System,pointer,layout)}}
}
#[global_allocator]
static OBSERVED_ALLOCATOR:ObservedAllocator=ObservedAllocator;

#[test]
fn controlled_value_actual_allocator_observes_refusal_before_payload_materialization(){
    let text=DslValue::String("x".repeat(100000));let nulls=DslValue::Array(vec![DslValue::Null;20000]);
    let mut callback=|_|true;let mut control=NativeDecodeControl::new(128,&mut callback);
    LARGEST_ALLOCATION.store(0,std::sync::atomic::Ordering::Relaxed);OBSERVE_ALLOCATIONS.store(true,std::sync::atomic::Ordering::Relaxed);
    let text_result=String::from_value_controlled(&text,&mut control);let slots_result=Vec::<Option<u64>>::from_value_controlled(&nulls,&mut control);
    OBSERVE_ALLOCATIONS.store(false,std::sync::atomic::Ordering::Relaxed);
    assert!(text_result.is_err()&&slots_result.is_err());assert_eq!(control.owned_bytes(),0);assert!(LARGEST_ALLOCATION.load(std::sync::atomic::Ordering::Relaxed)<65536);
}

#[test]
fn controlled_value_binary32_words_survive_controlled_intrinsic_bridge(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();
 for word in fixture["binary32Words"].as_array().unwrap(){let bits=u32::from_str_radix(word.as_str().unwrap(),16).unwrap();let input=f32::from_bits(bits).to_value();let mut accepted=|_|true;let mut control=NativeDecodeControl::new(4096,&mut accepted);assert_eq!(f32::from_value_controlled(&input,&mut control).unwrap().to_bits(),bits,"{}",word.as_str().unwrap());}
}
#[test]
fn controlled_value_binary32_words_survive_ordinary_intrinsic_bridge(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();
 for word in fixture["binary32Words"].as_array().unwrap(){let bits=u32::from_str_radix(word.as_str().unwrap(),16).unwrap();assert_eq!(f32::from_value(f32::from_bits(bits).to_value()).unwrap().to_bits(),bits,"{}",word.as_str().unwrap());}
}

#[test]
fn controlled_value_binary32_nan_bridge_has_explicit_binary64_words(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();
 for row in fixture["binary32NanBridges"].as_array().unwrap(){let bits=u32::from_str_radix(row["word"].as_str().unwrap(),16).unwrap();let DslValue::Number(Number::Float(value))=f32::from_bits(bits).to_value()else{panic!("binary64 bridge")};assert_eq!(value.to_bits(),u64::from_str_radix(row["binary64Word"].as_str().unwrap(),16).unwrap());}
}

#[derive(Debug, PartialEq, crate::FromValue, serde::Deserialize)]
#[value(crate="crate")]
struct ScalarOwnerDefaults {
    #[value(default="ScalarOwnerDefaults::one", default_controlled="ScalarOwnerDefaults::controlled_one")]
    #[serde(default="ScalarOwnerDefaults::one")]
    count: u32,
}
impl ScalarOwnerDefaults {
    fn one() -> u32 { 1 }
    fn controlled_one(control: &mut NativeDecodeControl<'_>) -> Result<u32, ValueError> {
        control.checkpoint()?;
        Ok(1)
    }
}
#[test]
fn controlled_value_scalar_owner_defaults_are_not_recursive_types() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();
    for row in fixture["scalarOwnerDefaultInputs"].as_array().unwrap() {
        let oracle: ScalarOwnerDefaults = serde_json::from_value(row["input"].clone()).unwrap();
        assert_eq!(oracle.count, row["expected"]["count"].as_u64().unwrap() as u32);
        let actual = decode::<ScalarOwnerDefaults>(&row["input"]).unwrap();
        assert_eq!(actual, oracle);
    }
    let mut reject = |_| false;
    assert!(ScalarOwnerDefaults::from_value_controlled(&DslValue::Object(vec![]), &mut NativeDecodeControl::new(1_000_000, &mut reject)).is_err());
}

#[derive(Debug, PartialEq, crate::FromValue, serde::Deserialize)]
#[value(crate="crate")]
struct CapacityOwner { items: [u32; CapacityOwner::SIZE] }
impl CapacityOwner { const SIZE: usize = 2; }
#[test]
fn controlled_value_owner_named_array_capacity_is_not_recursive_storage() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();
    let oracle: CapacityOwner = serde_json::from_value(fixture["scalarOwnerConstInput"].clone()).unwrap();
    assert_eq!(oracle.items, [7, 9]);
    assert_eq!(decode::<CapacityOwner>(&fixture["scalarOwnerConstInput"]).unwrap(), oracle);
}

#[test]
fn controlled_value_borrowed_object_keys_cancel_inside_hashing_without_copying_keys() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔗️borrowed-keys.json")).unwrap();
    let key = fixture["unit"].as_str().unwrap().repeat(usize::try_from(fixture["repeat"].as_u64().unwrap()).unwrap());
    let maximum = usize::try_from(fixture["maximumOwnedBytes"].as_u64().unwrap()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let count = usize::try_from(case["entries"].as_u64().unwrap()).unwrap();
        let input = DslValue::object((0..count).map(|_| (key.clone(), DslValue::Null)));
        let mut reached = false;
        let mut stages = 0usize;
        let cancel_stage = usize::try_from(case["cancelStage"].as_u64().unwrap()).unwrap();
        let mut callback = |progress: crate::native_decoding::NativeDecodeProgress| {
            if progress.total == key.len() && progress.completed == 0 { stages += 1; }
            if stages == cancel_stage && progress.total == key.len() && progress.completed >= 65536 && progress.completed < progress.total { reached = true; false } else { true }
        };
        let mut control = NativeDecodeControl::new(maximum, &mut callback);
        let error = match input.object_controlled(&mut control) { Err(error) => error, Ok(_) => panic!("borrowed-key interior cancellation was not reached: {}",case["id"]) };
        assert_eq!(error.kind.as_str(), fixture["expectedKind"].as_str().unwrap(), "{}", case["id"]);
        assert!(control.owned_bytes() < key.len());
        assert!(reached, "{}", case["id"]);
    }
    let duplicate = DslValue::object([(key.clone(), DslValue::Null), (key.clone(), DslValue::Null)]);
    let error = match duplicate.object_controlled(&mut NativeDecodeControl::new(maximum, &mut |_|true)) { Err(error) => error, Ok(_) => panic!("duplicate literal key admitted") };
    assert_eq!(error.kind, crate::ValueRefusalKind::InvalidValue);
    assert_eq!(error.message,"duplicate object key");
    let ordered = DslValue::object([(key, DslValue::Bool(true)), ("".into(), DslValue::Bool(false))]);
    let mut yes = |_|true;
    let mut control = NativeDecodeControl::new(maximum, &mut yes);
    assert_eq!(ordered.object_controlled(&mut control).unwrap(), ordered.as_object().unwrap());
}
