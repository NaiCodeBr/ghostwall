import { useState, useEffect } from "react";

function Threats() {
  const [threats, setThreats] = useState<any[]>([]);

  useEffect(() => {
    fetchThreats();
  }, []);

  const fetchThreats = async () => {
    try {
      const response = await fetch("http://localhost:8080/api/threat-intel");
      const data = await response.json();
      setThreats(data.signatures);
    } catch (error) {
      console.error("Failed to fetch threats:", error);
    }
  };

  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold mb-8">Threat Intelligence</h1>
      
      <div className="bg-gray-800 rounded-lg overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-700">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                ID
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                Vendor
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                Pattern
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                Risk Score
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                Tags
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-700">
            {threats.map((threat) => (
              <tr key={threat.id} className="hover:bg-gray-700">
                <td className="px-6 py-4 whitespace-nowrap text-sm font-mono">
                  {threat.id}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm">
                  {threat.vendor}
                </td>
                <td className="px-6 py-4 text-sm">
                  {threat.fingerprint_pattern}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm">
                  <span className={`px-2 py-1 rounded text-xs ${
                    threat.risk_score >= 30 ? 'bg-red-600' : 
                    threat.risk_score >= 20 ? 'bg-yellow-600' : 'bg-green-600'
                  }`}>
                    {threat.risk_score}
                  </span>
                </td>
                <td className="px-6 py-4 text-sm">
                  {threat.tags.map((tag: string, i: number) => (
                    <span key={i} className="inline-block px-2 py-1 bg-gray-600 rounded text-xs mr-1">
                      {tag}
                    </span>
                  ))}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default Threats;
