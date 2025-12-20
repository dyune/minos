use std::fmt;

#[derive(Debug, Clone)]
pub(crate) enum Exception {
    UnknownCommand(String),
    IllegalMemoryAccess(usize),
    IllegalKernelState(String)
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exception::UnknownCommand(cmd) => write!(f, "unrecognized string: {}", cmd),
            Exception::IllegalMemoryAccess(addr) => write!(f, "illegal memory access at: {}", addr),
            Exception::IllegalKernelState(msg) => write!(f, "illegal kernel state: {}", msg),
        }
    }
}

