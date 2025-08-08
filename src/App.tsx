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
  const [visible, setVisible] = useState(false);

  const isListeningRef = useRef(false);
  const unlistenKeyLoggerRef = useRef<(() => void) | null>(null);
  const unlistenStartEventRef = useRef<(() => void) | null>(null);
  const unlistenStopEventRef = useRef<(() => void) | null>(null);
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

    const setup = async () => {
      try {
        if (!unlistenKeyLoggerRef.current) {
          const unlisten = await listen("key-logger", (event) => {
            if (!isListeningRef.current) return;

            const newEvent = event.payload as InputEvent;
            if (newEvent.event_type === "key_press") {
              setKeyEvents((prev) => {
                const next = [...prev.slice(-9), newEvent];
                return next;
              });
              setVisible(true);
              if (hideTimerRef.current) {
                window.clearTimeout(hideTimerRef.current);
              }
              hideTimerRef.current = window.setTimeout(() => {
                setVisible(false);
              }, 5000);
            }
          });
          if (!disposed) {
            unlistenKeyLoggerRef.current = unlisten;
          } else {
            unlisten();
          }
        }

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
      if (hideTimerRef.current) {
        window.clearTimeout(hideTimerRef.current);
        hideTimerRef.current = null;
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
          flex items-center gap-2 transition-all duration-300 key-row
          ${isLatest ? 'scale-105 latest' : 'opacity-80'}
        `}
      >
        {hasModifiers && (
          <>
            {event.modifiers.map((modifier, modIndex) => (
              <div
                key={modIndex}
                className="keycap keycap-mod"
              >
                {modifier}
              </div>
            ))}
            <span className="text-white/60 text-md">+</span>
          </>
        )}

        <div className={`
          keycap keycap-main font-mono font-bold text-lg
          ${isLatest ? 'keycap-glow' : ''}
        `}>
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
            ${visible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-2'}
          `}
        >
          <div className="flex flex-col justify-end items-end gap-2">
            {keyEvents.slice(-9).map((event, index) => renderKeyBox(event, index))}
          </div>
        </div>
      )}
    </main>
  );
}

export default App;