# ShrinkShot

A simple CLI tool to detect and cut out empty areas of an image (screenshot) in order to make it smaller without resizing its content.

## Features

The program simply scans for whole pixel columns and lines which are identical to their neigbours. These columns and lines can be cut out from the image, no information will be loss, but the image will be smaller and more compact.

The program itself only a CLI tool, which tries to detect empty areas in a specified PNG image, then removes them and save the shrinked image.

> The V1 program did not perform the conversion itself, but rather created the parameters for ImageMagick's `convert` utility, which was doing the actual job.

You may assign a hotkey to yout script, which makes a screenshot of a window, passing through it on shrinkshot, and saves it.

## Installation

> The V1 program required to install ImageMagick and pull `upng` submodule.

Just build the app with `cargo build`, or let `build.sh` to do it.

## Usage

```
$ shrinkshot screenshot.png result.png
```

If any problem occurs, `shrinkshot` prints error messages to `stderr`.

### Known issues to be fixed

- If you make a screenshot by marking the cut region by hand (sometimes called snipping tool), empty area detection may fail on the background image. A simple solution is just simply ignore 3-4 pixels on the borders, this hack will be applied soon.
- Noisy regions are not detected.

### Algorithm enhancements

It would be great to split the image to more regions
(e.g. horizontal stripes), and cut out (same width) areas from it
at different (horizontal) positions.

The most known use case of it is the status bar, which should be
cut separately:

This image has no columns to cut (only lines):
````
  ---------------------------
 | content           content |
 | on left           on the  |
 |                right side |
 |                           |
 |                           |
 |          status           |
  ---------------------------
````

But it should be:
````
  ----------------------
 | content     content |
 | on left     on the  |
 |          right side |
 |                     |
 |                     |
 |       status        |
  ---------------------
````

The "status" line occupies small vertical size,
so it's a good candidate to handle differently from the main area.

Empty areas marked with numbers (vertical only):
````
  ---------------------------   --
 | content 111111    content |  |
 | on left 111111    on the  |  | large area
 |         111111 right side |  |
 |         111111            |  |
 |         111111            |  --
 | 22222222 status 222222222 |  | small area
  ---------------------------   --
````

The program should detect main area (primary target for shrinking),
and try to shrink same amout from smaller areas.

### Enhance distribution

ShrinkShot is now a CLI tool for. It should be turned to an easy-to-install solution for making shrinked screenshots with a hotkey.

I don't want to turn it to a boxed software with printed manual, but the distribution should be more user-friendly:
- Linux: provide a shell script, which can be assigned to a hotkey. Probably release it as a `.deb` package.
- MacOS: have no idea.
- Windows: have no idea.

## Credits

The idea and some sample images come from a question issued by *@Thomas* on the Software Recommendation (StackExchange) site.

## Copyright

Use it as you want. I'll be happy if you integrate it and mention it in the credits.
