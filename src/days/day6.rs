const MARKER_SIZE: usize = 4;

fn all_diff(buffer: &Vec<char>) -> bool {
    for i in 0..buffer.len() - 1 {
        for j in i + 1..buffer.len() {
            if buffer[i] == buffer[j] {
                return false;
            }
        }
    }
    return true;
}

pub fn part1(input: String) -> usize {
    let mut iter = input.chars();
    let mut ring = Vec::new();
    for _ in 0..MARKER_SIZE {
        ring.push(iter.next().unwrap());
    }
    let mut ring_pos = 0;
    let mut count = MARKER_SIZE;
    for c in iter {
        if all_diff(&ring) {
            return count;
        }
        count += 1;
        ring[ring_pos] = c;
        ring_pos = (ring_pos + 1) % MARKER_SIZE;
    }
    return count;
}

pub fn part2(_input: String) -> i32 {
    return 0;
}
