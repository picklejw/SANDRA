const BASE_DOMAIN = "http://localhost:8080";

interface AddCamera {
  name: String,
  desc: String,
  src_ip: String,
  username: String,
  password: String,
  onvif_port: String,
  rtsp_url: String,
}

export const addCamera = ({ name, desc, src_ip, username, password, onvif_port, rtsp_url }: AddCamera) => {
  return fetch(`${BASE_DOMAIN}/api/user/add_camera`, {
    method: "POST",
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    mode: 'cors',
    body: JSON.stringify({
      // id: "1",
      name,
      desc,
      src_ip,
      username,
      password,
      onvif_port,
      rtsp_url
    })
  }).then((res) => res.json())
}