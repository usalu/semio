/** 📐️ Reports the bytes a backing file may allocate, which is its declared extent: the physical identity pinned by receipts, equal to the former block count when the backing was preallocated and stable while a sparse backing grows. */
export function nativeImageBackingAllocatedBytesV1<T extends number|bigint>(row:Readonly<{size:T}>):T{return row.size;}
