# Diablo II save file generator

Generate valid d2s file using simple parameters.

After run, application will ask to provide following parameters:
- name
  - from 2 to 15 characters
  - only letters with maximum one hyphen ("-") or underscore ("_")
  - need to start and end with a letter
- class
  - number or class name from following:
    - 1 - Amazon
    - 2 - Sorceress
    - 3 - Necromancer
    - 4 - Paladin
    - 5 - Barbarian
    - 6 - Druid
    - 7 - Assassin
- mode
  - SC for softcore, HC for hardcore
- level
  - value from 1 to 99, note completed difficulty constraints
- completed difficulty
  - name of difficulty that character completed: NORMAL, NIGHTMARE or HELL, or NONE if character should be created at the start of the game
  - there is minimal level constraint to finish each difficulty (due to "Rite of Passage" quests requirements)
    - Normal - level 20
    - Nightmare - level 40
    - Hell - level 60
- stashed gold
  - value of gold from 0 to 2500000 (2.5 million)
  - gold would be placed in character stash

Based on that parameters d2s file would be generated that follows game rules regarding character statistics, skill points etc. Stats and skill points would be available in game for manual distribution. 
All completed difficulties (selected one and all below that) would have completed all quests and discovered all waypoints. All quests rewards would be included in character statistics, that is:
- \+ 4 skill points for each completed difficulty ("Den of Evil", "Radament's Lair" and "The Fallen Angel" quests rewards)
- \+ 5 stats points for each completed difficulty ("Lam Esen's Tome" quest reward)
- \+ 20 life for each completed difficulty ("The Golden Bird" quest reward)
- \+ 10% to all resistances for each completed difficulty ("Prison of Ice" reward)

Application creates file named `{character_name}.d2s` in working directory. If such file already exists, execution will fail with an error, leaving existing file untouched.

**References**

During development, I used following resources that describes d2s file format:
- [krisives/d2s-format](https://github.com/krisives/d2s-format)
- [nokka/d2s](https://github.com/nokka/d2s)
- [Diablo II Saved Game File Format](https://user.xmission.com/~trevin/DiabloIIv1.09_File_Format.shtml)

Test cases for characters at level 1 was prepared using Diablo II LoD v.1.14d. Other test cases was prepared using [Hero Editor v1.04](https://www.moddb.com/games/diablo-2-lod/downloads/hero-editor-v-104) together with values calculated with [maxroll.gg D2 planner](https://maxroll.gg/d2/d2planner/).
