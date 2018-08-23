use resource::Resource;

lazy_static! {
    static ref RESOURCE: Resource = Resource::open(b"log://").unwrap();
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Level {
    Emergency = 0,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Informational,
    Debug,
}

impl Default for Level {
    fn default() -> Self {
        Level::Informational
    }
}

pub fn write(value: &[u8]) -> Option<()> {
    write_with_level(Level::default(), value)
}

pub fn write_with_level(level: Level, value: &[u8]) -> Option<()> {
    match RESOURCE.write(&[&[level as u8], value].concat()) {
        Ok(_) => Some(()),
        Err(_) => None,
    }
}
