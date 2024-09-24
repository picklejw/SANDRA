import { ModalBackdrop, ModalContent, ModalHeader, Heading, ModalCloseButton, Icon, CloseIcon, ModalBody, ModalFooter, ButtonText, Button } from '@gluestack-ui/themed';
import { useContext, useState } from 'react';

import { Text, View, Pressable, Modal, StyleSheet, TextInput } from 'react-native';
import { create } from 'react-test-renderer';
import { AppContext } from '~/context';
import { addCamera } from '~/utils/api';


const showNewCameraView = (setShowModal: Function) => {
  let name = ""
  let desc = ""
  let src_ip = ""
  let username = "admin"
  let password = ""
  let onvif_port = ""
  let rtsp_url = ""

  return (
    <View style={styles.newContainer}>
      <View>
        <Text>Name:</Text>
        <TextInput
          style={styles.input}
          placeholder="You name it!"
          onChangeText={text => name = text}
          defaultValue={name}
        />
      </View>
      <View>
        <Text>Description:</Text>
        <TextInput
          style={styles.input}
          placeholder="Just for you"
          onChangeText={text => desc = text}
          defaultValue={desc}
        />
      </View>
      <View>
        <Text>IP Address:</Text>
        <TextInput
          style={styles.input}
          placeholder="IP Address"
          onChangeText={text => src_ip = text}
          defaultValue={src_ip}
        />
      </View>
      <View>
        <Text>Username:</Text>
        <TextInput
          style={styles.input}
          placeholder="Username"
          onChangeText={text => username = text}
          defaultValue={username}
        />
      </View>
      <View>
        <Text>Password:</Text>
        <TextInput
          style={styles.input}
          placeholder="Password"
          onChangeText={text => password = text}
          defaultValue={password}
        />
      </View>
      <View>
        <Text>ONVIF Port:</Text>
        <TextInput
          style={styles.input}
          placeholder="ONVIF Port"
          onChangeText={text => onvif_port = text}
          defaultValue={onvif_port}
        />
      </View>
      <View>
        <Text>RTSP URL:</Text>
        <TextInput
          style={styles.input}
          placeholder="RTSP URL"
          onChangeText={text => rtsp_url = text}
          defaultValue={rtsp_url}
        />
      </View>
      <Pressable onPress={() => {
        addCamera({
          name,
          desc,
          src_ip,
          username,
          password,
          onvif_port,
          rtsp_url
        })
      }}>
        <Text>Add Camera</Text>
      </Pressable>
    </View>
  )
}

const showCameras = (cameras) => {
  return cameras.map((cam) => (
    <View style={styles.cameraRow}>
      <View></View>
      <View></View>
    </View>
  ))
}

export default function Settings() {
  const { user, setUser } = useContext(AppContext)
  console.log(user)

  return (
    <View>
      {showCameras([])}
      {showNewCameraView()}


    </View>
  );
}

const styles = StyleSheet.create({
  newContainer: {
    width: '100%',
    borderColor: 'grey',
    borderWidth: 1,
    marginBottom: 10,
    padding: 10,
    paddingHorizontal: 10,
    borderBottomRightRadius: 15,
    borderBottomLeftRadius: 15,
  },
  input: {
    width: '100%',
    height: 40,
    borderColor: 'grey',
    borderWidth: 1,
    marginBottom: 10,
    paddingHorizontal: 10
  },
  camerasContainer: {

  },
  cameraRow: {

  }
})

