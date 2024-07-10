import { Stack } from "expo-router";
import React from "react";
import { StyledProvider } from "@gluestack-style/react"
import { GluestackUIProvider, Text, Box } from "@gluestack-ui/themed"
import { config as uiConfig } from "@gluestack-ui/config"
import { config } from "../gluestack-style.config"


export default function RootLayout() {
  return (
    <StyledProvider config={config}>
      <GluestackUIProvider config={uiConfig}>
        <Stack
          screenOptions={{
            headerStyle: {
              backgroundColor: '#f4511e',
            },
            headerTintColor: '#fff',
            headerTitleStyle: {
              fontWeight: 'bold',
            },
          }}
        >
          <Stack.Screen name="(tabs)" options={{ headerShown: false }} />
        </Stack>
      </GluestackUIProvider>
    </StyledProvider>
  );
}