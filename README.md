# Why
Are you sick of always having to remove artefacts from your project folder, so that you don't push a '.idea' folder to your repo? No?
...
Well, I do and if you do to, this is the Tool for you!
<br>
i just realized that i basically build global gitignore instead of some useful tool, lol.
<br>
i'll still leave it up, even though i am quite a little bit embarressd that i didn't notice sooner.

# Description
When running the command for the first time it creates a config file at '~/.config/clear_git_config.txt' and exits the process. The config file already contains some examples, to show the formatting style that has to be used.

The config file gets checked every time the command is run, except if it is run with the '-V' or '-h' flag.

# Features
There is also a '-r' flag which you can give a comma seperated list of values that then get deleted togther with the config file values.
