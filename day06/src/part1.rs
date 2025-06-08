use std::collections::{BTreeMap, HashMap};

pub fn part1(input: &str) -> anyhow::Result<()> {
    let mut map: BTreeMap<usize, HashMap<char,usize>> = BTreeMap::new();
    let mut column_index;
    for line in input.split("\n") {
        column_index = 0;
        let mut chars = line.chars();
        while let Some(chr) = chars.next() {
            let char_count_map = map.entry(column_index).or_insert(HashMap::new());
            let char_count_vec = char_count_map.entry(chr).or_insert(0);
            *char_count_vec += 1;
            column_index +=1;
        }
    }
    let mut result = String::new();
    for column in map {
        let max = column.1.iter().max_by_key(|e|e.1);
        if let Some((chr, _)) = max {
            result.push(*chr);
        }
    }
    println!("{result}");
    Ok(())
}