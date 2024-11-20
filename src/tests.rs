

#[cfg(test)]
pub mod exact {
    use std::{fs::File, io::{BufReader, Cursor}};

    use crate::read_key;


    #[test]
    fn test_keys() {
        let mut f = File::open("test_data/keys").expect("couldnt open file");
        let mut reader = BufReader::new(f);
        read_key(&mut reader);
        assert!(false)

    }

}

