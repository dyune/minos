use std::cmp::PartialEq;
use std::collections::VecDeque;
use crate::interpreter::interpreter;
use crate::job::Job;
use crate::shellmemory::{Frame, FrameTable, ProgMemory, VarMemory, FRAME_SIZE};

#[derive(PartialEq)]
pub(crate) enum Mode {
    FCFS,
    RR,
}

pub(crate) struct Kernel<'a> {
    pub(crate) job_queue: VecDeque<Job>, // A Job should not outlive the Kernel
    pub(crate) lru_cache: VecDeque<&'a Frame>,
    pub(crate) prog_memory: ProgMemory,
    pub(crate) var_memory: VarMemory,
    pub(crate) frame_table: FrameTable,
    mode: Mode
}

impl<'a> Kernel<'a> {
    
    pub(crate) fn new(
        mode: Mode,
        prog_memory: ProgMemory,
        var_memory: VarMemory,
        frame_table: FrameTable,
    ) -> Kernel<'a> {
        Kernel{
            job_queue: VecDeque::new(),
            lru_cache: VecDeque::new(),
            prog_memory,
            var_memory,
            frame_table,
            mode
        }
    }
    
    pub(crate) fn get_mut_pmem(&mut self) -> &mut ProgMemory {
        &mut self.prog_memory
    }
    
    pub(crate) fn get_mut_varmem(&mut self) -> &mut VarMemory {
        &mut self.var_memory
    }
    
    pub(crate) fn get_mut_ft(&mut self) -> &mut FrameTable {
        &mut self.frame_table
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
    
    pub(crate) fn terminate_job(&mut self, job: Job) -> Option<Job> { 
        self.job_queue.pop_front()
    }
    
    pub(crate) fn execute_schedule(
        &mut self,
    ) -> Result<&str, &str> {
        if self.job_queue.len() == 0 {
            return Err("No job to execute")
        }
        while self.job_queue.len() > 0 {
            let job = self.job_queue.pop_front();
            if let Some(mut j) = job {
                self.execute_fcfs(&mut j);
            } else {
                return Err("Error fetching job")
            }
        }
        Ok(("cock"))
    }
    
    fn execute_fcfs(&mut self, job: &mut Job) {
        
        let limit = job.size;
        let pt = &job.program.page_table;
        let mut line_count = 0;

        for page_idx in pt {
            for _ in 0..FRAME_SIZE {

                if limit == line_count {
                    break
                }

                let line = self.prog_memory
                    .read(crate::job::find_mem_idx(page_idx.clone() as usize, job.pc))
                    .split(';')
                    .map(str::to_string)
                    .collect();

                line_count += 1;

                interpreter(line, self);
                job.pc += 1;
            }
        }
    }
}

