import { Pressable, TextInput, View, StyleSheet } from "react-native";
import { Text, Box, get } from "@gluestack-ui/themed"
import React, { useState, type ReactNode } from "react";
import { User } from '~/models'
import { create } from "react-test-renderer";
import { doLogin, doSignup } from "~/utils/auth"
import { AppContext } from '~/context'

type ViewOptions = 'signup' | 'login' | 'isAuth';

interface AuthViewProps {
  children: (user: User) => React.ReactNode
}

interface AuthHandler {
  (username: String, password: String, setShowView: Function, setUser: Function, gid?: String): void;
}
const handleLogin: AuthHandler = (username, password, setShowView, setUser) => {
  console.log(username)
  console.log(password)
  doLogin(username, password).then(async (res) => {
    if (res.success) {
      setUser(res.user)
      setShowView("isAuth")
    }
  })
}

const renderLogin = (setShowView: Function, setMessage: Function, setUser: Function) => {
  let username = ""
  let password = ""
  return (
    <View style={styles.container}>
      <Text style={styles.title}>Login</Text>
      <TextInput
        style={styles.input}
        placeholder="Username"
        onChangeText={text => username = text}
        defaultValue=""
      />
      <TextInput
        style={styles.input}
        placeholder="Password"
        onChangeText={text => password = text}
        defaultValue=""
        secureTextEntry
      />
      <Pressable onPress={() => handleLogin(username, password, setShowView, setUser)}><Text>Login</Text></Pressable>
      <Pressable onPress={() => setShowView("signup")}><Text>Show Create Account</Text></Pressable>
    </View>
  )
}

const handleSignup: AuthHandler = (username, password, setShowView, gid) => {
  console.log(username)
  console.log(password)
  doSignup(username, password, gid).then(async (res) => {
    if (res.success) {
      setShowView("isAuth")
    }
  })
}

const renderSignup = (setShowView: Function, setMessage: Function, setUser: Function) => {
  let username = ""
  let password = ""
  let confimPassword = ""
  let gid = ""
  return (
    <View style={styles.container}>
      <Text style={styles.title}>Login</Text>
      <TextInput
        style={styles.input}
        placeholder="Username"
        onChangeText={text => username = text}
        defaultValue=""
      />
      <TextInput
        style={styles.input}
        placeholder="Password"
        onChangeText={text => password = text}
        defaultValue=""
        secureTextEntry
      />
      <TextInput
        style={styles.input}
        placeholder="Confirm Password"
        onChangeText={text => confimPassword = text}
        defaultValue=""
        secureTextEntry
      />
      <TextInput
        style={styles.input}
        placeholder="Group ID"
        onChangeText={text => gid = text}
        defaultValue=""
        secureTextEntry
      />
      <Pressable
        onPress={() => {
          debugger
          if (password === confimPassword) {
            handleSignup(username, password, setShowView, setUser, gid)
          } else {
            setMessage("Passwords do not match, please try again.")
          }
        }}
      >
        <Text>Create Account</Text>
      </Pressable>
    </View>
  )
}

export default function AuthView({ children }: AuthViewProps) {
  const [showView, setShowView] = useState<ViewOptions>("login");
  const [showMessage, setMessage] = useState<String>("");
  const [user, setUser] = useState<User>(null);

  return (
    <AppContext.Provider value={{user, setUser}}>
      {
        (() => {
          if (showView === "login" || showView === "signup") {
            return (
              <>
                {(() => {
                  if (showView === "signup") {
                    return renderSignup(setShowView, setMessage, setUser)
                  }
                  return renderLogin(setShowView, setMessage, setUser)
                })()
                }
                <Text>{showMessage}</Text>
              </>
            )
          }

          return children(user);
        })()
      }
    </AppContext.Provider>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    paddingHorizontal: 20,
  },
  title: {
    fontSize: 24,
    marginBottom: 20,
  },
  input: {
    width: '100%',
    height: 40,
    borderColor: 'gray',
    borderWidth: 1,
    marginBottom: 10,
    paddingHorizontal: 10,
  },
});
