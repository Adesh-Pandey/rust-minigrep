# Rust-Mini-Grep
cli tool for searching text in a file, file in a dir.

1. searching for text inside a file

 (Optional: IGNORE_CASE=1) cargo run -- -sif FILE_NAME WORD_TO_SEARCH

 2. searching for file in a directory

  cargo run -- -s FILE_NAME STARTING_LOCATION 


searching for the file function uses a BFS strategy.
  
WARNING - There are good amount of error handled with panic or unwrap which can be fixed.
  
