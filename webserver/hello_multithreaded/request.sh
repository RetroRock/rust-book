#!/bin/bash

index="http://127.0.0.1:7878"
sleepEndpoint="$index/sleep"

while true
do
	echo "Requesting: " $index
	curl -s $index > /dev/null &

	echo "Requesting: " $sleepEndpoint
	curl -s $sleepEndpoint > /dev/null &

	sleep 0.05
done
