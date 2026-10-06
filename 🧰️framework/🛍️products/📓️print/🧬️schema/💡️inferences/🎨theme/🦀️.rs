//! 🎨️ Owned CSS paint parsing shares the authored color contract with numerical inference.
#[derive(Clone,Debug)]
pub struct Paint { pub hex:String,pub alpha:f64 }
impl Paint {
    pub fn name(&self)->String{if self.alpha>=1.0{format!("semio-print-color-{}",self.hex)}else{format!("semio-print-color-{}-A{}",self.hex,self.alpha.to_string().replace('.',"p"))}}
    pub fn declaration(&self)->String{let name=self.name();format!("\\definecolor{{{name}}}{{HTML}}{{{}}}\n{}",self.hex,if self.alpha<1.0{format!("\\SemioVizPaintAlpha{{{name}}}{{{}}}\n",self.alpha)}else{String::new()})}
}
pub fn parse(value:&str)->Result<Paint,String>{
    let value=value.trim().to_ascii_lowercase();
    let invalid=||format!("invalid colour: {value}");
    if value=="transparent"{return Ok(Paint{hex:"000000".into(),alpha:0.0});}
    static NAMES:std::sync::OnceLock<semio_framework_value::DslValue>=std::sync::OnceLock::new();
    let names=NAMES.get_or_init(||semio_framework_pack_json::from_json_str(include_str!("../../📸️snapshot/📊️chart/🎨️color/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored color schema"));
    let hex=names["x-semio-named-colors"].get(&value).and_then(semio_framework_value::DslValue::as_str).or_else(||value.strip_prefix('#'));
    if let Some(hex)=hex{
        if ![3,4,6,8].contains(&hex.len())||!hex.chars().all(|c|c.is_ascii_hexdigit()){return Err(invalid());}
        let hex=if hex.len()<=4{hex.chars().flat_map(|c|[c,c]).collect::<String>()}else{hex.into()}.to_ascii_uppercase();
        return Ok(Paint{alpha:if hex.len()==8{u8::from_str_radix(&hex[6..],16).map_err(|_|invalid())? as f64/255.0}else{1.0},hex:hex[..6].into()});
    }
    let (kind,body)=value.split_once('(').ok_or_else(invalid)?;
    if !["rgb","rgba","hsl","hsla"].contains(&kind){return Err(invalid());}
    let body=body.strip_suffix(')').ok_or_else(invalid)?;
    let parts=body.split(|c:char|c==','||c=='/'||c.is_whitespace()).filter(|part|!part.is_empty()).collect::<Vec<_>>();
    if !(3..=4).contains(&parts.len()){return Err(invalid());}
    let number=|part:&str,percent:f64|->Result<f64,String>{let raw=part.strip_suffix('%').unwrap_or(part);let value=raw.parse::<f64>().map_err(|_|invalid())?;if !value.is_finite(){return Err(invalid());}Ok(if part.ends_with('%'){value*percent/100.0}else{value})};
    let alpha=if parts.len()==4{number(parts[3],1.0)?.clamp(0.0,1.0)}else{1.0};
    let channels=if kind.starts_with("rgb"){[number(parts[0],255.0)?.clamp(0.0,255.0),number(parts[1],255.0)?.clamp(0.0,255.0),number(parts[2],255.0)?.clamp(0.0,255.0)]}else{
        if !parts[1].ends_with('%')||!parts[2].ends_with('%'){return Err(invalid());}
        let (raw,factor)=if let Some(raw)=parts[0].strip_suffix("deg"){(raw,1.0)}else if let Some(raw)=parts[0].strip_suffix("grad"){(raw,0.9)}else if let Some(raw)=parts[0].strip_suffix("rad"){(raw,180.0/std::f64::consts::PI)}else if let Some(raw)=parts[0].strip_suffix("turn"){(raw,360.0)}else{(parts[0],1.0)};
        let hue=(number(raw,1.0)?*factor).rem_euclid(360.0);let saturation=number(parts[1],1.0)?.clamp(0.0,1.0);let light=number(parts[2],1.0)?.clamp(0.0,1.0);
        let chroma=(1.0-(2.0*light-1.0).abs())*saturation;let secondary=chroma*(1.0-(hue/60.0%2.0-1.0).abs());let base=light-chroma/2.0;
        let channels=if hue<60.0{[chroma,secondary,0.0]}else if hue<120.0{[secondary,chroma,0.0]}else if hue<180.0{[0.0,chroma,secondary]}else if hue<240.0{[0.0,secondary,chroma]}else if hue<300.0{[secondary,0.0,chroma]}else{[chroma,0.0,secondary]};channels.map(|value|(value+base)*255.0)
    };
    Ok(Paint{hex:channels.iter().map(|value|format!("{:02X}",value.round() as u8)).collect(),alpha})
}
