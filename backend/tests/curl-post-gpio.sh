#!/bin/bash

curl -i \
	-H "Content-Type: application/json" \
	-X POST \
	-d @gpio.json \
	http://localhost:3030/gpio
