#!/bin/bash

while read line;
do
echo "Building $line docset"
rsdocs-dashing target/doc/$line $line
$GOPATH/bin/dashing build --config $line/dashing.json --source $line/build
done < crates.txt