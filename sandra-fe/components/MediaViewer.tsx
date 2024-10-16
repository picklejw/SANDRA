// media viewer can view images, records or live streams.
// PROPS: type=image|mp4|stream src=string
import { Text, Box } from "@gluestack-ui/themed"
import { useEffect, useRef, useState } from "react";
import type { config } from "~/gluestack-style.config";
import type { CameraType } from "~/utils/camera";

export default function MediaViewer() {
  const [stream, setStream] = useState<MediaProvider | null>(null);
  const [title, setTitle] = useState<String>("");
  const videoRef = useRef<HTMLVideoElement | null>(null);
  useEffect(() => {
    if (videoRef.current) {
      mediaController.registerGetStreamData((name: String, stm: MediaProvider) => {
        console.log(`cb for player: ${name}`)
        console.log(stm)
        setTitle(title)
        setStream(stm)
        if (videoRef.current) {
          videoRef.current.srcObject = stm
        }
      })
    }
  }, [videoRef.current])

  const renderContent = () => {
    return (<>
      <Text style={{ color: "white", lineHeight: "100%" }}>{title !== "" ? title : "No Media"}</Text>
      <video ref={videoRef} style={{ width: "100%", height: "100%" }} autoPlay muted controls></video>
    </>)
  };

  return (
    <Box alignItems="center" justifyContent="center" style={{ backgroundColor: "black", height: '100%' }}>
      {renderContent()}
    </Box>
  );
}

interface Codec {
  Type: 'audio' | 'video'; // Define the expected values for Type
}


class WebRTCConnection {
  private sName: String;
  private stream: MediaStream | undefined;
  private cameras: String[];
  private config: RTCConfiguration;
  private pc: RTCPeerConnection | null;
  private registeredCallback?: (name: String, stream: MediaStream) => void;


  constructor() {
    this.sName = "";
    this.stream = undefined;
    this.cameras = [];
    this.config = {
      iceServers: [
        {
          urls: ["stun:stun.l.google.com:19302"],
        },
      ],
    };
    this.pc = null;
  }


  // constructor() {}

  // playStream(name: String, setStream: MediaStream) {
  //   this.stream = setStream;
  //   this.sName = name;
  //   if (this.registeredCallback) {
  //     console.log("has reg cb")
  //     this.registeredCallback(name, setStream)
  //   }
  // }

  registerGetStreamData(cb: (name: String, stream: MediaStream) => void) {
    console.log("registering in comp")
    if (this.stream) {
      cb(this.sName, this.stream)
    }
    this.registeredCallback = cb;
  }

  addCameraName(name: String): void {
    this.cameras.push(name);
  }

  getCameraNames(): String[] {
    return this.cameras;
  }

  getStream(): MediaStream | undefined {
    return this.stream;
  }

  initPC(): MediaStream | undefined {
    this.pc = new RTCPeerConnection(this.config);
    this.stream = new MediaStream();

    this.pc.onnegotiationneeded = async () => {
      const offer = await this.pc!.createOffer();
      await this.pc!.setLocalDescription(offer);
      this.getRemoteSdp();
    };

    this.pc.ontrack = (event: RTCTrackEvent) => {
      if (this.stream) {
        this.stream.addTrack(event.track);
        this.log(`${event.streams.length} track is delivered`);
        this.registeredCallback?.("HELLO!", this.stream)
      }
    };

    this.pc.oniceconnectionstatechange = () => this.log(this.pc!.iceConnectionState);

    return this.stream;
  }

  log(msg: string): void {
    console.log(msg)
  }

  changeCamera(selectedCamera: String): void {
    this.sName = selectedCamera;
    this.initPC();
    this.getCodecInfo();
  }

  getCodecInfo(): void {
    fetch(`http://localhost:8080/api/user/camera_webrtc?suuid=${this.sName}`)
      .then((res) => res.json())
      .then((codecs) => {
        Object.entries(codecs as Record<string, Codec>).forEach(([_, { Type }]: [string, Codec]) => {
          this.pc!.addTransceiver(Type, { direction: "sendrecv" });
        });
      });
  }

  getRemoteSdp(): void {
    const reqBody = {
      suuid: this.sName,
      data: this.pc?.localDescription ? btoa(this.pc.localDescription.sdp) : null,
    };
    fetch("http://localhost:8080/api/user/camera_webrtc", {
      method: "POST",
      body: JSON.stringify(reqBody),
      headers: { "Content-Type": "application/json" },
    })
      .then((res) => res.text())
      .then((sdp) => {
        if (this.pc) {
          this.pc.setRemoteDescription(new RTCSessionDescription({ type: "answer", sdp: atob(sdp) }));
        }
      });
  }
}


export const mediaController = new WebRTCConnection()