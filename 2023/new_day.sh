#!/usr/bin/bash
if [ -z "$1" ]; then
    echo "Usage: new_day.sh <day>"
    exit 1
fi

if [ -e advent$1.py ] || [ -e input/$1.txt ] || [ -e examples/${1}1.txt ]; then
    echo "Files already exist"
    exit 1
fi

echo "Creating advent$1.py"
sed 's/$d/'$1/g template.py > advent$1.py
chmod +x advent$1.py

echo "Creating input/$1.txt"
touch input/$1.txt
echo "Creating examples/${1}1.txt"
touch examples/${1}1.txt

code advent$1.py input/$1.txt examples/${1}1.txt
