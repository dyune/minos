use std::{
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
    sync::atomic::{AtomicIsize, Ordering},
};
use crate::kernel::{Kernel};
use crate::shellmemory::{FrameTable, ProgMemory, FRAME_SIZE};

#[derive(Debug, Clone)]
pub(crate) enum FailProgramCreation{
    ExistsAlready,
    Error(String)
}

static GLOBAL_PID: AtomicIsize = AtomicIsize::new(0);

fn assign_pid() -> isize {
    GLOBAL_PID.fetch_add(1, Ordering::Relaxed)
}

pub(crate) fn find_mem_idx(frame_idx: usize, pc: usize) -> usize {
    let offset = pc % FRAME_SIZE;
    FRAME_SIZE * frame_idx + offset
}

#[derive(Clone, Debug)]
pub(crate) struct Job {
    pub(crate) pid : isize,
    pub(crate) pc: usize,
    pub(crate) size: usize,
    pub(crate) filename: String,
    pub(crate) program: Rc<Program>
}

impl Job {
    pub(crate) fn new<'a>(
        size: Option<usize>,
        filename: String,
        program: Option<Program>,
        kern: &mut Kernel,
    ) -> Result<Job, &'a str> {
        
        // This option is left for when a job already exists
        if program.is_none() && size.is_none() {
            for job in kern.job_queue.iter() {
                if job.filename == filename {
                    return Ok(
                        Job{
                            pid: assign_pid(),
                            pc: 0,
                            size: job.program.size,
                            filename,
                            program: Rc::clone(&job.program)
                        }
                    )
                }
            }
            return Err("Illegal state, no job found")
        }

        if size.is_none() {
            return Err("Illegal state, abort execution")
        }
        
        Ok(
            Job{
                pid: assign_pid(),
                pc: 0,
                size: size.unwrap(),
                filename,
                program: Rc::new(program.unwrap()),
            }
        )
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Program {
    pub(crate) filename: String,
    pub(crate) size: usize,
    pub(crate) page_table: Vec<isize>,
}

impl Program {
    pub(crate) fn new(
        kern: &mut Kernel,
        filename: &str,
    ) -> Result<Program, FailProgramCreation> {
        
        let mut size = 0;
        let file = File::open(filename);
        let mut lines: Vec<String> = vec![];
        
        for job in kern.job_queue.iter() {
            if job.filename == filename {
                dbg!("exists");
                return Err(FailProgramCreation::ExistsAlready)
            }
        }
        
        if let Ok(f) = file {
            let reader = BufReader::new(f);
            for (idx, line) in reader.lines().enumerate() {
                match line {
                    Ok(ln) => { lines.push(ln); size += 1; }
                    Err(e) => {
                        return Err(
                            FailProgramCreation::Error(
                                format!("failed to read line {} from {} due to {}", idx, filename, e)
                            )
                        )
                    } 
                }
            }
        } else {
            return Err(
                FailProgramCreation::Error(
                    format!("failed to open {}", filename)
                )
            )
        }
        
        let num_frames = (size + FRAME_SIZE - 1) / FRAME_SIZE;
        let ft = kern.get_mut_ft();
        let page_table = ft.alloc_frames(num_frames, String::from(filename));
        
        let mut line_count = 0;
        let p_mem = kern.get_mut_pmem();
        
        for frame_idx in &page_table {
            let mut frame_count = 0;
            
            // Allocate at most 4 frames FOR NOW
            while line_count < size && frame_count < num_frames {
                for _ in 0..FRAME_SIZE {
                    
                    if line_count == size { break }
                    
                    let ln = lines.get(line_count);
                    
                    if let Some(l) = ln {
                        p_mem.write_to_frame(*frame_idx as usize, line_count, l.clone());
                        line_count += 1;
                    } else {
                        break
                    }
                }
                frame_count += 1;
            } 
        }
        
        Ok(
            Program{
                filename: filename.parse().unwrap(),
                size,
                page_table
            }
        )
    }
}

