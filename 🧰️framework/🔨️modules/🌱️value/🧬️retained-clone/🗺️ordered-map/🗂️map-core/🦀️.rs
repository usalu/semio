//! 🗂️ Map core of the retained ordered map: the page capacity, the map record, its default, lookup and iteration implementations and the serde and retirement bridges. See [`super`].

use super::*;

pub const RETAINED_ORDERED_MAP_PAGE_CAPACITY: usize = 16;

/// 🗺️ Stores strictly ordered entries in fixed-capacity native pages with resumable directory growth.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetainedOrderedMap<K, V> {
    pub(super) pages: Vec<Vec<(K, V)>>,
    pub(super) len: usize,
}

impl<K, V> Default for RetainedOrderedMap<K, V> {
    fn default() -> Self {
        Self { pages: Vec::new(), len: 0 }
    }
}

impl<K, V> RetainedOrderedMap<K, V> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn get_index(&self, ordinal: usize) -> Option<(&K, &V)> {
        if ordinal >= self.len {
            return None;
        }
        let page = self.pages.get(ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY)?;
        let (key, value) = page.get(ordinal % RETAINED_ORDERED_MAP_PAGE_CAPACITY)?;
        Some((key, value))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.pages.iter().flat_map(|page| page.iter().map(|(key, value)| (key, value)))
    }

    /// 🧺️ Surrenders every entry in key order so a failed construction retires each value through its own owner.
    pub fn into_entries(self) -> impl Iterator<Item = (K, V)> {
        self.pages.into_iter().flatten()
    }

    #[cfg(test)]
    pub(crate) fn from_sorted_entries_for_test(entries: Vec<(K, V)>) -> Result<Self, crate::ValueError>
    where
        K: Ord,
    {
        if entries.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvalidValue, "retained ordered-map fixture entries are not strictly ordered"));
        }
        let len = entries.len();
        let page_count = len.div_ceil(RETAINED_ORDERED_MAP_PAGE_CAPACITY);
        let mut pages = Vec::with_capacity(page_count.checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map fixture page count overflow"))?);
        let mut entries = entries.into_iter();
        for _ in 0..page_count {
            let mut page = Vec::with_capacity(RETAINED_ORDERED_MAP_PAGE_CAPACITY);
            page.extend(entries.by_ref().take(RETAINED_ORDERED_MAP_PAGE_CAPACITY));
            pages.push(page);
        }
        Ok(Self { pages, len })
    }
}

/// 🧊️ Constructs and edits standalone cold dictionaries before retained admission.
impl<K: Ord, V> RetainedOrderedMap<K, V> {
    pub fn new() -> Self { Self::default() }
    pub fn get<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&V> where K: std::borrow::Borrow<Q> {
        let ordinal = self.search(key).ok()?;
        self.get_index(ordinal).map(|(_, value)| value)
    }
    fn search<Q: Ord + ?Sized>(&self, key: &Q) -> Result<usize,usize> where K: std::borrow::Borrow<Q> {
        let mut lower=0;let mut upper=self.len;
        while lower<upper {let middle=lower+(upper-lower)/2;match self.get_index(middle).expect("dense map ordinal").0.borrow().cmp(key){Ordering::Less=>lower=middle+1,Ordering::Greater=>upper=middle,Ordering::Equal=>return Ok(middle)}}
        Err(lower)
    }
    pub fn cold_insert(&mut self, key: K, value: V) -> Option<V> {
        let ordinal=match self.search(&key){Ok(ordinal)=>return Some(std::mem::replace(&mut self.pages[ordinal/RETAINED_ORDERED_MAP_PAGE_CAPACITY][ordinal%RETAINED_ORDERED_MAP_PAGE_CAPACITY].1,value)),Err(ordinal)=>ordinal};
        if self.pages.last().is_none_or(|page|page.len()==RETAINED_ORDERED_MAP_PAGE_CAPACITY){self.pages.push(Vec::with_capacity(RETAINED_ORDERED_MAP_PAGE_CAPACITY));}
        let mut carry=(key,value);let mut slot=ordinal%RETAINED_ORDERED_MAP_PAGE_CAPACITY;
        for page in &mut self.pages[ordinal/RETAINED_ORDERED_MAP_PAGE_CAPACITY..] {
            let next=if page.len()==RETAINED_ORDERED_MAP_PAGE_CAPACITY{page.pop()}else{None};page.insert(slot,carry);
            match next{Some(next)=>{carry=next;slot=0},None=>break}
        }
        self.len+=1;None
    }
    pub fn cold_remove<Q: Ord + ?Sized>(&mut self, key:&Q)->Option<V> where K:std::borrow::Borrow<Q> {
        let ordinal=self.search(key).ok()?;let page=ordinal/RETAINED_ORDERED_MAP_PAGE_CAPACITY;
        let (_,value)=self.pages[page].remove(ordinal%RETAINED_ORDERED_MAP_PAGE_CAPACITY);
        for index in page..self.pages.len()-1 {let next=self.pages[index+1].remove(0);self.pages[index].push(next);}
        self.len-=1;if self.pages.last().is_some_and(Vec::is_empty){self.pages.pop();}Some(value)
    }
}

impl<K:Ord,V> FromIterator<(K, V)> for RetainedOrderedMap<K, V> {
    fn from_iter<I:IntoIterator<Item=(K,V)>>(entries:I)->Self{let mut map=Self::default();for(key,value)in entries{map.cold_insert(key,value);}map}
}

impl<K:Ord,V,const N:usize> From<[(K,V);N]> for RetainedOrderedMap<K,V>{fn from(entries:[(K,V);N])->Self{entries.into_iter().collect()}}

impl<'de,K:Ord+serde::Deserialize<'de>,V:serde::Deserialize<'de>> serde::Deserialize<'de> for RetainedOrderedMap<K, V> {
    fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{
        struct Visitor<K,V>(std::marker::PhantomData<(K,V)>);
        impl<'de,K:Ord+serde::Deserialize<'de>,V:serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<K,V>{
            type Value=RetainedOrderedMap<K,V>;
            fn expecting(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{formatter.write_str("an object with unique ordered keys")}
            fn visit_map<M:serde::de::MapAccess<'de>>(self,mut input:M)->Result<Self::Value,M::Error>{let mut map=RetainedOrderedMap::default();while let Some((key,value))=input.next_entry::<K,V>()?{if map.get(&key).is_some(){return Err(serde::de::Error::custom("duplicate ordered-map key"));}map.cold_insert(key,value);}Ok(map)}
        }
        deserializer.deserialize_map(Visitor(std::marker::PhantomData))
    }
}

impl<K: Serialize, V: Serialize> Serialize for RetainedOrderedMap<K, V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.len))?;
        for (key, value) in self.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

impl<K: RetireOwned, V: RetireOwned> RetireOwned for RetainedOrderedMap<K, V> {
    fn retirement(self) -> Box<dyn crate::retirement::RetirementCursor> {
        self.pages.retirement()
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { self.pages.retirement_birth_bytes() }
    fn controlled_retirement_supported() -> bool { K::controlled_retirement_supported() && V::controlled_retirement_supported() }
}
