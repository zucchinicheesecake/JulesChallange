pub mod blockchain;
pub mod consensus;
pub mod wallet;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
