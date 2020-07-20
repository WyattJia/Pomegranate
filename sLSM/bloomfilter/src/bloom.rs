extern crate bit_vec;
extern crate rand;

use std::cmp;
use std::f64;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use rand::prelude::*;
use bit_vec::BitVec;

use fnv_1a::fnv_1a;


pub struct Bloom<T: ?Sized> {
    bitmap: BitVec,
    bitmap_bits: u64,
    k_num: u32,
    fnv: fnv_1a, // need not ?

    _phantom: PhantomData<T>,
}


impl<T: ?Sized> Bloom<T> {

    /*
     * Pub
     * double denom = 0.480453013918201; // (ln(2))^2
     * double size = -1 * (double) n * (log(fp) / denom);
     * m_bits = vector<bool>((int) size);
     * double ln2 = 0.693147180559945;
     * m_numHashes = (int) ceil( (size / n) * ln2);  // ln(2)
     * Priv
     * uint8_t m_numHashes;
     * vector<bool> m_bits;

     * void add(const Key *data)
     * bool mayContain(const Key *data, size_t len)
     * uint64_t nthHash(uint32_t n, uint64_t hashA, uint64_t hashB, uint64_t filterSize)
     * array<uint64_t, 2> hash(const Key *data, size_t len)
     *
     * */


    pub fn new(bitmap_size: usize, items_count: usize) -> Self {
        assert!(bitmap_size > 0 && items_count > 0);
        let bitmap_bits = (bitmap_size as u64) * 8u64;
        let k_num = Self::optimal_k_num(bitmap_bits, items_count);
        let bitmap = BitVec::from_elem(bitmap_bits as usize, false);
        let sips = [Self::sip_new(), Self::sip_new()];
        Self {
            bitmap,
            bitmap_bits,
            k_num,
            sips,
            _phantom: PhantomData,
        }
    }

    /// Create a new bloom filter structure.
    /// items_count is an estimation of the maximum number of items to store.
    /// fp_p is the wanted rate of false positives, in ]0.0, 1.0[
    pub fn new_for_fp_rate(items_count: usize, fp_p: f64) -> Self {
        let bitmap_size = Self::compute_bitmap_size(items_count, fp_p);
        Bloom::new(bitmap_size, items_count)
    }

    /// Create a bloom filter structure with an existing state.
    /// The state is assumed to be retrieved from an existing bloom filter.
    pub fn from_existing(
        bitmap: &[u8],
        bitmap_bits: u64,
        k_num: u32,
        sip_keys: [(u64, u64); 2],
        ) -> Self {
        let sips = [
            SipHasher13::new_with_keys(sip_keys[0].0, sip_keys[0].1),
            SipHasher13::new_with_keys(sip_keys[1].0, sip_keys[1].1),
        ];
        Self {
            bitmap: BitVec::from_bytes(bitmap),
            bitmap_bits,
            k_num,
            sips,
            _phantom: PhantomData,
        }
    }

    /// Compute a recommended bitmap size for items_count items
    /// and a fp_p rate of false positives.
    /// fp_p obviously has to be within the ]0.0, 1.0[ range.
    pub fn compute_bitmap_size(items_count: usize, fp_p: f64) -> usize {
        assert!(items_count > 0);
        assert!(fp_p > 0.0 && fp_p < 1.0);
        let log2 = f64::consts::LN_2;
        let log2_2 = log2 * log2;
        ((items_count as f64) * f64::ln(fp_p) / (-8.0 * log2_2)).ceil() as usize
    }

    /// Record the presence of an item.
    pub fn set(&mut self, item: &T)
        where
        T: Hash,
        {
            let mut hashes = [0u64, 0u64];
            for k_i in 0..self.k_num {
                let bit_offset = (self.bloom_hash(&mut hashes, &item, k_i) % self.bitmap_bits) as usize;
                self.bitmap.set(bit_offset, true);
            }
        }

    /// Check if an item is present in the set.
    /// There can be false positives, but no false negatives.
    pub fn check(&self, item: &T) -> bool
        where
        T: Hash,
        {
            let mut hashes = [0u64, 0u64];
            for k_i in 0..self.k_num {
                let bit_offset = (self.bloom_hash(&mut hashes, &item, k_i) % self.bitmap_bits) as usize;
                if self.bitmap.get(bit_offset).unwrap() == false {
                    return false;
                }
            }
            true
        }

    /// Record the presence of an item in the set,
    /// and return the previous state of this item.
    pub fn check_and_set(&mut self, item: &T) -> bool
        where
        T: Hash,
        {
            let mut hashes = [0u64, 0u64];
            let mut found = true;
            for k_i in 0..self.k_num {
                let bit_offset = (self.bloom_hash(&mut hashes, &item, k_i) % self.bitmap_bits) as usize;
                if self.bitmap.get(bit_offset).unwrap() == false {
                    found = false;
                    self.bitmap.set(bit_offset, true);
                }
            }
            found
        }

    /// Return the bitmap as a vector of bytes
    pub fn bitmap(&self) -> Vec<u8> {
        self.bitmap.to_bytes()
    }

    /// Return the number of bits in the filter
    pub fn number_of_bits(&self) -> u64 {
        self.bitmap_bits
    }

    /// Return the number of hash functions used for `check` and `set`
    pub fn number_of_hash_functions(&self) -> u32 {
        self.k_num
    }

    /// Return the keys used by the sip hasher
    pub fn sip_keys(&self) -> [(u64, u64); 2] {
        [self.sips[0].keys(), self.sips[1].keys()]
    }

    fn optimal_k_num(bitmap_bits: u64, items_count: usize) -> u32 {
        let m = bitmap_bits as f64;
        let n = items_count as f64;
        let k_num = (m / n * f64::ln(2.0f64)).ceil() as u32;
        cmp::max(k_num, 1)
    }

    fn bloom_hash(&self, hashes: &mut [u64; 2], item: &T, k_i: u32) -> u64
        where
            T: Hash,
        {
            if k_i < 2 {
                let sip = &mut self.sips[k_i as usize].clone();
                item.hash(sip);
                let hash = sip.finish();
                hashes[k_i as usize] = hash;
                hash
            } else {
                (hashes[0] as u128).wrapping_add((k_i as u128).wrapping_mul(hashes[1] as u128)) as u64
                    % 0xffffffffffffffc5
            }
        }

    /// Clear all of the bits in the filter, removing all keys from the set
    pub fn clear(&mut self) {
        self.bitmap.clear()
    }

    fn sip_new() -> SipHasher13 {
        let mut rng = thread_rng();
        SipHasher13::new_with_keys(rng.gen(), rng.gen())
    }
}

