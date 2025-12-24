mod interpreter;
mod shellmemory;
mod kernel;
mod job;
mod errors;

use {
    std::io::Write,
    std::io,
};
use chrono::Local;

fn main() {
    println!("Minos Shell v0.0.0 - minsh");
    let var_mem = shellmemory::VarMemory::new(shellmemory::VAR_SIZE);
    let p_mem = shellmemory::ProgMemory::new(shellmemory::MEM_SIZE);
    let frame_t = shellmemory::FrameTable::new();
    
    let mut kernel = kernel::Kernel::new(
        kernel::Mode::FCFS,
        p_mem,
        var_mem,
        frame_t
    );
    
    let mut cwd = String::from("/");
    
    let prompt = '$';
    
    loop {
        
        let mut buf = String::new();
        let time = Local::now().format("%H:%M");
        print!("{time}~{cwd} {prompt} ");
        
        io::stdout().flush().expect("Terminated due to stdout flush error");
        io::stdin().read_line(&mut buf).expect("Failed to read from stdin");

        let args: Vec<String> = buf.split(';').map(str::to_string).collect();

        interpreter::interpreter(
            args,
            &mut kernel,
        );
        // kernel.get_mut_pmem().dump("ok", false);
        // kernel.get_mut_ft().frame_dump();
        // kernel.memory_dump();
    }
}
