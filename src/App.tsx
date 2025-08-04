import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

import "./App.css";
import { useEffect, useState } from "react";
import TitleBar from "./components/title-bar";

// interface InputEvent {
//   event_type: string;
//   x?: number;
//   y?: number;
//   key?: string;
//   button?: string;
//   modifiers: string[];
//   active_app?: string;
//   active_window_title?: string;
//   timestamp: number;
// }

// interface TaskData {
//   name: string;
//   data: InputEvent[];
// }

function App() {
  const [taskName, setTaskName] = useState("");
  const [isListening, setIsListening] = useState(false);
  const [isMonitoring, setIsMonitoring] = useState(false);
  const [unlisten, setUnlisten] = useState<(() => void) | null>(null);

  const startMonitoring = async () => {
    if (!taskName.trim()) {
      alert("Please enter a task name");
      return;
    }

    try {
      await invoke("start_monitoring", { taskName });
      setIsMonitoring(true);

      if (!isListening) {
        const unlistenFn = await listenForStopMonitoring();
        setUnlisten(() => unlistenFn);
        setIsListening(true);
      }
    } catch (error) {
      console.error("Error starting monitoring:", error);
    }
  }

  const stopMonitoring = async () => {
    try {
      const result = await invoke("stop_monitoring");
      console.log("Monitoring stopped successfully:", result);
      setIsMonitoring(false);
    } catch (error) {
      console.error("Error stopping monitoring:", error);
    }
  }

  const listenForStopMonitoring = async () => {
    const unlistenFn = await listen("stop_monitoring", async () => {
      console.log("Received stop monitoring event");
      await stopMonitoring();
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
    <main className="bg-white/40 backdrop-blur-md rounded-lg shadow-lg overflow-hidden w-full h-screen border-8 border-gray-400">
      <div>
        <TitleBar />

        <div className="pt-20 flex flex-col items-center justify-center space-y-6">
          <div className="flex flex-col items-center space-y-4">
            <input
              type="text"
              placeholder="task name"
              value={taskName}
              onChange={(e) => setTaskName(e.target.value)}
              disabled={isMonitoring}
              className="border-2 border-gray-400 rounded-md p-2 w-80 text-zinc-800 focus:outline-none focus:border-gray-600 transition-colors duration-200"
            />

            <button
              className={`w-40 h-12 rounded-md font-medium text-sm tracking-wide transition-all duration-300 ${isMonitoring
                ? 'bg-zinc-200 hover:bg-zinc-300 text-zinc-800 border border-zinc-300 shadow-sm hover:shadow-md'
                : 'bg-zinc-100 hover:bg-zinc-200 text-zinc-800 border border-zinc-200 shadow-sm hover:shadow-md'
                }`}
              onClick={isMonitoring ? stopMonitoring : startMonitoring}
            >
              {isMonitoring ? 'STOP' : 'START'}
            </button>
          </div>

          {isMonitoring && (
            <div className="text-sm text-zinc-600">
              Monitoring: {taskName}
            </div>
          )}
        </div>
      </div>
    </main>
  );
}

export default App;
