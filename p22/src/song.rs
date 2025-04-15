pub fn print_song() -> String {
    let mut song = String::new();
    for i in 1u32..=12 {
        match i {
            1 => song.push_str("On the first day of Christmas my true love sent to me\n"),
            2 => song.push_str("On the second day of Christmas my true love sent to me\n"),
            3 => song.push_str("On the third day of Christmas my true love sent to me\n"),
            4 => song.push_str("On the fourth day of Christmas my true love sent to me\n"),
            5 => song.push_str("On the fifth day of Christmas my true love sent to me\n"),
            6 => song.push_str("On the sixth day of Christmas my true love sent to me\n"),
            7 => song.push_str("On the seventh day of Christmas my true love sent to me\n"),
            8 => song.push_str("On the eighth day of Christmas my true love sent to me\n"),
            9 => song.push_str("On the ninth day of Christmas my true love sent to me\n"),
            10 => song.push_str("On the tenth day of Christmas my true love sent to me\n"),
            11 => song.push_str("On the eleventh day of Christmas my true love sent to me\n"),
            _ => song.push_str("On the twelfth day of Christmas my true love sent to me\n"),
        }
        for j in (i..=12).rev() {
            match j {
                12 => song.push_str("Twelve drummers drumming,\n"),
                11 => song.push_str("Eleven pipers piping,\n"),
                10 => song.push_str("Ten lords a-leaping,\n"),
                9 => song.push_str("Nine ladies dancing,\n"),
                8 => song.push_str("Eight maids a-milking,\n"),
                7 => song.push_str("Seven swans a-swimming,\n"),
                6 => song.push_str("Six geese a-laying,\n"),
                5 => song.push_str("Five golden rings,\n"),
                4 => song.push_str("Four calling birds,\n"),
                3 => song.push_str("Three French hens,\n"),
                2 => song.push_str("Two turtle doves,\n"),
                _ => song.push_str("A partridge in a pear tree.\n"),
            }
        }
    }
    song
}

#[cfg(test)]
mod song_tests;
