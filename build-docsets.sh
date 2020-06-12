#!/bin/bash

while read i;
do
line=`echo $i | awk -F ',' '{print $1}'`
version=`echo $i | awk -F ',' '{print $2}'`
echo "Building $line docset"
rsdocs-dashing target/doc/$line $line
./dashing/bin/dashing build --config $line/dashing.json --source $line/build
echo " \
<entry> \
<version>$version</version> \
<url>https://github.com/dustinknopoff/crate-docs/releases/download/latest/$line.tgz</url> \
</entry> \
" > $line.xml

done < crates.txt