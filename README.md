# ShrinkShot

A simple CLI tool to detect and cut out empty areas of an image (screenshot) in order to make it smaller without resizing its content. 

## Features

The program simply scans for whole pixel columns and lines which are identical to their neigbours. These columns and lines can be cut out from the image, no information will be loss, but the image will be smaller and more compact.

The program itself only a CLI tool, which tries to detect empty areas in a specified PNG image, then calls - with a bunch of parameters - ImageMagick's `convert` utility, which is doing the actual job.

You may assign a hotkey to yout script, which makes a screenshot of a window, passing through it on shrinkshot, and saves it.

## Usage

```
$ shrinkshot screenshot.png result.png
```

To make it work, first, you should install ImageMagick:

- MacOS: `brew install imagemagick`
- Debian/Ubuntu Linux: `sudo apt-get install imagemagick` 
- Windows: download, install (not sure)
- etc.

If any problem occurs, `shrinkshot` prints error messages to `stderr`.

## TODO

### Needs more test

- Only tested on sample images you can found in the `test/` directory. 
- Not tested on Linux distros yet.
- Test on Windows.

### Known issues to be fixed

If you make a screenshot by marking the area by hand (sometimes called snipping tool), empty area detection may fail on background image. A simple solution is just simply ignore 3-4 pixels on the borders, this hack will be applied soon.

### Enhance distribution

ShrinkShot is now a CLI tool for. It should be turned to an easy-to-install solution for making shrinked screenshots with a hotkey.

I don't want to turn it to a boxed software with printed manual, but the distribution should be more user-friendly:
- Linux: provide a shell script, which can be assigned to a hotkey. Probably release it as a `.deb` package.
- MacOS: have no idea.

### Conceptional issues

The program now detects empty areas by checking whole pixel rows and columns. It works well, but there are some cases, when empty regions does not occupy the whole height/width of the image.

There's an example, which the current method couldn't handle:
```
 -------------          ---------- 
| 111111      |    =>  | 111111   | 
|     2222222 |        |  2222222 |
 -------------          ----------
```

## Credits

The idea and some sample images come from a question issued by *Thomas* on the Software Recommendation (StackExchange) site.

The actual conversation is done by **ImageMagick**.

ShrinkShot is using **UPNG** library to load and parse PNG files. It's included in the repository as a GIT submodule.

## Copyright

Use it as you want. I'll be happy if you integrate it and mention it in the credits.
