import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";
import "./App.css";

interface InputEvent {
  event_type: string;
  key?: string;
  modifiers: string[];
  timestamp: number;
}

function App() {
  const [keyEvents, setKeyEvents] = useState<InputEvent[]>([]);
  const [isListening, setIsListening] = useState(false);
  const [visible, setVisible] = useState(false);

  const isListeningRef = useRef(false);
  const unlistenRefs = useRef<{
    keyLogger: (() => void) | null;
    startEvent: (() => void) | null;
    stopEvent: (() => void) | null;
  }>({
    keyLogger: null,
    startEvent: null,
    stopEvent: null
  });
  const hideTimerRef = useRef<number | null>(null);

  useEffect(() => {
    isListeningRef.current = isListening;
  }, [isListening]);

  const startMonitoring = async () => {
    try {
      await invoke("start_monitoring");
      setIsListening(true);
      setVisible(false);
    } catch (error) {
      console.error("Error starting monitoring:", error);
    }
  };

  const stopMonitoring = async () => {
    try {
      await invoke("stop_monitoring");
      setKeyEvents([]);
      setIsListening(false);
      setVisible(false);
    } catch (error) {
      console.error("Error stopping monitoring:", error);
    }
  };

  useEffect(() => {
    let disposed = false;

    const setupEventListeners = async () => {
      try {
        const keyLoggerUnlisten = await listen("key-logger", (event) => {
          if (!isListeningRef.current) return;

          const newEvent = event.payload as InputEvent;
          if (newEvent.event_type === "key_press") {
            setKeyEvents((prev) => [...prev.slice(-9), newEvent]);
            setVisible(true);

            if (hideTimerRef.current) window.clearTimeout(hideTimerRef.current);
            hideTimerRef.current = window.setTimeout(() => setVisible(false), 5000);
          }
        });

        const startUnlisten = await listen("start_monitoring", startMonitoring);
        const stopUnlisten = await listen("stop_monitoring", stopMonitoring);

        if (!disposed) {
          unlistenRefs.current = {
            keyLogger: keyLoggerUnlisten,
            startEvent: startUnlisten,
            stopEvent: stopUnlisten
          };
        } else {
          keyLoggerUnlisten();
          startUnlisten();
          stopUnlisten();
        }
      } catch (error) {
        console.error("Error setting up event listeners:", error);
      }
    };

    setupEventListeners();

    return () => {
      disposed = true;
      Object.values(unlistenRefs.current).forEach(unlisten => unlisten?.());
      if (hideTimerRef.current) {
        window.clearTimeout(hideTimerRef.current);
        hideTimerRef.current = null;
      }
    };
  }, []);

  const renderKeyBox = (event: InputEvent, index: number) => {
    const isLatest = index === keyEvents.length - 1;
    const key = event.key || 'unknown';
    const hasModifiers = event.modifiers.length > 0;

    return (
      <div
        key={`${event.timestamp}-${index}`}
        className={`flex items-center gap-2 key-row ${isLatest ? 'latest' : 'opacity-80'}`}
      >
        {hasModifiers && (
          <>
            {event.modifiers.map((modifier, modIndex) => (
              <div key={modIndex} className="keycap keycap-mod">
                {modifier}
              </div>
            ))}
            <span className="text-white/60 text-md">+</span>
          </>
        )}
        <div className={`keycap keycap-main font-mono font-bold text-lg ${isLatest ? 'keycap-glow' : ''}`}>
          {key}
        </div>
      </div>
    );
  };

  return (
    <main className="w-full h-screen bg-transparent pointer-events-none">
      {keyEvents.length > 0 && (
        <div
          className={`fixed bottom-6 right-6 transition-all duration-300 ease-out
            ${visible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-2'}`}
        >
          <div className="flex flex-col justify-end items-end gap-2">
            {keyEvents.map(renderKeyBox)}
          </div>
        </div>
      )}
    </main>
  );
}

export default App;