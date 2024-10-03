// this is the con, gettroller for fetching live feed urls from our endpoint to see cameras in realtime
import { Text, Box } from "@gluestack-ui/themed"
import { Animated, TouchableOpacity, View, Dimensions } from "react-native";
import React, { useEffect, useState, useRef, useContext } from "react";
import { mediaController } from "./MediaViewer"
import { getCameras } from "~/utils/api"

const LiveFeedContainer = {
  flexDirection: "row",
  marginLeft: 3,
  shadowColor: '#171717',
  shadowOffset: { width: -3, height: 0 },
  shadowOpacity: 0.2,
  shadowRadius: 3,
  width: 300
}

// interface Props {
//   cameras: Camera[]
// }

interface Camera {
  name: String,
  rtsp_url: String
}

export default function LiveFeedSelection() {
  const [isCollapsed, setCollapse] = useState<boolean>()
  const [diffToggleCalc, setDiffToggleCalc] = useState((Dimensions.get("window").height / 2) * -1)
  const [cameras, setCameras] = useState<Camera[]>([])
  useEffect(() => {
    getCameras().then((cameras) => setCameras(cameras))
  }, [])


  const animations = useRef({
    width: new Animated.Value(0),
    negWidth: new Animated.Value(0)
  }).current;

  useEffect(() => {
    const subscription = Dimensions.addEventListener(
      'change',
      ({ window, screen }) => {
        const diff = (window.height / 2) * -1
        if (diff != diffToggleCalc) {
          setDiffToggleCalc(diff)
        }
      },
    );

    return () => subscription?.remove()
  }, [])

  const toggleView = (setCollapsed: boolean) => {
    const duration = 200
    if (setCollapsed) {
      Animated.timing(animations.width, {
        toValue: 280,
        duration,
        // easing: Easing.linear,
        useNativeDriver: true,
      }).start(() => setCollapse(setCollapsed));

      Animated.timing(animations.negWidth, {
        toValue: -280,
        duration,
        useNativeDriver: true,
      }).start();
    } else {
      Animated.timing(animations.width, {
        toValue: 0,
        duration,
        useNativeDriver: true,
      }).start(() => setCollapse(setCollapsed));

      Animated.timing(animations.negWidth, {
        toValue: 0,
        duration,
        useNativeDriver: true,
      }).start();
    }
  }

  return (
    <Animated.View style={{ ...LiveFeedContainer, marginLeft: animations.negWidth, left: animations.width }}>
      <View style={{ display: 'flex', flexDirection: 'row-reverse', width: 300 }}>
        <Animated.View style={{ width: '20px', marginLeft: animations.width, left: animations.negWidth }}>
          <TouchableOpacity style={{}} onPress={() => toggleView(!isCollapsed)}>
            <View style={{ transform: [{ rotate: '270deg' }, { translateX: diffToggleCalc }, { translateY: 0 }], height: 20, width: 150, alignSelf: 'center' }}>
              <Text style={{ textAlign: "center" }}>{isCollapsed ? "Expand Feeds" : "Collpase"}</Text>
            </View >
          </TouchableOpacity>
        </Animated.View>
        <Animated.View style={{ flex: 1, width: '20px', marginLeft: animations.negWidth }}>
          <Text>Live Feed Selection</Text>
          <div>
            {cameras.map(({name}, i) => {
              // debugger
              return (
                <div key={i}>
                  <span>Name:</span> {name}
                  <button onClick={() => {
                    debugger
                    mediaController.changeCamera(name)
                  }}>Show Camera</button>
                </div>
              )
            })}
          </div>
        </Animated.View>
      </View>
    </Animated.View>
  );
}
