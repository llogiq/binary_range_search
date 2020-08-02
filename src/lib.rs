#![no_std]

use core::ops::Range;
use core::borrow::Borrow;

/// Search for a range of values within a sorted slice.
///
/// As it uses binary search, worst-case runtime bound is _O(log N)_.
///
/// ```
///# use binary_range_search::search_by;
/// fn lt(x: &u8, y: &u8) -> bool { x < y }
/// assert!(search_by(&[0_u8; 0], 0..0_u8, lt).is_empty());
/// assert!(search_by(&[0_u8], 1..2, lt).is_empty());
/// assert_eq!(&[1], search_by(&[1_u8], 0..2, lt));
/// assert_eq!(&[5, 6], search_by(&(0..10).collect::<Vec<u8>>(), 5..7, lt));
/// assert_eq!(&[0], search_by(&(0..10).collect::<Vec<u8>>(), 0..1, lt));
/// assert_eq!(&[9], search_by(&(0..10).collect::<Vec<u8>>(), 9..11, lt));
/// ```
pub fn search_by<K, T, L: Fn(&T, &K) -> bool>(ts: &[T], r: Range<K>, lt: L) -> &[T] {
	let mut size = ts.len();
	if size == 0 {
		return ts;
	}
	let mut base = 0usize;
	while size > 1 {
		let half = size / 2;
		let mid = base + half;
		let cmp = &ts[mid];
		if lt(cmp, r.start.borrow()) {
			base = mid;
		} else if lt(cmp, r.end.borrow()) {
			// we have now two base..mid to search for the start
			// and mid..(base + size) 
			let mut lbase = base;
			let mut lsize = half;
			while lsize > 1 {
				let lhalf = lsize / 2;
				let lmid = lbase + lhalf;
				let cmp = &ts[lmid];
				if lt(cmp, r.start.borrow()) {
					lbase = lmid;
				}
				lsize -= lhalf;
			}
			if lt(&ts[lbase], r.start.borrow()) {
				lbase += 1;
			}
			let mut rbase = mid;
			let mut rsize = size - half;
			while rsize > 1 {
				let rhalf = rsize / 2;
				let rmid = rbase + rhalf;
				let cmp = &ts[rmid];
				if lt(cmp, r.end.borrow()) {
					rbase = rmid;
				}
				rsize -= rhalf;
			}
			if lt(&ts[rbase], r.end.borrow()) {
				rbase += 1;
			}
			return &ts[lbase..rbase];
		} // else cmp > range.end so the range is left of mid
		size -= half;
	}
	let cmp = &ts[base];
	if lt(cmp, r.end.borrow()) && !lt(cmp, r.start.borrow()) {
		&ts[base..base + 1]
	} else {
		&[]
	}
}
