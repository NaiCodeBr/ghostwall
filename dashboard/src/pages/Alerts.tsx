import { useState, useEffect } from "react";
import { AlertTriangle, Info, ShieldAlert, XCircle } from "lucide-react";

interface Alert {
  id: string;
  level: "info" | "low" | "medium" | "high" | "critical";
  title: string;
  message: string;
  timestamp: string;
  deviceId?: string;
}

function Alerts() {
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const [filter, setFilter] = useState<string>("all");

  useEffect(() => {
    // Mock alerts - in production, fetch from API
    setAlerts([
      {
        id: "1",
        level: "high",
        title: "Cluster de sensoriamento detectado",
        message: "Detectado cluster de 3 dispositivos ESP32 com padrões compatíveis com sensoriamento RF",
        timestamp: new Date().toISOString(),
        deviceId: "cluster-001",
      },
      {
        id: "2",
        level: "medium",
        title: "Dispositivo operando continuamente",
        message: "Dispositivo AA:BB:CC:DD:EE:FF operando por mais de 24 horas",
        timestamp: new Date(Date.now() - 3600000).toISOString(),
        deviceId: "AA:BB:CC:DD:EE:FF",
      },
      {
        id: "3",
        level: "info",
        title: "Varredura RF concluída",
        message: "Varredura detectou 5 dispositivos na rede",
        timestamp: new Date(Date.now() - 7200000).toISOString(),
      },
    ]);
  }, []);

  const filteredAlerts = filter === "all" 
    ? alerts 
    : alerts.filter(alert => alert.level === filter);

  const getIcon = (level: string) => {
    switch (level) {
      case "critical":
        return <XCircle className="w-5 h-5" />;
      case "high":
        return <AlertTriangle className="w-5 h-5" />;
      case "medium":
        return <ShieldAlert className="w-5 h-5" />;
      default:
        return <Info className="w-5 h-5" />;
    }
  };

  const getColorClass = (level: string) => {
    switch (level) {
      case "critical":
        return "bg-red-600";
      case "high":
        return "bg-orange-600";
      case "medium":
        return "bg-yellow-600";
      case "low":
        return "bg-blue-600";
      default:
        return "bg-gray-600";
    }
  };

  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold mb-8">Alerts</h1>
      
      <div className="mb-6">
        <select
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
          className="px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:border-green-500"
        >
          <option value="all">All Levels</option>
          <option value="critical">Critical</option>
          <option value="high">High</option>
          <option value="medium">Medium</option>
          <option value="low">Low</option>
          <option value="info">Info</option>
        </select>
      </div>

      <div className="space-y-4">
        {filteredAlerts.map((alert) => (
          <div
            key={alert.id}
            className="bg-gray-800 rounded-lg p-6 border-l-4"
            style={{ borderLeftColor: alert.level === "critical" ? "#dc2626" : 
                                    alert.level === "high" ? "#ea580c" :
                                    alert.level === "medium" ? "#ca8a04" :
                                    alert.level === "low" ? "#2563eb" : "#6b7280" }}
          >
            <div className="flex items-start justify-between">
              <div className="flex items-start space-x-3">
                <div className={`p-2 rounded-lg ${getColorClass(alert.level)} text-white`}>
                  {getIcon(alert.level)}
                </div>
                <div>
                  <h3 className="text-lg font-semibold">{alert.title}</h3>
                  <p className="text-gray-400 mt-1">{alert.message}</p>
                  {alert.deviceId && (
                    <p className="text-sm text-gray-500 mt-2">
                      Device: {alert.deviceId}
                    </p>
                  )}
                </div>
              </div>
              <div className="text-sm text-gray-500">
                {new Date(alert.timestamp).toLocaleString()}
              </div>
            </div>
          </div>
        ))}
      </div>

      {filteredAlerts.length === 0 && (
        <div className="text-center py-12 text-gray-400">
          No alerts found
        </div>
      )}
    </div>
  );
}

export default Alerts;
