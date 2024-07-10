import { Tabs } from "expo-router"
import React, { useEffect, useState } from "react";
import { View, Text, Dimensions, Platform, Button } from "react-native";

const isWeb = Platform.OS === 'web'
const navAsTabs = (width?: number) => (width || Dimensions.get('window').width) < 768;

const NavStyleDesktop = {
  order: -1,
  marginLeft: 60,
  height: 60
}

const Logo = () => (<Text>LOGO</Text>)
const tabBarIconRender = (el: React.JSX | React.ReactElement) => !isWeb ? el : ""

export default () => {
  const [needsTabBar, setNeedsTabBar] = useState(navAsTabs())

  useEffect(() => {
    const subscription = Dimensions.addEventListener(
      'change',
      ({ window, screen }) => {
        const check = navAsTabs(window.width);
        if (check != needsTabBar) {
          setNeedsTabBar(check)
        }
      },
    );

    return () => {
      subscription?.remove()
    };
  });

  return (
    <>
      {!needsTabBar ? (<View style={{ position: 'absolute', top: 0, left: 0 }}><Logo /></View>) : ""}
      <Tabs
        screenOptions={{
          tabBarStyle: !needsTabBar && isWeb ? NavStyleDesktop : {},
          headerLeft: () => <Logo />,
          headerShown: needsTabBar

        }}
      >
        <Tabs.Screen
          name="index"
          options={{
            title: "Dashboard",
            tabBarIcon: ({ color, size }) => (
              tabBarIconRender(<Text>🖥️</Text>)
            ),
            href: "/",
          }}
        />
        <Tabs.Screen
          name="settings"
          options={{
            title: "Settings",
            tabBarIcon: ({ color, size }) => (
              tabBarIconRender(<Text>⚙️</Text>)
            ),
            href: "/settings",
          }}
        />
        <Tabs.Screen
          name="temp/me"
          options={{
            title: "Secret View",
            href: "/temp/me",
            tabBarItemStyle: { display: 'none', }
          }}
        />
      </Tabs>
    </>
  )
}