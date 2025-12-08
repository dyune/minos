use std::collections::HashMap;

pub const FRAME_SIZE: usize = 4;
pub const DEMAND_PAGE_LIMIT: usize = 2;
pub const MEM_SIZE: usize = 1024;
pub const VAR_SIZE: usize = 100;
pub const NUM_FRAMES: usize = MEM_SIZE / FRAME_SIZE;

#[derive(Debug)]
pub struct VarMemory {
    size: usize,
    var_mem: Vec<VarEntry>,
}

impl VarMemory {
    pub(crate) fn new(size: usize) -> VarMemory {
        VarMemory{
            size,
            var_mem: vec![VarEntry::new(); size]
        }
    }
    
    pub(crate) fn get(&self, key: &str) -> Option<String> {
        for ent in &self.var_mem {
            if let Some(ref k) = ent.key {
                if k == key {
                    let str = ent.val.clone();
                    if let Some(s) = str {
                        return Some(s)
                    }
                    panic!("fatal: could not allocate space for a variable ")
                }
            }
        }
        None
    }
    
    pub(crate) fn set(&mut self, key: String, val: String) {
        for ent in self.var_mem.iter_mut() {
            if ent.key == None {
                ent.key = Some(key);
                ent.val = Some(val);
                return
            }
        }
    }
}

#[derive(Clone, Debug)]
struct VarEntry {
    key: Option<String>,
    val: Option<String>,
}

impl VarEntry {
    fn new() -> VarEntry {
        VarEntry{
            key: None, 
            val: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Frame {
    valid: bool,
    id: usize,
    owner_pid: isize,
}

impl Frame {
    pub(crate) fn new(id: usize) -> Frame {
        Frame{
            valid: false,
            id,
            owner_pid: -1
        }
    }
    
    pub(crate) fn set_invalid(&mut self) {
        self.valid = false;
        self.owner_pid = -1;
    }
    
    pub(crate) fn set_valid(&mut self, owner_id: isize) {
        self.valid = true;
        self.owner_pid = owner_id;
    }
}

pub struct FrameTable {
    frames: Vec<Frame>,
}

impl FrameTable {
    pub(crate) fn new() -> FrameTable {
        let mut vec: Vec<Frame> = vec![];
        for i in 0..NUM_FRAMES {
            vec.push(Frame::new(i));
        }
        FrameTable{frames: vec}
    }
    
    fn find_free_frame(&self) -> isize {
        for (idx, frame) in self.frames.iter().enumerate() {
            if !frame.valid {
                return idx as isize
            }
        }
        -1
    }
    
    pub(crate) fn alloc_frames(&mut self, num_frames: usize) -> Vec<isize> {
        let mut free: Vec<isize> = vec![];
        
        for _ in 0..num_frames {
            let idx = self.find_free_frame();
            if idx != -1 {
                free.push(idx);
                let frame = self.frames.get_mut(idx as usize);
                if let Some(frame) = frame {
                    frame.valid = true;
                } else {
                    let err = format!("Error retrieving frame: {:?}", frame);
                    panic!("{err}")
                }
            } else {
                panic!("TODO: Implement page replacement")
            }
        }
        free
    }
}

#[derive(Debug)]
pub struct ProgMemory {
    size: usize,
    prog_mem: Vec<ProgEntry>,
}

impl ProgMemory {
    pub(crate) fn new(size: usize) -> ProgMemory {
        ProgMemory{
            size,
            prog_mem: vec![ProgEntry::new(); size]
        }
    }
    
    pub(crate) fn read(&self, idx: usize) -> String {
        if idx > self.size {
            let size = self.size;
            self.fatal_dump(format!("Write out of bounds: {idx} when valid range is [0, {size}").as_str());
            panic!()
        }
        self.prog_mem[idx].line.clone()
    }
    
    pub(crate) fn write(&mut self, idx: usize, val: String) {
        if idx > self.size {
            let size = self.size;
            self.fatal_dump(format!("Write out of bounds: {idx} when valid range is [0, {size}").as_str());
            panic!()
        }
        self.prog_mem[idx].line = val;
    }
    
    pub(crate) fn write_from_frame(&mut self, frame_idx: usize, offset: usize, val: String) {
        let idx = frame_idx * FRAME_SIZE + offset;
        if idx > self.size {
            let size = self.size;
            self.fatal_dump(format!("Write out of bounds: {idx} when valid range is [0, {size}").as_str());
            panic!()
        }
        self.prog_mem[idx].line = val;
    }
    
    pub(crate) fn fatal_dump(&self, msg: &str) {
        eprintln!("\n===== FATAL ERROR =====");
        eprintln!("ERROR: {}", msg);
        eprintln!("===== MEMORY DUMP =====");
        
        eprintln!("Starting at index 0 below");
        // Iterate through memory contents and print them
        for (index, entry) in self.prog_mem.iter().enumerate() {
            eprintln!("[{:04}]: {}", index, entry.line);
        }

        panic!("The above error occurred while attempting a memory operation.")
    }
}

#[derive(Clone, Debug)]
struct ProgEntry {
    line: String,
}

impl ProgEntry {
    pub(crate) fn new() -> ProgEntry {
        ProgEntry{line: String::new()}
    }
    
    pub(crate) fn reset(&mut self) {
        self.line = String::new()
    }
}


