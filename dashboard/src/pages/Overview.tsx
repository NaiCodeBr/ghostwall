import { useState, useEffect } from "react";
import { Activity, Wifi, Shield, AlertTriangle } from "lucide-react";

function Overview() {
  const [stats, setStats] = useState({
    devices: 0,
    exposureScore: 0,
    riskLevel: "Info",
    threats: 0,
  });

  useEffect(() => {
    // Fetch stats from API
    fetchStats();
  }, []);

  const fetchStats = async () => {
    try {
      const response = await fetch("http://localhost:8080/api/devices");
      const data = await response.json();
      setStats({
        devices: data.devices.length,
        exposureScore: 45,
        riskLevel: "Medium",
        threats: 2,
      });
    } catch (error) {
      console.error("Failed to fetch stats:", error);
    }
  };

  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold mb-8">Overview</h1>
      
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <StatCard
          icon={<Wifi className="w-8 h-8" />}
          title="Devices Detected"
          value={stats.devices}
          color="blue"
        />
        <StatCard
          icon={<Activity className="w-8 h-8" />}
          title="Exposure Score"
          value={`${stats.exposureScore}/100`}
          color="green"
        />
        <StatCard
          icon={<Shield className="w-8 h-8" />}
          title="Risk Level"
          value={stats.riskLevel}
          color="yellow"
        />
        <StatCard
          icon={<AlertTriangle className="w-8 h-8" />}
          title="Active Threats"
          value={stats.threats}
          color="red"
        />
      </div>

      <div className="bg-gray-800 rounded-lg p-6">
        <h2 className="text-xl font-semibold mb-4">Recent Activity</h2>
        <p className="text-gray-400">No recent activity to display.</p>
      </div>
    </div>
  );
}

function StatCard({ icon, title, value, color }: any) {
  const colorClasses = {
    blue: "bg-blue-500",
    green: "bg-green-500",
    yellow: "bg-yellow-500",
    red: "bg-red-500",
  };

  return (
    <div className="bg-gray-800 rounded-lg p-6">
      <div className={`inline-flex p-3 rounded-lg ${colorClasses[color]} text-white mb-4`}>
        {icon}
      </div>
      <h3 className="text-gray-400 text-sm">{title}</h3>
      <p className="text-2xl font-bold mt-1">{value}</p>
    </div>
  );
}

export default Overview;
