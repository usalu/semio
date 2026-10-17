//! 📤️ Deflate encoder of the deflate codec: the controlled LZ77 and fixed-Huffman compressor with its progress, control and one-shot entry point. See [`super`].

use super::*;

//#region 🔖️Deflate
/// 🧭️ Explicit physical compression work units without a dependency on artifact controls.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum DeflateEncodePhase{Initialize,ScanInput,MatchSearch,WriteOutput,Finish}

/// 📊️ Slot initialization, input bytes, candidate work, and output bytes use separate phases.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct DeflateEncodeProgress{pub phase:DeflateEncodePhase,pub completed:usize,pub total:usize}

/// 🧮️ The caller admits cumulative owned allocations and cancellation at physical work boundaries.
pub trait DeflateEncodeControl{
    fn admit(&mut self,bytes:usize)->Result<(),ValueError>;
    fn checkpoint(&mut self,progress:DeflateEncodeProgress)->Result<(),ValueError>;
}

/// 🛫️ Produces the exact ordinary fixed-Huffman stream under explicit ownership admission.
pub fn deflate_controlled<C:DeflateEncodeControl>(raw:&[u8],control:&mut C)->Result<Vec<u8>,ValueError>{
    control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::ScanInput,completed:0,total:raw.len()})?;
    if raw.len()>u32::MAX as usize{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Deflate input exceeds position width"))}
    let output_limit=raw.len().checked_mul(9).and_then(|bytes|bytes.checked_add(17)).map(|bytes|bytes/8).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Deflate output capacity overflow"))?;
    let initialize_total=HASH_SIZE.checked_add(raw.len()).and_then(|slots|slots.checked_add(318)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Deflate initialization overflow"))?;
    let owned=HASH_SIZE.checked_add(raw.len()).and_then(|slots|slots.checked_mul(size_of::<u32>())).and_then(|bytes|bytes.checked_add(318*size_of::<u16>())).and_then(|bytes|bytes.checked_add(output_limit)).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Deflate ownership overflow"))?;
    control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::Initialize,completed:0,total:initialize_total})?;
    control.admit(owned)?;
    let mut initialized=0;
    let lengths=fixed_literal_length_lengths();let distances=fixed_distance_lengths();
    let codes=controlled_codes(&lengths,control,&mut initialized,initialize_total)?;
    let distance_codes=controlled_codes(&distances,control,&mut initialized,initialize_total)?;
    let mut head=controlled_positions(HASH_SIZE,control,&mut initialized,initialize_total)?;
    let mut prev=controlled_positions(raw.len(),control,&mut initialized,initialize_total)?;
    control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::Initialize,completed:initialized,total:initialize_total})?;
    control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::WriteOutput,completed:0,total:output_limit})?;
    let mut bytes=Vec::new();bytes.try_reserve_exact(output_limit).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Deflate output allocation"))?;
    let mut writer=BitWriter{bytes,buffer:0,bits:0};writer.write_bits(1,1);writer.write_bits(1,2);
    let mut searched=0usize;let mut emitted=0usize;let mut i=0usize;
    while i<raw.len(){
        let mut best_len=0usize;let mut best_dist=0usize;
        if raw.len()-i>=MIN_MATCH{
            let h=hash3(raw[i],raw[i+1],raw[i+2]);let mut candidate=head[h];let mut chain=0usize;
            while candidate!=u32::MAX&&chain<MAX_CHAIN{
                controlled_search_step(control,&mut searched)?;
                let cpos=candidate as usize;if i-cpos>WINDOW_SIZE{break}
                let max_possible=(raw.len()-i).min(MAX_MATCH);let mut len=0usize;
                while len<max_possible{
                    controlled_search_step(control,&mut searched)?;
                    if raw[cpos+len]!=raw[i+len]{break}len+=1;
                }
                if len>best_len{best_len=len;best_dist=i-cpos;}
                candidate=prev[cpos];chain+=1;
            }
        }
        let consumed=if best_len>=MIN_MATCH{
            let li=length_index_for(best_len as u16);let symbol=257+li;
            writer.write_huffman_code(codes[symbol],lengths[symbol]);
            let extra=LENGTH_EXTRA[li];if extra>0{writer.write_bits((best_len as u16-LENGTH_BASE[li]) as u32,extra as u32);}
            let di=distance_index_for(best_dist as u32);writer.write_huffman_code(distance_codes[di],distances[di]);
            let extra=DIST_EXTRA[di];if extra>0{writer.write_bits(best_dist as u32-DIST_BASE[di],extra as u32);}best_len
        }else{writer.write_huffman_code(codes[raw[i] as usize],lengths[raw[i] as usize]);1};
        if writer.bytes.len()-emitted>=256{emitted=writer.bytes.len();control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::WriteOutput,completed:emitted,total:output_limit})?;}
        let end=i+consumed;while i<end{
            if raw.len()-i>=MIN_MATCH{let h=hash3(raw[i],raw[i+1],raw[i+2]);prev[i]=head[h];head[h]=i as u32;}
            i+=1;if i%256==0{control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::ScanInput,completed:i,total:raw.len()})?;}
        }
    }
    control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::MatchSearch,completed:searched,total:0})?;
    control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::ScanInput,completed:i,total:raw.len()})?;
    writer.write_huffman_code(codes[256],lengths[256]);let bytes=writer.finish();
    if bytes.len()>output_limit{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Deflate output exceeded proved capacity"))}
    control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::WriteOutput,completed:bytes.len(),total:output_limit})?;
    control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::Finish,completed:bytes.len(),total:bytes.len()})?;Ok(bytes)
}

