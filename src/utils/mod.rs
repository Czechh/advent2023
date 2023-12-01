use std::io::Read;

pub fn read_to_vec(file_name: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    std::fs::File::open(file_name)
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();

    buf
}
