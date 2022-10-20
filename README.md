# YOUTUBE LINK TO WORKSHOP VIDEO FOR PART 1 BELOW:

< COMING SOON >

---------------------

# PART 1
Connect to soma.fm: https://somafm.com/groovesalad/songhistory.html

Pull in the song history

Store song history
  - Played At (Don't need it -- need to parse out text from time HH:MM
  - Artist
  - Song
  - Album

  * Some rows are "HH:MM	Break / Station ID"

---------------------

# PART 2:

### YOUR HOMEWORK:

For each song in the playlist datastructure

Search youtube for music video based on "artist" + "song"

https://www.youtube.com/results?search_query=Groove+Matter+%2B+97+Ways

Take the result of each loop iteration, pick the first item in the result, and store it in a YPlaylistItem struct

Your end result should be a vector of YPlaylistItem structs.

See tests in the youtube.rs spider for examples of some code that will help, and also the additional instructions in the comment at the top of that file.

#### WARNING!!!  ATTENTION!!!  Do the above behind a VPN, and implement a time delay between each call to Youtube (use 10sec delay to be safe)

### Submit your solution to PART 2 as a PR to this repo
