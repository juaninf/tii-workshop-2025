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
