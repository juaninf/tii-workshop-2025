use super::*;

#[test]
fn test_write_song() {
    let expected_output = "On the first day of Christmas my true love sent to me\nTwelve drummers drumming,\nEleven pipers piping,\nTen lords a-leaping,\nNine ladies dancing,\nEight maids a-milking,\nSeven swans a-swimming,\nSix geese a-laying,\nFive golden rings,\nFour calling birds,\nThree French hens,\nTwo turtle doves,\nA partridge in a pear tree.\n";
    let nice_slice = &write_song()[0..12];
    assert_eq!(nice_slice[0], expected_output);
}

#[test]
fn test_next_from_iter() {
    let mut iter = SongIter {
        song_line_verse_type_counter: 0,
        song_associated_line_verse_type_counter: 0,
    };
    assert_eq!(
        iter.next(),
        Some("1: On the first day of Christmas my true love sent to me\n".to_string())
    );
    //
    assert_eq!(
        iter.next(),
        Some("2: A partridge in a pear tree.\n".to_string())
    );
    //
    assert_eq!(
        iter.next(),
        Some("3: On the second day of Christmas my true love sent to me\n".to_string())
    );
    //
    assert_eq!(iter.next(), Some("4: Two turtle doves,\n".to_string()));
}

#[test]
fn test_duplicate_iterator() {
    let song_iter = SongIter {
        song_line_verse_type_counter: 0,
        song_associated_line_verse_type_counter: 0,
    };

    for line in song_iter.duplicate(2) {
        // every line printed twice
        print!("{line}");
    }
}

#[test]
fn test_song_to_string() {
    let v = vec!["2".to_string(), "3".to_string(), "4".to_string()];
    let mut song_iter = v.into_iter();
    let concatenated_string = song_to_string(song_iter);
    assert_eq!(concatenated_string, "234");
}

#[test]
fn test_song_to_file() {
    let file_path = "test_song.txt".to_string();
    let v = vec!["2".to_string(), "3".to_string(), "4".to_string()];
    let mut song_iter = v.into_iter();
    song_to_file(song_iter, file_path);
    let file_content = std::fs::read_to_string("test_song.txt").unwrap();
    assert_eq!(file_content, "234");
}

#[test]
fn test_song_to_tcp() {
    let v = vec!["2".to_string(), "3".to_string(), "4".to_string()];
    let mut song_iter = v.into_iter();
    song_to_tcp(song_iter, "127.0.0.1:1234".to_string());
    //assert_eq!(tcp, "234");
}
