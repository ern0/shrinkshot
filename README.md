# ShrinkShot

A simple CLI tool to detect and cut out
empty areas of a PNG image
(typically: a screenshot)
in order to make it smaller
without resizing its content.

## Features

The program simply scans for whole pixel columns and lines
which are identical to their neigbours.
These columns and lines can be cut out from the image,
no information will be lost,
but the image will be smaller and more compact.

If you make a screenshot by marking the cut region by hand
(sometimes called snipping tool),
empty area detection may fail on the surrounding background.
Shrinkshot uses a simple solution:
just simply ignores 2-2 pixels at the borders
(see `IGNORED_MARGIN_SIZE`).

> The V1 program did not perform the conversion itself,
but rather created the argument list for
ImageMagick's `convert` utility,
which was doing the actual job.

You may assign a hotkey to yout script,
which makes a screenshot of a window,
passing through it on shrinkshot,
and saves it.

## Installation

> The V1 program required to install ImageMagick and pull `upng` submodule.

Just build the app with `cargo build`,
or let `build.sh` to do it.

## Usage

```
$ shrinkshot screenshot.png result.png
```

If any problem occurs, `shrinkshot` prints error messages to `stderr`.

### Known issues

#### Detection failure

Checking for identical neighbour lines and columns fails on
- noisy areas
- gradients.

It requires more sophisticated method
than simply comparing neighbour pixels.

#### Elimination of padding

Text are padded with empty pixel lines.
The progra, may reduces these padding lines to 1 pixel height,
which makes the text ugly, too dense.

Maybe smaller areas should not be shrinked.

### Algorithm enhancements

It would be great to split the image to more regions
(e.g. horizontal stripes),
and cut out (same width) areas from it
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

The program should detect main area
(primary target for shrinking),
and try to shrink same amount at smaller areas.

## Credits

The idea and some sample images come from a question
issued by *@Thomas* on the
Software Recommendation (StackExchange) site.

## Copyright

Use it as you want.
I'll be happy if you integrate it and mention it in the credits.
