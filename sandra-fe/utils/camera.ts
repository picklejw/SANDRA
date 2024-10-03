let config = {
  iceServers: [
    {
      urls: ["stun:stun.l.google.com:19302"],
    },
  ],
};

function getRemoteSdp(name: String, pc: RTCPeerConnection) {
  const reqBody = {
    suuid: name,
    data: pc.localDescription ? btoa(pc.localDescription.sdp) : null,
  };
  fetch("/api/user/camera_webrtc", {
    method: "POST",
    body: JSON.stringify(reqBody),
    headers: {
      "Content-Type": "application/json",
    },
  })
    .then((res) => res.text())
    .then((t) => {
      pc.setRemoteDescription(
        new RTCSessionDescription({
          type: "answer",
          sdp: atob(t),
        })
      );
    });
}

type Codecs = Record<string, { Type: "video" | "audio" }>
const getCodecInfo = async (name: String, pc: RTCPeerConnection) => {
  const codecs: Codecs = await fetch("http://localhost:8080/api/user/camera_webrtc?suuid=" + name).then((res) => res.json());
  Object.entries(codecs).forEach(([_, { Type }]) => {
    pc.addTransceiver(Type, {
      direction: "sendrecv",
    });
  });
}

// export interface CameraType {
//   rtsp_url: String;
//   name: String;
// }

// export default class Camera {
//   data: CameraType;
//   stream?: MediaStream;
//   pc?: RTCPeerConnection;

//   constructor(newData: CameraType) {
//     if (!newData.name ||!newData.rtsp_url) {
//       throw new Error("Need name and rtsp_url to init Camera")
//     }
//     this.data = { ...newData };
//   }

//   getWebRTC() {
//     this.getWebRTC = this.getWebRTC.bind(this);
//     this.stream = new MediaStream()
//     if (!this.pc) {
//       this.pc = new RTCPeerConnection(config);
//       this.pc.onnegotiationneeded = async () => {
//         if (this.pc) {
//           let offer = await this.pc?.createOffer();
//           await this.pc?.setLocalDescription(offer);
//           getRemoteSdp(this.data.name, this.pc);
//         }
//       }
//       this.pc.ontrack = (event) => {
//         this.stream?.addTrack(event.track);
//         console.log("Ready to set stream... needs to be done before on comp")
//       };
//       this.pc.oniceconnectionstatechange = (e) => console.log(this.pc?.iceConnectionState);
//     }
//     getCodecInfo(this.data.name, this.pc)
//     return this.stream;
//   }

//   stopWebRTC() {
//     if (this.stream) {
//       this.stream.getTracks().forEach(track => track.stop());
//       this.stream = undefined;
//     }
//   }
// }