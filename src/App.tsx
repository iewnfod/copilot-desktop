import AIPage from "./AI";
import "./App.css";
import SettingsPage from "./Settings";
import {Box} from "@mui/joy";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {DEFAULT_PREFERENCE, Preference} from "./preference.ts";

function App() {
  const [route, _setRoute] = useState<string>(window.location.pathname);
  const [pre, setPre] = useState<Preference>(DEFAULT_PREFERENCE);

  useEffect(() => {
    invoke("get_preference").then((pre) => {
      // @ts-ignore
      setPre(pre ?? DEFAULT_PREFERENCE);
    });
  }, []);

  return (
    <Box className="container">
      {
        route === "/ai" ? <AIPage/> :
        route === "/settings" ? (
          <SettingsPage
            pre={pre}
            setPre={setPre}
          />
        ) : <></>
      }
    </Box>
  );
}

export default App;
