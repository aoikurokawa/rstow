#!/bin/bash

yarn ts-mocha -p ./tsconfig.json -t 1000000 --paths tests/*.spec.ts
