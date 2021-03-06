#!/bin/bash
HANA_DIR="$HOME/.hana"

function install {
  echo "Installing Hana CLI ..."

  LATEST=$(curl -qs https://api.github.com/repos/iankressin/hana-cli/releases/latest | grep tag_name | head -n 1 | cut -d '"' -f 4);
  URL="https://github.com/iankressin/hana-cli/releases/download/${LATEST}/hana-cli"

  echo $URL

  echo "--> Configuring directories ..."
  mkdir $HANA_DIR
  cd $HANA_DIR

  echo "--> Downloading ..."
  bash -c "ls"
  bash -c "curl --fail -# -L $URL > hana"
  BIN="hana"
	  chmod +x $BIN || fail "chmod +x failed"

  echo "*** DONE ***"
  echo "Now, please add $HANA_DIR to your PATH"
}

install
