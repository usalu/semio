//! 🎛️ Physical baseline JPEG export policy.

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct JpgEncodeComponent {
    pub id:u8,
    pub h_sampling:u8,
    pub v_sampling:u8,
}

/// 🎚️ Native quantization and sampling choices for one export request.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct JpgEncodeOptions {
    pub quality:u8,
    pub components:Vec<JpgEncodeComponent>,
}

impl Default for JpgEncodeOptions {
    fn default()->Self {Self{quality:90,components:vec![JpgEncodeComponent{id:1,h_sampling:2,v_sampling:2},JpgEncodeComponent{id:2,h_sampling:1,v_sampling:1},JpgEncodeComponent{id:3,h_sampling:1,v_sampling:1}]}}
}

impl JpgEncodeOptions {
    /// 📐️ Projects an observed native frame into an explicit export profile.
    pub fn from_frame(frame:Option<&super::super::observations::JpgFrameHeader>)->Self {
        let Some(frame)=frame.filter(|frame|!frame.components.is_empty()) else{return Self::default();};
        Self{quality:90,components:frame.components.iter().map(|component|JpgEncodeComponent{id:component.id,h_sampling:component.h_sampling,v_sampling:component.v_sampling}).collect()}
    }
}
