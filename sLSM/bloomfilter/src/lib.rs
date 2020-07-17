pub mod bloom;
pub mod fnv_1a; 
pub mod hashmap;

pub use crate::bloom::BloomFilter;
pub use crate::hashmap::HashMap;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
