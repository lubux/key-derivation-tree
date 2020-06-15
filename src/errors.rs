use std::fmt;

#[derive(Debug)]
pub struct InvalidAccessError {
    pub key_id : u64
}


impl fmt::Display for InvalidAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The constrained PRF has no access to key with id {}", self.key_id)
    }
}

