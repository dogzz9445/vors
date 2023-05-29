# VORS

[![License](https://img.shields.io/github/license/miroiu/nodify?style=for-the-badge)](https://github.com/dogzz9445/vors/LICENSE)
[![C#](https://img.shields.io/static/v1?label=docs&message=WIP&color=orange&style=for-the-badge)](https://github.com/dogzz9445/vors/wiki)

Voice chat over rust for cross platform

# Project

Name / Tech / Project / License / Comment

Integration

Server

Client-Native

Client-Mobile
Client-Web

Unity-SDK
Unity-example-2D
Unity-example-3D


# 해야할일
- xtask 


# Notice
This repository could be integrated to other repository (ex: vors-client, vors-server to vors[merged] )

# TODO:
- Make client web server gui
- client web firewall 
- listup client api
- audio changer
- Make all of server


# How to build

Linux
```
libasound2-dev libjack-dev
```

# Structures
Now (Room > Channel) Future (Server > Group > Channel)

1. 채널 내에 사용자들에 대해서 개별적으로 음량 조절을 API로
2. 모든 채널에 브로드캐스트 및 리스닝 가능한 기능
3. 가변적 채널 생성 기능 및 이동, API 제공
4. 클라이언트의 음성을 스트림으로 뺄 수 있게

# Development
```
cargo xtask prepare-deps
cargo xtask run-both
```

# References
- silent_rs
https://github.com/Flone-dnb/silent-server-rs

- ALVR
https://github.com/alvr-org/ALVR

# Commits

- 2023-03-23: xtask Copy from ALVR