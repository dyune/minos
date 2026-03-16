use std::{
    cmp::PartialEq,
    collections::VecDeque,
    mem::drop,
    rc::Rc,
};
use crate::interpreter::interpreter;
use crate::job::{Job, find_mem_idx};
use crate::shellmemory::{Frame, FrameTable, ProgMemory, VarMemory, FRAME_SIZE};

#[derive(PartialEq)]
pub(crate) enum Mode {
    FCFS,
    RR,
    SJF,
}

pub(crate) struct Kernel<'a> {
    pub(crate) job_queue: VecDeque<Job>, // A Job should not outlive the Kernel
    pub(crate) lru_cache: VecDeque<&'a Frame>,
    pub(crate) prog_memory: ProgMemory,
    pub(crate) var_memory: VarMemory,
    pub(crate) frame_table: FrameTable,
    pub(crate) mode: Mode
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
            },
            Mode::SJF => {
                self.queue_sjf(job)
            }
        }
    }
    
    pub(crate) fn queue_sjf(&mut self, job: Job) {
        let len = job.program.size;
        
        for (i, j) in self.job_queue.iter().enumerate() {
            if j.program.size > len {
                self.job_queue.insert(i, job);
                return
            }
        }
        self.job_queue.push_back(job)
    }
    
    pub(crate) fn dealloc_program(&mut self, job: Job) -> Result<usize, &str> {
        let program = job.program;
        let rc = Rc::strong_count(&program);

        // rc == 1 means that we should be the last the program holding a ref to the program
        if rc == 1 {
            dbg!("dropping program");
            let pt = program.page_table.clone();
            for page in pt.iter() {
                let frame = self.frame_table.frames.get_mut(*page as usize);
                if let Some(f) = frame {
                    f.set_invalid();
                } else {
                    panic!("Failed to find frame while deallocating memory")
                }
            }
        }

        // drop a reference to the program or drop entirely if we are the last user
        drop(program);
        Ok(rc - 1)
    }
    
    pub(crate) fn execute_schedule(
        &mut self,
    ) -> Result<(), &str> {

        if self.job_queue.len() == 0 {
            return Err("No job to execute")
        }
        
        let res: Result<(), &str>;
        match self.mode {
            Mode::FCFS => res = self.execute_fcfs(),
            Mode::SJF => res = self.execute_fcfs(),
            Mode::RR => res = self.execute_rr(2),
        }
        res
    }
    
    fn execute_fcfs(&mut self) -> Result<(), &str> {
        while self.job_queue.len() > 0 {
            let job = self.job_queue.pop_front();
            if let Some(j) = job {
                self.execute_whole_program(j);
            } else {
                return Err("Error fetching job")
            }
        }
        Ok(())
    }
    
    fn execute_rr(&mut self, round: usize) -> Result<(), &str> {
        while self.job_queue.len() > 0 {
            let job = self.job_queue.pop_front();
            if let Some(mut j) = job {
                let res = self.execute_rr_program(&mut j, round);
                match res {
                    Some(_) => { self.job_queue.push_back(j); println!("oops"); },
                    None => continue
                }
            } else {
                return Err("Error fetching job")
            }
        }
        Ok(())
    }

    fn execute_rr_program<'b>(&mut self, job: &'b mut Job, rounds: usize) -> Option<&'b Job> {
        let mut line_count = 0;

        // Get necessary data to avoid borrowing self during the operation
        while line_count < rounds && job.pc < job.size {
            println!("LINE COUNT: {}", line_count);
            println!("{} line {}", job.program.filename, job.pc);

            // Calculate page index and validate it's inside the page table
            let page_idx = job.pc / FRAME_SIZE;
            // Check if page_idx is valid for this job
            if !job.program.page_table.contains(&(page_idx as isize)) {
                println!("Invalid page index: {}", page_idx);
                return None; // Or handle this error appropriately
            }

            // Read the line from memory
            let mem_idx = find_mem_idx(page_idx, job.pc);
            let line_text = self.prog_memory.read(mem_idx);
            let line = line_text.split(';').map(str::to_string).collect();

            line_count += 1;

            // Process the line
            interpreter(line, self);
            job.pc += 1;
        }

        // Return None if the job is completed, otherwise return the job
        if job.pc == job.size {
            None
        } else {
            Some(job)
        }
    }
    
    fn execute_whole_program(&mut self, mut job: Job) {
        
        let limit = job.size;
        let pt = &job.program.page_table;
        let mut line_count = 0;

        for page_idx in pt {
            for _ in 0..FRAME_SIZE {

                if limit == line_count {
                    break
                }

                let line = self.prog_memory
                    .read(find_mem_idx(page_idx.clone() as usize, job.pc))
                    .split(';')
                    .map(str::to_string)
                    .collect();

                line_count += 1;

                interpreter(line, self);
                job.pc += 1;
            }
        }
        self.dealloc_program(job).expect("TODO: panic message");
    }

    pub(crate) fn memory_dump(&self) {
        println!("-=-=-=-=-= Dumping Memory =-=-=-=-=-");
        for (i, f) in self.frame_table.frames.iter().enumerate() {
            if f.valid {
                println!("Frame {i}");
                for j in 0..FRAME_SIZE {
                    let line = self.prog_memory.read(i * FRAME_SIZE + j);
                    println!("[{:02}]: {}", j, line);
                }
            }
        }
    }
}

