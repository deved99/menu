# menu
This is the menu system I use. It includes 7
menus.

## menu
This is designed to be the first menu you open. It
has a configuration file at
`~/.config/menu/config.json`. For example:
```
[
    {"name": "Run", "cmd": "./scripts/run"},
    {"name": "Power", "dir": "./power"}
]
```
This is a list of entries: selecting `Run`, `menu`
will run `./scripts/run`; selecting `Power`,
`menu` will change directory to `./power`, then
will read `./config.json`, like this one:
```
[
    {"name": "Poweroff", "cmd": "poweroff"},
    {"name": "Reboot", "cmd": "reboot"}
]
```
This enables the creation of submenus.


## emojis
This script asks the user to select an emoji, than
writes the choosen one. It gets the emoji list
from `~/.config/local/emoji.json`, which is
structured like this:
```
{
  "current": { "" },
  "backup": {...},
  "renames": {...},
  "most_used": {...}
}
```

## files
This is a pretty basic file manager. If the
selected entry is a directory, it will enter that
directory and list the files. Otherwise, it will
open the selected entry with `openfile`.

## music
It reads a file that contains all my songs, then
plays the choosen one. More preciselly, it
executes `openfile SONG` where SONG is the choosen
song and `openfile` can be found
[here](https://github.com/deved99/openfile).

## pass
This is a simple wrapper around
[pass](https://www.passwordstore.org/). It lists
all files in `~/.password-store`, then type the
selected passwords.

## run
This script lists all files in $PATH, executes the
one you choose.

## videos
This script assumes `~/Videos` is composed by
folders which contain videos, sorted with the
wanted order. It keeps track of the progress in
`~/Videos/watched.json`. This too uses `openfile`.
