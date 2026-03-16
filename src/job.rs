use std::{
    rc::Rc,
    sync::atomic::{AtomicIsize, Ordering},
};
use crate::kernel::{Kernel};
pub(crate) use crate::program::Program;
use crate::shellmemory::{FRAME_SIZE};

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
