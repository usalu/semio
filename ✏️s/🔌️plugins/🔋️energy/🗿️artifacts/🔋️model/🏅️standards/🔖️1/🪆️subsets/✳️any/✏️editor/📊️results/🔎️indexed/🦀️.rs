//! 🔎️ Prepared surface partitions support fixed original reads without decode maps or repaint sorting.
use super::{ResultField,SURFACE_ENERGY_BANDS,hex_to_rgb01};
use semio_framework_tool_run::payload_stream::ToolRunPayloadView;
use semio_framework_ui_locale::Locale;

/// 🧱️ The renderer borrows complete original bytes through one bounded read interface.
pub trait EnergyResultSource{
 fn complete(&self)->bool;
 fn byte_len(&self)->usize;
 fn read_fixed<const N:usize>(&self,offset:usize)->Option<[u8;N]>;
}
impl EnergyResultSource for ToolRunPayloadView<'_>{
 fn complete(&self)->bool{ToolRunPayloadView::complete(self)}
 fn byte_len(&self)->usize{self.total_bytes()}
 fn read_fixed<const N:usize>(&self,offset:usize)->Option<[u8;N]>{ToolRunPayloadView::read_fixed(self,offset)}
}
/// 📊️ Fixed header metadata is captured once; each painted face probes two prepared id partitions.
pub struct IndexedEnergyResults<S:EnergyResultSource>{source:S,counts:[usize;2],bounds:[[f64;2];4]}
fn column(field:ResultField)->usize{match field{ResultField::ConductionLoss=>0,ResultField::ConductionGain=>1,ResultField::SolarTransmitted=>2,ResultField::SolarAbsorbed=>3}}
impl<S:EnergyResultSource> IndexedEnergyResults<S>{
 pub fn new(source:S)->Option<Self>{
  if !source.complete(){return None;}let header=source.read_fixed::<12>(0)?;if &header[..4]!=b"ESF2"{return None;}let counts=[u32::from_le_bytes(header[4..8].try_into().ok()?)as usize,u32::from_le_bytes(header[8..12].try_into().ok()?)as usize];let count=counts[0].checked_add(counts[1])?;if source.byte_len()!=76usize.checked_add(count.checked_mul(24)?)?{return None;}let mut bounds=[[0.0;2];4];for (field,bound)in bounds.iter_mut().enumerate(){let bytes=source.read_fixed::<16>(12+field*16)?;*bound=[f64::from_le_bytes(bytes[..8].try_into().ok()?),f64::from_le_bytes(bytes[8..].try_into().ok()?)];if !bound[0].is_finite()||!bound[1].is_finite()||bound[0]>bound[1]{return None;}}Some(Self{source,counts,bounds})
 }
 pub fn bounds(&self,field:ResultField)->[f64;2]{self.bounds[column(field)]}
 pub fn color(&self,id:u32,field:ResultField)->Option<[f64;3]>{
  let mut start=0;for count in self.counts{let mut low=0;let mut high=count;while low<high{let middle=low+(high-low)/2;let candidate=u32::from_le_bytes(self.source.read_fixed::<4>(76+(start+middle)*24)?);if candidate<id{low=middle+1;}else{high=middle;}}if low<count&&u32::from_le_bytes(self.source.read_fixed::<4>(76+(start+low)*24)?)==id{let band=self.source.read_fixed::<4>(76+(start+low)*24+20)?[column(field)]as usize;return SURFACE_ENERGY_BANDS.get(band).map(|hex|hex_to_rgb01(hex));}start+=count;}None
 }
 pub fn caption(&self,field:ResultField,locale:Locale)->String{let labels=match locale{Locale::En=>["Conduction loss","Conduction gain","Solar transmitted","Solar absorbed"],Locale::De=>["Wärmeleitungsverlust","Wärmeleitungsgewinn","Solare Transmission","Solare Absorption"]};let[low,high]=self.bounds(field);let format=|value:f64|{let text=format!("{value:.1}");match locale{Locale::En=>text,Locale::De=>text.replace('.',",")}};format!("{} · {} – {} kWh",labels[column(field)],format(low),format(high))}
 pub fn scale_label(locale:Locale)->&'static str{match locale{Locale::En=>"Result scale, low to high",Locale::De=>"Ergebnisskala, niedrig bis hoch"}}
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
