#!/bin/bash

cd backend

gnome-terminal --tab -- bash -c 'cargo watch -x run'


cd ..

cd frontend

gnome-terminal --tab -- bash -c 'npm run watch'