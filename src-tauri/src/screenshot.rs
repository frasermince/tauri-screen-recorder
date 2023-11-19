use maplit::hashmap;
use screenshots::Screen;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

fn zero_shot_prompt() -> String {
    let prompt = "
You are a game playing agent I want you to describe interesting things in the environment and your conclusions about the game.
Focus more on what is explorable than details. Feel free to decide nothing in the scene is particularly interesting.
Remember game play information not artistic details.
We will have several sections for you to respond with. Some of these might be empty or unchanged if nothing interesting is happening.
For every observation we will you give the previous result and you can choose to keep the individual parts or not depending on their relevance.
PLEASE ENSURE YOU OUTPUT THE REQUESTED STRUCTURE

Please choose from the following actions to meet your goals:
Interact <entity>
Move <direction>
Move towards <object>
Select <Menu State>
Explore
Equip Object
Grind Area


Give me data in the following format:

Format: {
    \"Game\": \"[Name of the game as best as you can tell]\",
    \"Observation\": \"[A description of the scene and what you think is happening.]\",
    \"World Information\": \"[New high level information learned about the world in bullet points]\",
    \"Objects to Explore\": \"[Bulleted list of objects to examine in the world]\",
    \"Goals\": \"[Bulleted list of goals based on observations]\",
    \"Action\": \"[Current action to meet your top goals]\",
    \"Reward\": \"[Reward for past action -1 to 1]\"
}
\"USER:[img-6]Generate the specified format for the image
Previous:
None
ASSISTANT:
 \"";
    return format!("{}", prompt);
}

fn response_prompt() -> String {
    let prompt = "
    {
        \"Game\": \"\",
        \"Observation\": \"\",
        \"World Information\": \"\",
        \"Objects to Explore\": \"\",
        \"Goals\": \"\",
        \"Action\": \"\",
        \"Reward\": \"\"
    }";
    return format!("{}", prompt);
}

pub fn default_json(uuid: Uuid) -> HashMap<&'static str, Value> {
    hashmap! {
        "id" => json!(uuid.to_string()),
        "image" => json!(format!("{}.png", uuid)),
        "conversations" => json!(vec![hashmap!{
            "from" => "human".to_string(),
            "message" => zero_shot_prompt()
        }, hashmap!{
            "from" => "gpt".to_string(),
            "message" => response_prompt()
        }]),
    }
}

pub fn append_to_json_file(path: &str, object: HashMap<&'static str, Value>) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut data: Value = serde_json::from_str(&contents).unwrap();
    if let Some(array) = data.as_array_mut() {
        array.push(json!(object));
    }

    contents = serde_json::to_string(&data).unwrap();
    drop(file);

    // Reopen the file in write mode
    let mut file = OpenOptions::new().write(true).open(path).unwrap();

    file.write_all(contents.as_bytes()).unwrap()
}

pub fn capture() {
    let id = Uuid::new_v4();
    let start = SystemTime::now();
    for screen in Screen::all().unwrap() {
        println!("capturer {screen:?}");
        let image = screen.capture_area(30, 198, 1255, 750).unwrap();
        let buffer = image.buffer();
        fs::write(
            format!("/Users/frasermince/Documents/champion/images/{}.png", id,),
            buffer,
        )
        .unwrap();
    }
    default_json(id);
    append_to_json_file(
        "/Users/frasermince/Documents/champion/data.json",
        default_json(id),
    );
    println!("Done: {:?}", start.elapsed().ok());
}
