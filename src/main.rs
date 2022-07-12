use std::env::args;
use std::fs;
use std::io::Write;
use std::path::Path;

struct Brainf {
    mem: [u8; 30000],
    dp: usize,
    ip: usize
}

impl Brainf {
    fn execute(code : String) {
        let mut bf = Brainf { 
            mem : [0; 30000],
            dp : 0,
            ip : 0
        };

        let arr = code.into_bytes();
        let end = arr.len();

        loop {
            /* We choice to not wrap but saturate with overflow */
            match arr[bf.ip] as char {
                '+' => {
                    if bf.mem[bf.dp] < u8::MAX{
                        bf.mem[bf.dp] += 1;
                    }
                },
                '-'=> {
                    if bf.mem[bf.dp] > u8::MIN{
                        bf.mem[bf.dp] -= 1;
                    }
                },
                '>'=> {
                    if bf.dp < usize::MAX {
                        bf.dp+=1;
                    }
                },
                '<'=> {
                    if bf.dp > usize::MIN {
                        bf.dp-=1;
                    }
                },
                '['=> {
                    let mut d = 0;

                    if bf.mem[bf.dp] == 0 {
                        for _ in bf.ip .. arr.len()-1 {
                            bf.ip+=1;
                            if arr[bf.ip] == b']' {
                                if d == 0 {
                                    break;
                                } else {
                                    d-= 1;
                                }
                            } else if arr[bf.ip] == b'[' {
                                    d+=1;
                            }
                        }
                    }
                },
                ']'=> {
                    let mut d = 0;
                    let end = bf.ip;

                    if bf.mem[bf.dp] != 0 {
                        for _ in 1 .. end {
                            bf.ip-=1;
                            if arr[bf.ip] == b'[' {
                                if d == 0 {
                                    break;
                                } else {
                                    d-=1;
                                }
                            } else if arr[bf.ip] == b']' {
                                    d+=1;
                            }
                        }
                    }
                },
                '.'=>{
                    print!("{}", bf.mem[bf.dp] as char);
                },
                ','=> {
                    /* Implement input */
                },
                _ => {
                    /* Ignore all other */
                },
            }


            bf.ip+=1;
            if bf.ip == end {
                break;
            }
        }
        std::io::stdout().flush().expect("Could not write output");
    }
}

fn main() {
    let mut arguments = args();
    let file = arguments.nth(1);
    match file {
        Some(name) => {
            if let Ok(content) = fs::read_to_string(Path::new(name.as_str())){
                Brainf::execute(content);
            } else {
                println!("Failed reading content of {}", name);
            }
        },
        None        => {
            println!("Expect filename as first argument");
        },
    }
}


