// this is a flexi, get, getble view for alerts... 
// PROPS isCollabsable=bool
import { Text, Box } from "@gluestack-ui/themed"
import type { transform } from "@babel/core";
import { Animated, TouchableOpacity, View, Dimensions } from "react-native";
import React, { useEffect, useState, useRef } from "react";
import WebSocketClient from "~/utils/ws"

const AlertsContainer = {
  flexDirection: "row",
  marginLeft: 3,
  shadowColor: '#171717',
  shadowOffset: { width: -3, height: 0 },
  shadowOpacity: 0.2,
  shadowRadius: 3,
  width: 300
}

export default function AlertsViewer() {
  const [isCollapsed, setCollapse] = useState<boolean>()
  const [alerts, setAlerts] = useState<string[]>([])
  const [diffToggleCalc, setDiffToggleCalc] = useState(Dimensions.get("window").height / 2)

  const animations = useRef({
    left: new Animated.Value(0),
    marginLeft: new Animated.Value(0)
  }).current;

  useEffect(() => {
    const subscription = Dimensions.addEventListener(
      'change',
      ({ window, screen }) => {
        const diff = window.height / 2
        if (diff != diffToggleCalc) {
          setDiffToggleCalc(diff)
        }
      },
    );
    const wsClient = new WebSocketClient('ws://127.0.0.1:8080/ws')
    wsClient.on('Cam_Motion', ({topic}) => {
      // console.log('Received someEvent with body:', body);
      setAlerts([...alerts, topic])
    });
    return () => subscription?.remove()
  })

  const toggleView = (setCollapsed: boolean) => {
    if (setCollapsed) {
      Animated.timing(animations.left, {
        toValue: 280,
        duration: 200,
        useNativeDriver: true,
      }).start(() => setCollapse(setCollapsed));

      Animated.timing(animations.marginLeft, {
        toValue: -280,
        duration: 200,
        useNativeDriver: true,
      }).start();
    } else {
      Animated.timing(animations.left, {
        toValue: 0,
        duration: 200,
        useNativeDriver: true,
      }).start(() => setCollapse(setCollapsed));

      Animated.timing(animations.marginLeft, {
        toValue: 0,
        duration: 200,
        useNativeDriver: true,
      }).start();
    }
  }

  return (
    <Animated.View style={{ ...AlertsContainer, marginLeft: animations.marginLeft, left: animations.left }}>
      <View>
        <TouchableOpacity style={{ width: 20 }} onPress={() => toggleView(!isCollapsed)}>
          <View style={{ transform: [{ rotate: '90deg' }, { translateX: diffToggleCalc }, { translateY: 0 }], height: 20, width: 150, alignSelf: 'center' }}>
            <Text style={{ textAlign: "center" }}>{isCollapsed ? "Expand Alerts" : "Collpase"}</Text>
          </View >
        </TouchableOpacity>
        <Box style={{ marginLeft: 20 }}>
          <Text>Alerts</Text>
          <Box>
            {alerts?.map((alert) => (
              <div>
                <p>{alert}</p>
              </div>
            ))}
          </Box>
        </Box>
      </View>
    </Animated.View>
  );
}
