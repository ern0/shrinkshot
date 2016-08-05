# ShrinkShot

A simple CLI tool to detect and cut out empty areas of an image (screenshot) in order to make them smaller without resizing their content.

## Features

The program simply scans for whole pixel columns and lines which are identical to their neigbours. These columns and lines can be cut out from the image, no information will be loss, but the image will be smaller and more compact.

This is not an out-of-box application: the program itself only a CLI tool, which tries to detect empty areas in a specified PNG image, then prints out a bunch of CLI arguments to be passed to ImageMagick's `convert` utility, which is doing the actual job.

You may assign a hotkey to yout script, which makes a screenshot from a window, passing through it on shrinkshot+convert, and saves it. Currently, I don't provide this kinda script, as I'm a newbie Mac user and I don't know how to do it.

## Usage

By calling `shrinkshot` with a PNG filename specified, it dumps out a bunch of parameters for `convert` (ImageMagick's CLI app):
```
$ shrinkshot image.png
-chop 4x0+2+0 -chop 4x0+11+0 -chop 2x0+19+0 -chop 12x0+26+0 -chop 0x5+0+2 -chop 0x2+0+6 -chop 0x6+0+11
```

To make it work, first, you should install ImageMagick:

- MacOS: `brew install imagemagick`
- Debian/Ubuntu Linux: `sudo apt-get install imagemagick` 
- etc.

Then you may pass the parameters to `convert` utility>
```
convert image.png `shrinkshot image.png` result.png
```

If any problem occurs, `shrinkshot` prints error messages to `stderr`.

## TODO

### Needs more test

- Only tested on sample images you can found in the `test/` directory. 
- Not tested on Linux distros yet.

### Known issues to be fixed

Rightmost and bottom areas will be not removed due to a missing check at the end of the processing. This will be fixed soon.

If you make a screenshot by marking the area by hand (sometimes called snipping tool), empty area detection may fail on background image. A simple solution is by just simply ignore 3-4 pixels on the borders, this hack will be applied soon.

### Enhance distribution

ShrinkShot is now a CLI tool for doing a half job. It should be turned to an easy-to-install solution for making shrinked screenshots with a hotkey.

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

The actual conversation is done by **ImageMagick*.

ShrinkShot is using **UPNG** library to load and parse PNG files. It's included in the repository as a GIT submodule.

## Copyright

Use it as you want. I'll be happy if you mention it in the credits.