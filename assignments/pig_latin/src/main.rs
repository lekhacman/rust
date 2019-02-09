/**
Convert strings to pig latin.
The first consonant of each word is moved to
the end of the word and “ay” is added, so “first” becomes “irst-fay.”
Words that start with a vowel have “hay” added to the end instead
(“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
*/

fn main() {
    assert_eq!(piggy_lib::to_pig_latin("hello"), "ellohay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("pig"), "igpay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("latin"), "atinlay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("banana"), "ananabay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("happy"), "appyhay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("duck"), "uckday".to_string());
    assert_eq!(piggy_lib::to_pig_latin("me"), "emay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("too"), "ootay".to_string());

    assert_eq!(piggy_lib::to_pig_latin("smile" ), "ilesmay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("string" ), "ingstray".to_string());
    assert_eq!(piggy_lib::to_pig_latin("stupid" ), "upidstay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("glove" ), "oveglay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("trash" ), "ashtray".to_string());
    assert_eq!(piggy_lib::to_pig_latin("floor"), "oorflay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("store"), "orestay".to_string());

    assert_eq!(piggy_lib::to_pig_latin("eat" ), "eathay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("omelet" ), "omelethay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("are" ), "arehay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("egg" ), "egghay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("explain" ), "explainhay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("always" ), "alwayshay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("ends" ), "endshay".to_string());
//    assert_eq!(piggy_lib::to_pig_latin("honest" ), "honesthay".to_string());
    assert_eq!(piggy_lib::to_pig_latin("I"), "Ihay".to_string());
}

mod piggy_lib {
    const CONSONANTS: [char; 21] = [
        'b',
        'c',
        'd',
        'f',
        'g',
        'h',
        'j',
        'k',
        'l',
        'm',
        'n',
        'p',
        'q',
        'r',
        's',
        't',
        'v',
        'x',
        'z',
        'w',
        'y',
    ];
    pub fn to_pig_latin(word: &str) -> String {
        let mut pig_word = String::from(word);

        let mut consonant = String::new();

        for c in word.chars() {
            if CONSONANTS.contains(&c) {
                consonant.push(c);
                pig_word.remove(0);
            } else {
                if consonant.len() == 0 { consonant.push('h') }
                break;
            }
        }
//        pig_word = word.s
        consonant.push_str("ay");
        pig_word.push_str(&consonant);

        pig_word
    }
}
