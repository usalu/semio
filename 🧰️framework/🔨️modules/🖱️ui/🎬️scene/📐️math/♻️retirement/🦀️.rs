//! ♻️ Exact component scratch backings keep numeric identity independent of wire labels.

#[derive(Default)]
pub(super) struct NumericSeen {
    slots: Vec<Option<(u64,u32)>>,
    occupied: Vec<usize>,
}
impl NumericSeen {
    pub(super) fn capacity_bytes(count:usize)->Option<usize> { let slots=if count==0{0}else{count.checked_mul(2)?.checked_next_power_of_two()?};slots.checked_mul(std::mem::size_of::<Option<(u64,u32)>>())?.checked_add(count.checked_mul(std::mem::size_of::<usize>())?) }
    pub(super) fn new(count:usize)->Self { let slots=if count==0{0}else{(count*2).next_power_of_two()};Self{slots:vec![None;slots],occupied:Vec::with_capacity(count)} }
    pub(super) fn len(&self)->usize {self.occupied.len()}
    pub(super) fn is_empty(&self)->bool {self.occupied.is_empty()}
    pub(super) fn owns_backing(&self)->bool {self.slots.capacity()!=0||self.occupied.capacity()!=0}
    pub(super) fn first_probe(&self,number:u64)->usize {let mut hash=number;hash^=hash>>30;hash=hash.wrapping_mul(0xbf58476d1ce4e5b9);hash^=hash>>27;hash=hash.wrapping_mul(0x94d049bb133111eb);hash^=hash>>31;(hash as usize)&(self.slots.len()-1)}
    pub(super) fn probe(&self,index:usize)->Option<(u64,u32)> {self.slots[index]}
    pub(super) fn next_probe(&self,index:usize)->usize {(index+1)&(self.slots.len()-1)}
    pub(super) fn insert_at(&mut self,index:usize,number:u64,group:u32) {assert!(self.slots[index].is_none()&&self.occupied.len()<self.occupied.capacity());self.slots[index]=Some((number,group));self.occupied.push(index);}
    pub(super) fn pop_first(&mut self)->Option<(u64,u32)> {self.occupied.pop().and_then(|index|self.slots[index].take())}
}
protocol::value::artifact_retire_struct!(NumericSeen { slots,occupied });

#[derive(Default)]
pub(super) struct IneligibleGroups {
    flags: Vec<u8>,
    offsets: [usize;3],
    count: usize,
}
impl IneligibleGroups {
    pub(super) fn new(lengths:[usize;3])->Self {Self{flags:vec![0;lengths.iter().sum()],offsets:[0,lengths[0],lengths[0]+lengths[1]],count:0}}
    pub(super) fn len(&self)->usize {self.count}
    pub(super) fn is_empty(&self)->bool {self.count==0}
    pub(super) fn owns_backing(&self)->bool {self.flags.capacity()!=0}
    pub(super) fn contains(&self,key:&u64)->bool {let domain=(key>>32)as usize;self.flags.get(self.offsets[domain]+(*key as u32)as usize)==Some(&1)}
    pub(super) fn insert(&mut self,key:u64) {let index=self.offsets[(key>>32)as usize]+(key as u32)as usize;if self.flags[index]==0{self.flags[index]=1;self.count+=1;}}
}
impl protocol::value::retirement::RetireOwned for IneligibleGroups {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::retirement::RetireOwned::retirement(self.flags)}
    fn retirement_birth_bytes(&self)->Option<usize> {protocol::value::retirement::RetireOwned::retirement_birth_bytes(&self.flags)}
    fn controlled_retirement_supported()->bool {true}
}
