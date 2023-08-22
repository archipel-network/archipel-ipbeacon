#!/bin/bash

set -e

echo "Installing Archipel IPBeacon"

rm -fv /usr/bin/archipel-ipbeacon
rm -fv /etc/systemd/system/archipel-ipbeacon.service

cp -fv target/release/daemon /usr/bin/archipel-ipbeacon
cp -fv archipel-ipbeacon.service /etc/systemd/system/archipel-ipbeacon.service

systemctl daemon-reload