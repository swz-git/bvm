```
 _                    
| |____   ___ __ ___  
| '_ \ \ / / '_ ` _ \ 
| |_) \ V /| | | | | |
|_.__/ \_/ |_| |_| |_|

fast bun version manager
```

Currently bvm only works on Linux but the plan is to support Windows and MacOS in the future as well.

## Install

`cargo install --git https://github.com/swz-git/bvm`

## Shell setup

add this to your `.bashrc` (or equivalent for your shell, `.zshrc` for zsh)

`eval "$(bvm env)"`

## Help

`bvm --help` or `bvm [SUBCOMMAND] --help`