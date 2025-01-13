import AIPage from "./AI";
import "./App.css";
import SettingsPage from "./Settings";
import {Box} from "@mui/joy";
import {useState} from "react";
import { Toaster } from 'react-hot-toast';

function App() {
  const [route, _setRoute] = useState<string>(window.location.pathname);

  return (
    <Box className="container">
      <Toaster/>

      {
        route === "/ai" ? <AIPage/> :
        route === "/settings" ? (
          <SettingsPage/>
        ) : <></>
      }
    </Box>
  );
}

export default App;
