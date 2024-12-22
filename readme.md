# Valorant WL tracker
> A dead simple applet to display current mmr data as a stream overlay

# About
The software allows you to get up to date information about some of a player's current mmr information, such as current wins, losses, rr, and rank. This data can then be templated into a file, and displayed in any broadcasting software. Given some interval (default every 2 minutes), the data is fetched again and updated. 

# Getting started
There is no need to install anything, just download the portable binary from the [latest release](https://github.com/lionelkarlen/wltracker/releases/latest), and copy it to a folder. \
You will need a configuration file `config.toml`, in the same folder. See below for the structure. You may also specify the path to the config file manually as the first argument. \
You will need an api key for the Henrik-3 valorant api. This is a fast and painless process. Check [here](https://docs.henrikdev.xyz/authentication-and-authorization) on how to get one. \

## Example configuration file
```toml
[user]
username="henrik3" # valorant username
tag="euw3" # valorant tag
region="eu" # one of ["eu", "na", "latam", "br", "ap", "kr"]; optional, defaults to "eu"
platform="pc" # of of ["pc", "console"]; optional, defaults to "pc"

[auth]
key="" # api key; see https://docs.henrikdev.xyz/authentication-and-authorization on how to get your own

[display]
template="w/l - {w}/{l}" # template string; accepted literals are ["{w}", "{l}", "{season}", "{change}", "{rr}", "{rank}"]
file_path="" # file output path; -> This is the file you put as your obs source

[internal]
interval=120 # value in seconds to wait between api calls (keep this reasonable, even though the basic api key supports 30req/min); optional, defaults to 120 seconds
```

### Template literals
A template literal is a piece of text, surrounded by `{}`, which is then replaced by that value when rendered.
The full list of template literals are:
- "{w}" -> The current wins of the player in the latest season
- "{l}" -> The current losses of the player in the latest season (includes draws, as is the convention)
- "{season}" -> The name of the latest season (in the format `e9a1`)
- "{change}" -> The rr gain/loss of the last game
- "{rr}" -> The current rr of the player
- "{rank}" -> The current rank of the player

NOTE: What is meant by "latest season" is the last season in which the player has already played, i.e. it will never show 0 wins and 0 losses;


# Contributions guide
If you are interesting in contributing, simply fork the repo and open a pull request with your feature or bug fix.

# Known issues/Roadmap
- [ ] Refactor the format_literals function
- [ ] Improve error readability based on API response status codes
- [ ] Add manual trigger for data update

# Acknowledgements
- [Henrik-3](https://github.com/Henrik-3/unofficial-valorant-api) for his amazing valorant api
