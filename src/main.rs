use serde::{Deserialize, Serialize};
use std::io;



fn main() {
    
    // Repeatedly prompt for input
    loop {    
        println!("\nWhat word would you like the definition for?\n'Q' to quit");

        let mut user_input = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut user_input).unwrap();
        if user_input.trim().to_string() != "Q" {
            query_fn(user_input);
        } else {
            return;
        }
    }
}

fn query_fn(user_input: String) {


    pub type Response = Vec<Body>;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Body {
        pub word: String,
        // Ignore these fields
        // pub phonetic: String,
        // pub phonetics: Vec<Phonetic>,
        pub meanings: Vec<Meaning>,
    }

    // Ignore these fields
    // #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    // #[serde(rename_all = "camelCase")]
    // pub struct Phonetic {
    //     pub text: String,
    //     pub audio: Option<String>,
    // }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Meaning {
        pub part_of_speech: String,
        pub definitions: Vec<Definition>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Definition {
        pub definition: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
pub struct WordNotFound {
    pub title: String,
    pub message: String,
    pub resolution: String,
    }


    // format and make a synchronous get request
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", user_input);
    let res = reqwest::blocking::get(url).unwrap();

    //Handle invalid words
    if "200" != res.status().as_str() {
        let word_not_found = res.json::<WordNotFound>();
        println!("{}", word_not_found.unwrap().message);
        return;
    }

    let nested = res.json::<Response>();

    let dictionary_def = &nested.unwrap()[0];

    println!("Word: {}", dictionary_def.word);

    let def_vec = &dictionary_def.meanings;

    for def in def_vec.iter() {
        println!("\nPart of Speech: {}", def.part_of_speech);
        for d in def.definitions.iter() {
            println!("Definition: {}", d.definition);
        }
    }
}