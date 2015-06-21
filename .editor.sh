#!/bin/sh
pynvim "+colo cake16" "+AutoSaveToggle"  "+set mouse=a" src/main.rs 2>&1 1>/tmp/pynvim.$$.out &
