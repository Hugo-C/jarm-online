# jarm_online

This repository provides a way to compute [JARM hash](https://github.com/salesforce/jarm) via a web server.  
The API is written in Rust and leverage underneath [a rust library](https://github.com/Hugo-C/rustJarm).  
The GUI is made with Vue.js with the future goal to store previously computed hash, their maliciousness (based on a list
of known hash) and their overlap with top visited websites.

<div align="center">

# Live version: [jarm.online](https://jarm.online/)

[![Website](https://img.shields.io/website?down_color=lightgrey&style=for-the-badge&up_color=brightgreen&up_message=online&url=https%3A%2F%2Fjarm.online%2F)](https://jarm.online/)  
[![Docker publish](https://github.com/Hugo-C/jarm-online/actions/workflows/docker-publish.yml/badge.svg)](https://github.com/Hugo-C/jarm-online/actions/workflows/docker-publish.yml)  
[![dependency status](https://deps.rs/repo/github/Hugo-C/jarm-online/status.svg)](https://deps.rs/repo/github/Hugo-C/jarm-online)

</div>

## Setup

API/GUI can be setup via [docker](docker-compose.yml).  
To run the service, simply use:

```shell
docker-compose up
```

A full set up from scratch can be used via ansible with [ansible_playbook.yml](ansible_playbook.yml) (tested on ubuntu
22.04.1 LTS minimal)  
For development see [GUI's README](jarm_online_gui/README.md) to spinup the GUI locally and leverage auto-reload.

## API Endpoints

The list of endpoints currently available (see also [examples](examples)).

### Scan a website to obtain it's jarm fingerprint

````http request
GET api/v1/jarm?host=<host>&port=<port>
````

It takes a required `host` parameter and optionally `port` (default to 443).

### Retrieve domains from alexa top 1 million that match a jarm hash

````http request
GET api/v1/alexa-overlap?jarm_hash=<jarm-hash>
````

The returned list is ordered by top alexa rank first

### Retrieve recently scanned hosts

````http request
GET api/v1/last-scans
````

The returned list is ordered by oldest scans first. No pagination is proposed.

## Images statistics

|                                image                                 |                                                                                                                                                        RAM usage                                                                                                                                                        |                                                                             image size (compressed)                                                                              |
|:--------------------------------------------------------------------:|:-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------:|:--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------:|
| [jarm_online_api](https://hub.docker.com/r/hugocker/jarm_online_api) | [![Netdata ram usage](https://netdata.jarm.online/api/v1/badge.svg?chart=cgroup_jarm_online_api_container.mem&after=-60&precision=1)](https://netdata.jarm.online/spaces/gcp-free/rooms/local/overview#selectedIntegrationCategory=deploy.operating-systems&chartName-val=menu_cgroup&local--chartName-val=menu_cgroup) | [![Docker Image Size (tag)](https://img.shields.io/docker/image-size/hugocker/jarm_online_api/latest?style=flat-square)](https://hub.docker.com/r/hugocker/jarm_online_api/tags) |
| [jarm_online_gui](https://hub.docker.com/r/hugocker/jarm_online_gui) | [![Netdata ram usage](https://netdata.jarm.online/api/v1/badge.svg?chart=cgroup_jarm_online_gui_container.mem&after=-60&precision=1)](https://netdata.jarm.online/spaces/gcp-free/rooms/local/overview#selectedIntegrationCategory=deploy.operating-systems&chartName-val=menu_cgroup&local--chartName-val=menu_cgroup) | [![Docker Image Size (tag)](https://img.shields.io/docker/image-size/hugocker/jarm_online_gui/latest?style=flat-square)](https://hub.docker.com/r/hugocker/jarm_online_gui/tags) |

## GUI looks

![](Doc/website_current_look.png)