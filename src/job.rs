use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use crate::shellmemory::{FrameTable, ProgMemory, FRAME_SIZE};

#[derive(Clone, Debug)]
pub(crate) struct Job<> {
    pc: isize,
    size: usize,
    pid: usize,
    filename: String,
    program: Rc<Program>
}

#[derive(Clone, Debug)]
pub(crate) struct Program {
    filename: String,
    size: usize,
    page_table: Vec<isize>,
}

impl Program {
    pub(crate) fn new(
        ft: &mut FrameTable, 
        p_mem: &mut ProgMemory, 
        filename: &str,
    ) -> Result<Program, String> {
        let mut size = 0;
        let file = File::open(filename);
        let mut lines: Vec<String> = vec![];
        if let Ok(f) = file {
            let reader = BufReader::new(f);
            for (idx, line) in reader.lines().enumerate() {
                match line {
                    Ok(ln) => {lines.push(ln); size += 1;}
                    Err(e) => {
                        return Err(
                            format!("failed to read line {} from {} due to {}", idx, filename, e)
                        )
                    } 
                }
            }
        } else {
            return Err(format!("failed to open {}", filename))
        }
        
        let page_table = ft.alloc_frames(size / FRAME_SIZE);
        let mut line_count = 0;
        for frame_idx in &page_table {
            let mut frame_count = 0;
            while line_count < size && frame_count < 4 {
                let ln = lines.get(line_count);
                if let Some(l) = ln {
                    p_mem.write_from_frame(*frame_idx as usize, line_count, l.clone());
                    frame_count += 1;
                    line_count += 1;
                } else {
                    return Err(format!("allocation error for {}", filename))
                }
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

