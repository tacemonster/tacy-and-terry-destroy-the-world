#!/usr/bin/env python3
# -*- coding: utf-8 -*-

#from https://gist.github.com/cortical-iv/a22ef122e771b994454e02b6b4e481c3
"""
Getting Started using the Destiny 2 Api
An annotated guide to some of the public endpoints available for examining a user's
characters, items, and clan using the Destiny 2 API. You will need to use your api key for
this to work. Just insert it as a string where it says <my_api_key> in the beginning.
It is broken into four parts:
    0: Imports, variables, and fixed parameters defined
    1: Main hooks (destiny2_api_public to make requests, and the url generators)
    2: Helper functions that use those hooks to do useful things
    3: Simple examples of the hooks and helper functions in action. Enter a user_name
        and user_platform to look at their profile, characters, items, clan.
If this were a serious project, these would be different modules.
Code segments are also broken into commented cells (separated by #%%) so you can step through
this cell-by-cell (e.g., in Spyder), sort of like you would in a Jupyter notebook.
Caveats:
1. This code is not optimized, and is verbose. This is intentional. E.g., the
number of calls to 'get_user_id', each of which makes a request to bungie.net, is obscene.
2. Please let me know if you have a suggestion for improvements.
3. Will likely not work for pc players  (they have bnet#s appended to their username
and this code current doesn't handle that).
Acknowledgements:
Thanks to the folks at bungie api discord, the place to go for discussion
and help: https://discord.gg/WJDSUgj
"""
#%% imports
import requests
import json

#%%variables that you might want to change on different runs
user_name = 'shark90%231673'  #put name of person whose info/clan you want to explore
user_platform = 'pc'  #either 'psn' or 'ps4' or 'xbone' or 'xbox'  (pc is busted)
save_to_file = 0  #flag: set to 1 if you want certain bits saved to file to peruse

#%% fixed parameters
my_api_key = ''  #put your api key here!
baseurl = 'https://www.bungie.net/Platform/Destiny2/'
baseurl_groupv2 = 'https://www.bungie.net/Platform/GroupV2/'

membership_types = {'xbox': '1',  'xbone': '1', 'psn': '2', 'pc': '4', 'ps4': '2'}

#Following conversions have names I use for user summary stats as keys,
#and names that bungie uses as values for when I extract from end points.
#%% api hooks
def destiny2_api_public(url, api_key):
    """This is the main function for everything. It requests the info from the bungie servers
    by sending a url."""
    my_headers = my_headers = {"X-API-Key": my_api_key}
    response = requests.get(url, headers = my_headers)
    return ResponseSummary(response)


class ResponseSummary:
    '''
    Object contains all the important information about the request sent to bungie.
    '''
    def __init__(self, response):
        self.status = response.status_code
        self.url = response.url
        self.data = None
        self.message = None
        self.error_code = None
        self.error_status = None
        self.exception = None
        if self.status == 200:
            result = response.json()
            self.message = result['Message']
            self.error_code = result['ErrorCode']
            self.error_status = result['ErrorStatus']
            if self.error_code == 1:
                try:
                    self.data = result['Response']
                except Exception as ex:
                    print("ResponseSummary: 200 status and error_code 1, but there was no result['Response']")
                    print("Exception: {0}.\nType: {1}".format(ex, ex.__class__.__name__))
                    self.exception = ex.__class__.__name__
            else:
                print('No data returned for url: {0}.\n {1} was the error code with status 200.'.format(self.url, self.error_code))
        else:
            print('Request failed for url: {0}.\n.Status: {0}'.format(self.url, self.status))

    def __repr__(self):
        """What will be displayed/printed for the class instance."""
        disp_header =       "<" + self.__class__.__name__ + " instance>\n"
        disp_data =         ".data: " + str(self.data) + "\n\n"
        disp_url =          ".url: " + str(self.url) + "\n"
        disp_message =      ".message: " + str(self.message) + "\n"
        disp_status =       ".status: " + str(self.status) + "\n"
        disp_error_code =   ".error_code: " + str(self.error_code) + "\n"
        disp_error_status = ".error_status: " + str(self.error_status) + "\n"
        disp_exception =    ".exception: " + str(self.exception)
        return disp_header + disp_data + disp_url + disp_message + \
               disp_status + disp_error_code + disp_error_status + disp_exception


"""
URL GENERATORS
The following functions create urls in the format that the bungie servers want them.
In the docs for each function I give the url to bungie docs, partly to help but also so
you can see what I may have left out --- I'm not always including all possible query strings.
I named each url generator according to the bungie end point (e.g., if the end point is X
then the function is X_url)
"""
def search_destiny_player_url(user_name, user_platform):
    """Main point is to get the user's id from their username.
        https://bungie-net.github.io/multi/operation_get_Destiny2-SearchDestinyPlayer.html
    """
    membership_type = membership_types[user_platform]
    return baseurl + 'SearchDestinyPlayer/' + membership_type + '/' + user_name + '/'


def get_profile_url(user_name, user_platform,  components, my_api_key):
    """Get information about different aspects of user's character like equipped items.
        https://bungie-net.github.io/multi/operation_get_Destiny2-GetProfile.html
    Note components are just strings: '200,300' : you need at least one component."""
    user_id = get_user_id(user_name, user_platform, my_api_key)
    membership_type = membership_types[user_platform]
    return baseurl + membership_type + '/' + 'Profile/' + user_id + '/?components=' + components


