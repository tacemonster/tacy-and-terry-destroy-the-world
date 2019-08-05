Copyright © 2018 Tacy Bechtel and Terry Tower
This program is licensed under the "GNU License"
Please see the file LICENSE in the source distribution of this software for license terms.

# Purpose 

This program allows a user to check what gear a player is using. The program needs to be provided with three pieces of information. The first is an API key.  The second is the platform that the player is using. The third the the characters name.

# Usage

The API key needs to be stored in a file named "api-key.txt". This file needs to be located in the directory from which you will run the code. This will typically be the main directory or the src directory. This key can be obtained from [Bungie](https://www.bungie.net/en/Application]).

The other two parameters will be passed in from the command line in the style
```
cargo run [platform] [player name]
```

Destiny can be played on Playstation, Xbox, or Computer. Each of these platforms are isolated from reach other. In order to access the correct set of databases the platform identifier is included in the API request. The platform is encoded with the following identifiersi: 1 identifies with Xbox, 2 with the Playstation 4, and 4 with a computer.

If you want to test it out but don't have a palyer name, feel free to use *shark90%231673* 

# Implementations

This program is relies heavily on the API interface that Bungie provides. When the information required has been entered an initial request is sent to Bungie. This request requires the player name and platform to construct the correct url to send. This request also requires the api key attached to header when the request is sent. The response is then parsed to obtain the Member ID. This is how Bungie tracks each player. The Member ID is then used to obtain what gear a player has equipped at any given time. The response to this request is a set of gear IDs imbedded in a json object.

When we started this project, we found documentation saying that we should download and use an SQLite database to perform these requests. Once we got it up and running, we found that none of the values we were trying to identify were being matched by anything in the database. At this point we realized that the database is out of date, and the item information can now be requested using the API.

The item names and flavor text is taken from the response to these recent calls and displayed. 

Features to add:
- add a CLI that will accept player's name and platform and then search.
- Output gear to file?

## License

This program is licensed under the "GNU License".  Please
see the [LICENSE](https://www.github.com/tacemonster/tacy-and-terry-destroy-the-world/bloc/master/LICENSE]) file in the source distribution of this
software for licent and terms.
