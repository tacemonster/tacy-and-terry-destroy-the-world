// Copyright © 2018 Tacy Bechtel and Terry Tower
// [This program is licensed under the "GNU License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

fn main() {
    //Keeping the API key in a seperate file prevents it from being accidently commited to the repo.
    //If the get_api_key() fails make sure that it is stored in a text file named api-key.txt
    let api_key = inventory::get_api_key();
    
    //The platform corresponds to where the game is being played.
    //2 is xBox and 4 is PC
    let platform = '2';
    //let platform = '4';
    //Gamer tags are plane for xBox, PC names need '%23' to represent the pound sign and then the Battle.net identifier appended to the name
    let player_name = String::from("cortical_iv");
    //let player_name = String::from("shark90%231673");
	
    //get_member_id(api_key::String, platform::char, player_name::String) -> String
    let mut membership_id = inventory::get_member_id(&api_key, platform, player_name);
    //strip_quotes(membership_id) -> String
    membership_id = inventory::strip_quotes(membership_id);//removes quotes
    //all_equipment(api_key::String, membership_id::String, platform::char) -> Vec <String>
    let items = inventory::all_equipment(&api_key, membership_id, platform);
    println!("\n\nYou are equipped with the following items:\n");
    inventory::print_equipped(items);
}