def get_character_url(user_name, user_platform, character_id, components, my_api_key):
    """Similar to get_profile but does it for a single character. Note individual character
    id's are returned by get_profile.
        https://bungie-net.github.io/multi/operation_get_Destiny2-GetCharacter.html """
    user_id = get_user_id(user_name, user_platform, my_api_key)
    membership_type = membership_types[user_platform]
    return baseurl + membership_type + '/' + 'Profile/' + user_id + \
                            '/Character/' + character_id + '/?components=' + components


def get_item_url(user_name, user_platform, item_instance_id, components, my_api_key):
    """Pull item with item instance id (for instance if you have two instances of
    Uriel's Gift, this will let you pull information about one particular instance.
    You will get the itemInstanceId from item stats returned from get_profile or
    get_character.
        https://bungie-net.github.io/multi/operation_get_Destiny2-GetItem.html"""
    user_id = get_user_id(user_name, user_platform, my_api_key)
    membership_type = membership_types[user_platform]
    return baseurl + membership_type + '/Profile/' + user_id + \
                            '/item/' + item_instance_id + '/?components=' + components

def get_entity_definition_url(entity_hash, entity_type, my_api_key):
    """
    Hooking up with the manifest!
        https://bungie-net.github.io/multi/operation_get_Destiny2-GetDestinyEntityDefinition.html
    If you've got a hash, and know the entity type, you can use this (or just download
    the manifest and make your  own database to do it 10x faster)
    """
    return baseurl + 'Manifest/' + entity_type + '/' + entity_hash

#%% helper functions
"""
HELPER FUNCTIONS (ImHelping)
These functions all use the above url-generators and api endpoints, or some processed
data from the endpoints, in the use-case bits below.
"""
def get_user_id(user_name, user_platform, my_api_key):
    """Uses search_destiny_player end point to get user id. Returns None if there is a problem."""
    player_summary = destiny2_api_public(search_destiny_player_url(user_name, user_platform), my_api_key)
    if player_summary.error_code == 1:
        if player_summary.data:
            return player_summary.data[0]['membershipId']
        else:
            print('There is no data for {0} on {1}'.format(user_name, user_platform))
            return None
    else:
        print('There was an error getting id for {0}. Status: {1}'.format(user_name, player_summary.status))
        return None


def save_readable_json(data, filename):
    """This is if you want to save response data to filename in human-readable json format"""
    with open(filename, 'w') as fileObject:
        try:
            fileObject.write(json.dumps(data, indent = 3))
            print('Saved data to ' + filename)
        except:
            print('ya blew it saving ' + filename)

#%% ########################################################
#        END FUNCTION AND CLASS DEFINITIONS             #
#########################################################


"""
EXAMPLES
"""
if __name__ == '__main__':
    #%%SearchDestinyPlayer to get user id
    player_url = search_destiny_player_url(user_name, user_platform)
    player_summary = destiny2_api_public(player_url, my_api_key)
    user_id = player_summary.data[0]['membershipId']

    #%% Or just get id using a helper function
    user_id_alt = get_user_id(user_name, user_platform, my_api_key)

    #%%GetProfile
    #Component types include: 100 profiles; 200 characters; 201 non-equipped items (need oauth);
    #205: CharacterEquipment: what they currently have equipped. All can see this
    components = '200,205'
    profile_url = get_profile_url(user_name, user_platform, components, my_api_key)
    user_profile_summary = destiny2_api_public(profile_url, my_api_key)

    #%%extract character id's from profile
    user_characters = user_profile_summary.data['characters']['data']
    character_ids = list(user_characters.keys())
    user_character_0 = user_characters[character_ids[0]]

    #%%GetCharacter
    #This basically is GetProfile but for just one character that you show an id for
    #Note if you search for inventory components (201) if you get nothing it might say privacy: 2
    #public:1, private: 2
    character_components = '201,205'
    char_url = get_character_url(user_name, user_platform, character_ids[0], character_components, my_api_key)
    character_summary = destiny2_api_public(char_url, my_api_key)

    #Note this contains equipment and itemComponents (later isempty)
    character_items = character_summary.data['equipment']['data']['items']  #
    first_item = character_items[0]
    item_instance_id = first_item['itemInstanceId']
    item_hash = str(first_item['itemHash'])  #wtf is this? you might ask

    second_item = character_items[1]
    second_item_hash = str(second_item['itemHash'])

    fifth_item = character_items[4]
    fifth_item_hash = str(fifth_item['itemHash'])

    """
    Characteritems includes all equipped stuff. But you can't just read them off the data.
    They are encoded in hashes. To decode them you need to use the manifest. Which brings
    us to...GetEntityDefinition: https://destiny.plumbing/
    """

    #%% Access manifest via GetDestinyEntityDefinition,
    item_url =  get_entity_definition_url(item_hash, 'DestinyInventoryItemDefinition', my_api_key)
    item_summary = destiny2_api_public(item_url, my_api_key)

    #now you have all sorts of information about that inventory item:
    #This will pretty much have everything you need
    item_data = item_summary.data
    #item_data['displayProperties']

    #print(item_data.keys())
    #Let's get name and type
    item_name = item_data['displayProperties']['name']
    item_type = item_data['itemTypeAndTierDisplayName']
    print("\nWhat you've got here is a {0}. It's name is {1}.".format(item_type, item_name))
