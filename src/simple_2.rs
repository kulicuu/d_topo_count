use std::collections::HashSet;

const RESOLUTION: usize = 20;
const SQUARE: usize = RESOLUTION * RESOLUTION;

pub fn routine_300
()
{
    let arr = build_space();
    let count = count_neighborhoods(&arr);
    println!("\n Count for space-1: {} \n", count);

    let arr = build_space_2();
    let count = count_neighborhoods(&arr);
    println!("\n Count for space-2: {} \n", count);

    let arr = build_space_3();
    let count = count_neighborhoods(&arr);
    println!("\n Count for space-3: {} \n", count);
}

fn get_create_nghd(
    nc: &mut Vec<HashSet<usize>>,
    el: usize,
)
-> usize
{
    for (idx, hs) in nc.iter().enumerate() {
        if hs.contains(&el) {
            return idx
        }
    }
    let mut hs: HashSet<usize> = HashSet::new();
    hs.insert(el);
    nc.push(hs);
    return nc.len() - 1
}

fn get_nghd(
    nc: &mut Vec<HashSet<usize>>,
    el: usize,
)
-> i32
{
    for (idx, hs) in nc.iter().enumerate() {
        if hs.contains(&el) {
            return idx as i32
        }
    }
    return -1
}

fn count_neighborhoods(arr: &[bool; SQUARE])
-> usize
{
    let mut nc: Vec<HashSet<usize>> = vec![];
    for cursor in 0..SQUARE {
        if arr[cursor] == true {
            let cursor_n_idx = get_create_nghd(&mut nc, cursor);
            if (cursor % RESOLUTION) < (RESOLUTION - 1) {
                let east = cursor + 1;
                if arr[east] == true {
                    let eni = get_nghd( // existing neighborhood
                        &mut nc,
                        east,
                    );
                    if eni > -1 { 
                        let mut temp = (nc[cursor_n_idx]).clone();
                        for el in &(nc[eni as usize]) {
                            temp.insert(*el);
                        }
                        nc[cursor_n_idx] = temp;
                        nc.remove(eni as usize);
                    } else {
                        nc[cursor_n_idx].insert(east);
                    }
                }
            }
            if cursor < (SQUARE - RESOLUTION) {
                let south = cursor + RESOLUTION;
                if arr[south] == true {
                    let eni = get_nghd( // existing_neighborhood_idx
                        &mut nc,
                        south,
                    );
                    if eni > -1 {
                        let mut temp = (nc[cursor_n_idx]).clone();
                        for el in &(nc[eni as usize]) {
                            temp.insert(*el);
                        }
                        nc[cursor_n_idx] = temp;
                        nc.remove(eni as usize);
                    } else {
                        nc[cursor_n_idx].insert(south);
                    }
                }
            }
        }
    }
    nc.len()
}

// Build a 2-D space into a single array with row-major ordering. 
fn build_space
()
-> [bool; SQUARE as usize]
{
    let mut arr: [bool; SQUARE as usize] = [false; SQUARE as usize];

    arr[((2 * RESOLUTION) + 3) as usize] = true; // 43
    arr[((2 * RESOLUTION) + 4) as usize] = true; // 44
    arr[((2 * RESOLUTION) + 5) as usize] = true; // 45
    arr[((3 * RESOLUTION) + 3) as usize] = true; // 63
    arr[((3 * RESOLUTION) + 5) as usize] = true; // 65

    // Now we make another small island 
    arr[((10 * RESOLUTION) + 11) as usize] = true; // 211
    arr[((10 * RESOLUTION) + 12) as usize] = true; // 212
    arr[((11 * RESOLUTION) + 11) as usize] = true; // 221
    arr[((12 * RESOLUTION) + 11) as usize] = true; // 241

    arr
}

fn build_space_2
()
-> [bool; SQUARE as usize]
{
    let mut arr: [bool; SQUARE as usize] = [false; SQUARE as usize];

    arr[((2 * RESOLUTION) + 3) as usize] = true; // 43
    arr[((2 * RESOLUTION) + 4) as usize] = true; // 44
    arr[((2 * RESOLUTION) + 5) as usize] = true; // 45
    arr[((3 * RESOLUTION) + 3) as usize] = true; // 63
    arr[((3 * RESOLUTION) + 5) as usize] = true; // 65

    // Now we make another small island 
    arr[((10 * RESOLUTION) + 11) as usize] = true; // 211
    arr[((10 * RESOLUTION) + 12) as usize] = true; // 212
    arr[((11 * RESOLUTION) + 11) as usize] = true; // 221
    arr[((12 * RESOLUTION) + 11) as usize] = true; // 241

    arr[((15 * RESOLUTION) + 7) as usize] = true; // 211
    arr[((15 * RESOLUTION) + 8) as usize] = true; // 212
    arr[((15 * RESOLUTION) + 9) as usize] = true; // 221
    arr[((16 * RESOLUTION) + 7) as usize] = true; // 241
    arr[((16 * RESOLUTION) + 9) as usize] = true; // 241
    arr[((16 * RESOLUTION) + 10) as usize] = true; // 241
    arr[((16 * RESOLUTION) + 11) as usize] = true; // 241

    arr
}

fn build_space_3
()
-> [bool; SQUARE as usize]
{
    let mut arr: [bool; SQUARE as usize] = [false; SQUARE as usize];

    arr[((2 * RESOLUTION) + 3) as usize] = true; 
    arr[((2 * RESOLUTION) + 4) as usize] = true; 
    arr[((2 * RESOLUTION) + 5) as usize] = true; 
    arr[((3 * RESOLUTION) + 3) as usize] = true; 
    arr[((3 * RESOLUTION) + 5) as usize] = true; 
    arr[((4 * RESOLUTION) + 3) as usize] = true; 
    arr[((4 * RESOLUTION) + 5) as usize] = true; 
    arr[((5 * RESOLUTION) + 3) as usize] = true; 
    arr[((5 * RESOLUTION) + 4) as usize] = true; 
    arr[((5 * RESOLUTION) + 5) as usize] = true; 



    // arr[((15 * RESOLUTION) + 7) as usize] = true; // 211
    // arr[((15 * RESOLUTION) + 8) as usize] = true; // 212
    // arr[((15 * RESOLUTION) + 9) as usize] = true; // 221
    // arr[((16 * RESOLUTION) + 7) as usize] = true; // 241
    // arr[((16 * RESOLUTION) + 9) as usize] = true; // 241
    // arr[((16 * RESOLUTION) + 10) as usize] = true; // 241
    // arr[((16 * RESOLUTION) + 11) as usize] = true; // 241

    arr
}