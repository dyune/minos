use std::cmp::PartialEq;
use std::collections::VecDeque;
use crate::job::Job;
use crate::shellmemory::Frame;

#[derive(PartialEq)]
pub(crate) enum Mode {
    FCFS,
    RR,
}

pub(crate) struct Kernel<'a> {
    pub(crate) job_queue: VecDeque<Job>, // A Job should not outlive the Kernel
    pub(crate) lru_cache: VecDeque<&'a Frame>,
    mode: Mode
}

impl<'a> Kernel<'a> {
    
    pub(crate) fn new(mode: Mode) -> Kernel<'a> {
        Kernel{
            job_queue: VecDeque::new(),
            lru_cache: VecDeque::new(),
            mode
        }
    }
    
    pub(crate) fn queue_job(&mut self, job: Job) {
        match self.mode {
            Mode::FCFS => {
                self.job_queue.push_back(job)
            },
            Mode::RR => {
                self.job_queue.push_back(job)
            }
        }
    }
    
    pub(crate) fn pop_job(&mut self, job: Job) -> Option<Job> { 
        self.job_queue.pop_front()
    }
    
    // pub(crate) fn execute_schedule(&mut self) -> Result<String, String> {
    //     Ok(())
    // }
    
}

