#!/bin/bash

function get_godot_server() {
	local VERSION=$1
	GODOT_SERVER_URL=https://downloads.tuxfamily.org/godotengine/${VERSION}/Godot_v${VERSION}-stable_linux_server.64.zip
	if [ ! -f $CACHE_DIR/godot_server.64 ]; then
		echo "Downloading Godot Server v$VERSION"
		curl -s $GODOT_SERVER_URL -o godot_server.zip || exit 1
		unzip -o godot_server.zip
		cp Godot_v${VERSION}-stable_linux_server.64 $CACHE_DIR/godot_server.64
		touch "$CACHE_DIR/._sc_"
	else
		echo "Using cached Godot Server executable"
	fi
}

function start_server() {
	echo "Starting Godot server"
	$CACHE_DIR/godot_server.64 --main-pack $BUILD_DIR/main.pck

}