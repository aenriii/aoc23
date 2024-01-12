
static INPUT : &str = include_str!("2/real.txt");

const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

// Lines are formatted as:
// Game {id}: {num} {color}, ...; {num} {color}, ... 

pub fn part1() {

    let mut data = INPUT.as_bytes();
    let mut master_idx = 0usize;
    let len = INPUT.len();

    let mut sum: u32 = 0;

    loop {
        if master_idx >= len {
            break;
        }
        let mut possible = true; 
        let mut game_id = 0u8;

        // Quickly get the game id
        data = unsafe { data.get_unchecked(5..) };
        master_idx += 5;
        while master_idx < len && unsafe { *data.get_unchecked(0) } != b':' {
            game_id = game_id * 10 + (unsafe { *data.get_unchecked(0) } - b'0');
            data = unsafe { data.get_unchecked(1..) };
            master_idx += 1;
        }
        if master_idx >= len {
            eprintln!("Bad state, should be unreachable (in getting game id)");
            break; 
        }

        let mut curr_num = 0u8;

        // for each line
        while master_idx < len && unsafe { *data.get_unchecked(0) } != b'\n' {
            let byte = unsafe { *data.get_unchecked(0) };
            // if not number
            if (byte < b'0' || byte > b'9') && curr_num != 0 {
                match byte {
                    // if red
                    b'r' => {
                        if curr_num > MAX_RED {
                            curr_num = 0;
                            possible = false;
                        }
                        data = unsafe { data.get_unchecked(3..) };
                        master_idx += 3;
                    },
                    // if green
                    b'g' => {
                        if curr_num > MAX_GREEN {
                            curr_num = 0;
                            possible = false;
                        }
                        data = unsafe { data.get_unchecked(5..) };
                        master_idx += 5;
                    },
                    // if blue
                    b'b' => {
                        if curr_num > MAX_BLUE {
                            curr_num = 0;
                            possible = false;
                        }
                        data = unsafe { data.get_unchecked(4..) };
                        master_idx += 4;
                    },
                    // if comma or semicolon
                    b',' | b';' => {
                        curr_num = 0;
                        data = unsafe { data.get_unchecked(1..) };
                        master_idx += 1;
                    },
                    // else
                    _ => {
                        data = unsafe { data.get_unchecked(1..) };
                        master_idx += 1;
                    }
                }
                continue;
            } /* else if number */ else if byte >= b'0' && byte <= b'9' {
                curr_num = curr_num * 10 + (byte - b'0');
                data = unsafe { data.get_unchecked(1..) };
                master_idx += 1;
            } else {
                data = unsafe { data.get_unchecked(1..) };
                master_idx += 1;
            }
        }
        if possible {unsafe {
            sum = sum.unchecked_add(game_id as u32);
        }}
        data = unsafe { data.get_unchecked(1..) };
        master_idx += 1;
    }
    println!("{}", sum);
}

pub fn part2() {

    let mut data = INPUT.as_bytes();
    let mut master_idx = 0usize;
    let len = INPUT.len();

    let mut sum: u32 = 0;

    loop {
        if master_idx >= len {
            break;
        }
        let mut game_id = 0u8;

        let mut max_red = 0u8;
        let mut max_green = 0u8;
        let mut max_blue = 0u8;

        // Quickly get the game id
        data = unsafe { data.get_unchecked(5..) };
        master_idx += 5;
        while master_idx < len && unsafe { *data.get_unchecked(0) } != b':' {
            game_id = game_id * 10 + (unsafe { *data.get_unchecked(0) } - b'0');
            data = unsafe { data.get_unchecked(1..) };
            master_idx += 1;
        }
        if master_idx >= len {
            eprintln!("Bad state, should be unreachable (in getting game id)");
            break; 
        }

        let mut curr_num = 0u8;

        // for each line
        while master_idx < len && unsafe { *data.get_unchecked(0) } != b'\n' {
            let byte = unsafe { *data.get_unchecked(0) };
            // if not number
            if (byte < b'0' || byte > b'9') && curr_num != 0 {
                match byte {
                    // if red
                    b'r' => {
                        if curr_num > max_red {
                            max_red = curr_num;
                        }
                        curr_num = 0;
                        data = unsafe { data.get_unchecked(3..) };
                        master_idx += 3;
                    },
                    // if green
                    b'g' => {
                        if curr_num > max_green {
                            max_green = curr_num;
                        }
                        curr_num = 0;
                        data = unsafe { data.get_unchecked(5..) };
                        master_idx += 5;
                    },
                    // if blue
                    b'b' => {
                        if curr_num > max_blue {
                            max_blue = curr_num;
                        }
                        curr_num = 0;
                        data = unsafe { data.get_unchecked(4..) };
                        master_idx += 4;
                    },
                    // if comma or semicolon
                    b',' | b';' => {
                        curr_num = 0;
                        data = unsafe { data.get_unchecked(1..) };
                        master_idx += 1;
                    },
                    // else
                    _ => {
                        data = unsafe { data.get_unchecked(1..) };
                        master_idx += 1;
                    }
                }
                continue;
            } /* else if number */ else if byte >= b'0' && byte <= b'9' {
                curr_num = curr_num * 10 + (byte - b'0');
                data = unsafe { data.get_unchecked(1..) };
                master_idx += 1;
            } else {
                data = unsafe { data.get_unchecked(1..) };
                master_idx += 1;
            }
        }
        unsafe {
            sum = sum.unchecked_add((max_blue as u32) * (max_green as u32) * (max_red as u32));
        }
        data = unsafe { data.get_unchecked(1..) };
        master_idx += 1;
    }
    println!("{}", sum);
}