#!/usr/bin/env bash

git pull
killall -s KILL rocket-worker-t
tmux kill-session -t amyip_net
tmux new-session -d -s amyip_net "cargo run --release"