#!/bin/bash

convert test/d* `./shrinkshot test/d*` /tmp/result.png
open /tmp/result.png
