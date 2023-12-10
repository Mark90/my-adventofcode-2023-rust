#!/bin/bash

YEAR=2023
if cargo aoc credentials | grep -q Error
then
  echo "Set your session token first"
  exit 1
fi

for num in {1..25}; do
  if [ ! -f input/${YEAR}/day${num}.txt ]; then
    cargo aoc input -d ${num} -y ${YEAR}
    sleep 1
  else
    echo "Already have day ${num}"
  fi
done
