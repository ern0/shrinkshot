#!/bin/bash


convert test/a* `./shrinkshot test/a*` result.png
open result.png
