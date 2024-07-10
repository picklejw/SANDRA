// media viewer can view images, records or live streams.
// PROPS: type=image|mp4|stream src=string
import { Text, Box } from "@gluestack-ui/themed"


export default function MediaViewer() {
  return (
        <Box alignItems="center" justifyContent="center" style={{backgroundColor: "black", height: '100%'}}>
          <Text style={{color: "white", lineHeight: "100%"}}>No Media</Text>
        </Box>
  );
}