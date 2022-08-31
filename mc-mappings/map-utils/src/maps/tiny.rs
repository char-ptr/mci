use std::io::Read;

pub fn parse_tiny_mappings<R:Read>(reader:&mut R) {
    let mut data_buf = String::new();
    reader.read_to_string(&mut data_buf);

    

}
