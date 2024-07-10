import { Dimensions, View } from "react-native";
import LiveFeedSelection from "~/components/LiveFeedSelection"
import MediaViewer from "~/components/MediaViewer"
import Alerts from "~/components/Alerts"
import { Text, Box } from "@gluestack-ui/themed"

const navAsTabs = (width?: number) => (width || Dimensions.get('window').width) < 768;

const Container = {
  flex:1,
  flexDirection: 'row'
};

export default function Dashboard() {
  return (
    <Box style={Container}>
        <LiveFeedSelection />
        <Box style={{flex: 1}}>
          <MediaViewer />
        </Box>
        <Alerts />
    </Box>
  );
}