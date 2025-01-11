import AIPage from "./AI";
import "./App.css";
import SettingsPage from "./Settings";

function App() {
  const route = window.location.pathname;

  return (
    <div>
      {
        route === "/ai" ? <AIPage/> :
        route === "/settings" ? <SettingsPage/> :
        <></>
      }
    </div>
  );
}

export default App;