fn controlled_initialize_step<C:DeflateEncodeControl>(control:&mut C,completed:&mut usize,total:usize)->Result<(),ValueError>{
    *completed+=1;if *completed%256==0{control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::Initialize,completed:*completed,total})?;}Ok(())
}

pub(super) fn controlled_search_step<C:DeflateEncodeControl>(control:&mut C,completed:&mut usize)->Result<(),ValueError>{
    *completed=completed.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Deflate match work overflow"))?;
    if *completed%256==0{control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::MatchSearch,completed:*completed,total:0})?;}Ok(())
}

fn controlled_codes<C:DeflateEncodeControl>(lengths:&[u8],control:&mut C,completed:&mut usize,total:usize)->Result<Vec<u16>,ValueError>{
    let mut counts=[0u16;16];let mut next=[0u16;16];for &length in lengths{if length>0{counts[length as usize]+=1;}}
    let mut code=0u16;for bits in 1..16{code=(code+counts[bits-1])<<1;next[bits]=code;}
    control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::Initialize,completed:*completed,total})?;
    let mut codes=Vec::new();codes.try_reserve_exact(lengths.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Deflate code allocation"))?;
    for &length in lengths{let code=if length==0{0}else{let code=next[length as usize];next[length as usize]+=1;code};codes.push(code);controlled_initialize_step(control,completed,total)?;}Ok(codes)
}

pub(super) fn controlled_positions<C:DeflateEncodeControl>(count:usize,control:&mut C,completed:&mut usize,total:usize)->Result<Vec<u32>,ValueError>{
    control.checkpoint(DeflateEncodeProgress{phase:DeflateEncodePhase::Initialize,completed:*completed,total})?;
    let mut positions=Vec::new();positions.try_reserve_exact(count).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Deflate position allocation"))?;
    for _ in 0..count{positions.push(u32::MAX);controlled_initialize_step(control,completed,total)?;}Ok(positions)
}

pub(super) const WINDOW_SIZE: usize = 32768;

pub(super) const MIN_MATCH: usize = 3;

pub(super) const MAX_MATCH: usize = 258;

const HASH_BITS: u32 = 15;

pub(super) const HASH_SIZE: usize = 1 << HASH_BITS;

pub(super) const MAX_CHAIN: usize = 128;

pub(super) fn hash3(a: u8, b: u8, c: u8) -> usize {
    let value = (a as u32) | (b as u32) << 8 | (c as u32) << 16;
    ((value.wrapping_mul(2654435761)) >> (32 - HASH_BITS)) as usize
}

/// 🗜️ One fixed-Huffman (`BTYPE=1`) block over the whole input, found via a greedy hash-chain
/// LZ77 match finder (min match 3, max 258, 32 KiB window, `MAX_CHAIN` probes) — a real matcher,
/// not a stored-block passthrough, so segment payloads still shrink.
pub fn deflate(raw: &[u8]) -> Vec<u8> {
    let lit_len_lengths = fixed_literal_length_lengths();
    let dist_lengths = fixed_distance_lengths();
    let lit_len_codes = canonical_codes(&lit_len_lengths);
    let dist_codes = canonical_codes(&dist_lengths);
    let mut writer = BitWriter::new();
    writer.write_bits(1, 1);
    writer.write_bits(1, 2);
    let mut head = vec![u32::MAX; HASH_SIZE];
    let mut prev = vec![u32::MAX; raw.len()];
    let insert = |position: usize, raw: &[u8], head: &mut [u32], prev: &mut [u32]| {
        if position + MIN_MATCH <= raw.len() {
            let h = hash3(raw[position], raw[position + 1], raw[position + 2]);
            prev[position] = head[h];
            head[h] = position as u32;
        }
    };
    let mut i = 0usize;
    while i < raw.len() {
        let mut best_len = 0usize;
        let mut best_dist = 0usize;
        if i + MIN_MATCH <= raw.len() {
            let h = hash3(raw[i], raw[i + 1], raw[i + 2]);
            let mut candidate = head[h];
            let mut chain = 0usize;
            while candidate != u32::MAX && chain < MAX_CHAIN {
                let cpos = candidate as usize;
                if i - cpos > WINDOW_SIZE {
                    break;
                }
                let max_possible = (raw.len() - i).min(MAX_MATCH);
                let mut len = 0usize;
                while len < max_possible && raw[cpos + len] == raw[i + len] {
                    len += 1;
                }
                if len > best_len {
                    best_len = len;
                    best_dist = i - cpos;
                }
                candidate = prev[cpos];
                chain += 1;
            }
        }
        if best_len >= MIN_MATCH {
            let length_index = length_index_for(best_len as u16);
            let length_symbol = 257 + length_index;
            writer.write_huffman_code(lit_len_codes[length_symbol], lit_len_lengths[length_symbol]);
            let extra_len = LENGTH_EXTRA[length_index];
            if extra_len > 0 {
                writer.write_bits((best_len as u16 - LENGTH_BASE[length_index]) as u32, extra_len as u32);
            }
            let dist_index = distance_index_for(best_dist as u32);
            writer.write_huffman_code(dist_codes[dist_index], dist_lengths[dist_index]);
            let extra_dist = DIST_EXTRA[dist_index];
            if extra_dist > 0 {
                writer.write_bits(best_dist as u32 - DIST_BASE[dist_index], extra_dist as u32);
            }
            let end = i + best_len;
            while i < end {
                insert(i, raw, &mut head, &mut prev);
                i += 1;
            }
        } else {
            writer.write_huffman_code(lit_len_codes[raw[i] as usize], lit_len_lengths[raw[i] as usize]);
            insert(i, raw, &mut head, &mut prev);
            i += 1;
        }
    }
    writer.write_huffman_code(lit_len_codes[256], lit_len_lengths[256]);
    writer.finish()
}
//#endregion 🔖️Deflate
