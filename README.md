# SANDRA
SURVEILLANCE APPLICATION NOTIFICATION DELIVERY REALTIME ANALYSIS (wip)


Built with Rust/Actix/MongoDB and React Native (with Web). To support all platforms with the same code base. 

Reads RTSP ONVIF events broadcast via service discovery on all avalible networks automatically. Cameras do not need to be manually added but will support so in later development. 
Utilizes WebRTC for low latnecy video/audio streaming. WIP for WebSocket connection to relay ONVIF events for ReoLink Cameras 510a which have build in object recognition for people, vehicles to FE consumption.

PR to support ONVIF to have work with Reolink 510a:
https://github.com/lumeohq/onvif-rs/pull/128


** It's a little difficult to spot with contrast see object identification at bottom of gif
![Kapture 2024-10-18 at 07 40 40](https://github.com/user-attachments/assets/1b9fe2b9-3ee5-4e6d-a7ef-987d6c80cfcb)
