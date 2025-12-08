use crate::shellmemory::{VarMemory, ProgMemory, FrameTable};
use crate::kernel::Kernel;
use std::fs::{File};
use std::io::{BufRead, BufReader};
use crate::job;

fn bad_cmd(input: &str) {
    print!("minsh: unrecognized command: {input}");
    return;
}

pub(crate) fn err_msg(input: &str) {
    println!("minsh: err: {input}");
    return;
}

pub fn interpreter(
    input: Vec<String>,
    var_mem: &mut VarMemory,
    p_mem: &mut ProgMemory,
    kernel: &mut Kernel,
    ft: &mut FrameTable,
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
                echo(var_mem,&arg_arr[1])
            },
            "set" => {
                if arg_arr.len() < 3 {
                    bad_cmd("usage: set <VAR> <VALUE>");
                    return
                }
                set(var_mem, &arg_arr[1..])
            },
            "exec" => {
                if arg_arr.len() < 2 {
                    bad_cmd("usage: exec <FILENAME>");
                    return
                }
                exec(&arg_arr[1], kernel, ft, p_mem)
            },
            "cat" => {
                if arg_arr.len() < 2 {
                    bad_cmd("usage: cat <FILENAME>");
                    return
                }
                cat(&arg_arr[1])
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

fn exec(filename: &str, kern: &mut Kernel, ft: &mut FrameTable, p_mem: &mut ProgMemory) {
    let prog_res = job::Program::new(ft, p_mem, filename);
    match prog_res {
        Ok(p) => println!("nice"),
        Err(e) => err_msg(e.as_str())
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

fn ls() {
    println!("Unimplemented")
}

fn cd() {
    println!("Unimplemented")
}

fn touch() {
    
    println!("Unimplemented")
}




