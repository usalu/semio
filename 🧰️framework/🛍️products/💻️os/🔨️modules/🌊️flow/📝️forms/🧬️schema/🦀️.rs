//! 📝️ Original Flow widget field loans supplied to an explicitly selected form producer.
/// 🏭️ Owns the actual child field and form constructions independently of Flow's snapshot source.
pub trait FlowFormProducer {
 type Field;
 type Form;
 fn slider(&self,id:&str,label:&str,value:f64,min:f64,max:f64,step:f64)->Self::Field;
 fn note(&self,id:&str,text:&str)->Self::Field;
 fn image(&self,id:&str,source:&str)->Self::Field;
 fn variable(&self,id:&str,name:&str,schema:&str)->Self::Field;
 fn finish(&self,fields:Vec<Self::Field>)->Self::Form;
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
