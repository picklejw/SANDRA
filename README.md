# SANDRA
SURVEILLANCE APPLICATION NOTIFICATION DELIVERY REALTIME ANALYSIS (wip)


Built with Rust/Actix/MongoDB and React Native (with Web). To support all platforms with the same code base. 

Reads RTSP ONVIF events broadcast via service discovery on all avalible networks automatically. Cameras are automatically discovered on the network using mDNS and added to the portal. No static IP address configuration needed just pop it on the network and run the software to view and receive events. 

Utilizes WebRTC for low latnecy video/audio streaming. WIP for WebSocket connection to relay ONVIF events for ReoLink Cameras 510a usually have build in object recognition for people, vehicles to front end for viewing.

PR to support ONVIF to have work with Reolink 510a:
https://github.com/lumeohq/onvif-rs/pull/128


** It's a little difficult to spot with contrast see object identification at bottom of gif. These events come from the cameras themselves I have no control over errors in identification but to be able to get notifications and process them on larger hardware like a GPU. Saves money on the home electric bill and with AI models being run on home servers it also saves some processing room for those actions.
![Kapture 2024-10-18 at 07 40 40](https://github.com/user-attachments/assets/1b9fe2b9-3ee5-4e6d-a7ef-987d6c80cfcb)
