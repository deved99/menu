# menu
This is the menu system I use. It includes 3
menus.

## run
This script lists all files in $PATH, executes the
one you choose.

## music
it reads a file that contains all my songs, then
plays the choosen one. More preciselly, it
executes `openfile SONG` where SONG is the choosen
song and `openfile` can be found
[here](https://github.com/deved99/openfile).

## videos
This script assumes ```~/Videos``` is composed by
folders which contain videos, sorted with the
wanted order. It keeps track of the progress in
```~/Videos/watched.json```. This too uses `openfile`.
