#!/bin/bash

DRIVE_DIR="$HOME/.hana"

function install {
  echo "Installing Local Drive ..."

  URL="https://github.com/iankressin/hana-cli/releases/download/0.0.2/drive-cli"

  echo "--> Configuring directories ..."
  mkdir $DRIVE_DIR
  cd $DRIVE_DIR

  echo "--> Downloading ..."
  bash -c "ls"
  bash -c "curl --fail -# -L $URL > drive"
  BIN="drive"
	  chmod +x $BIN || fail "chmod +x failed"

  echo "*** DONE ***"
  echo "Now, please add $DRIVE_DIR to your PATH"
}

install
