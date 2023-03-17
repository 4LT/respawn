# Respawn

_Seth "4LT" Rader_

# About

Utility for processing Quake maps by modifying, adding, and deleting entities.

# Usage

`respawn <input-file>`

    Reads `<input-file>` and spits out a processed map with the same suffixed
    with "-post".

`respawn <input-file> <output-file>`
    
    Same, but with an explicitly provided output filename.

# Features

## Skill patching

Entities with keys starting with "easy:", "medium:", or "hard:" will be copied
and patched.

E.g. a trigger\_counter with a key/value pair of `"hard:count" "4"` will be
copied into a new entity with the same keys and values, but with `count` set to
4 and `spawnflags` set appropriately ("Not on easy" and "Not on hard").

# License

CC0 or MIT or Apache-2.0, your choice.

The respective licenses can be found at
* https://raw.githubusercontent.com/spdx/license-list-data/v3.11/text/CC0-1.0.txt
* https://raw.githubusercontent.com/spdx/license-list-data/v3.11/text/MIT.txt
* https://raw.githubusercontent.com/spdx/license-list-data/v3.11/text/Apache-2.0.txt

