

static INPUT : &str = include_str!("3/real.txt");
pub fn part1() {

    let data = INPUT.as_bytes();
    let mut master_idx = 0usize;
    let mut sum = 0u32;
    let len = INPUT.len();

    let line_len = {
        let mut line_len = 0usize;
        while master_idx < len && unsafe { *data.get_unchecked(master_idx) } != b'\n' {
            master_idx += 1;
            line_len += 1;
        }
        master_idx = 0;
        line_len
    };
    while master_idx < len {
        // println!("Master idx: {}", master_idx);
        while master_idx < len && let byte = unsafe { *data.get_unchecked(master_idx) } && (byte < b'0' || byte > b'9') {
            master_idx += 1
        }
        if master_idx >= len {
            break;
        }
        let mut num = 0u32;
        let mut nearby = false;
        while master_idx < len && let byte = unsafe { *data.get_unchecked(master_idx) } && byte >= b'0' && byte <= b'9' {
            num = num * 10 + (byte - b'0') as u32;
            if !(master_idx < line_len + 1) {
                #[cfg(feature = "debug")] if (crate::ARGS.verbosity_level >= super::VERBOSITY_DEBUG) {println!("checking top row");}
                // top left
                if  !nearby // gate to prevent unnecessary checks
                    && !(master_idx % line_len == 0) // gate to prevent out of bounds
                    && let top_left = unsafe { *data.get_unchecked(master_idx - line_len - 1) } // get top left
                    && !(top_left >= b'0' && top_left <= b'9') // gate to prevent checking a number
                    && top_left != b'.' // periods are not symbols
                    && top_left != b'\n' // if its a newline :boom:
                {
                    nearby = true;
                }
                // top
                if  !nearby 
                    && let top = unsafe { *data.get_unchecked(master_idx - line_len) } 
                    && !(top >= b'0' && top <= b'9') 
                    && top != b'.' 
                    && top != b'\n' 
                {
                    nearby = true;
                }
                // top right
                if  !nearby 
                    && !(master_idx % line_len == line_len - 1) 
                    && let top_right = unsafe { *data.get_unchecked(master_idx - line_len + 1) } 
                    && !(top_right >= b'0' && top_right <= b'9') 
                    && top_right != b'.' 
                    && top_right != b'\n'
                {
                    nearby = true;
                }
            }
            // check middle. we know this y level exists because we are on it
            if  !nearby 
                && !(master_idx % line_len == 0) 
                && let left = unsafe { *data.get_unchecked(master_idx - 1) } 
                && !(left >= b'0' && left <= b'9') 
                && left != b'.' 
                && left != b'\n'
            {
                nearby = true;
            }
            if  !nearby 
                && !(master_idx % line_len == line_len - 1) 
                && let right = unsafe { *data.get_unchecked(master_idx + 1) } 
                && !(right >= b'0' && right <= b'9') 
                && right != b'.' 
                && right != b'\n'
            {
                nearby = true;
            }
            // bottom row
            if master_idx < len - line_len - 1 {
                #[cfg(feature = "debug")] if (crate::ARGS.verbosity_level >= super::VERBOSITY_DEBUG) {println!("checking bottom row");}
                // bottom left
                if  !nearby 
                    && !(master_idx % line_len == 0) 
                    && let bottom_left = unsafe { *data.get_unchecked(master_idx + line_len) } 
                    && !(bottom_left >= b'0' && bottom_left <= b'9') 
                    && bottom_left != b'.' 
                    && bottom_left != b'\n'
                {
                    nearby = true;
                }
                // bottom
                if  !nearby 
                    && let bottom = unsafe { *data.get_unchecked(master_idx + line_len + 1) } 
                    && !(bottom >= b'0' && bottom <= b'9') 
                    && bottom != b'.' 
                    && bottom != b'\n'
                {
                    nearby = true;
                }
                // bottom right
                if  !nearby 
                    && !(master_idx % line_len == line_len - 1) 
                    && let bottom_right = unsafe { *data.get_unchecked(master_idx + line_len + 2) } 
                    && !(bottom_right >= b'0' && bottom_right <= b'9') 
                    && bottom_right != b'.' 
                    && bottom_right != b'\n'
                {
                    nearby = true;
                }
            }
            #[cfg(feature = "debug")] {
                if (crate::ARGS.verbosity_level >= super::VERBOSITY_TRACE) {
                    println!("viewing num: {}", byte as char);
                    if master_idx < line_len + 1 {

                        println!("xxx");

                    } else {
                        if master_idx % line_len == 0 {
                            print!("x");
                        } else {
                            print!("{}", unsafe { *data.get_unchecked(master_idx - line_len - 1) } as char);
                        }
                        print!("{}", unsafe { *data.get_unchecked(master_idx - line_len) } as char);
                        if master_idx % line_len == line_len - 1 {
                            println!("x");
                        } else {
                            println!("{}", unsafe { *data.get_unchecked(master_idx - line_len + 1) } as char);
                        }
                    }

                    if master_idx % line_len == 0 {
                        print!("x");
                    } else {
                        print!("{}", unsafe { *data.get_unchecked(master_idx - 1) } as char);
                    }
                    print!("{}", unsafe { *data.get_unchecked(master_idx) } as char);
                    if master_idx % line_len == line_len - 1 {
                        println!("x");
                    } else {
                        println!("{}", unsafe { *data.get_unchecked(master_idx + 1) } as char);
                    }

                    if master_idx < len - line_len - 1 {

                        if master_idx % line_len == 0 {
                            print!("x");
                        } else {
                            print!("{}", unsafe { *data.get_unchecked(master_idx + line_len) } as char);
                        }
                        print!("{}", unsafe { *data.get_unchecked(master_idx + line_len + 1) } as char);
                        if master_idx % line_len == line_len - 1 {
                            println!("x");
                        } else {
                            println!("{}", unsafe { *data.get_unchecked(master_idx + line_len + 2) } as char);
                        }

                    } else {
                        println!("xxx");
                    }
                }
            }
            master_idx += 1;
        }
        if nearby {
            #[cfg(feature = "debug")] if (crate::ARGS.verbosity_level >= super::VERBOSITY_INFO) {println!("NEARBY = true FOR {}", num);}
            sum += num;
        } else {
            #[cfg(feature = "debug")] if (crate::ARGS.verbosity_level >= super::VERBOSITY_INFO) {println!("NEARBY = false FOR {}", num);}
        }
    }
    println!("{}", sum);
}
