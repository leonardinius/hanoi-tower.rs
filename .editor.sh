#!/bin/sh
pynvim "+colo flatui" "+AutoSaveToggle"  "+set mouse=a" src/main.rs 2>&1 1>/tmp/pynvim.$$.out &
