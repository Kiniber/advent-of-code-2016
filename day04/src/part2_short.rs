use regex::Regex;

pub fn part2_short(input: &str) -> anyhow::Result<()> {
    let regex = Regex::new(
        r"(?<encrypted_name_part>(([a-z]+)-?)+)-(?<sector_id>\d+)",
    )?;
    let results = regex.captures_iter(input).collect::<Vec<_>>();
    let secret_room = results
        .into_iter()
        .map(|captures| {
            let encrypted_name_part = captures.name("encrypted_name_part").unwrap().as_str();
            let sector_id = captures.name("sector_id").unwrap().as_str().parse::<u16>().unwrap();
            Room::new(encrypted_name_part, sector_id)
        })
        .find(|room|room.get_decrypted_name().starts_with("northpole object storage"))
        .expect("No secret room found");
    //.for_each(|(encrypted_name_part, sector_id, checksum, _)|println!("{encrypted_name_part:?} - {sector_id:?} - {checksum:?}"))
    println!("Secret room id: {}", secret_room.sector_id);
    Ok(())
}

struct Room<'a> {
    encrypted_name_part: &'a str,
    sector_id: u16,
}

impl<'a> Room<'a> {
    fn new(encrypted_name_part: &'a str, sector_id: u16) -> Self {
        Self {
            encrypted_name_part,
            sector_id,
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
}
