const doLogin = (username, password) => {
  return fetch(`/api/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      username,
      password,
    }),
  }).then((res) => res.json());
};

class WebRTCConnection {
  constructor(videoElemId) {
    this.sName = "";
    this.stream;
    this.cameras = [];
    this.videoElem = document.getElementById(videoElemId);
    this.config = {
      iceServers: [
        {
          urls: ["stun:stun.l.google.com:19302"],
        },
      ],
    };
    this.pc = null;
  }

  addCameraName(name) {
    this.cameras.push(name);
  }

  getCameraNames() {
    return this.cameras;
  }

  getStream() {
    return this.stream;
  }

  initPC() {
    this.pc = new RTCPeerConnection(this.config);
    this.stream = new MediaStream();

    this.pc.onnegotiationneeded = async () => {
      let offer = await this.pc.createOffer();
      await this.pc.setLocalDescription(offer);
      this.getRemoteSdp();
    };

    this.pc.ontrack = (event) => {
      this.stream.addTrack(event.track);
      this.videoElem.srcObject = this.stream;
      this.log(`${event.streams.length} track is delivered`);
    };

    this.pc.oniceconnectionstatechange = () => this.log(this.pc.iceConnectionState);

    return this.stream;
  }

  log(msg) {
    document.getElementById("div").innerHTML += msg + "<br>";
  }

  changeCamera(selectedCamera) {
    this.sName = selectedCamera;
    this.initPC();
    this.getCodecInfo();
  }

  getCodecInfo() {
    fetch(`/api/user/camera_webrtc?suuid=${this.sName}`)
      .then((res) => res.json())
      .then((codecs) => {
        Object.entries(codecs).forEach(([_, { Type }]) => {
          this.pc.addTransceiver(Type, { direction: "sendrecv" });
        });
      });
  }

  getRemoteSdp() {
    const reqBody = {
      suuid: this.sName,
      data: this.pc.localDescription ? btoa(this.pc.localDescription.sdp) : null,
    };
    fetch("/api/user/camera_webrtc", {
      method: "POST",
      body: JSON.stringify(reqBody),
      headers: { "Content-Type": "application/json" },
    })
      .then((res) => res.text())
      .then((sdp) => {
        this.pc.setRemoteDescription(new RTCSessionDescription({ type: "answer", sdp: atob(sdp) }));
      });
  }
}

// Usage Example
const webrtc = new WebRTCConnection("videoElem");

document.addEventListener("DOMContentLoaded", () => {
  doLogin("joe", "password").then(() => {
    fetch(`/api/user/get_cameras`, { credentials: "include", mode: "cors" })
      .then((res) => res.json())
      .then((rtsp_res) => {
        const camContainer = document.getElementById("cameras-sel");
        rtsp_res.forEach(({ name }) => {
          const camOpt = document.createElement("option");
          camOpt.value = name;
          camOpt.innerText = name;
          camContainer.appendChild(camOpt);
          webrtc.addCameraName(name);
        });

        // Initialize with the first camera
        const firstCamera = rtsp_res[0].name;
        webrtc.changeCamera(firstCamera);
      });
  });
});

// Event listener for camera selection
document.getElementById("cameras-sel").addEventListener("change", (event) => {
  webrtc.changeCamera(event.target.value);
});
