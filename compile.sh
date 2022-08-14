cargo build --release
echo Finished compilation
cp "target/release/hinted_guessing_game" "main"
echo Moved compiled binary
# Credit to the command below which pauses the script goes to https://stackoverflow.com/a/92813/
read -n1 -r -p "Press any key to continue . . ." key