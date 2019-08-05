// Copyright © 2018 Tacy Bechtel and Terry Tower
// [This program is licensed under the "GNU License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

This program allows a user to check what gear a player is using. The program needs to be provided with three pieces of information: first, an API key; second, the platform that the player is using; and third, the character's name.

The API key needs to be stored in a file named "api-key.txt". This file needs to be located in the directory where you will run the code. This will most likely be the main directory. **along side main.rs and lib.rs in the src directory.** This key can be obtained from Bungie[https://www.bungie.net/en/Application].

The other two parameters will be passed in from the command line in the style
```
cargo run [platform] [player name]
```

Destiny can be played on Playstation, Xbox, or Computer. Each of these platforms are isolated from reach other. In order to access the correct set of databases the platform identifier is included in the API request. The platform is encoded with the following identifiersi: 1 identifies with Xbox, 2 with the Playstation 4, and 4 with a computer.

The player's name can be observed by looking at the player and recording the name or looking into the roster table and the name of every member of your party will be displayed there.

This program is heavily reliant to the API interface that Bungie provides. When the information required has been entered an initial request is sent to Bungie. This request requires the player name and platform to construct the correct url to send. This request also requires the api key attached to header when the request is sent. The responce is then parced to obtain the member id. This is how Bungie trackes each player. The member id is then used to obtain what gear a player has equiped at any given time. The responce to this request is a set of gear ids imbeded in a json object. Each of these items used to be kept in an SQLite database. This is no longer required and this information can now be requested using the API. 
The item names and flavor text is taken from the response to these recent calls and displayed. 

Features to add:
- add a CLI that will accept player's name and platform and then search.
- Output gear to file?

## License

This program is licensed under the "GNU License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.
