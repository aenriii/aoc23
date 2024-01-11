
static INPUT: &str = include_str!("1/real.txt");

pub fn part1() {
    // get the input as a byte slice
    let mut data = INPUT.as_bytes();
    // initialize the sum to 0
    let mut sum = 0u32;
    // initialize the current line's numbers to 0
    let mut l_nums = [0u8;10];
    // initialize the current line's index to 0
    let mut l_idx = 0u8;
    // initialize the master index to 0
    let mut master_idx = 0usize;
    // get the length of the input
    let len = INPUT.len();

    loop {
        // check if we're at the end of the file
        if master_idx >= len {
            // if the current line has any numbers, add them to the sum
            if l_idx != 0 {
                
                sum += (((l_nums[0]) * 10) + (l_nums[l_idx as usize])) as u32;
            }
            break;
        }
        // get the current byte (we know we're not at the end of the file, so this is safe)
        let byte = unsafe { *data.get_unchecked(0) };
        // if we're at a newline, we've reached the end of the current line
        if byte == b'\n' {
            // if the current line has any numbers, add them to the sum
            if l_idx != 0 {
                sum += (((l_nums[0]) * 10) + (l_nums[l_idx as usize])) as u32;
            }
            // reset the current line's numbers and index
            l_nums = [0u8;10];
            l_idx = 0;
            // move to the next line
            data = unsafe { data.get_unchecked(1..) };
            // increment the master index to keep track of where we are in the file
            master_idx += 1;
            continue;
        }
        // if the current byte isn't a number, skip it
        if byte < b'0' || byte > b'9' {
            data = unsafe { data.get_unchecked(1..) };
            master_idx += 1;
            continue;
        }
        // if we've reached this point, we know that the current byte is a number
        l_nums[l_idx as usize] = byte - b'0'; // +1 to avoid 0, as we use 0 as a sentinel value
        l_idx += 1;
        // based on my estimations, we should never have more than 10 numbers on a line
        // if im wrong we can just increase this number later
        if l_idx == 10 {
            panic!("uh oh");
        }
        master_idx += 1;
        data = unsafe { data.get_unchecked(1..) };
    }
    println!("{}", sum);
}
pub fn part2() {
    let mut data = INPUT.as_bytes();
    let mut sum = 0u32;
    let mut l_nums = [0u8;10];
    let mut l_idx = 0u8;
    let mut master_idx = 0usize;
    let len = INPUT.len();
    loop {
        if master_idx >= len {
            if l_idx != 0 {
                sum += (((l_nums[0]) * 10) + (l_nums[l_idx as usize - 1])) as u32;
            }
            break;
        }
        let byte = unsafe { *data.get_unchecked(0) };
        if byte == b'\n' {
            if l_idx != 0 {
                sum += (((l_nums[0]) * 10) + (l_nums[l_idx as usize - 1])) as u32;
            }
            l_nums = [0u8;10];
            l_idx = 0;
            data = unsafe { data.get_unchecked(1..) };
            master_idx += 1;
            continue;
        }
        if byte < b'0' || byte > b'9' {

            if len - master_idx >= 5 {
                let word = unsafe { data.get_unchecked(0..5) };
                if word == b"three".as_slice() {
                    l_nums[l_idx as usize] = 3; 
                    l_idx += 1;
                    data = unsafe { data.get_unchecked(1..) };
                    master_idx += 1;
                    continue;
                }
                if word == b"seven".as_slice() {
                    l_nums[l_idx as usize] = 7;
                    l_idx += 1;
                    data = unsafe { data.get_unchecked(1..) };
                    master_idx += 1;
                    continue;
                }
                if word == b"eight".as_slice() {
                    l_nums[l_idx as usize] = 8;
                    l_idx += 1;
                    data = unsafe { data.get_unchecked(1..) };
                    master_idx += 1;
                    continue;
                }
            }
            if len - master_idx >= 4 {
                let word = unsafe { data.get_unchecked(0..4) };
                if word == b"five".as_slice() {
                    l_nums[l_idx as usize] = 5;
                    l_idx += 1;
                    data = unsafe { data.get_unchecked(1..) };
                    master_idx += 1;
                    continue;
                }
                if word == b"nine".as_slice() {
                    l_nums[l_idx as usize] = 9;
                    l_idx += 1;
                    data = unsafe { data.get_unchecked(1..) };
                    master_idx += 1;
                    continue;
                }
                if word == b"four".as_slice() {
                    l_nums[l_idx as usize] = 4;
                    l_idx += 1;
                    data = unsafe { data.get_unchecked(1..) };
                    master_idx += 1;
                    continue;
                }
            }
            if len - master_idx >= 3 {
                let word = unsafe { data.get_unchecked(0..3) };
                if word == b"one".as_slice() {
                    l_nums[l_idx as usize] = 1;
                    l_idx += 1;
                    data = unsafe { data.get_unchecked(1..) };
                    master_idx += 1;
                    continue;
                }
                if word == b"two".as_slice() {
                    l_nums[l_idx as usize] = 2;
                    l_idx += 1;
                    data = unsafe { data.get_unchecked(1..) };
                    master_idx += 1;
                    continue;
                }
                if word == b"six".as_slice() {
                    l_nums[l_idx as usize] = 6;
                    l_idx += 1;
                    data = unsafe { data.get_unchecked(1..) };
                    master_idx += 1;
                    continue;
                }
            }

            data = unsafe { data.get_unchecked(1..) };
            master_idx += 1;
            continue;
        }
        l_nums[l_idx as usize] = byte - b'0'; // we dont need to +1 here because we're not using 0 as a sentinel value, we check the index instead
        l_idx += 1;
        if l_idx == 10 {
            panic!("uh oh");
        }
        master_idx += 1;
        data = unsafe { data.get_unchecked(1..) };
    }
    println!("{}", sum);
}