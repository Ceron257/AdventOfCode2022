/*
  * we have 1 ore collecting bot
  * Factory:
    * builds 1 bot per minute
    * consumes required resources *at beginning*
  * every bot can collect 1 unit per minute

  +----------------+-------------------------+------------+
  | ingredient     | bot                     | collects   |
  +----------------+-------------------------+------------+
  | ore + obsidian | geode cracking bot      | open geode |
  | ore + clay     | obsidian collecting bot | obsidian   |
  | ore            | clay collecting bot     | clay       |
  +----------------+-------------------------+------------+
*/

fn main() {}