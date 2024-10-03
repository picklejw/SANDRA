export const doLogin = (username: String, password: String) => {
  return fetch(`/api/auth/login`,{
    method: "POST",
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify({
      username,
      password
    })
  }).then((res) => res.json())
}

export const doSignup = (username: String, password: String, gid: String) => {
  return fetch(`/api/auth/signup`,{
    method: "POST",
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify({
      username,
      password
    })
  }).then((res) => res.json())
}