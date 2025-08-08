import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

import "./App.css";
import { useEffect, useRef, useState } from "react";

interface InputEvent {
  event_type: string;
  key?: string;
  modifiers: string[];
  timestamp: number;
}

function App() {
  const [keyEvents, setKeyEvents] = useState<InputEvent[]>([]);
  const [isListening, setIsListening] = useState(false);

  const isListeningRef = useRef(false);
  const unlistenKeyLoggerRef = useRef<(() => void) | null>(null);
  const unlistenStartEventRef = useRef<(() => void) | null>(null);
  const unlistenStopEventRef = useRef<(() => void) | null>(null);

  useEffect(() => {
    isListeningRef.current = isListening;
  }, [isListening]);

  const startMonitoring = async () => {
    try {
      await invoke("start_monitoring");
      setIsListening(true);
    } catch (error) {
      console.error("Error starting monitoring:", error);
    }
  };

  const stopMonitoring = async () => {
    try {
      await invoke("stop_monitoring");
      setKeyEvents([]);
      setIsListening(false);
    } catch (error) {
      console.error("Error stopping monitoring:", error);
    }
  };

  useEffect(() => {
    let disposed = false;

    const setup = async () => {
      try {
        // Single global key-logger listener (lifetime of component)
        if (!unlistenKeyLoggerRef.current) {
          const unlisten = await listen("key-logger", (event) => {
            if (!isListeningRef.current) return;

            const newEvent = event.payload as InputEvent;
            if (newEvent.event_type === "key_press") {
              setKeyEvents((prev) => [...prev.slice(-9), newEvent]);
            }
          });
          if (!disposed) {
            unlistenKeyLoggerRef.current = unlisten;
          } else {
            // component unmounted before listen resolved
            unlisten();
          }
        }

        // Tray start/stop events
        const startUnlisten = await listen("start_monitoring", async () => {
          await startMonitoring();
        });
        const stopUnlisten = await listen("stop_monitoring", async () => {
          await stopMonitoring();
        });

        if (!disposed) {
          unlistenStartEventRef.current = startUnlisten;
          unlistenStopEventRef.current = stopUnlisten;
        } else {
          startUnlisten();
          stopUnlisten();
        }
      } catch (error) {
        console.error("Error setting up event listeners:", error);
      }
    };

    setup();

    return () => {
      disposed = true;
      if (unlistenKeyLoggerRef.current) {
        unlistenKeyLoggerRef.current();
        unlistenKeyLoggerRef.current = null;
      }
      if (unlistenStartEventRef.current) {
        unlistenStartEventRef.current();
        unlistenStartEventRef.current = null;
      }
      if (unlistenStopEventRef.current) {
        unlistenStopEventRef.current();
        unlistenStopEventRef.current = null;
      }
    };
  }, []);

  const renderKeyBox = (event: InputEvent, index: number) => {
    const isLatest = index === keyEvents.length - 1;
    const key = event.key ? event.key : 'unknown';
    const hasModifiers = event.modifiers.length > 0;

    return (
      <div
        key={`${event.timestamp}-${index}`}
        className={`
          flex items-center gap-2 transition-all duration-300
          ${isLatest ? 'scale-110' : 'opacity-75'}
        `}
      >
        {hasModifiers && (
          <>
            {event.modifiers.map((modifier, modIndex) => (
              <div
                key={modIndex}
                className="px-2 py-1 bg-white/10 backdrop-blur-sm rounded text-md font-medium text-white/80"
              >
                {modifier}
              </div>
            ))}
            <span className="text-white/60 text-md">+</span>
          </>
        )}

        <div className={`
          px-3 py-2 bg-white/15 backdrop-blur-sm rounded-lg font-mono font-bold text-lg text-white
          min-w-[45px] text-center border border-white/20
          ${isLatest ? 'bg-white/25 border-white/40 shadow-lg' : ''}
        `}>
          {key}
        </div>
      </div>
    );
  };

  return (
    <main className="w-full h-screen bg-transparent pointer-events-none">
      {keyEvents.length > 0 && (
        <div className="fixed bottom-6 right-6">
          <div className="flex flex-col gap-3">
            {keyEvents.slice(-9).map((event, index) =>
              renderKeyBox(event, index)
            )}
          </div>
        </div>
      )}
    </main>
  );
}

export default App;