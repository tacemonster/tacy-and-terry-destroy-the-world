// Copyright © 2018 Tacy Bechtel and Terry Tower
// [This program is licensed under the "GNU License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

/*
fn usage() -> ! {
  eprintln!("Usage: cargo run [platform] [player name]");
  std::process::exit(1)
}
*/

fn main() {
    //Keeping the API key in a seperate file prevents it from being accidently commited to the repo.
    //If the get_api_key() fails make sure that it is stored in a text file named api-key.txt
    let api_key = inventory::get_api_key();
    //The platform corresponds to where the game is being played.
    //1 is xBox, 2 is PS4, and 4 is PC
    let platform;
    //Gamer tags are plane for xBox, PC names need '%23' to represent the pound sign and then the Battle.net identifier appended to the name
    let player_name;

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
      platform = '4';
      player_name = String::from("shark90%231673");
    } else {
      platform = args[1].parse::<char>().unwrap();
      player_name = args[2].clone();
    }
	
    //get_member_id(api_key::String, platform::char, player_name::String) -> String
    let mut membership_id = inventory::get_member_id(&api_key, platform, player_name);
    //strip_quotes(membership_id) -> String
    membership_id = inventory::strip_quotes(membership_id);//removes quotes
    //all_equipment(api_key::String, membership_id::String, platform::char) -> Vec <String>
    let items = inventory::all_equipment(&api_key, membership_id, platform);
    println!("\n\nYou are equipped with the following items:\n");
    inventory::print_equipped(items);
}
