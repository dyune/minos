use std::collections::VecDeque;
use crate::job::Job;
use crate::shellmemory::Frame;

pub(crate) struct Kernel<'a> {
    job_queue: VecDeque<Job>, // A Job should not outlive the Kernel
    lru_cache: VecDeque<&'a Frame>
}

impl<'a> Kernel<'a> {
    pub(crate) fn new() -> Kernel<'a> {
        Kernel{
            job_queue: VecDeque::new(),
            lru_cache: VecDeque::new(),
        }
    }
    
}
