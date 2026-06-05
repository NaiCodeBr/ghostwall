import { Link, useLocation } from "react-router-dom";
import { 
  Home, 
  Wifi, 
  Map, 
  ShieldAlert, 
  Bell,
  FileText, 
  Settings 
} from "lucide-react";

const menuItems = [
  { path: "/", icon: Home, label: "Overview" },
  { path: "/devices", icon: Wifi, label: "Devices" },
  { path: "/exposure", icon: Map, label: "Exposure Map" },
  { path: "/threats", icon: ShieldAlert, label: "Threats" },
  { path: "/alerts", icon: Bell, label: "Alerts" },
  { path: "/reports", icon: FileText, label: "Reports" },
  { path: "/settings", icon: Settings, label: "Settings" },
];

function Sidebar() {
  const location = useLocation();

  return (
    <aside className="w-64 bg-gray-800 border-r border-gray-700">
      <div className="p-6">
        <h1 className="text-2xl font-bold text-green-400">GHOSTWALL</h1>
        <p className="text-sm text-gray-400 mt-1">RF Privacy Firewall</p>
      </div>
      
      <nav className="mt-6">
        {menuItems.map((item) => {
          const Icon = item.icon;
          const isActive = location.pathname === item.path;
          
          return (
            <Link
              key={item.path}
              to={item.path}
              className={`flex items-center px-6 py-3 transition-colors ${
                isActive
                  ? "bg-gray-700 text-green-400 border-r-4 border-green-400"
                  : "text-gray-300 hover:bg-gray-700 hover:text-white"
              }`}
            >
              <Icon className="w-5 h-5 mr-3" />
              {item.label}
            </Link>
          );
        })}
      </nav>
    </aside>
  );
}

export default Sidebar;
