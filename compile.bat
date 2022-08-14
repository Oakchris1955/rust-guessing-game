@echo off

cargo build --release
echo Finished compilation
copy "target\release\hinted_guessing_game.exe" "main.exe"
echo Moved compiled binary
pause