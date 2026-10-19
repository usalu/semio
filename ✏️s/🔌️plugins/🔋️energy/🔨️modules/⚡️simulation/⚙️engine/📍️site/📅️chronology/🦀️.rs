//! 📅️ Hourly authored source chronology, exact calendar selection and explicit warmup-day neighbours.
use super::{WeatherCalendar,WeatherRecord};
use semio_s_plugin_energy_engine_calendar::SimDate;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NeighbourPolicy{Continuous,RepeatDay}
fn month_days(year:u16,month:u8)->u8{let leap=year%4==0&&(year%100!=0||year%400==0);match month{1|3|5|7|8|10|12=>31,4|6|9|11=>30,2=>if leap{29}else{28},_=>0}}
fn valid_date(year:u16,month:u8,day:u8)->bool{year!=0&&day!=0&&day<=month_days(year,month)}
/// 📆️ Synthetic inputs use exact calendar days while advancing by whole month fragments.
pub fn date_after_days(mut date:SimDate,mut days:u32)->Option<SimDate>{if !valid_date(date.year,date.month,date.day){return None;}loop{let remaining=(month_days(date.year,date.month)-date.day)as u32;if days<=remaining{date.day+=days as u8;return Some(date);}days-=remaining+1;date.day=1;if date.month==12{date.year=date.year.checked_add(1)?;date.month=1;}else{date.month+=1;}}}
fn seasonal_key(month:u8,day:u8,hour:u8)->u64{((month as u64-1)*31+day as u64-1)*24+hour as u64}
fn key(policy:WeatherCalendar,record:WeatherRecord,origin:WeatherRecord)->u64{let seasonal=seasonal_key(record.month,record.day,record.hour);match policy{WeatherCalendar::ActualDates=>record.year as u64*12*31*24+seasonal,WeatherCalendar::TypicalYear=>if seasonal<seasonal_key(origin.month,origin.day,origin.hour){seasonal+12*31*24}else{seasonal}}}
/// 🕒️ One immutable source row is admitted per retained weather-copy turn.
pub fn admit_row(policy:WeatherCalendar,first:WeatherRecord,previous:Option<WeatherRecord>,current:WeatherRecord)->bool{
 if !valid_date(current.year,current.month,current.day)||current.hour>23||![0,60].contains(&current.minute){return false;}
 let Some(previous)=previous else{return true;};
 if key(policy,current,first)<=key(policy,previous,first){return false;}
 let mut date=SimDate::new(previous.year,previous.month,previous.day);let hour=if previous.hour==23{if date.year==u16::MAX&&date.month==12&&date.day==31{return false;}date.advance_day();0}else{previous.hour+1};
 current.hour==hour&&current.month==date.month&&current.day==date.day&&(policy==WeatherCalendar::TypicalYear||current.year==date.year)
}
/// 🔎️ Binary selection reads validated original timestamp keys without allocating an index.
pub fn record_index(policy:WeatherCalendar,len:usize,mut record:impl FnMut(usize)->Option<WeatherRecord>,date:SimDate,hour:u8)->Option<usize>{
 if !valid_date(date.year,date.month,date.day)||hour>23{return None;}let first=record(0)?;let mut target=first;target.year=date.year;target.month=date.month;target.day=date.day;target.hour=hour;let target=key(policy,target,first);let(mut low,mut high)=(0,len);
 while low<high{let middle=low+(high-low)/2;if key(policy,record(middle)?,first)<target{low=middle+1;}else{high=middle;}}
 (low<len&&key(policy,record(low)?,first)==target).then_some(low)
}
/// 🌗️ Finite input boundaries hold the closest authored record; warmup repeats only its chosen complete day.
pub fn neighbours(len:usize,mut record:impl FnMut(usize)->Option<WeatherRecord>,index:usize,policy:NeighbourPolicy)->Option<[WeatherRecord;3]>{
 let current=record(index)?;let(previous,next)=match policy{NeighbourPolicy::Continuous=>(index.saturating_sub(1),index.checked_add(1)?.min(len.checked_sub(1)?)),NeighbourPolicy::RepeatDay=>{let start=index.checked_sub(current.hour as usize)?;let end=record(start.checked_add(23)?)?;if (end.year,end.month,end.day,end.hour)!=(current.year,current.month,current.day,23){return None;}(start+(current.hour as usize+23)%24,start+(current.hour as usize+1)%24)}};Some([record(previous)?,current,record(next)?])
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
