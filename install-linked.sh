#!/bin/bash

set -e

echo "Installing Archipel IPBeacon from current directory"

rm -fv /usr/bin/archipel-ipbeacon
rm -fv /etc/systemd/system/archipel-ipbeacon.service

ln -fs $(pwd)/target/release/daemon /usr/bin/archipel-ipbeacon
ln -fs $(pwd)/archipel-ipbeacon.service /etc/systemd/system/archipel-ipbeacon.service

systemctl daemon-reload