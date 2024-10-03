import { Dimensions, View, StyleSheet } from "react-native";
import { AppContext } from "~/context";
import LiveFeedSelection from "~/components/LiveFeedSelection"
import MediaViewer from "~/components/MediaViewer"
import Alerts from "~/components/Alerts"
import { Text, Box } from "@gluestack-ui/themed"
import { useContext, useEffect, useState } from "react";
import { get } from "@gluestack-style/react";
import { parseSync } from "@babel/core";

const navAsTabs = (width?: number) => (width || Dimensions.get('window').width) < 768;

const Container = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: 'row'
  }
});

export default function Dashboard() {
  const { user, setUser } = useContext(AppContext)

  return (
    <Box style={Container.container}>
      <LiveFeedSelection />
      <Box style={{ flex: 1 }}>
        <MediaViewer />
      </Box>
      <Alerts />
    </Box>
  );
}

    // addCamera({
    //   name: "hi",
    //   desc: "sc",
    //   src_ip: "1212",
    //   username: "dcdc",
    //   password: "ll",
    //   onvif_port: "2323",
    //   rtsp_url: "ionfowienf"
    // }).then(() => {
      // fetch(`http://localhost:8080/api/user/echo`, {
      //   // method: "POST",
      //   // headers: { 'Content-Type': 'application/json' },
      //   credentials: 'include',
      //   mode: 'cors',
      //   // body: ""
      // }).then((res) => console.log(res.status.toString()))
    // }).catch(()=> {
    //   fetch(`http://localhost:8080/api/user/echo`, {
    //     // method: "POST",
    //     // headers: { 'Content-Type': 'application/json' },
    //     credentials: 'include',
    //     mode: 'cors',
    //     // body: ""
    //   }).then((res) => console.log(res.status.toString()))