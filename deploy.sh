#!/bin/bash
HASH=$(git rev-parse HEAD;)
trunk build && netlify deploy --prod --dir build --message $HASH;
