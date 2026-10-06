// 🧩️ Canonical child-group framing projects the retained source without a whole byte or value copy.
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource,FieldProjectionView as V,projection_path_error};
use semio_framework_ui_locale::{Locale,Terminology,LocalizedLabel};
use semio_framework_value::{NativeEncodeControl,ValueError,list::PagedList};
use semio_framework_os_kernel::{os_pack,os_spr};

/// 🏷️ Borrows labels from their admitted outer page owner.
pub trait ChildLabelSources:Send+Sync{
    fn len(&self)->usize;
    fn label_at(&self,index:usize)->Option<&LocalizedLabel>;
}
impl<const N:usize> ChildLabelSources for PagedList<LocalizedLabel,N>{
    fn len(&self)->usize{PagedList::len(self)}
    fn label_at(&self,index:usize)->Option<&LocalizedLabel>{self.get(index)}
}

/// 🫳️ One child address, operation collection and label matrix borrow the original retained owners.
pub struct ChildGroupSource<'a>{pub owner:&'a str,pub slot:&'a str,pub child_id:&'a str,pub schema:&'a str,pub operations:&'a dyn os_spr::operation_bytes::OperationSourceCollection,pub labels:&'a dyn ChildLabelSources}

/// 🗂️ Publication retains the complete child collection while canonical framing reads ordinal paths.
pub trait ChildGroupSources:Send+Sync{
    fn len(&self)->usize;
    fn group_at(&self,index:usize)->Option<ChildGroupSource<'_>>;
}

/// 🔎️ Projects the declared intrinsic field order and every original raw operation octet.
pub struct ChildGroupProjection<'a>{pub groups:&'a dyn ChildGroupSources}
impl FieldProjectionSource for ChildGroupProjection<'_>{
    fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{
        match path{
            []=>Ok(V::Record(&[1])),
            [0]=>Ok(V::IntrinsicArray(self.groups.len())),
            [0,index]=>{self.groups.group_at(*index).ok_or_else(projection_path_error)?;Ok(V::IntrinsicObject(6))},
            [0,index,field]=>{
                let group=self.groups.group_at(*index).ok_or_else(projection_path_error)?;
                match field{0=>Ok(V::IntrinsicText(group.owner)),1=>Ok(V::IntrinsicText(group.slot)),2=>Ok(V::IntrinsicText(group.child_id)),3=>Ok(V::IntrinsicArray(group.operations.len())),4=>Ok(V::IntrinsicText(group.schema)),5=>Ok(V::IntrinsicArray(group.labels.len())),_=>Err(projection_path_error())}
            },
            [0,index,3,operation]=>{
                let group=self.groups.group_at(*index).ok_or_else(projection_path_error)?;
                let source=group.operations.source_at(*operation).ok_or_else(projection_path_error)?;
                Ok(V::IntrinsicArray(source.len()))
            },
            [0,index,3,operation,byte]=>{
                let group=self.groups.group_at(*index).ok_or_else(projection_path_error)?;
                let source=group.operations.source_at(*operation).ok_or_else(projection_path_error)?;
                Ok(V::IntrinsicNumber(semio_framework_value::Number::UInt(u64::from(*source.get(*byte).ok_or_else(projection_path_error)?))))
            },
            [0,index,5,label]=>{
                self.groups.group_at(*index).and_then(|group|group.labels.label_at(*label)).ok_or_else(projection_path_error)?;
                Ok(V::IntrinsicObject(Terminology::COUNT))
            },
            [0,index,5,label,terminology]=>{
                self.groups.group_at(*index).and_then(|group|group.labels.label_at(*label)).ok_or_else(projection_path_error)?;
                Terminology::ALL.get(*terminology).ok_or_else(projection_path_error)?;
                Ok(V::IntrinsicObject(Locale::COUNT))
            },
            [0,index,5,label,terminology,locale]=>{
                let label=self.groups.group_at(*index).and_then(|group|group.labels.label_at(*label)).ok_or_else(projection_path_error)?;
                let terminology=*Terminology::ALL.get(*terminology).ok_or_else(projection_path_error)?;
                let locale=*Locale::ALL.get(*locale).ok_or_else(projection_path_error)?;
                Ok(V::IntrinsicText(label.resolve(terminology,locale)))
            },
            _=>Err(projection_path_error()),
        }
    }
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
        match path{
            [0,group]=>{self.groups.group_at(*group).ok_or_else(projection_path_error)?;["owner","slot","child_id","ops","op_schema","labels"].get(index).copied().ok_or_else(projection_path_error)},
            [0,group,5,label]=>{self.groups.group_at(*group).and_then(|group|group.labels.label_at(*label)).ok_or_else(projection_path_error)?;Terminology::ALL.get(index).map(|axis|axis.as_str()).ok_or_else(projection_path_error)},
            [0,group,5,label,terminology]=>{self.groups.group_at(*group).and_then(|group|group.labels.label_at(*label)).ok_or_else(projection_path_error)?;Terminology::ALL.get(*terminology).ok_or_else(projection_path_error)?;Locale::ALL.get(index).map(|axis|axis.as_str()).ok_or_else(projection_path_error)},
            _=>Err(projection_path_error()),
        }
    }
}

/// ✍️ Appends exact child-group wire to admitted output using the caller's complete encoding policy.
pub fn encode_groups_into(groups:&dyn ChildGroupSources,options:&os_pack::codec::PackEncodeOptions,output:&mut dyn os_spr::operation_bytes::OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<usize,os_spr::ProtocolError>{
    os_spr::operation_bytes::with_operation_encode_policy(options,control,|control|{
        control.checkpoint().map_err(os_pack::PackRefusal::from)?;
        if groups.len()==0{return Ok(0)}
        let mut fields=control.allocate_vec(1).map_err(os_pack::PackRefusal::from)?;
        let name=control.copy_text("value").map_err(os_pack::PackRefusal::from)?;
        fields.push(semio_framework_dsl_record::FieldSpec{id:1,key:name,position:None,shape:semio_framework_dsl_record::Shape::Value,optional:false,flatten:false,defines:None,is_call_name:false});
        let spec=semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Lines,fields);
        os_pack::record::encode_projected_record_body_into(&spec,&ChildGroupProjection{groups},options,output,control).map_err(Into::into)
    })
}
