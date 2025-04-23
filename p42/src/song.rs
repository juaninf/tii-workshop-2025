struct SongIter {
    song_line_verse_type_counter: usize,
    song_associated_line_verse_type_counter: usize,
}

impl Iterator for SongIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        const SONG_ASSOC: [&str; 12] = [
            "A partridge in a pear tree.\n",
            "Two turtle doves,\n",
            "Three French hens,\n",
            "Four calling birds,\n",
            "Five golden rings,\n",
            "Six geese a-laying,\n",
            "Seven swans a-swimming,\n",
            "Eight maids a-milking,\n",
            "Nine ladies dancing,\n",
            "Ten lords a-leaping,\n",
            "Eleven pipers piping,\n",
            "Twelve drummers drumming,\n",
        ];
        const SONG_DAY: [&str; 12] = [
            "On the first day of Christmas my true love sent to me\n",
            "On the second day of Christmas my true love sent to me\n",
            "On the third day of Christmas my true love sent to me\n",
            "On the fourth day of Christmas my true love sent to me\n",
            "On the fifth day of Christmas my true love sent to me\n",
            "On the sixth day of Christmas my true love sent to me\n",
            "On the seventh day of Christmas my true love sent to me\n",
            "On the eighth day of Christmas my true love sent to me\n",
            "On the ninth day of Christmas my true love sent to me\n",
            "On the tenth day of Christmas my true love sent to me\n",
            "On the eleventh day of Christmas my true love sent to me\n",
            "On the twelfth day of Christmas my true love sent to me\n",
        ];

        let v = self.song_line_verse_type_counter;
        let o = self.song_associated_line_verse_type_counter;

        if v >= SONG_DAY.len() {
            return None;
        }
        let line_no = v * (v + 3) / 2 + o + 1;
        let out = if o == 0 {
            self.song_associated_line_verse_type_counter = 1;
            format!("{line_no}: {}", SONG_DAY[v])
        } else {
            let idx = v - (o - 1);
            let s = format!("{line_no}: {}", SONG_ASSOC[idx]);
            if o == v + 1 {
                self.song_line_verse_type_counter += 1;
                self.song_associated_line_verse_type_counter = 0;
            } else {
                self.song_associated_line_verse_type_counter += 1;
            }
            s
        };
        Some(out)
    }
}

use std::iter;

trait DuplicateExt: Iterator + Sized {
    fn duplicate(self, n: usize) -> impl Iterator<Item = Self::Item>
    where
        Self::Item: Clone,
    {
        self.flat_map(move |x| iter::repeat(x).take(n))
    }
}

impl<I: Iterator> DuplicateExt for I {}

pub fn write_song() -> Vec<String> {
    let mut song_vec: Vec<String> = Vec::new();
    for i in 1u32..=12 {
        let mut song = String::new();
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
        song_vec.push(song);
    }
    song_vec
}

pub fn song_to_string(mut iteration: impl Iterator<Item = String>) -> String {
    let mut song = String::new();
    for line in iteration {
        println!("{line}");
        song.push_str(&line);
    }
    song
}

pub fn song_to_file(mut iteration: impl Iterator<Item = String>, path: String) {
    use std::fs::File;
    use std::io::Error;
    use std::io::Write;
    use std::path::Path;

    let path = Path::new(&path);
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    for line in iteration {
        println!("{line}");
        if let Err(why) = file.write(line.as_bytes()) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    }
}

pub fn song_to_tcp(mut iteration: impl Iterator<Item = String>, path: String) {
    use std::io::Write;
    use std::net::TcpStream;

    let mut stream = TcpStream::connect(path).expect("Could not connect to server");
    for line in iteration {
        println!("{line}");
        stream
            .write(line.as_bytes())
            .expect("Failed to write to stream");
    }
}

#[cfg(test)]
mod song_tests;
