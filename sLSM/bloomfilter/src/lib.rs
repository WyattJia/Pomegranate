pub mod bloom;
pub mod fnv_1a; 

pub use crate::bloom::Bloom;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
