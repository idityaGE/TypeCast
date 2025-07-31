import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

import "./App.css";
import { useEffect, useState } from "react";

function App() {
  const [isListening, setIsListening] = useState(false);
  const [unlisten, setUnlisten] = useState<(() => void) | null>(null);

  const startMonitoring = async () => {
    try {
      const response = await invoke("start_task_monitoring");
      console.log("Monitoring started successfully:", response);

      if (!isListening) {
        const unlistenFn = await listenForStopMonitoring();
        setUnlisten(() => unlistenFn);
        setIsListening(true);
      }
    } catch (error) {
      console.error("Error starting monitoring:", error);
    }
  }

  const listenForStopMonitoring = async () => {
    const unlistenFn = await listen("stop_task_monitoring", () => {
      console.log("Received stop monitoring event:");
      const response = invoke("stop_task_monitoring");
      console.log("Monitoring stopped successfully:", response);
    });
    return unlistenFn;
  }

  useEffect(() => {
    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, [unlisten]);

  return (
    <main className="container">
      <button onClick={startMonitoring}>Start Monitoring</button>
    </main>
  );
}

export default App;
