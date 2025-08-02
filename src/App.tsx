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

interface TaskData {
  name: string;
  data: InputEvent[];
}

function App() {
  const [taskName, setTaskName] = useState("");
  const [isListening, setIsListening] = useState(false);
  const [isMonitoring, setIsMonitoring] = useState(false);
  const [unlisten, setUnlisten] = useState<(() => void) | null>(null);
  const [taskData, setTaskData] = useState<TaskData | null>(null);
  const [eventCounts, setEventCounts] = useState<Record<string, number>>({});

  const startMonitoring = async () => {
    if (!taskName.trim()) {
      alert("Please enter a task name");
      return;
    }
    
    try {
      const response = await invoke("start_monitoring", { taskName });
      console.log("Monitoring started successfully:", response);
      setIsMonitoring(true);
      setTaskData(null);
      
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
      const data = await invoke<TaskData | null>("stop_monitoring");
      console.log("Monitoring stopped successfully:", data);
      setTaskData(data);
      setIsMonitoring(false);
      
      // Calculate event type counts
      if (data && data.data) {
        const counts: Record<string, number> = {};
        data.data.forEach(event => {
          counts[event.event_type] = (counts[event.event_type] || 0) + 1;
        });
        setEventCounts(counts);
      }
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

  // Format timestamp to a readable date/time
  const formatTimestamp = (timestamp: number): string => {
    const date = new Date(timestamp);
    return date.toLocaleString();
  };

  return (
    <main className="container">
      <h1>Task Monitoring App</h1>
      
      <div className="monitoring-controls">
        <input
          type="text"
          placeholder="Enter task name"
          value={taskName}
          onChange={(e) => setTaskName(e.target.value)}
          disabled={isMonitoring}
        />
        <br />
        {isMonitoring ? (
          <button className="stop-btn" onClick={stopMonitoring}>Stop Monitoring</button>
        ) : (
          <button className="start-btn" onClick={startMonitoring}>Start Monitoring</button>
        )}
      </div>

      {taskData && (
        <div className="results-container">
          <h2>Task Results: {taskData.name}</h2>
          
          <div className="summary">
            <h3>Summary</h3>
            <div className="event-counts">
              {Object.entries(eventCounts).map(([type, count]) => (
                <div className="event-count" key={type}>
                  <span className="event-type">{type}:</span> 
                  <span className="count">{count}</span>
                </div>
              ))}
            </div>
            <p>Total events recorded: {taskData.data.length}</p>
            {taskData.data.length > 0 && (
              <p>
                Time period: {formatTimestamp(taskData.data[0].timestamp)} to {' '}
                {formatTimestamp(taskData.data[taskData.data.length - 1].timestamp)}
              </p>
            )}
          </div>
          
          <h3>Event Details</h3>
          <div className="event-list">
            {taskData.data.length > 0 ? (
              <table className="events-table">
                <thead>
                  <tr>
                    <th>Time</th>
                    <th>Event Type</th>
                    <th>Details</th>
                    <th>Modifiers</th>
                  </tr>
                </thead>
                <tbody>
                  {taskData.data.slice(0, 100).map((event, index) => (
                    <tr key={index}>
                      <td>{formatTimestamp(event.timestamp)}</td>
                      <td>{event.event_type}</td>
                      <td>
                        {event.key && <span>Key: {event.key}</span>}
                        {event.button && <span>Button: {event.button}</span>}
                        {(event.x !== undefined && event.y !== undefined) && 
                          <span>Position: ({event.x.toFixed(0)}, {event.y.toFixed(0)})</span>
                        }
                      </td>
                      <td>{event.modifiers.join(', ')}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            ) : (
              <p>No events recorded</p>
            )}
            {taskData.data.length > 100 && (
              <p className="note">Showing first 100 events of {taskData.data.length}</p>
            )}
          </div>
        </div>
      )}
    </main>
  );
}

export default App;
