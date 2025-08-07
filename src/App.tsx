import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

import "./App.css";
import { useEffect, useState } from "react";

interface InputEvent {
  event_type: string;
  x?: number;
  y?: number;
  key?: string;
  button?: string;
  modifiers: string[];
  active_app?: string;
  active_window_title?: string;
  timestamp: number;
}

function App() {
  const [log, setLog] = useState<InputEvent>();
  const [isListening, setIsListening] = useState(false);
  const [unlistenKeyLogger, setUnlistenKeyLogger] = useState<(() => void) | null>(null);
  const [unlistenStartEvent, setUnlistenStartEvent] = useState<(() => void) | null>(null);
  const [unlistenStopEvent, setUnlistenStopEvent] = useState<(() => void) | null>(null);

  const startMonitoring = async () => {
    try {
      await invoke("start_monitoring");

      if (!isListening) {
        const keyLoggerUnlisten = await listen("key-logger", (event) => {
          const newEvent = event.payload as InputEvent;
          setLog(newEvent);
        });
        setUnlistenKeyLogger(() => keyLoggerUnlisten);

        setIsListening(true);
      }
    } catch (error) {
      console.error("Error starting monitoring:", error);
    }
  };

  const stopMonitoring = async () => {
    try {
      await invoke("stop_monitoring");

      if (unlistenKeyLogger) {
        unlistenKeyLogger();
        setUnlistenKeyLogger(null);
      }

      setIsListening(false);
      setLog(undefined);
    } catch (error) {
      console.error("Error stopping monitoring:", error);
    }
  };

  useEffect(() => {
    const setupEventListeners = async () => {
      try {
        const startUnlisten = await listen("start_monitoring", async () => {
          await startMonitoring();
        });
        setUnlistenStartEvent(() => startUnlisten);

        const stopUnlisten = await listen("stop_monitoring", async () => {
          await stopMonitoring();
        });
        setUnlistenStopEvent(() => stopUnlisten);

      } catch (error) {
        console.error("Error setting up event listeners:", error);
      }
    };

    setupEventListeners();

    return () => {
      if (unlistenKeyLogger) {
        unlistenKeyLogger();
      }
      if (unlistenStartEvent) {
        unlistenStartEvent();
      }
      if (unlistenStopEvent) {
        unlistenStopEvent();
      }
    };
  }, []);

  const formatEventDisplay = (event: InputEvent) => {
    switch (event.event_type) {
      case "key_press":
        return `Key: ${event.key}${event.modifiers.length > 0 ? ` (${event.modifiers.join('+')})` : ''}`;
      case "key_release":
        return `Released: ${event.key}`;
      case "mouse_press":
        return `Mouse: ${event.button} pressed`;
      case "mouse_release":
        return `Mouse: ${event.button} released`;
      case "mouse_move":
        return `Mouse moved to (${event.x}, ${event.y})`;
      case "wheel":
        return `Wheel: Δx=${event.x}, Δy=${event.y}`;
      default:
        return `Event: ${event.event_type}`;
    }
  };

  return (
    <main className="w-full h-screen text-white pointer-events-none">
      <div className="p-4">

        {log && (
          <div className="bg-black bg-opacity-50 p-4 rounded">
            <p className="text-lg font-mono">
              {formatEventDisplay(log)}
            </p>
            <p className="text-sm text-gray-300 mt-1">
              Time: {new Date(log.timestamp).toLocaleTimeString()}
            </p>
          </div>
        )}
      </div>
    </main>
  );
}

export default App;