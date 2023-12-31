<div align="center">A simple guitar tuner in your terminal</div>
<br>

![render1699094556281](https://github.com/Goose97/terminal-guitar-tuner/assets/35915460/c4f17e2d-d93c-40dd-8236-b140cbf1416b)

## Installation

### macOS

```
brew tap goose97/terminal-guitar-tuner https://github.com/Goose97/terminal-guitar-tuner
brew install goose97/terminal-guitar-tuner/terminal-guitar-tuner
```

Since it's a self-host Homebrew formula, Apple won't trust application from unidentified developers. You will need to allow this application in the "Security & Privacy" settings.

### Linux: coming soon

## Credits

- The program use an auto-correlation pitch detecting algorithm described in the paper [A smarter way to find pitch](https://www.cs.otago.ac.nz/graphics/Geoff/tartini/papers/A_Smarter_Way_to_Find_Pitch.pdf) (_Philip McLeod, Geoff Wyvill_)
- Some code references the implementation from [this repo](https://github.com/sevagh/pitch-detection/blob/master/misc/mcleod/README.md)
