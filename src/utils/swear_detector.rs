use rand::{self, Rng};
use serenity::model::prelude::Message;

struct SwearWord {
    words: &'static [&'static str],
    responses: &'static [&'static str],
}

const BANNED_WORDS: &[SwearWord] = &[
    SwearWord {
        words: &["fuck", "fucking", "fucker"],
        responses: &[
            "Orh hor say F word I tell mummy",
            "Orh hor say F word u naughty naughty u teasing me",
        ],
    },

    SwearWord {
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
        words: &["chink", "cheenk"],
        responses: &[
            "Chink, a English-language ethnic slur usually referring to a person of Chinese descent.",
            "A chink is usually defined from the person's facial appearance, such as having small eyes. (Aidan)",
            "The word that u have just used (Chink), is thought to have originated from ancient China as the Qing dynasty is sometimes pronounced as \"Chink\" in America",
            "When I came to the U.S. as an adopted child from Vietnam (Bill). I was playing on a swing set  when a kid my age walked out of his house and came up to me and said, \"Hey chink, get off the swing I want to use it.\"  I didn\'t know what the word meant and went home and looked it up in the dictionary. [Definition of CHINK] : a narrow beam of light shining through a hole of a wall or building. I laughed so hard until my good friend Tyron Jamal Jones James Johnson explained to me that it means the same as the N word people applied to him. We both went back the next day and kicked the living racist \"chink\" out of his loathsome armor. \"Words have many meanings, but hate has only one intention, to cut at a person\'s self worth so the user of that word can feel a false sense of worth.\"",    
        ]
    },

    SwearWord {
        words: &["royce", "fat", "obese", "chunky", "thorston", "kamal"],
        responses: &[
            "little fat fuck", "royce kinda gay", "i love physics",
            "asian leaksss", "omg kamal yacob hehehehehehehor",
            "i hate tryhards LAZY EYE"
        ]
    },

    SwearWord {
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

    debug!("Looking for swears...");

    // Loop through swear words and return selected choice
    for swear_word in BANNED_WORDS {
        for word in swear_word.words {
            if content.contains(word) {
                debug!("Found swear word, {}!", word);
                let response = swear_word.responses
                    [rand::thread_rng().gen_range(0..swear_word.responses.len())];
                return Some(response.to_string());
            }
        }
    }

    // No swear words were found
    None
}
