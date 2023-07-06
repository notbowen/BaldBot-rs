use std::io::{Read, Write};
use std::{collections::HashMap, fs::File};

use rand::{self, Rng};
use serde::Deserialize;

use serenity::model::prelude::Message;

struct SwearWord {
    display_name: &'static str,
    words: &'static [&'static str],
    responses: &'static [&'static str],
}

const BANNED_WORDS: &[SwearWord] = &[
    SwearWord {
        display_name: "F Word",
        words: &["fuck", "fucking", "fucker"],
        responses: &[
            "Orh hor say F word I tell mummy",
            "Orh hor say F word u naughty naughty u teasing me",
        ],
    },

    SwearWord {
        display_name: "N Word",
        words: &["neger", "negro", "neeger", "nigger", "nigga", "黑鬼", "hei gui", "niger"],
        responses: &[
            "Orh hor say N word I tell mummy",
            "Orh hor say N word u naughty naughty u teasing me",
            "Nigger, a contemptuous term for a black or dark-skinned person. Nigger is an infamous word in current English, so much so that when people are called upon to discuss it, they more often than not refer to it euphemistically as \"the N-word.\" In senses 1 and 2, the word ranks as almost certainly the most offensive and inflammatory racial slur in English, a term expressive of hatred and bigotry. Sense 3 is also now rarely used and is often considered offensive. The word's self-referential uses by and among Black people are not always intended or taken as offensive (although many object to those uses as well), but its use by a person who is not Black to refer to a Black person can only be regarded as a deliberate expression of contemptuous racism. First known use of the word \"Nigger\" you may ask. The answer? 1755",
            "Black people might sometimes address other of their black friends, \"My Nigger!\"",
            "This word is often derogatory and Whites are banned from using that word. The reason for this is that the history of the word \"Nigger\" dates back to when Blacks were still used as slaves and people would not treat them like normal human beings and call them \"Negro\".",
            "In the 1800s, there were human zoos showcasing black people (Niggers!)",
            "NIGGGAAAAAAAAAAAAAAAAAAAAAA"
        ]
    },

    SwearWord {
        display_name: "Chink word",
        words: &["chink", "cheenk"],
        responses: &[
            "Chink, a English-language ethnic slur usually referring to a person of Chinese descent.",
            "A chink is usually defined from the person's facial appearance, such as having small eyes. (Aidan)",
            "The word that u have just used (Chink), is thought to have originated from ancient China as the Qing dynasty is sometimes pronounced as \"Chink\" in America",
            "When I came to the U.S. as an adopted child from Vietnam (Bill). I was playing on a swing set  when a kid my age walked out of his house and came up to me and said, \"Hey chink, get off the swing I want to use it.\"  I didn\'t know what the word meant and went home and looked it up in the dictionary. [Definition of CHINK] : a narrow beam of light shining through a hole of a wall or building. I laughed so hard until my good friend Tyron Jamal Jones James Johnson explained to me that it means the same as the N word people applied to him. We both went back the next day and kicked the living racist \"chink\" out of his loathsome armor. \"Words have many meanings, but hate has only one intention, to cut at a person\'s self worth so the user of that word can feel a false sense of worth.\"",    
        ]
    },

    SwearWord {
        display_name: "Fat word",
        words: &["royce", "fat", "obese", "chunky", "thorston", "kamal"],
        responses: &[
            "little fat fuck", "royce kinda gay", "i love physics",
            "asian leaksss", "omg kamal yacob hehehehehehehor",
            "i hate tryhards LAZY EYE"
        ]
    },

    SwearWord {
        display_name: "Tryhard",
        words: &["tryhard", "aidan", "bill", "min qi", "minqi", "4a1"],
        responses: &[
            "eww tryhard go and study medicine lah",
            "tryhard.exe",
        ]
    }
];

pub async fn get_swear_response(message_content: String) -> Option<String> {
    // Get content and remove spaces
    let mut content = message_content;
    content.retain(|c| !c.is_whitespace());

    // Loop through swear words and return selected choice
    for swear_word in BANNED_WORDS {
        for word in swear_word.words {
            if content.contains(word) {
                // Generate response
                let response = swear_word.responses
                    [rand::thread_rng().gen_range(0..swear_word.responses.len())];

                // Get word count
                let word_count = increment_word_count(swear_word.display_name).await;

                // Return response
                return Some(format!(
                    "{}\n{} count: {}",
                    response, swear_word.display_name, word_count
                ));
            }
        }
    }

    // No swear words were found
    None
}

/// Gets the word count from a JSON file, increments it, saves it and returns the new value
///
/// # Arguments
/// * `display_name` - The display name of the swear word
async fn increment_word_count(display_name: &str) -> u32 {
    // Open file and load data, else create the file if not found
    let mut file = match File::open("word_count.json") {
        Ok(file) => file,
        Err(_) => {
            let mut file = File::create("word_count.json").unwrap();
            file.write_all("{}".as_bytes()).unwrap();
            File::open("word_count.json").unwrap()
        }
    };
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut data: HashMap<String, u32> =
        serde_json::from_str(&data).expect("Word count JSON data to be well formatted");

    // Find the display_name and increment it, else init value to 1
    let new_count = match data.get(display_name) {
        Some(count) => count + 1,
        None => 1,
    };

    // Save new data
    data.insert(display_name.to_string(), new_count);
    let data = serde_json::to_string(&data).unwrap();
    let mut file = File::create("word_count.json").unwrap();
    file.write_all(data.as_bytes()).unwrap();

    new_count
}
