## Description
A lightweight wrapper around mpv/youtube-dl to check if streams are online and play them without using a browser. Also a chance to play around with rust.

## Compatability
this was designed with Linux in mind and only tested there.

## Usage
Run `streamscript --help` for output about commands and requirements.
### creating a config
should be in `~/.config/streamscript/config.toml` and have entries like:

```
[[stream]]
name = mystream
url = https://example.com
mode = audio
```
look at the example config toml for reference.

### sorting
I may decide to paginate and add sorting later but for now it's easy enough to do with grep.

```
# by mode
rust-streamscript list | grep -A 1 -B 1 mode

# by name
rust-streamscript list | grep -B 3 word
```
