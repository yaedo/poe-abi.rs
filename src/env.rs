use resource::Resource;
use std::io::Read;

lazy_static! {
    static ref RESOURCE: Resource = Resource::open(b"env://").unwrap();
}

pub fn get(key: &[u8]) -> Option<Vec<u8>> {
    let mut data = vec![];
    match RESOURCE.meta(key)?.read_to_end(&mut data) {
        Ok(_) => Some(data),
        Err(_) => None,
    }
}

pub fn set(key: &[u8], value: &[u8]) -> Option<()> {
    match RESOURCE.meta(key)?.write(value) {
        Ok(_) => Some(()),
        Err(_) => None,
    }
}
