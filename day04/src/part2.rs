use std::collections::{BTreeSet, HashMap};

use regex::Regex;

pub fn part2(input: &str) -> anyhow::Result<()> {
    let regex = Regex::new(
        r"(?<encrypted_name_part>(([a-z]+)-?)+)-(?<sector_id>\d+)\[(?<checksum>[a-z]+)\]",
    )?;
    let results = regex.captures_iter(input).collect::<Vec<_>>();
    let secret_room = results
        .into_iter()
        .map(|captures| {
            let encrypted_name_part = captures.name("encrypted_name_part").unwrap().as_str();
            let sector_id = captures.name("sector_id").unwrap().as_str().parse::<u16>().unwrap();
            let checksum = captures.name("checksum").unwrap().as_str();
            Room::new(encrypted_name_part, sector_id, checksum)
        })
        .filter(Room::validate)
        .find(|room|room.get_decrypted_name().starts_with("northpole object storage"))
        .expect("No secret room found");
    //.for_each(|(encrypted_name_part, sector_id, checksum, _)|println!("{encrypted_name_part:?} - {sector_id:?} - {checksum:?}"))
    println!("Secret room id: {}", secret_room.sector_id);
    Ok(())
}

struct Room<'a> {
    encrypted_name_part: &'a str,
    sector_id: u16,
    checksum: &'a str,
    char_set: BTreeSet<SortingElement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SortingElement {
    amount: i32,
    chr: char,
}
// Must be same ordering as Ord, otherwise chaos!
impl PartialOrd for SortingElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.amount.partial_cmp(&self.amount) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.chr.partial_cmp(&other.chr)
        //other.chr.partial_cmp(&self.chr)
    }
}
// Must be same ordering as PartialOrd, otherwise chaos!
impl Ord for SortingElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.amount.cmp(&self.amount) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.chr.cmp(&other.chr)
        //other.chr.cmp(&self.chr)
    }
}

impl<'a> Room<'a> {
    fn new(encrypted_name_part: &'a str, sector_id: u16, checksum: &'a str) -> Self {
        Self {
            encrypted_name_part,
            sector_id,
            checksum,
            char_set: Room::count_chars(encrypted_name_part),
        }
    }

    fn get_decrypted_name(&self) -> String {
        let sector_id = self.sector_id;
        self.encrypted_name_part.chars().map(|c|
            match c {
                '-' => ' ',
                c => {
                    let res = (((c as u16) - ('a' as u16)  + sector_id) % 26u16) as u8 + ('a' as u8);
                    let chr = res as char;
                    //println!("converting '{c}' + '{sector_id} = {res}:{chr}");
                    chr
                }
            }
        ).collect()
    }

    fn validate(&self) -> bool {
        let set = &self.char_set;

        let amount = set
            .iter()
            .take(5)
            .zip(self.checksum.chars().into_iter())
            .filter(|(SortingElement { chr ,..}, checksum_char)| {
                //println!("{chr}?{checksum_char}: {amount}");
                chr == checksum_char
            })
            .count();
        if amount == 5 {
            /*
            let details = set
                .iter()
                //.take(10)
                .fold(
                    String::new(),
                    |mut builder, SortingElement { amount, chr }| {
                        let s = format!(" {chr}:{amount}");
                        builder.push_str(s.as_str());
                        builder
                    },
                );
             */
            //println!("valid: {checksum} {details}");
            //println!("+ valid: {encrypted_name_part}-{sector_id}[{checksum}] {details}");
            return true;
        } else {
            /*
            let details = set
                .iter()
                //.take(10)
                .fold(
                    String::new(),
                    |mut builder, SortingElement { amount, chr }| {
                        let s = format!(" {chr}:{amount}");
                        builder.push_str(s.as_str());
                        builder
                    },
                );
             */
            //eprintln!("invalid: {checksum} {details}");

            //println!("invalid: {encrypted_name_part}-{sector_id}[{checksum}] {details}");
        }
        return false;
    }

    fn count_chars(encrypted_name_part: &str) -> BTreeSet<SortingElement> {
        let replaced = encrypted_name_part.replace("-", "");
        let map = replaced.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
        map.into_iter()
            .map(|(chr, amount)| SortingElement { amount, chr })
            .fold(BTreeSet::new(), |mut set, e| {
                set.insert(e);
                set
            })
    }
}
