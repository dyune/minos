use crate::shellmemory::{VarMemory, ProgMemory, FrameTable};
use crate::kernel::{Kernel, Mode};
use std::fs::{File};
use std::io::{BufRead, BufReader};
use crate::job;
use crate::job::FailProgramCreation;

fn bad_cmd(input: &str) {
    println!("minsh: unrecognized command: {input}");
    return;
}

pub(crate) fn err_msg(input: &str) {
    println!("minsh: err: {input}");
    return;
}

pub fn interpreter(
    input: Vec<String>,
    kernel: &mut Kernel,
) {

    for arg in input {
        if arg.trim().is_empty()  {
            return;
        }

        // split by ; for multi-cmd processing
        let arg_arr: Vec<String> = arg.split_whitespace().map(str::to_string).collect();

        // as_str() does not consume anything, only returns str slice
        match arg_arr[0].as_str() {
            "echo" => {
                if arg_arr.len() > 2 {
                    bad_cmd(arg_arr.join(" ").as_str());
                    return
                }
                echo(kernel.get_mut_varmem(),&arg_arr[1])
            },
            "set" => {
                if arg_arr.len() < 3 {
                    bad_cmd("usage: set <VAR> <VALUE>");
                    return
                }
                set(kernel.get_mut_varmem(), &arg_arr[1..])
            },
            "exec" => {
                if arg_arr.len() < 2 {
                    bad_cmd("usage: exec <FILENAME 1> <FILENAME 2> <etc...>");
                    return
                }
                exec(&arg_arr[1..], kernel)
            },
            "cat" => {
                if arg_arr.len() < 2 {
                    bad_cmd("usage: cat <FILENAME>");
                    return
                }
                cat(&arg_arr[1])
            },
            "setmod" => {
                if arg_arr.len() < 2 {
                    bad_cmd("usage: setmod <FCFS, RR, SJF>");
                    return
                }
                setmod(&arg_arr[1], kernel)
            }
            _ => bad_cmd(arg.as_str())
            
        }
    }
}

fn echo(var_mem: &VarMemory, input: &str) {
    if let Some(val) = var_mem.get(input) {
        let res = val.as_str();
        println!("{res}");
    } else {
        println!("{input}");
    }
}

fn set(var_mem: &mut VarMemory, input: &[String]) {
    if input.len() < 2 {
        bad_cmd("usage: set <VAR> <VALUE>")
    }
    var_mem.set(input[0].clone(), input[1].clone())
}

fn exec(
    filenames: &[String], 
    kern: &mut Kernel,
) {
    for file in filenames {
        let prog_res = job::Program::new(
            kern,
            file
        );
        match prog_res {
            Ok(p) => {
                let job = job::Job::new(
                    Some(p.size),
                    str::to_string(file),
                    Some(p),
                    kern,
                );
                if let Ok(j) = job {
                    kern.queue_job(j);
                } else {
                    err_msg(job.unwrap_err());
                    return
                }
            },
            Err(e) => match e {
                FailProgramCreation::Error(e) => { 
                    err_msg(e.as_str());
                    return;
                },
                FailProgramCreation::ExistsAlready => {
                    let job = job::Job::new(
                        None,
                        str::to_string(file),
                        None,
                        kern,
                    );
                    if let Ok(j) = job {
                        kern.queue_job(j);
                    } else {
                        err_msg(job.unwrap_err());
                        return
                    }
                }
            }
        }
    }
    // kern.memory_dump();
    let res = kern.execute_schedule();
    if let Err(r) = res {
        err_msg(r);
        return
    }

}

fn cat(filename: &str) {
    let file = File::open(filename);
    if let Ok(f) = file {
        let reader = BufReader::new(f);
        for (idx, line) in reader.lines().enumerate() {

            match line {
                Ok(ln) => println!("{}", ln),
                Err(e) => {
                    err_msg(
                        format!("failed to read line {} from {} due to {}", idx, filename, e).
                            as_str()
                    );
                    return
                }
            }
        }
    } else {
        err_msg(format!("failed to open {}", filename).as_str());
    }
}

fn setmod(mode: &str, kernel: &mut Kernel) {
    match mode {
        "FCFS" => { 
            kernel.mode = Mode::FCFS;
            println!("Scheduler running in FCFS")
        }
        "SJF" => {
            kernel.mode = Mode::SJF;
            println!("Scheduler running in SJF")
        }
        "RR" => {
            kernel.mode = Mode::RR;
            println!("Scheduler running in RR")
        }
        _ => err_msg(format!("unknown scheduler mode: {mode}").as_str())
    }
}

fn ls() {
    println!("Unimplemented")
}

fn cd() {
    println!("Unimplemented")
}

fn touch() {
    println!("Unimplemented")
}




