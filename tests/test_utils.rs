#[cfg(test)]

mod tests {

    use sqlite_database_file_dissect::utils::varint::decode_varint_to_usize;

    #[test]
    fn test_varint(){
        let (v, l) = decode_varint_to_usize(&[130, 52]).unwrap();
        assert_eq!(308, v);
        assert_eq!(2, l)
    }
}
