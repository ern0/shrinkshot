#!/bin/bash

F=a*

convert test/${F} `./shrinkshot test/${F}` /tmp/result.png
open /tmp/result.png
