# jarm_online
This repository provides a way to compute [JARM hash](https://github.com/salesforce/jarm) via a web server.  
The API is written in rust and leverage underneath [a rust library](https://github.com/Hugo-C/rustJarm).
The GUI is made with Vue.js with the future goal to store previously computed hash, their maliciousness (based on a list of known hash) and their overlap with top visited websites.

## Setup
*WIP*  
the API can be setup through [a docker](Dockerfile).  
See [GUI's README](jarm_online_gui/README.md) to spinup the GUI locally.

## Endpoints
A single endpoint is currently available:
````http request
GET /jarm?host=<host>&port=<port>
````
It takes a required `host` parameter and optionally `port` (default to 443).  
See [example.http](examples.http) for examples.