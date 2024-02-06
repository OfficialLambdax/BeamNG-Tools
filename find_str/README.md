# Find String

## Motivation
I can sometimes not find things in the BeamNG game installation. Sometimes lua or other things refer to functions or definitions where it isnt easily possible to tell where the things are coming from, or how they are defined or how they can be fully used.

## Usage
Command line tool.
- `find_str SearchString`
- `find_str SearchString extension1,extension2`

## Innerworkings
1. Will lookup the game installation from HKCU/Software/BeamNG/BeamNG.drive/rootpath
2. Will read each file with a specified extension into memory (default: lua,js,jbeam)
3. Will do a simple *does X string in Y exists*
4. Will list the results as this
```
Found string in file - PathToFile
  @ This Line
  : Full line where the string was found (max of 200 characters, where the Search string is highlighted)
```
