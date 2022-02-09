// Code to work out what the mythic affixes are this week and next week.
// Based on https://wowaffixes.info by Isak (https://wisak.me/)

use chrono::prelude::*;

/// For each week in the season rota, the IDs of its 4 affixes
const WEEKLY_AFFIXES: [[u8; 4]; 12] = [
    [10, 11, 124, 128],
    [9, 6, 3, 128],
    [10, 122, 12, 128],
    [9, 123, 4, 128],
    [10, 7, 14, 128],
    [9, 8, 124, 128],
    [10, 6, 13, 128],
    [9, 11, 3, 128],
    [10, 123, 12, 128],
    [9, 122, 14, 128],
    [10, 8, 4, 128],
    [9, 7, 13, 128],
];

/// Get the whole number of weeks elapsed since the start of the season
fn get_week_number(datetime: chrono::DateTime<chrono::Utc>) -> u32 {
    // Make a timedate representing the start of the affix season
    let season_start = Utc.ymd(2021, 6, 30).and_hms(0, 0, 0);

    // Work out the whole number of weeks since the affix season start
    let time_in_season = datetime - season_start;
    time_in_season.num_weeks() as u32
}

/// Get the IDs of the 4 affixes active at the given timestamp
pub fn get_affixes(datetime: chrono::DateTime<chrono::Utc>) -> [u8; 4] {
    let week_number = get_week_number(datetime);
    return WEEKLY_AFFIXES[week_number as usize % WEEKLY_AFFIXES.len()];
}

/// Get the English name of the affix with the given ID
pub fn affix_name(affix_id: u8) -> &'static str {
    match affix_id {
        1 => "Overflowing",
        2 => "Skittish",
        3 => "Volcanic",
        4 => "Necrotic",
        5 => "Teeming",
        6 => "Raging",
        7 => "Bolstering",
        8 => "Sanguine",
        9 => "Tyrannical",
        10 => "Fortified",
        11 => "Bursting",
        12 => "Grievous",
        13 => "Explosive",
        14 => "Quaking",
        119 => "Beguiling",
        120 => "Awakened",
        121 => "Prideful",
        122 => "Inspiring",
        123 => "Spiteful",
        124 => "Storming",
        128 => "Tormented",
        _ => "Unknown affix",
    }
}

/// Get the bot-format emoji for the affix with the given ID
pub fn affix_emoji(affix_id: u8) -> &'static str {
    match affix_id {
        1 => "",
        2 => "",
        3 => "<:volcanic:941085705545871390>",
        4 => "<:necrotic:941085705558433862>",
        5 => "",
        6 => "<:raging:941085705491349594>",
        7 => "<:bolstering:941085705440993340>",
        8 => "<:sanguine:941085705529085952>",
        9 => "<:tyrannical:941085705600372756>",
        10 => "<:fortified:941085705055117363>",
        11 => "<:bursting:941085705399074846>",
        12 => "<:grievous:941085705478766612>",
        13 => "<:explosive:941085705432612954>",
        14 => "<:quaking:941085705478742076>",
        119 => "",
        120 => "",
        121 => "",
        122 => "<:inspiring:941085705440993390>",
        123 => "<:spiteful:941085705214500886>",
        124 => "<:storming:941085705491329094>",
        128 => "<:tormented:941085705503907860>",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_affixes() {
        // Check a week's affixes
        let datetime = Utc.ymd(2022, 2, 9).and_hms(21, 0, 0);
        let affix_ids = get_affixes(datetime);
        let affixes: Vec<&'static str> = affix_ids.iter().map(|id| affix_name(*id)).collect();

        assert_eq!(
            affixes,
            vec!["Fortified", "Spiteful", "Grievous", "Tormented",]
        );

        // Check a different week's affixes
        let datetime = Utc.ymd(2022, 2, 16).and_hms(21, 0, 0);
        let affix_ids = get_affixes(datetime);
        let affixes: Vec<&'static str> = affix_ids.iter().map(|id| affix_name(*id)).collect();

        assert_eq!(
            affixes,
            vec!["Tyrannical", "Inspiring", "Quaking", "Tormented",]
        );
    }
}
