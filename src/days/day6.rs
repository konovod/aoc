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

pub fn part_abstract(input: String, marker: usize) -> usize {
    let mut iter = input.chars();
    let mut ring = Vec::new();
    for _ in 0..marker {
        ring.push(iter.next().unwrap());
    }
    let mut ring_pos = 0;
    let mut count = marker;
    for c in iter {
        if all_diff(&ring) {
            return count;
        }
        count += 1;
        ring[ring_pos] = c;
        ring_pos = (ring_pos + 1) % marker;
    }
    return count;
}

pub fn part1(input: String) -> usize {
    return part_abstract(input, 4);
}

pub fn part2(input: String) -> usize {
    return part_abstract(input, 14);
}
