import { useLocalSearchParams, useNavigation } from 'expo-router';

import { Text } from 'react-native';

export default function Settings() {

  const { slug } = useLocalSearchParams();

  return <Text>Hello me! {slug}</Text>;
}

