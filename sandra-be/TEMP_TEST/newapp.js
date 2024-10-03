let config = {
  iceServers: [
    {
      urls: ["stun:stun.l.google.com:19302"],
    },
  ],
};

class MediaController {
  constructor() {
    this.cameras = {};
    this.activeCamera = null;
  }

  addCamera(cameraData) {
    const camera = new Camera(cameraData);
    this.cameras[cameraData.name] = camera;
    return camera;
  }

  switchCamera(name) {
    if (this.activeCamera) {
      this.activeCamera.stopWebRTC();
    }
    this.activeCamera = this.cameras[name];
    return this.activeCamera.getWebRTC();
  }
}

class Camera {
  constructor(newData) {
    if (!newData.name || !newData.rtsp_url) {
      throw new Error("Need name and rtsp_url to init Camera");
    }
    this.data = { ...newData };
    this.stream = null;
    this.pc = null;
  }

  getWebRTC() {
    if (!this.pc) {
      this.pc = new RTCPeerConnection(config);
      this.pc.onnegotiationneeded = async () => {
        let offer = await this.pc.createOffer();
        await this.pc.setLocalDescription(offer);
        getRemoteSdp(this.data.name, this.pc);
      };
      this.pc.ontrack = (event) => {
        if (!this.stream) {
          this.stream = new MediaStream();
        }
        this.stream.addTrack(event.track);
        console.log("Track added to stream");
      };
      this.pc.oniceconnectionstatechange = () => {
        console.log(this.pc.iceConnectionState);
      };
    }
    getCodecInfo(this.data.name, this.pc);
    return this.stream;
  }

  stopWebRTC() {
    if (this.stream) {
      this.stream.getTracks().forEach((track) => track.stop());
      this.stream = null;
    }
  }
}

function getRemoteSdp(name, pc) {
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

const getCodecInfo = async (name, pc) => {
  const codecs = await fetch("/api/user/camera_webrtc?suuid=" + name).then((res) => res.json());
  Object.entries(codecs).forEach(([_, { Type }]) => {
    pc.addTransceiver(Type, {
      direction: "sendrecv",
    });
  });
};

// Initialization
const mediaController = new MediaController();

// Example of adding cameras

// Switching to a specific camera feed
document.getElementById("cameras-sel").addEventListener("change", (event) => {
  const selectedCamera = event.target.value; // Assume options have values set to camera names
  debugger;
  const stream = mediaController.switchCamera(selectedCamera);

  const videoElem = document.getElementById("videoElem");
  videoElem.srcObject = stream; // Set the video source to the selected camera's stream
});

doLogin = (username, password) => {
  return fetch(`/api/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      username,
      password,
    }),
  }).then((res) => res.json());
};
document.addEventListener("DOMContentLoaded", () => {
  doLogin("joe", "password").then(() => {
    fetch(`/api/user/get_cameras`, {
      credentials: "include",
      mode: "cors",
    })
      .then((res) => res.json())
      .then((rtsp_res) => {
        suuid_id = rtsp_res[0].name;
        camContainer = document.getElementById("cameras-sel");
        rtsp_res.forEach(({ name, rtsp_url }) => {
          mediaController.addCamera({ name, rtsp_url });
          const camOpt = document.createElement("option");
          camOpt.value = name;
          camOpt.innerText = name;
          camContainer.appendChild(camOpt);
        });
        // getCodecInfo();
      });
  });
});
