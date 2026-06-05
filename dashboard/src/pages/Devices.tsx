import { useState, useEffect } from "react";
import { RefreshCw, Search } from "lucide-react";

interface Device {
  id: string;
  mac_address: string;
  vendor: string | null;
  ssid: string | null;
  rssi: number;
  channel: number;
  device_type: string;
}

function Devices() {
  const [devices, setDevices] = useState<Device[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchTerm, setSearchTerm] = useState("");

  useEffect(() => {
    fetchDevices();
  }, []);

  const fetchDevices = async () => {
    setLoading(true);
    try {
      const response = await fetch("http://localhost:8080/api/devices");
      const data = await response.json();
      setDevices(data.devices);
    } catch (error) {
      console.error("Failed to fetch devices:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleScan = async () => {
    setLoading(true);
    try {
      await fetch("http://localhost:8080/api/scan", { method: "POST" });
      await fetchDevices();
    } catch (error) {
      console.error("Scan failed:", error);
    } finally {
      setLoading(false);
    }
  };

  const filteredDevices = devices.filter(
    (device) =>
      device.mac_address.toLowerCase().includes(searchTerm.toLowerCase()) ||
      (device.vendor && device.vendor.toLowerCase().includes(searchTerm.toLowerCase())) ||
      (device.ssid && device.ssid.toLowerCase().includes(searchTerm.toLowerCase()))
  );

  return (
    <div className="p-8">
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-3xl font-bold">Devices</h1>
        <button
          onClick={handleScan}
          disabled={loading}
          className="flex items-center px-4 py-2 bg-green-600 hover:bg-green-700 rounded-lg transition-colors disabled:opacity-50"
        >
          <RefreshCw className={`w-5 h-5 mr-2 ${loading ? "animate-spin" : ""}`} />
          Scan
        </button>
      </div>

      <div className="mb-6">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-5 h-5" />
          <input
            type="text"
            placeholder="Search devices..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full pl-10 pr-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:border-green-500"
          />
        </div>
      </div>

      <div className="bg-gray-800 rounded-lg overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-700">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                MAC Address
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                Vendor
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                SSID
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                RSSI
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                Channel
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                Type
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-700">
            {filteredDevices.map((device) => (
              <tr key={device.id} className="hover:bg-gray-700">
                <td className="px-6 py-4 whitespace-nowrap font-mono text-sm">
                  {device.mac_address}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm">
                  {device.vendor || "Unknown"}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm">
                  {device.ssid || "N/A"}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm">
                  {device.rssi} dBm
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm">
                  {device.channel}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm">
                  <span className="px-2 py-1 bg-gray-600 rounded text-xs">
                    {device.device_type}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default Devices;
